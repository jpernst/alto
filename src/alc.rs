use std::ops::Deref;
use std::ptr;
use std::ffi::{CString, CStr};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::path::Path;
use std::marker::PhantomData;
use std::io::{self, Write};
use std::mem;
use std::env;
use std::fs;
use tempdir::TempDir;

use ::{AltoError, AltoResult};
use sys;
use al::*;
use ext;


/// Configuration hints for OpenAL-Soft.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct SoftHints {
	/// Size in sample frames of an output update period [64-8192].
	pub period_size: Option<u16>,
	/// Number of periods in output ring buffer [2-16].
	pub periods: Option<u8>,
	/// Speakers or headphones.
	pub stereo_mode: Option<SoftStereoMode>,
	/// Separation of stereo channels.
	pub cf_level: Option<SoftStereoCrossfeedLevel>,
	/// Resampler algorithm used in the internal mixer.
	pub resampler: Option<SoftResampler>,
	/// Realtime priority for the mixer thread.
	pub rt_prio: Option<bool>,
}


/// OpenAL-Soft stereo mode.
/// In headphones mode, stereo crossfeed or HRTF may be used automatically.
/// HRTF can be explicitly enabled regardless of this setting.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SoftStereoMode {
	Speakers,
	Headphones,
}


/// OpenAL-Soft stereo crossfeed modes.
/// Reduces the perceived separation between the left and right channels.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub enum SoftStereoCrossfeedLevel {
	No = 0,
	Low,
	Middle,
	High,
	LowEasy,
	MiddleEasy,
	HighEasy,
}


/// Resamplers provided by OpenAL-Soft.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SoftResampler {
	Point,
	Linear,
	Sinc4,
	Sinc8,
	BSinc,
}


/// Attributes that may be supplied during context creation.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct ContextAttrs {
	/// `ALC_FREQUENCY`
	pub frequency: Option<sys::ALCint>,
	/// `ALC_REFRESH`
	pub refresh: Option<sys::ALCint>,
	/// `ALC_MONO_SOURCES`
	pub mono_sources: Option<sys::ALCint>,
	/// `ALC_STEREO_SOURCES`
	pub stereo_sources: Option<sys::ALCint>,
	/// `ALC_HRTF_SOFT`
	/// Requires `ALC_SOFT_HRTF`
	pub soft_hrtf: Option<bool>,
	/// `ALC_HRTF_ID_SOFT`
	/// Requires `ALC_SOFT_HRTF`
	pub soft_hrtf_id: Option<sys::ALCint>,
	/// `ALC_MAX_AUXILIARY_SENDS`
	/// Requires `ALC_EXT_EFX`
	pub max_auxiliary_sends: Option<sys::ALCint>,
}


/// Attributes that may be supplied during context creation from a loopback device.
/// Requires `ALC_SOFT_loopback`
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct LoopbackAttrs {
	/// `ALC_MONO_SOURCES`
	pub mono_sources: Option<sys::ALCint>,
	/// `ALC_STEREO_SOURCES`
	pub stereo_sources: Option<sys::ALCint>,
	/// `ALC_HRTF_SOFT`
	/// Requires `ALC_SOFT_HRTF`
	pub soft_hrtf: Option<bool>,
	/// `ALC_HRTF_ID_SOFT`
	/// Requires `ALC_SOFT_HRTF`
	pub soft_hrtf_id: Option<sys::ALCint>,
	/// `ALC_MAX_AUXILIARY_SENDS`
	/// Requires `ALC_EXT_EFX`
	pub max_auxiliary_sends: Option<sys::ALCint>,
}


/// Channel format for a loopback context.
/// Requires `ALC_SOFT_loopback`
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LoopbackFormatChannels {
	/// `ALC_MONO_SOFT`
	Mono,
	/// `ALC_STEREO_SOFT`
	Stereo,
	/// `ALC_QUAD_SOFT`
	Quad,
	/// `ALC_5POINT1_SOFT`
	Mc51,
	/// `ALC_6POINT1_SOFT`
	Mc61,
	/// `ALC_7POINT1_SOFT`
	Mc71,
}


/// Sample format for a loopback context.
/// Requires `ALC_SOFT_loopback`
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LoopbackFormatType {
	/// `ALC_UNSIGNED_BYTE_SOFT`
	U8,
	/// `ALC_SHORT_SOFT`
	I16,
	/// `ALC_FLOAT_SOFT`
	F32,
}


/// The current HRTF mode of a device.
/// Requires `ALC_SOFT_HRTF`
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SoftHrtfStatus {
	/// `ALC_HRTF_DISABLED_SOFT`
	Disabled,
	/// `ALC_HRTF_ENABLED_SOFT`
	Enabled,
	/// `ALC_HRTF_DENIED_SOFT`
	Denied,
	/// `ALC_HRTF_REQUIRED_SOFT`
	Required,
	/// `ALC_HRTF_HEADPHONES_DETECTED_SOFT`
	HeadphonesDetected,
	/// `ALC_HRTF_UNSUPPORTED_FORMAT_SOFT`
	UnsupportedFormat,

	Unknown(sys::ALCint),
}


rental!{
	mod rent {
		pub rental AlApi<'rental>(Box<::sys::AlApi>, ::ext::AlcNullCache<'rental>);
	}
}

#[doc(hidden)]
pub use self::rent::AlApi;


/// This struct is the entry point of the API. Instantiating it will load an OpenAL implementation.
/// From here, available devices can be queried and opened.
pub struct Alto {
	api: AlApi<'static>,
	_hints_dir: Option<TempDir>,
}


/// Common capabilities expoed by both real and loopback devices.
pub unsafe trait DeviceTrait {
	/// Alto instance from which this device was opened.
	fn alto(&self) -> &Alto;
	/// Specifier string used to open this device.
	fn specifier(&self) -> &CStr;
	/// Raw handle as exposed by OpenAL.
	fn as_raw(&self) -> *mut sys::ALCdevice;
	/// `alcIsExtensionPresent()`
	fn is_extension_present(&self, ext::Alc) -> bool;
	#[doc(hidden)]
	fn extensions(&self) -> &ext::AlcCache;
	/// `alcGetIntegerv(ALC_CONNECTED)`
	/// Requires `ALC_EXT_disconnect`
	fn connected(&self) -> AltoResult<bool>;
	/// `alcGetStringiSOFT(ALC_HRTF_SPECIFIER_SOFT)`
	/// Requires `ALC_SOFT_HRTF`
	fn enumerate_soft_hrtfs(&self) -> AltoResult<Vec<CString>>;
	/// `alcGetIntegerv(ALC_HRTF_STATUS_SOFT)`
	/// Requires `ALC_SOFT_HRTF`
	fn soft_hrtf_status(&self) -> AltoResult<SoftHrtfStatus>;
	/// `alcGetIntegerv(ALC_MAX_AUXILIARY_SENDS)`
	/// Requires `ALC_EXT_EFX`
	fn max_auxiliary_sends(&self) -> AltoResult<sys::ALCint>;
}


/// A regular output device. This is typically a device as reported by the operating system.
pub struct Device<'a> {
	alto: &'a Alto,
	spec: CString,
	dev: *mut sys::ALCdevice,
	exts: ext::AlcCache<'a>,
	pause_rc: Arc<AtomicUsize>,
}


/// An RAII guard that keeps a device paused.
/// When this lock is dropped, the device will resume playback.
pub struct SoftPauseLock<'a: 'd, 'd>(&'d Device<'a>);


/// A sample frame that is supported as a loopback device output format.
pub unsafe trait LoopbackFrame: SampleFrame {
	fn channels(&ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint>;
	fn sample_ty(&ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint>;
}


/// A loopback device that outputs audio to a memory buffer.
/// Requires `ALC_SOFT_loopback`
pub struct LoopbackDevice<'a, F: LoopbackFrame> {
	alto: &'a Alto,
	spec: CString,
	dev: *mut sys::ALCdevice,
	exts: ext::AlcCache<'a>,
	marker: PhantomData<F>,
}


/// A capture device from which audio data can be sampled.
/// This is tyically an audio input as reported by the operating system.
pub struct CaptureDevice<'a, F: StandardFrame> {
	alto: &'a Alto,
	spec: CString,
	dev: *mut sys::ALCdevice,
	marker: PhantomData<F>,
}


impl Alto {
	/// Load the default OpenAL implementation for the platform.
	/// This will prefer OpenAL-Soft if it is present, otherwise it will search for a generic implementation.
	pub fn load_default() -> AltoResult<Alto> {
		let api = Box::new(sys::AlApi::load_default()?);
		Ok(Alto{
			api: AlApi::new(api, |a| unsafe { ext::AlcNullCache::new(a, ptr::null_mut()) }),
			_hints_dir: None,
		}).and_then(|a| a.check_version())
	}


	/// Load the default OpenAL implementation for the platform.
	/// This will prefer OpenAL-Soft if it is present, and will pass along the specified hints in a temporary
	/// configuration file that will be read by OpenAL-Soft.
	/// NOTE: This involves setting the value of the `ALSOFT_CONF` environment variable. If it is already set,
	/// then the provided hints will be discarded.
	pub fn load_default_with_soft_hints(hints: SoftHints) -> AltoResult<Alto> {
		let api = Box::new(sys::AlApi::load_default()?);
		Ok(Alto{
			api: AlApi::new(api, |a| unsafe { ext::AlcNullCache::new(a, ptr::null_mut()) }),
			_hints_dir: Alto::set_hints(hints),
		}).and_then(|a| a.check_version())
	}


	/// Loads a specific OpenAL implementation from a specififed path.
	pub fn load<P: AsRef<Path>>(path: P) -> AltoResult<Alto> {
		let api = Box::new(sys::AlApi::load(path)?);
		Ok(Alto{
			api: AlApi::new(api, |a| unsafe { ext::AlcNullCache::new(a, ptr::null_mut()) }),
			_hints_dir: None,
		}).and_then(|a| a.check_version())
	}


	/// Loads a specific OpenAL implementation from a specififed path.
	/// If that implementation is OpenAL-Soft, the specified hints will be passed along in a temporary
	/// configuration file.
	/// NOTE: This involves setting the value of the `ALSOFT_CONF` environment variable. If it is already set,
	/// then the provided hints will be discarded.
	pub fn load_with_soft_hints<P: AsRef<Path>>(path: P, hints: SoftHints) -> AltoResult<Alto> {
		let api = Box::new(sys::AlApi::load(path)?);
		Ok(Alto{
			api: AlApi::new(api, |a| unsafe { ext::AlcNullCache::new(a, ptr::null_mut()) }),
			_hints_dir: Alto::set_hints(hints),
		}).and_then(|a| a.check_version())
	}


	fn set_hints(hints: SoftHints) -> Option<TempDir> {
		if hints.period_size.is_none()
			&& hints.periods.is_none()
			&& hints.stereo_mode.is_none()
			&& hints.cf_level.is_none()
			&& hints.resampler.is_none()
			&& hints.rt_prio.is_none()
		{
			return None;
		}

		if env::var_os("ALSOFT_CONF").is_none() {
			if let Ok(hints_dir) = TempDir::new("alto") {
				let hints_path = hints_dir.path().join("alsoft.conf");
				if let Ok(mut config) = fs::File::create(&hints_path) {
					let _ = writeln!(config, "[general]");
					if let Some(period_size) = hints.period_size {
						let _ = writeln!(config, "period_size = {}", period_size);
					}
					if let Some(periods) = hints.periods {
						let _ = writeln!(config, "periods = {}", periods);
					}
					if let Some(stereo_mode) = hints.stereo_mode {
						let _ = writeln!(config, "stereo-mode = {}", match stereo_mode { SoftStereoMode::Speakers => "speakers", SoftStereoMode::Headphones => "headphones" });
					}
					if let Some(cf_level) = hints.cf_level {
						let _ = writeln!(config, "cf_level = {}", cf_level as u8);
					}
					if let Some(resampler) = hints.resampler {
						let _ = writeln!(config, "resampler = {}", match resampler {
							SoftResampler::Point => "point",
							SoftResampler::Linear => "linear",
							SoftResampler::Sinc4 => "sinc4",
							SoftResampler::Sinc8 => "sinc8",
							SoftResampler::BSinc => "bsinc",
						});
					}
					if let Some(rt_prio) = hints.rt_prio {
						let _ = writeln!(config, "rt-prio = {}", if rt_prio { 1 } else { 0 });
					}

					env::set_var("ALSOFT_CONF", &hints_path);
					Some(hints_dir)
				} else {
					None
				}
			} else {
				None
			}
		} else {
			None
		}
	}


	fn check_version(self) -> AltoResult<Alto> {
		let mut major = 0;
		unsafe { self.api.owner().alcGetIntegerv()(ptr::null_mut(), sys::ALC_MAJOR_VERSION, 1, &mut major); }
		let mut minor = 0;
		unsafe { self.api.owner().alcGetIntegerv()(ptr::null_mut(), sys::ALC_MINOR_VERSION, 1, &mut minor); }

		if major == 1 && minor >= 1 {
			Ok(self)
		} else {
			Err(AltoError::AlcUnsupportedVersion)
		}
	}


	/// Raw entry points of the OpenAL API.
	pub fn raw_api(&self) -> &AlApi { &self.api }


	/// `alcGetString(ALC_DEFAULT_DEVICE_SPECIFIER)`
	pub fn default_output(&self) -> AltoResult<CString> {
		self.api.rent(|exts| {
			let spec = if let Ok(ea) = exts.ALC_ENUMERATE_ALL_EXT() {
				unsafe { CStr::from_ptr(self.api.owner().alcGetString()(ptr::null_mut(), ea.ALC_DEFAULT_ALL_DEVICES_SPECIFIER?)) }
			} else {
				unsafe { CStr::from_ptr(self.api.owner().alcGetString()(ptr::null_mut(), sys::ALC_DEFAULT_DEVICE_SPECIFIER)) }
			};
			self.get_error(ptr::null_mut()).map(|_| spec.to_owned())
		})
	}


	/// `alcGetString(ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER)`
	pub fn default_capture(&self) -> AltoResult<CString> {
		let spec = unsafe { CStr::from_ptr(self.api.owner().alcGetString()(ptr::null_mut(), sys::ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER)) };
		self.get_error(ptr::null_mut()).map(|_| spec.to_owned())
	}


	/// `alcGetString(ALC_DEVICE_SPECIFIER)`
	pub fn enumerate_outputs(&self) -> AltoResult<Vec<CString>> {
		self.api.rent(|exts| {
			let spec = if let Ok(ea) = exts.ALC_ENUMERATE_ALL_EXT() {
				unsafe { self.api.owner().alcGetString()(ptr::null_mut(), ea.ALC_ALL_DEVICES_SPECIFIER?) }
			} else {
				unsafe { self.api.owner().alcGetString()(ptr::null_mut(), sys::ALC_DEVICE_SPECIFIER) }
			};
			self.get_error(ptr::null_mut()).and_then(|_| Alto::parse_enum_spec(spec as *const u8))
		})
	}


	/// `alcGetString(ALC_CAPTURE_DEVICE_SPECIFIER)`
	pub fn enumerate_captures(&self) -> AltoResult<Vec<CString>> {
		let spec = unsafe { self.api.owner().alcGetString()(ptr::null_mut(), sys::ALC_CAPTURE_DEVICE_SPECIFIER) };
		self.get_error(ptr::null_mut()).and_then(|_| Alto::parse_enum_spec(spec as *const u8))
	}


	fn parse_enum_spec(spec: *const u8) -> AltoResult<Vec<CString>> {
		let mut specs = Vec::with_capacity(0);

		let mut i = 0;
		loop {
			if unsafe { ptr::read(spec.offset(i)) == 0 && ptr::read(spec.offset(i + 1)) == 0 } {
				break;
			}

			i += 1;
		}

		specs.extend(unsafe { ::std::slice::from_raw_parts(spec as *const u8, i as usize) }.split(|c| *c == 0).map(|d| CString::new(d).unwrap()));

		Ok(specs)
	}


	/// `alcOpenDevice()`
	pub fn open<'s, S: Into<Option<&'s CStr>>>(&self, spec: S) -> AltoResult<Device> {
		let spec = if let Some(spec) = spec.into() {
			spec.to_owned()
		} else {
			self.default_output()?
		};

		let dev = unsafe { self.api.owner().alcOpenDevice()(spec.as_ptr()) };
		self.get_error(ptr::null_mut())?;

		if dev == ptr::null_mut() {
			Err(AltoError::AlcInvalidDevice)
		} else {
			Ok(Device{
				alto: self,
				spec: spec,
				dev: dev,
				exts: unsafe { ext::AlcCache::new(self.api.owner(), dev) },
				pause_rc: Arc::new(AtomicUsize::new(0))
			})
		}
	}


	/// `alcLoopbackOpenDeviceSOFT()`
	/// Requires `ALC_SOFT_loopback`
	pub fn open_loopback<'s, S: Into<Option<&'s CStr>>, F: LoopbackFrame>(&self, spec: S) -> AltoResult<LoopbackDevice<F>> {
		self.api.rent(|exts| {
			let sl = exts.ALC_SOFT_loopback()?;

			let spec = if let Some(spec) = spec.into() {
				spec.to_owned()
			} else {
				self.default_output()?
			};

			let dev = unsafe { sl.alcLoopbackOpenDeviceSOFT?(spec.as_ptr()) };
			self.get_error(ptr::null_mut())?;

			if dev == ptr::null_mut() {
				Err(AltoError::AlcInvalidDevice)
			} else {
				Ok(LoopbackDevice{
					alto: self,
					spec: spec,
					dev: dev,
					exts: unsafe { ext::AlcCache::new(self.api.owner(), dev) },
					marker: PhantomData
				})
			}
		})
	}


	/// `alcCaptureOpenDevice()`
	pub fn open_capture<'s, S: Into<Option<&'s CStr>>, F: StandardFrame>(&self, spec: S, freq: sys::ALCuint, len: sys::ALCsizei) -> AltoResult<CaptureDevice<F>> {
		let spec = if let Some(spec) = spec.into() {
			spec.to_owned()
		} else {
			self.default_output()?
		};

		let dev = unsafe { self.api.owner().alcCaptureOpenDevice()(spec.as_ptr(), freq, F::format().into_raw(None)?, len) };
		self.get_error(ptr::null_mut())?;

		if dev == ptr::null_mut() {
			Err(AltoError::AlcInvalidDevice)
		} else {
			Ok(CaptureDevice{alto: self, spec: spec, dev: dev, marker: PhantomData})
		}
	}


	#[doc(hidden)]
	pub fn get_error(&self, dev: *mut sys::ALCdevice) -> AltoResult<()> {
		match unsafe { self.api.owner().alcGetError()(dev)} {
			sys::ALC_NO_ERROR => Ok(()),
			e => Err(AltoError::from_alc(e)),
		}
	}
}


impl<'a> PartialEq for (DeviceTrait + 'a) {
	fn eq(&self, other: &(DeviceTrait + 'a)) -> bool {
		self.as_raw() == other.as_raw()
	}
}
impl Eq for DeviceTrait { }


impl<'a> Device<'a> {
	fn make_attrs_vec(&self, attrs: Option<ContextAttrs>) -> AltoResult<Vec<sys::ALCint>> {
		let mut attrs_vec = Vec::with_capacity(15);
		if let Some(attrs) = attrs {
			if let Some(freq) = attrs.frequency {
				attrs_vec.extend(&[sys::ALC_FREQUENCY, freq]);
			}
			if let Some(refresh) = attrs.refresh {
				attrs_vec.extend(&[sys::ALC_REFRESH, refresh]);
			}
			if let Some(mono) = attrs.mono_sources {
				attrs_vec.extend(&[sys::ALC_MONO_SOURCES, mono]);
			}
			if let Some(stereo) = attrs.stereo_sources {
				attrs_vec.extend(&[sys::ALC_STEREO_SOURCES, stereo]);
			}

			if let Ok(ash) = self.exts.ALC_SOFT_HRTF() {
				if let Some(hrtf) = attrs.soft_hrtf {
					attrs_vec.extend(&[ash.ALC_HRTF_SOFT?, if hrtf { sys::ALC_TRUE } else { sys::ALC_FALSE } as sys::ALCint]);
				}
				if let Some(hrtf_id) = attrs.soft_hrtf_id {
					attrs_vec.extend(&[ash.ALC_HRTF_ID_SOFT?, hrtf_id]);
				}
			}

			if let Ok(efx) = self.exts.ALC_EXT_EFX() {
				if let Some(max_sends) = attrs.max_auxiliary_sends {
					attrs_vec.extend(&[efx.ALC_MAX_AUXILIARY_SENDS?, max_sends]);
				}
			}

			attrs_vec.push(0);
		};
		Ok(attrs_vec)
	}


	/// `alcCreateContext()`
	pub fn new_context<A: Into<Option<ContextAttrs>>>(&self, attrs: A) -> AltoResult<Context> {
		let attrs_vec = self.make_attrs_vec(attrs.into());

		let ctx = unsafe { self.alto.api.owner().alcCreateContext()(self.dev, attrs_vec.map(|a| a.as_slice().as_ptr()).unwrap_or(ptr::null())) };
		self.alto.get_error(self.dev).map(|_| unsafe { Context::new(self, &self.alto.api, ctx) })
	}


	/// `alcDevicePauseSOFT()`
	/// Requires `ALC_SOFT_pause_device`
	pub fn soft_pause<'d>(&'d self) -> AltoResult<SoftPauseLock<'a, 'd>> {
		SoftPauseLock::new(self)
	}


	/// `alcDevicePauseSOFT()`
	/// Requires `ALC_SOFT_HRTF`
	pub fn soft_reset<A: Into<Option<ContextAttrs>>>(&self, attrs: A) -> AltoResult<()> {
		let ards = self.exts.ALC_SOFT_HRTF()?.alcResetDeviceSOFT?;
		let attrs_vec = self.make_attrs_vec(attrs.into());
		unsafe { ards(self.dev, attrs_vec.map(|a| a.as_slice().as_ptr()).unwrap_or(ptr::null())) };
		self.alto.get_error(self.dev)
	}
}


unsafe impl<'a> DeviceTrait for Device<'a> {
	#[inline]
	fn alto(&self) -> &Alto { &self.alto }
	#[inline]
	fn specifier(&self) -> &CStr { &self.spec }
	#[inline]
	fn as_raw(&self) -> *mut sys::ALCdevice { self.dev }


	fn is_extension_present(&self, ext: ext::Alc) -> bool {
		match ext {
			ext::Alc::Dedicated => self.exts.ALC_EXT_DEDICATED().is_ok(),
			ext::Alc::Disconnect => self.exts.ALC_EXT_DISCONNECT().is_ok(),
			ext::Alc::Efx => self.exts.ALC_EXT_EFX().is_ok(),
			ext::Alc::SoftHrtf => self.exts.ALC_SOFT_HRTF().is_ok(),
			ext::Alc::SoftPauseDevice => self.exts.ALC_SOFT_pause_device().is_ok(),
		}
	}


	fn extensions(&self) -> &ext::AlcCache { &self.exts }


	fn connected(&self) -> AltoResult<bool> {
		let mut value = 0;
		unsafe { self.alto.api.owner().alcGetIntegerv()(self.dev, self.exts.ALC_EXT_DISCONNECT()?.ALC_CONNECTED?, 1, &mut value); }
		self.alto.get_error(self.dev).map(|_| value == sys::ALC_TRUE as sys::ALCint)
	}


	fn enumerate_soft_hrtfs(&self) -> AltoResult<Vec<CString>> {
		let ash = self.exts.ALC_SOFT_HRTF()?;

		let mut value = 0;
		unsafe { self.alto.api.owner().alcGetIntegerv()(self.dev, ash.ALC_NUM_HRTF_SPECIFIERS_SOFT?, 1, &mut value); }
		self.alto.get_error(self.dev)?;

		let mut spec_vec = Vec::new();
		for i in 0 .. value {
			unsafe {
				let spec = ash.alcGetStringiSOFT?(self.dev, ash.ALC_HRTF_SPECIFIER_SOFT?, i) as *mut _;
				spec_vec.push(self.alto.get_error(self.dev).map(|_| CString::from_raw(spec))?);
			}
		}
		Ok(spec_vec)
	}


	fn soft_hrtf_status(&self) -> AltoResult<SoftHrtfStatus> {
		let ash = self.exts.ALC_SOFT_HRTF()?;

		let mut value = 0;
		unsafe { self.alto.api.owner().alcGetIntegerv()(self.dev, ash.ALC_HRTF_STATUS_SOFT?, 1, &mut value); }
		self.alto.get_error(self.dev).and_then(|_| match value {
			s if s == ash.ALC_HRTF_DISABLED_SOFT? => Ok(SoftHrtfStatus::Disabled),
			s if s == ash.ALC_HRTF_ENABLED_SOFT? => Ok(SoftHrtfStatus::Enabled),
			s if s == ash.ALC_HRTF_DENIED_SOFT? => Ok(SoftHrtfStatus::Denied),
			s if s == ash.ALC_HRTF_REQUIRED_SOFT? => Ok(SoftHrtfStatus::Required),
			s if s == ash.ALC_HRTF_HEADPHONES_DETECTED_SOFT? => Ok(SoftHrtfStatus::HeadphonesDetected),
			s if s == ash.ALC_HRTF_UNSUPPORTED_FORMAT_SOFT? => Ok(SoftHrtfStatus::UnsupportedFormat),
			s => Ok(SoftHrtfStatus::Unknown(s)),
		})
	}


	fn max_auxiliary_sends(&self) -> AltoResult<sys::ALCint> {
		let mut value = 0;
		unsafe { self.alto.api.owner().alcGetIntegerv()(self.dev, self.exts.ALC_EXT_EFX()?.ALC_MAX_AUXILIARY_SENDS?, 1, &mut value); }
		self.alto.get_error(self.dev).map(|_| value)
	}
}


impl<'a> PartialEq for Device<'a> {
	fn eq(&self, other: &Device<'a>) -> bool {
		self.dev == other.dev
	}
}
impl<'a> Eq for Device<'a> { }


impl<'a> Drop for Device<'a> {
	fn drop(&mut self) {
		unsafe { self.alto.api.owner().alcCloseDevice()(self.dev); }
		if let Err(_) = self.alto.get_error(self.dev) {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcCloseDevice` failed in Device drop");
		}
	}
}


unsafe impl<'a> Send for Device<'a> { }
unsafe impl<'a> Sync for Device<'a> { }


impl<'a: 'd, 'd> SoftPauseLock<'a, 'd> {
	fn new(dev: &'d Device<'a>) -> AltoResult<SoftPauseLock<'a, 'd>> {
		let adps = dev.exts.ALC_SOFT_pause_device()?.alcDevicePauseSOFT?;

		let old = dev.pause_rc.fetch_add(1, Ordering::SeqCst);
		if old == 0 {
			unsafe { adps(dev.dev) }
			if let Err(e) = dev.alto.get_error(dev.dev) {
				dev.pause_rc.fetch_sub(1, Ordering::SeqCst);
				return Err(e);
			}
		}

		Ok(SoftPauseLock(dev))
	}
}


impl<'a: 'd, 'd> Deref for SoftPauseLock<'a, 'd> {
	type Target = Device<'a>;

	fn deref(&self) -> &Device<'a> { self.0 }
}


impl<'a: 'd, 'd> Drop for SoftPauseLock<'a, 'd> {
	fn drop(&mut self) {
		let old = self.0.pause_rc.fetch_sub(1, Ordering::SeqCst);
		if old == 1 {
			unsafe { self.0.exts.ALC_SOFT_pause_device().unwrap().alcDeviceResumeSOFT.unwrap()(self.0.dev); }
			if let Err(_) = self.0.alto.get_error(self.0.dev) {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alcDeviceResumeSOFT` failed in SoftPauseLock drop");
			}
		}
	}
}


impl<'a, F: LoopbackFrame> LoopbackDevice<'a, F> {
	fn make_attrs_vec<A: Into<Option<LoopbackAttrs>>>(&self, freq: sys::ALCint, attrs: A) -> AltoResult<Vec<sys::ALCint>> {
		self.alto.api.rent(move|exts| {
			let asl = exts.ALC_SOFT_loopback()?;

			let mut attrs_vec = Vec::with_capacity(17);
			attrs_vec.extend(&[sys::ALC_FREQUENCY, freq]);
			attrs_vec.extend(&[asl.ALC_FORMAT_CHANNELS_SOFT?, F::channels(&asl)?]);
			attrs_vec.extend(&[asl.ALC_FORMAT_TYPE_SOFT?, F::sample_ty(&asl)?]);
			if let Some(attrs) = attrs.into() {
				if let Some(mono) = attrs.mono_sources {
					attrs_vec.extend(&[sys::ALC_MONO_SOURCES, mono]);
				}
				if let Some(stereo) = attrs.stereo_sources {
					attrs_vec.extend(&[sys::ALC_STEREO_SOURCES, stereo]);
				}

				if let Ok(ash) = self.exts.ALC_SOFT_HRTF() {
					if let Some(hrtf) = attrs.soft_hrtf {
						attrs_vec.extend(&[ash.ALC_HRTF_SOFT?, if hrtf { sys::ALC_TRUE } else { sys::ALC_FALSE } as sys::ALCint]);
					}
					if let Some(hrtf_id) = attrs.soft_hrtf_id {
						attrs_vec.extend(&[ash.ALC_HRTF_ID_SOFT?, hrtf_id]);
					}
				}

				if let Ok(efx) = self.exts.ALC_EXT_EFX() {
					if let Some(max_sends) = attrs.max_auxiliary_sends {
						attrs_vec.extend(&[efx.ALC_MAX_AUXILIARY_SENDS?, max_sends]);
					}
				}
			}
			attrs_vec.push(0);
			Ok(attrs_vec)
		})
	}


	/// `alcCreateContext()`
	pub fn new_context<A: Into<Option<LoopbackAttrs>>>(&self, freq: sys::ALCint, attrs: A) -> AltoResult<Context> {
		let attrs_vec = self.make_attrs_vec(freq, attrs.into())?;
		let ctx = unsafe { self.alto.api.owner().alcCreateContext()(self.dev, attrs_vec.as_slice().as_ptr()) };
		self.alto.get_error(self.dev).map(|_| unsafe { Context::new(self, &self.alto.api, ctx) })
	}


	/// `alcRenderSamplesSOFT()`
	pub fn soft_render_samples<R: AsBufferDataMut<F>>(&mut self, mut data: R) -> AltoResult<()> {
		let mut data = data.as_buffer_data_mut();
		if sys::ALCsizei::max_value() as usize / mem::size_of::<F>() < data.len() { return Err(AltoError::AlcInvalidValue) }

		self.alto.api.rent(move|exts| {
			let asl = exts.ALC_SOFT_loopback()?;

			unsafe { asl.alcRenderSamplesSOFT?(self.dev, data.as_mut_ptr() as *mut _, data.len() as sys::ALCsizei); }
			self.alto.get_error(self.dev)
		})
	}


	/// `alcDevicePauseSOFT()`
	/// Requires `ALC_SOFT_HRTF`
	pub fn soft_reset<A: Into<Option<LoopbackAttrs>>>(&self, freq: sys::ALCint, attrs: A) -> AltoResult<()> {
		let ards = self.exts.ALC_SOFT_HRTF()?.alcResetDeviceSOFT?;

		let attrs_vec = self.make_attrs_vec(freq, attrs.into());
		unsafe { ards(self.dev, attrs_vec.map(|a| a.as_slice().as_ptr()).unwrap_or(ptr::null())) };
		self.alto.get_error(self.dev)
	}
}


unsafe impl<'a, F: LoopbackFrame> DeviceTrait for LoopbackDevice<'a, F> {
	#[inline]
	fn alto(&self) -> &Alto { &self.alto }
	#[inline]
	fn specifier(&self) -> &CStr { &self.spec }
	#[inline]
	fn as_raw(&self) -> *mut sys::ALCdevice { self.dev }
	#[inline]
	fn connected(&self) -> AltoResult<bool> { Ok(true) }


	fn is_extension_present(&self, ext: ext::Alc) -> bool {
		match ext {
			ext::Alc::Dedicated => self.exts.ALC_EXT_DEDICATED().is_ok(),
			ext::Alc::Disconnect => self.exts.ALC_EXT_DISCONNECT().is_ok(),
			ext::Alc::Efx => self.exts.ALC_EXT_EFX().is_ok(),
			ext::Alc::SoftHrtf => self.exts.ALC_SOFT_HRTF().is_ok(),
			ext::Alc::SoftPauseDevice => self.exts.ALC_SOFT_pause_device().is_ok(),
		}
	}


	fn extensions(&self) -> &ext::AlcCache { &self.exts }


	fn enumerate_soft_hrtfs(&self) -> AltoResult<Vec<CString>> {
		let ash = self.exts.ALC_SOFT_HRTF()?;

		let mut value = 0;
		unsafe { self.alto.api.owner().alcGetIntegerv()(self.dev, ash.ALC_NUM_HRTF_SPECIFIERS_SOFT?, 1, &mut value); }
		self.alto.get_error(self.dev)?;

		let mut spec_vec = Vec::new();
		for i in 0 .. value {
			unsafe {
				let spec = ash.alcGetStringiSOFT?(self.dev, ash.ALC_HRTF_SPECIFIER_SOFT?, i) as *mut _;
				spec_vec.push(self.alto.get_error(self.dev).map(|_| CString::from_raw(spec))?);
			}
		}
		Ok(spec_vec)
	}


	fn soft_hrtf_status(&self) -> AltoResult<SoftHrtfStatus> {
		let ash = self.exts.ALC_SOFT_HRTF()?;

		let mut value = 0;
		unsafe { self.alto.api.owner().alcGetIntegerv()(self.dev, ash.ALC_HRTF_STATUS_SOFT?, 1, &mut value); }
		self.alto.get_error(self.dev).and_then(|_| match value {
			s if s == ash.ALC_HRTF_DISABLED_SOFT? => Ok(SoftHrtfStatus::Disabled),
			s if s == ash.ALC_HRTF_ENABLED_SOFT? => Ok(SoftHrtfStatus::Enabled),
			s if s == ash.ALC_HRTF_DENIED_SOFT? => Ok(SoftHrtfStatus::Denied),
			s if s == ash.ALC_HRTF_REQUIRED_SOFT? => Ok(SoftHrtfStatus::Required),
			s if s == ash.ALC_HRTF_HEADPHONES_DETECTED_SOFT? => Ok(SoftHrtfStatus::HeadphonesDetected),
			s if s == ash.ALC_HRTF_UNSUPPORTED_FORMAT_SOFT? => Ok(SoftHrtfStatus::UnsupportedFormat),
			s => Ok(SoftHrtfStatus::Unknown(s)),
		})
	}


	fn max_auxiliary_sends(&self) -> AltoResult<sys::ALCint> {
		let mut value = 0;
		unsafe { self.alto.api.owner().alcGetIntegerv()(self.dev, self.exts.ALC_EXT_EFX()?.ALC_MAX_AUXILIARY_SENDS?, 1, &mut value); }
		self.alto.get_error(self.dev).map(|_| value)
	}
}


impl<'a, F: LoopbackFrame> PartialEq for LoopbackDevice<'a, F> {
	fn eq(&self, other: &LoopbackDevice<'a, F>) -> bool {
		self.dev == other.dev
	}
}
impl<'a, F: LoopbackFrame> Eq for LoopbackDevice<'a, F> { }


impl<'a, F: LoopbackFrame> Drop for LoopbackDevice<'a, F> {
	fn drop(&mut self) {
		unsafe { self.alto.api.owner().alcCloseDevice()(self.dev); }
		if let Err(_) = self.alto.get_error(self.dev) {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcCloseDevice` failed in LoopbackDevice drop");
		}
	}
}


unsafe impl<'a, F: LoopbackFrame> Send for LoopbackDevice<'a, F> { }
unsafe impl<'a, F: LoopbackFrame> Sync for LoopbackDevice<'a, F> { }


impl<'a, F: StandardFrame> CaptureDevice<'a, F> {
	/// Alto struct from which this device was opened.
	#[inline]
	pub fn alto(&self) -> &Alto { &self.alto }
	/// Specifier used to open this device.
	#[inline]
	pub fn specifier(&self) -> &CStr { &self.spec }
	/// Raw device handle as reported by OpenAL.
	#[inline]
	pub fn as_raw(&self) -> *mut sys::ALCdevice { self.dev }


	/// `alcCaptureStart()`
	pub fn start(&mut self) -> AltoResult<()> {
		unsafe { self.alto.api.owner().alcCaptureStart()(self.dev); }
		self.alto.get_error(self.dev)
	}


	/// `alcCaptureStop()`
	pub fn stop(&mut self) -> AltoResult<()> {
		unsafe { self.alto.api.owner().alcCaptureStop()(self.dev); }
		self.alto.get_error(self.dev)
	}


	/// `alcGetIntegerv(ALC_CAPTURE_SAMPLES)`
	pub fn samples_len(&self) -> AltoResult<sys::ALCint> {
		let mut samples = 0;
		unsafe { self.alto.api.owner().alcGetIntegerv()(self.dev, sys::ALC_CAPTURE_SAMPLES, 1, &mut samples); }
		self.alto.get_error(self.dev).map(|_| samples)
	}


	/// `alcCaptureSamples()`
	pub fn capture_samples<R: AsBufferDataMut<F>>(&mut self, mut data: R) -> AltoResult<()> {
		let mut data = data.as_buffer_data_mut();
		if data.len() > self.samples_len()? as usize { return Err(AltoError::AlcInvalidValue) }

		unsafe { self.alto.api.owner().alcCaptureSamples()(self.dev, data.as_mut_ptr() as *mut _, data.len() as sys::ALCsizei); }
		self.alto.get_error(self.dev)
	}
}


impl<'a, F: StandardFrame> PartialEq for CaptureDevice<'a, F> {
	fn eq(&self, other: &CaptureDevice<'a, F>) -> bool {
		self.dev == other.dev
	}
}
impl<'a, F: StandardFrame> Eq for CaptureDevice<'a, F> { }


unsafe impl<'a, F: StandardFrame> Send for CaptureDevice<'a, F> { }
