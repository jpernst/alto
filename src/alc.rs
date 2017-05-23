use std::any::Any;
use std::ptr;
use std::ffi::{CString, CStr};
use std::sync::Arc;
use std::path::Path;
use std::marker::PhantomData;
use std::io::Write;
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
	pub max_aux_sends: Option<sys::ALCint>,
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
	pub max_aux_sends: Option<sys::ALCint>,
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


pub(crate) struct AltoInner {
	pub(crate) api: sys::AlApi,
	pub(crate) exts: ::ext::AlcNullCache,
	_hints_dir: Option<TempDir>,
}


/// This struct is the entry point of the API. Instantiating it will load an OpenAL implementation.
/// From here, available devices can be queried and opened.
pub struct Alto(pub(crate) Arc<AltoInner>);


/// Common capabilities expoed by both real and loopback devices.
pub unsafe trait DeviceObject: Any {
	/// AltoInner instance from which this device was opened.
	fn alto(&self) -> &Alto;
	/// Specifier string used to open this device.
	fn specifier(&self) -> Option<&CStr>;
	/// Raw handle as exposed by OpenAL.
	fn as_raw(&self) -> *mut sys::ALCdevice;
	/// `alcIsExtensionPresent()`
	fn is_extension_present(&self, ext::Alc) -> bool;
	/// `alcGetIntegerv(ALC_CONNECTED)`
	/// Requires `ALC_EXT_disconnect`
	fn connected(&self) -> AltoResult<bool>;
	/// `alcGetStringiSOFT(ALC_HRTF_SPECIFIER_SOFT)`
	/// Requires `ALC_SOFT_HRTF`
	fn enumerate_soft_hrtfs(&self) -> Vec<CString>;
	/// `alcGetIntegerv(ALC_HRTF_STATUS_SOFT)`
	/// Requires `ALC_SOFT_HRTF`
	fn soft_hrtf_status(&self) -> AltoResult<SoftHrtfStatus>;
	/// `alcGetIntegerv(ALC_MAX_AUXILIARY_SENDS)`
	/// Requires `ALC_EXT_EFX`
	fn max_aux_sends(&self) -> sys::ALCint;
	/// Return a new handle to this device.
	fn to_device(&self) -> Device;
}


pub(crate) struct DeviceInner {
	pub(crate) alto: Alto,
	spec: Option<CString>,
	pub(crate) dev: *mut sys::ALCdevice,
	pub(crate) exts: ext::AlcCache,
}


/// A regular output device. This is typically a device as reported by the operating system.
pub struct OutputDevice(pub(crate) Arc<DeviceInner>);


/// A sample frame that is supported as a loopback device output format.
pub unsafe trait LoopbackFrame: SampleFrame {
	fn channels(&ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint>;
	fn sample_ty(&ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint>;
}


/// A loopback device that outputs audio to a memory buffer.
/// Requires `ALC_SOFT_loopback`
pub struct LoopbackDevice<F: LoopbackFrame>(pub(crate) Arc<DeviceInner>, pub(crate) PhantomData<F>);


/// A handle to any kind of output device.
pub struct Device(pub(crate) Arc<DeviceInner>);


/// A capture device from which audio data can be sampled.
/// This is tyically an audio input as reported by the operating system.
pub struct Capture<F: StandardFrame> {
	alto: Alto,
	spec: Option<CString>,
	dev: *mut sys::ALCdevice,
	marker: PhantomData<F>,
}


impl Alto {
	/// Load the default OpenAL implementation for the platform.
	/// This will prefer OpenAL-Soft if it is present, otherwise it will search for a generic implementation.
	pub fn load_default() -> AltoResult<Alto> {
		let api = sys::AlApi::load_default()?;
		let exts = unsafe { ext::AlcNullCache::new(&api, ptr::null_mut()) };
		Ok(Alto(Arc::new(AltoInner{
			api: api,
			exts: exts,
			_hints_dir: None,
		}))).and_then(|a| a.check_version())
	}


	/// Load the default OpenAL implementation for the platform.
	/// This will prefer OpenAL-Soft if it is present, and will pass along the specified hints in a temporary
	/// configuration file that will be read by OpenAL-Soft.
	/// NOTE: This involves setting the value of the `ALSOFT_CONF` environment variable. If it is already set,
	/// then the provided hints will be discarded.
	pub fn load_default_with_soft_hints(hints: SoftHints) -> AltoResult<Alto> {
		let api = sys::AlApi::load_default()?;
		let exts = unsafe { ext::AlcNullCache::new(&api, ptr::null_mut()) };
		Ok(Alto(Arc::new(AltoInner{
			api: api,
			exts: exts,
			_hints_dir: Alto::set_hints(hints),
		}))).and_then(|a| a.check_version())
	}


	/// Loads a specific OpenAL implementation from a specififed path.
	pub fn load<P: AsRef<Path>>(path: P) -> AltoResult<Alto> {
		let api = sys::AlApi::load(path)?;
		let exts = unsafe { ext::AlcNullCache::new(&api, ptr::null_mut()) };
		Ok(Alto(Arc::new(AltoInner{
			api: api,
			exts: exts,
			_hints_dir: None,
		}))).and_then(|a| a.check_version())
	}


	/// Loads a specific OpenAL implementation from a specififed path.
	/// If that implementation is OpenAL-Soft, the specified hints will be passed along in a temporary
	/// configuration file.
	/// NOTE: This involves setting the value of the `ALSOFT_CONF` environment variable. If it is already set,
	/// then the provided hints will be discarded.
	pub fn load_with_soft_hints<P: AsRef<Path>>(path: P, hints: SoftHints) -> AltoResult<Alto> {
		let api = sys::AlApi::load(path)?;
		let exts = unsafe { ext::AlcNullCache::new(&api, ptr::null_mut()) };
		Ok(Alto(Arc::new(AltoInner{
			api: api,
			exts: exts,
			_hints_dir: Alto::set_hints(hints),
		}))).and_then(|a| a.check_version())
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

		env::var_os("ALSOFT_CONF").and_then(|_| {
			(|| -> ::std::io::Result<_> {
				let hints_dir = TempDir::new("alto")?;
				let hints_path = hints_dir.path().join("alsoft.conf");
				let mut config = fs::File::create(&hints_path)?;

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
				Ok(hints_dir)
			})().ok()
		})
	}


	fn check_version(self) -> AltoResult<Alto> {
		let mut major = 0;
		unsafe { self.0.api.alcGetIntegerv(ptr::null_mut(), sys::ALC_MAJOR_VERSION, 1, &mut major); }
		let mut minor = 0;
		unsafe { self.0.api.alcGetIntegerv(ptr::null_mut(), sys::ALC_MINOR_VERSION, 1, &mut minor); }

		if major == 1 && minor >= 1 {
			Ok(self)
		} else {
			Err(AltoError::AlcUnsupportedVersion)
		}
	}


	/// Raw entry points of the OpenAL API.
	pub fn raw_api(&self) -> &sys::AlApi { &self.0.api }


	/// `alcGetString(ALC_DEFAULT_DEVICE_SPECIFIER)`
	pub fn default_output(&self) -> Option<CString> {
		let spec = if let Ok(ext::ALC_ENUMERATE_ALL_EXT{ALC_DEFAULT_ALL_DEVICES_SPECIFIER: Ok(dads), ..}) = self.0.exts.ALC_ENUMERATE_ALL_EXT {
			unsafe { self.0.api.alcGetString(ptr::null_mut(), dads) }
		} else {
			unsafe { self.0.api.alcGetString(ptr::null_mut(), sys::ALC_DEFAULT_DEVICE_SPECIFIER) }
		};

		if spec == ptr::null() {
			None
		} else {
			unsafe { Some(CStr::from_ptr(spec).to_owned()) }
		}
	}


	/// `alcGetString(ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER)`
	pub fn default_capture(&self) -> Option<CString> {
		let spec = unsafe { self.0.api.alcGetString(ptr::null_mut(), sys::ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER) };

		if spec == ptr::null() {
			None
		} else {
			unsafe { Some(CStr::from_ptr(spec).to_owned()) }
		}
	}


	/// `alcGetString(ALC_DEVICE_SPECIFIER)`
	pub fn enumerate_outputs(&self) -> Vec<CString> {
		let spec = if let Ok(ext::ALC_ENUMERATE_ALL_EXT{ALC_ALL_DEVICES_SPECIFIER: Ok(ads), ..}) = self.0.exts.ALC_ENUMERATE_ALL_EXT {
			unsafe { self.0.api.alcGetString(ptr::null_mut(), ads) }
		} else {
			unsafe { self.0.api.alcGetString(ptr::null_mut(), sys::ALC_DEVICE_SPECIFIER) }
		};
		Alto::parse_enum_spec(spec as *const u8)
	}


	/// `alcGetString(ALC_CAPTURE_DEVICE_SPECIFIER)`
	pub fn enumerate_captures(&self) -> Vec<CString> {
		let spec = unsafe { self.0.api.alcGetString(ptr::null_mut(), sys::ALC_CAPTURE_DEVICE_SPECIFIER) };
		Alto::parse_enum_spec(spec as *const u8)
	}


	fn parse_enum_spec(spec: *const u8) -> Vec<CString> {
		let mut specs = Vec::with_capacity(0);

		if spec == ptr::null() {
			return specs;
		}

		let mut i = 0;
		loop {
			if unsafe { ptr::read(spec.offset(i)) == 0 && ptr::read(spec.offset(i + 1)) == 0 } {
				break;
			}

			i += 1;
		}

		specs.extend(unsafe { ::std::slice::from_raw_parts(spec as *const u8, i as usize) }.split(|c| *c == 0).map(|d| CString::new(d).unwrap()));

		specs
	}


	/// `alcOpenDevice()`
	pub fn open<'s>(&self, spec: Option<&'s CStr>) -> AltoResult<OutputDevice> {
		let spec = spec.map(|s| s.to_owned()).or_else(|| self.default_output());
		let dev = unsafe { self.0.api.alcOpenDevice(spec.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null())) };

		if dev == ptr::null_mut() {
			Err(AltoError::AlcInvalidDevice)
		} else {
			Ok(OutputDevice(Arc::new(DeviceInner{
				alto: Alto(self.0.clone()),
				spec: spec,
				dev: dev,
				exts: unsafe { ext::AlcCache::new(&self.0.api, dev) },
			})))
		}
	}


	/// `alcLoopbackOpenDeviceSOFT()`
	/// Requires `ALC_SOFT_loopback`
	pub fn open_loopback<'s, F: LoopbackFrame>(&self, spec: Option<&'s CStr>) -> AltoResult<LoopbackDevice<F>> {
		let asl = self.0.exts.ALC_SOFT_loopback()?;
		asl.alcRenderSamplesSOFT?;

		let spec = spec.map(|s| s.to_owned()).or_else(|| self.default_output());
		let dev = unsafe { asl.alcLoopbackOpenDeviceSOFT?(spec.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null())) };

		if dev == ptr::null_mut() {
			Err(AltoError::AlcInvalidDevice)
		} else {
			Ok(LoopbackDevice(
				Arc::new(DeviceInner{
					alto: Alto(self.0.clone()),
					spec: spec,
					dev: dev,
					exts: unsafe { ext::AlcCache::new(&self.0.api, dev) },
				}),
				PhantomData,
			))
		}
	}


	/// `alcCaptureOpenDevice()`
	pub fn open_capture<'s, F: StandardFrame>(&self, spec: Option<&'s CStr>, freq: sys::ALCuint, len: sys::ALCsizei) -> AltoResult<Capture<F>> {
		let spec = spec.map(|s| s.to_owned()).or_else(|| self.default_capture());
		let dev = unsafe { self.0.api.alcCaptureOpenDevice(spec.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null()), freq, F::format().into_raw(None)?, len) };

		if dev == ptr::null_mut() {
			Err(AltoError::AlcInvalidDevice)
		} else {
			Ok(Capture{alto: Alto(self.0.clone()), spec: spec, dev: dev, marker: PhantomData})
		}
	}


	#[doc(hidden)]
	pub fn get_error(&self, dev: *mut sys::ALCdevice) -> AltoResult<()> {
		match unsafe { self.0.api.alcGetError(dev)} {
			sys::ALC_NO_ERROR => Ok(()),
			e => Err(AltoError::from_alc(e)),
		}
	}
}


impl Clone for Alto {
	fn clone(&self) -> Alto { Alto(self.0.clone()) }
}


impl DeviceInner {
	#[inline] fn alto(&self) -> &Alto { &self.alto }
	#[inline] fn specifier(&self) -> Option<&CStr> { self.spec.as_ref().map(|s| s.as_ref()) }
	#[inline] fn as_raw(&self) -> *mut sys::ALCdevice { self.dev }


	pub fn is_extension_present(&self, ext: ext::Alc) -> bool {
		match ext {
			ext::Alc::Dedicated => self.exts.ALC_EXT_DEDICATED().is_ok(),
			ext::Alc::Disconnect => self.exts.ALC_EXT_DISCONNECT().is_ok(),
			ext::Alc::Efx => self.exts.ALC_EXT_EFX().is_ok(),
			ext::Alc::SoftHrtf => self.exts.ALC_SOFT_HRTF().is_ok(),
			ext::Alc::SoftPauseDevice => self.exts.ALC_SOFT_pause_device().is_ok(),
		}
	}


	pub fn connected(&self) -> AltoResult<bool> {
		let mut value = 0;
		unsafe { self.alto.0.api.alcGetIntegerv(self.dev, self.exts.ALC_EXT_DISCONNECT()?.ALC_CONNECTED?, 1, &mut value); }
		self.alto.get_error(self.dev).map(|_| value == sys::ALC_TRUE as sys::ALCint)
	}


	pub fn enumerate_soft_hrtfs(&self) -> Vec<CString> {
		let mut spec_vec = Vec::with_capacity(0);

		let _ = (|| -> AltoResult<_> {
			let ash = self.exts.ALC_SOFT_HRTF()?;
			let mut value = 0;
			unsafe { self.alto.0.api.alcGetIntegerv(self.dev, ash.ALC_NUM_HRTF_SPECIFIERS_SOFT?, 1, &mut value); }

			for i in 0 .. value {
				unsafe {
					let spec = ash.alcGetStringiSOFT?(self.dev, ash.ALC_HRTF_SPECIFIER_SOFT?, i) as *mut _;
					spec_vec.push(self.alto.get_error(self.dev).map(|_| CString::from_raw(spec))?);
				}
			}

			Ok(())
		})();

		spec_vec
	}


	pub fn soft_hrtf_status(&self) -> AltoResult<SoftHrtfStatus> {
		let ash = self.exts.ALC_SOFT_HRTF()?;

		let mut value = 0;
		unsafe { self.alto.0.api.alcGetIntegerv(self.dev, ash.ALC_HRTF_STATUS_SOFT?, 1, &mut value); }
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


	pub fn max_aux_sends(&self) -> sys::ALCint {
		let mut value = 0;
		let _ = (|| -> AltoResult<_> {
			unsafe { self.alto.0.api.alcGetIntegerv(self.dev, self.exts.ALC_EXT_EFX()?.ALC_MAX_AUXILIARY_SENDS?, 1, &mut value); }
			Ok(())
		})();
		value
	}
}


impl Drop for DeviceInner {
	fn drop(&mut self) {
		unsafe { self.alto.0.api.alcCloseDevice(self.dev); }
	}
}


impl OutputDevice {
	fn make_attrs_vec(&self, attrs: Option<ContextAttrs>) -> AltoResult<Option<Vec<sys::ALCint>>> {
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

			if let Ok(ash) = self.0.exts.ALC_SOFT_HRTF() {
				if let Some(hrtf) = attrs.soft_hrtf {
					attrs_vec.extend(&[ash.ALC_HRTF_SOFT?, if hrtf { sys::ALC_TRUE } else { sys::ALC_FALSE } as sys::ALCint]);
				}
				if let Some(hrtf_id) = attrs.soft_hrtf_id {
					attrs_vec.extend(&[ash.ALC_HRTF_ID_SOFT?, hrtf_id]);
				}
			}

			if let Ok(efx) = self.0.exts.ALC_EXT_EFX() {
				if let Some(max_sends) = attrs.max_aux_sends {
					attrs_vec.extend(&[efx.ALC_MAX_AUXILIARY_SENDS?, max_sends]);
				}
			}

			attrs_vec.push(0);
			Ok(Some(attrs_vec))
		} else {
			Ok(None)
		}
	}


	/// `alcCreateContext()`
	pub fn new_context(&self, attrs: Option<ContextAttrs>) -> AltoResult<Context> {
		let attrs_vec = self.make_attrs_vec(attrs)?;
		let ctx = unsafe { self.0.alto.0.api.alcCreateContext(self.0.dev, attrs_vec.map(|a| a.as_slice().as_ptr()).unwrap_or(ptr::null())) };
		if ctx == ptr::null_mut() {
			match self.0.alto.get_error(self.0.dev) {
				Ok(..) => Err(AltoError::AlcUnknownError),
				Err(e) => Err(e),
			}
		} else {
			unsafe { Ok(Context::new(self.to_device(), ctx)) }
		}
	}


	/// `alcDevicePauseSOFT()`
	/// Requires `ALC_SOFT_pause_device`
	pub fn soft_pause(&self) -> AltoResult<()> {
		let adps = self.0.exts.ALC_SOFT_pause_device()?.alcDevicePauseSOFT?;

		unsafe { adps(self.0.dev) }
		if let Err(e) = self.0.alto.get_error(self.0.dev) {
			return Err(e);
		}

		Ok(())
	}


	/// `alcDeviceResumeSOFT()`
	/// Requires `ALC_SOFT_pause_device`
	pub fn soft_resume(&self) {
		if let Ok(aspd) = self.0.exts.ALC_SOFT_pause_device() {
			if let Ok(adrs) = aspd.alcDeviceResumeSOFT {
				unsafe { adrs(self.0.dev); }
			}
		}
	}


	/// `alcDevicePauseSOFT()`
	/// Requires `ALC_SOFT_HRTF`
	pub fn soft_reset(&self, attrs: Option<ContextAttrs>) -> AltoResult<()> {
		let ards = self.0.exts.ALC_SOFT_HRTF()?.alcResetDeviceSOFT?;
		let attrs_vec = self.make_attrs_vec(attrs.into())?;
		unsafe { ards(self.0.dev, attrs_vec.map(|a| a.as_slice().as_ptr()).unwrap_or(ptr::null())) };
		self.0.alto.get_error(self.0.dev)
	}
}


unsafe impl DeviceObject for OutputDevice {
	#[inline] fn alto(&self) -> &Alto { self.0.alto() }
	#[inline] fn specifier(&self) -> Option<&CStr> { self.0.specifier() }
	#[inline] fn as_raw(&self) -> *mut sys::ALCdevice { self.0.as_raw() }
	#[inline] fn connected(&self) -> AltoResult<bool> { self.0.connected() }

	#[inline] fn is_extension_present(&self, ext: ext::Alc) -> bool { self.0.is_extension_present(ext) }
	#[inline] fn enumerate_soft_hrtfs(&self) -> Vec<CString> { self.0.enumerate_soft_hrtfs() }
	#[inline] fn soft_hrtf_status(&self) -> AltoResult<SoftHrtfStatus> { self.0.soft_hrtf_status() }
	#[inline] fn max_aux_sends(&self) -> sys::ALCint { self.0.max_aux_sends() }
	#[inline] fn to_device(&self) -> Device { Device(self.0.clone()) }
}


impl PartialEq for OutputDevice {
	fn eq(&self, other: &OutputDevice) -> bool {
		self.0.dev == other.0.dev
	}
}
impl Eq for OutputDevice { }


unsafe impl Send for OutputDevice { }
unsafe impl Sync for OutputDevice { }


impl<F: LoopbackFrame> LoopbackDevice<F> {
	fn make_attrs_vec(&self, freq: sys::ALCint, attrs: Option<LoopbackAttrs>) -> AltoResult<Vec<sys::ALCint>> {
		let asl = self.0.alto.0.exts.ALC_SOFT_loopback()?;

		let mut attrs_vec = Vec::with_capacity(17);
		attrs_vec.extend(&[sys::ALC_FREQUENCY, freq]);
		attrs_vec.extend(&[asl.ALC_FORMAT_CHANNELS_SOFT?, F::channels(&asl)?]);
		attrs_vec.extend(&[asl.ALC_FORMAT_TYPE_SOFT?, F::sample_ty(&asl)?]);
		if let Some(attrs) = attrs {
			if let Some(mono) = attrs.mono_sources {
				attrs_vec.extend(&[sys::ALC_MONO_SOURCES, mono]);
			}
			if let Some(stereo) = attrs.stereo_sources {
				attrs_vec.extend(&[sys::ALC_STEREO_SOURCES, stereo]);
			}

			if let Ok(ash) = self.0.exts.ALC_SOFT_HRTF() {
				if let Some(hrtf) = attrs.soft_hrtf {
					attrs_vec.extend(&[ash.ALC_HRTF_SOFT?, if hrtf { sys::ALC_TRUE } else { sys::ALC_FALSE } as sys::ALCint]);
				}
				if let Some(hrtf_id) = attrs.soft_hrtf_id {
					attrs_vec.extend(&[ash.ALC_HRTF_ID_SOFT?, hrtf_id]);
				}
			}

			if let Ok(efx) = self.0.exts.ALC_EXT_EFX() {
				if let Some(max_sends) = attrs.max_aux_sends {
					attrs_vec.extend(&[efx.ALC_MAX_AUXILIARY_SENDS?, max_sends]);
				}
			}
		}
		attrs_vec.push(0);
		Ok(attrs_vec)
	}


	/// `alcCreateContext()`
	pub fn new_context(&self, freq: sys::ALCint, attrs: Option<LoopbackAttrs>) -> AltoResult<Context> {
		let attrs_vec = self.make_attrs_vec(freq, attrs.into())?;
		let ctx = unsafe { self.0.alto.0.api.alcCreateContext(self.0.dev, attrs_vec.as_slice().as_ptr()) };
		if ctx == ptr::null_mut() {
			match self.0.alto.get_error(self.0.dev) {
				Ok(..) => Err(AltoError::AlcUnknownError),
				Err(e) => Err(e),
			}
		} else {
			unsafe { Ok(Context::new(self.to_device(), ctx)) }
		}
	}


	/// `alcRenderSamplesSOFT()`
	pub fn soft_render_samples<R: AsBufferDataMut<F>>(&mut self, mut data: R) {
		let mut data = data.as_buffer_data_mut();
		if sys::ALCsizei::max_value() as usize / mem::size_of::<F>() < data.len() { panic!("ALTO ERROR: data slice is too long for `alcRenderSamplesSOFT()`") }

		let asl = self.0.alto.0.exts.ALC_SOFT_loopback().unwrap();

		unsafe { asl.alcRenderSamplesSOFT.unwrap()(self.0.dev, data.as_mut_ptr() as *mut _, data.len() as sys::ALCsizei); }
	}


	/// `alcDevicePauseSOFT()`
	/// Requires `ALC_SOFT_HRTF`
	pub fn soft_reset(&self, freq: sys::ALCint, attrs: Option<LoopbackAttrs>) -> AltoResult<()> {
		let ards = self.0.exts.ALC_SOFT_HRTF()?.alcResetDeviceSOFT?;

		let attrs_vec = self.make_attrs_vec(freq, attrs.into());
		unsafe { ards(self.0.dev, attrs_vec.map(|a| a.as_slice().as_ptr()).unwrap_or(ptr::null())) };
		self.0.alto.get_error(self.0.dev)
	}
}


unsafe impl<F: LoopbackFrame> DeviceObject for LoopbackDevice<F> {
	#[inline] fn alto(&self) -> &Alto { self.0.alto() }
	#[inline] fn specifier(&self) -> Option<&CStr> { self.0.specifier() }
	#[inline] fn as_raw(&self) -> *mut sys::ALCdevice { self.0.as_raw() }
	#[inline] fn connected(&self) -> AltoResult<bool> { self.0.connected() }

	#[inline] fn is_extension_present(&self, ext: ext::Alc) -> bool { self.0.is_extension_present(ext) }
	#[inline] fn enumerate_soft_hrtfs(&self) -> Vec<CString> { self.0.enumerate_soft_hrtfs() }
	#[inline] fn soft_hrtf_status(&self) -> AltoResult<SoftHrtfStatus> { self.0.soft_hrtf_status() }
	#[inline] fn max_aux_sends(&self) -> sys::ALCint { self.0.max_aux_sends() }
	#[inline] fn to_device(&self) -> Device { Device(self.0.clone()) }
}


impl<F: LoopbackFrame> PartialEq for LoopbackDevice<F> {
	fn eq(&self, other: &LoopbackDevice<F>) -> bool {
		self.0.dev == other.0.dev
	}
}
impl<F: LoopbackFrame> Eq for LoopbackDevice<F> { }


unsafe impl<F: LoopbackFrame> Send for LoopbackDevice<F> { }
unsafe impl<F: LoopbackFrame> Sync for LoopbackDevice<F> { }


unsafe impl DeviceObject for Device {
	#[inline] fn alto(&self) -> &Alto { self.0.alto() }
	#[inline] fn specifier(&self) -> Option<&CStr> { self.0.specifier() }
	#[inline] fn as_raw(&self) -> *mut sys::ALCdevice { self.0.as_raw() }
	#[inline] fn connected(&self) -> AltoResult<bool> { self.0.connected() }

	#[inline] fn is_extension_present(&self, ext: ext::Alc) -> bool { self.0.is_extension_present(ext) }
	#[inline] fn enumerate_soft_hrtfs(&self) -> Vec<CString> { self.0.enumerate_soft_hrtfs() }
	#[inline] fn soft_hrtf_status(&self) -> AltoResult<SoftHrtfStatus> { self.0.soft_hrtf_status() }
	#[inline] fn max_aux_sends(&self) -> sys::ALCint { self.0.max_aux_sends() }
	#[inline] fn to_device(&self) -> Device { Device(self.0.clone()) }
}


impl PartialEq for DeviceObject {
	fn eq(&self, other: &DeviceObject) -> bool {
		self.as_raw() == other.as_raw()
	}
}
impl Eq for DeviceObject { }


impl<F: StandardFrame> Capture<F> {
	/// AltoInner struct from which this device was opened.
	#[inline] pub fn alto(&self) -> &Alto { &self.alto }
	/// Specifier used to open this device.
	#[inline] pub fn specifier(&self) -> Option<&CStr> { self.spec.as_ref().map(|s| s.as_ref()) }
	/// Raw device handle as reported by OpenAL.
	#[inline] pub fn as_raw(&self) -> *mut sys::ALCdevice { self.dev }


	/// `alcCaptureStart()`
	pub fn start(&mut self) {
		unsafe { self.alto.0.api.alcCaptureStart(self.dev); }
	}


	/// `alcCaptureStop()`
	pub fn stop(&mut self) {
		unsafe { self.alto.0.api.alcCaptureStop(self.dev); }
	}


	/// `alcGetIntegerv(ALC_CAPTURE_SAMPLES)`
	pub fn samples_len(&self) -> sys::ALCint {
		let mut samples = 0;
		unsafe { self.alto.0.api.alcGetIntegerv(self.dev, sys::ALC_CAPTURE_SAMPLES, 1, &mut samples); }
		samples
	}


	/// `alcCaptureSamples()`
	pub fn capture_samples<R: AsBufferDataMut<F>>(&mut self, mut data: R) -> AltoResult<()> {
		let mut data = data.as_buffer_data_mut();
		if data.len() > self.samples_len() as usize { return Err(AltoError::AlcInvalidValue) }

		unsafe { self.alto.0.api.alcCaptureSamples(self.dev, data.as_mut_ptr() as *mut _, data.len() as sys::ALCsizei); }
		Ok(())
	}
}


impl<F: StandardFrame> PartialEq for Capture<F> {
	fn eq(&self, other: &Capture<F>) -> bool {
		self.dev == other.dev
	}
}
impl<F: StandardFrame> Eq for Capture<F> { }


unsafe impl<F: StandardFrame> Send for Capture<F> { }
