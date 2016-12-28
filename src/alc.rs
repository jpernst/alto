use std::ops::Deref;
use std::ptr;
use std::ffi::{CString, CStr};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::fmt;
use std::error::Error as StdError;
use std::io::{self, Write};
use std::path::Path;
use std::marker::PhantomData;

use ::sys;
use ::al::*;
use ::ext;


#[derive(Debug)]
pub enum AlcError {
	InvalidDevice,
	InvalidContext,
	InvalidEnum,
	InvalidValue,
	OutOfMemory,

	UnsupportedVersion,
	ExtensionNotPresent,
	Al(AlError),
	Io(io::Error),
	UnknownError,
}


pub struct ContextAttrs {
	pub frequency: Option<sys::ALCint>,
	pub refresh: Option<sys::ALCint>,
	pub mono_sources: Option<sys::ALCint>,
	pub stereo_sources: Option<sys::ALCint>,
}


pub struct LoopbackAttrs {
	pub mono_sources: Option<sys::ALCint>,
	pub stereo_sources: Option<sys::ALCint>,
}


pub enum LoopbackFormatChannels {
	Mono,
	Stereo,
	Quad,
	Mc51,
	Mc61,
	Mc71,
}


pub enum LoopbackFormatType {
	U8,
	I16,
	F32,
}


pub type AlcResult<T> = ::std::result::Result<T, AlcError>;


rental!{
	mod rent {
		pub rental AlApi<'rental>(Box<::sys::AlApi>, ::ext::AlcNullCache<'rental>);
	}
}

#[doc(hidden)]
pub use self::rent::AlApi;


pub struct Alto {
	api: AlApi<'static>,
	ctx_lock: Mutex<()>,
}


pub trait DeviceTrait {
	fn alto(&self) -> &Alto;
	fn specifier(&self) -> &CStr;
	fn raw_device(&self) -> *mut sys::ALCdevice;
	fn extensions(&self) -> &ext::AlcCache;
	fn connected(&self) -> AlcResult<bool>;
}


pub struct Device<'a> {
	alto: &'a Alto,
	spec: CString,
	dev: *mut sys::ALCdevice,
	exts: ext::AlcCache<'a>,
	pause_rc: Arc<AtomicUsize>,
}


pub struct SoftPauseLock<'a: 'd, 'd>(&'d Device<'a>);


pub unsafe trait LoopbackFrame: SampleFrame {
	fn channels(&ext::ALC_SOFT_loopback) -> ext::ExtResult<sys::ALint>;
	fn sample_ty(&ext::ALC_SOFT_loopback) -> ext::ExtResult<sys::ALint>;
}


pub struct LoopbackDevice<'a, F: LoopbackFrame> {
	alto: &'a Alto,
	spec: CString,
	dev: *mut sys::ALCdevice,
	exts: ext::AlcCache<'a>,
	marker: PhantomData<F>,
}


pub struct CaptureDevice<'a> {
	alto: &'a Alto,
	spec: CString,
	dev: *mut sys::ALCdevice,
}


impl Alto {
	pub fn load_default() -> AlcResult<Alto> {
		let api = Box::new(sys::AlApi::load_default()?);
		Ok(Alto{
			api: AlApi::new(api, |a| unsafe { ext::AlcNullCache::new(a, ptr::null_mut()) }),
			ctx_lock: Mutex::new(()),
		}).and_then(|a| a.check_version())
	}


	pub fn load<P: AsRef<Path>>(path: P) -> AlcResult<Alto> {
		let api = Box::new(sys::AlApi::load(path)?);
		Ok(Alto{
			api: AlApi::new(api, |a| unsafe { ext::AlcNullCache::new(a, ptr::null_mut()) }),
			ctx_lock: Mutex::new(()),
		}).and_then(|a| a.check_version())
	}


	fn check_version(self) -> AlcResult<Alto> {
		let mut major = 0;
		unsafe { self.api.owner().alcGetIntegerv()(ptr::null_mut(), sys::ALC_MAJOR_VERSION, 1, &mut major); }
		let mut minor = 0;
		unsafe { self.api.owner().alcGetIntegerv()(ptr::null_mut(), sys::ALC_MINOR_VERSION, 1, &mut minor); }

		if major == 1 && minor >= 1 {
			Ok(self)
		} else {
			Err(AlcError::UnsupportedVersion)
		}
	}


	pub fn default_output(&self) -> AlcResult<CString> {
		self.api.rent(|exts| {
			let spec = if let Ok(ea) = exts.ALC_ENUMERATE_ALL_EXT() {
				unsafe { CStr::from_ptr(self.api.owner().alcGetString()(ptr::null_mut(), ea.ALC_DEFAULT_ALL_DEVICES_SPECIFIER?)) }
			} else {
				unsafe { CStr::from_ptr(self.api.owner().alcGetString()(ptr::null_mut(), sys::ALC_DEFAULT_DEVICE_SPECIFIER)) }
			};
			self.get_error(ptr::null_mut()).map(|_| spec.to_owned())
		})
	}


	pub fn default_capture(&self) -> AlcResult<CString> {
		let spec = unsafe { CStr::from_ptr(self.api.owner().alcGetString()(ptr::null_mut(), sys::ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER)) };
		self.get_error(ptr::null_mut()).map(|_| spec.to_owned())
	}


	pub fn enumerate_outputs(&self) -> AlcResult<Vec<CString>> {
		self.api.rent(|exts| {
			let spec = if let Ok(ea) = exts.ALC_ENUMERATE_ALL_EXT() {
				unsafe { self.api.owner().alcGetString()(ptr::null_mut(), ea.ALC_ALL_DEVICES_SPECIFIER?) }
			} else {
				unsafe { self.api.owner().alcGetString()(ptr::null_mut(), sys::ALC_DEVICE_SPECIFIER) }
			};
			self.get_error(ptr::null_mut()).and_then(|_| Alto::parse_enum_spec(spec as *const u8))
		})
	}


	pub fn enumerate_captures(&self) -> AlcResult<Vec<CString>> {
		let spec = unsafe { self.api.owner().alcGetString()(ptr::null_mut(), sys::ALC_CAPTURE_DEVICE_SPECIFIER) };
		self.get_error(ptr::null_mut()).and_then(|_| Alto::parse_enum_spec(spec as *const u8))
	}


	fn parse_enum_spec(spec: *const u8) -> AlcResult<Vec<CString>> {
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


	pub fn open(&self, spec: Option<&CStr>) -> AlcResult<Device> {
		let spec = if let Some(spec) = spec {
			spec.to_owned()
		} else {
			self.default_output()?
		};

		let dev = unsafe { self.api.owner().alcOpenDevice()(spec.as_ptr()) };
		self.get_error(ptr::null_mut())?;

		if dev == ptr::null_mut() {
			Err(AlcError::InvalidDevice)
		} else {
			Ok(Device{alto: self, spec: spec, dev: dev, exts: unsafe { ext::AlcCache::new(self.api.owner(), dev) }, pause_rc: Arc::new(AtomicUsize::new(0))})
		}
	}


	pub fn open_loopback<F: LoopbackFrame>(&self, spec: Option<&CStr>) -> AlcResult<LoopbackDevice<F>> {
		self.api.rent(|exts| {
			let sl = exts.ALC_SOFT_loopback()?;

			let spec = if let Some(spec) = spec {
				spec.to_owned()
			} else {
				self.default_output()?
			};

			let dev = unsafe { sl.alcLoopbackOpenDeviceSOFT?(spec.as_ptr()) };
			self.get_error(ptr::null_mut())?;

			if dev == ptr::null_mut() {
				Err(AlcError::InvalidDevice)
			} else {
				Ok(LoopbackDevice{alto: self, spec: spec, dev: dev, exts: unsafe { ext::AlcCache::new(self.api.owner(), dev) }, marker: PhantomData})
			}
		})
	}


	pub fn open_capture(&self, spec: Option<&CStr>, freq: sys::ALCuint, format: StandardFormat, size: sys::ALCsizei) -> AlcResult<CaptureDevice> {
		let spec = if let Some(spec) = spec {
			spec.to_owned()
		} else {
			self.default_output()?
		};

		let dev = unsafe { self.api.owner().alcCaptureOpenDevice()(spec.as_ptr(), freq, format.into_raw(), size) };
		self.get_error(ptr::null_mut())?;

		if dev == ptr::null_mut() {
			Err(AlcError::InvalidDevice)
		} else {
			Ok(CaptureDevice{alto: self, spec: spec, dev: dev})
		}
	}


	#[doc(hidden)]
	pub fn get_error(&self, dev: *mut sys::ALCdevice) -> AlcResult<()> {
		match unsafe { self.api.owner().alcGetError()(dev)} {
			sys::ALC_NO_ERROR => Ok(()),
			e => Err(e.into()),
		}
	}
}


impl fmt::Display for AlcError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.description())
	}
}


impl StdError for AlcError {
	fn description(&self) -> &str {
		match *self {
			AlcError::InvalidDevice => "ALC ERROR: Invalid Device",
			AlcError::InvalidContext => "ALC ERROR: Invalid Context",
			AlcError::InvalidEnum => "ALC ERROR: Invalid Enum",
			AlcError::InvalidValue => "ALC ERROR: Invalid Value",
			AlcError::OutOfMemory => "ALC ERROR: Invalid Memory",

			AlcError::UnsupportedVersion => "ALC ERROR: Unsupported Version",
			AlcError::ExtensionNotPresent => "ALC ERROR: Extension Not Present",
			AlcError::Al(ref al) => al.description(),
			AlcError::Io(ref io) => io.description(),
			AlcError::UnknownError => "ALC ERROR: Unknown Error",
		}
	}
}


impl From<sys::ALCenum> for AlcError {
	fn from(al: sys::ALCenum) -> AlcError {
		match al {
			sys::ALC_INVALID_DEVICE => AlcError::InvalidDevice,
			sys::ALC_INVALID_CONTEXT => AlcError::InvalidContext,
			sys::ALC_INVALID_ENUM => AlcError::InvalidEnum,
			sys::ALC_INVALID_VALUE => AlcError::InvalidValue,
			sys::ALC_OUT_OF_MEMORY => AlcError::OutOfMemory,
			_ => AlcError::UnknownError,
		}
	}
}


impl From<AlError> for AlcError {
	fn from(al: AlError) -> AlcError {
		AlcError::Al(al)
	}
}


impl From<ext::ExtensionError> for AlcError {
	fn from(_: ext::ExtensionError) -> AlcError {
		AlcError::ExtensionNotPresent
	}
}


impl From<io::Error> for AlcError {
	fn from(io: io::Error) -> AlcError {
		AlcError::Io(io)
	}
}


impl<'a> PartialEq for (DeviceTrait + 'a) {
	fn eq(&self, other: &(DeviceTrait + 'a)) -> bool {
		self.raw_device() == other.raw_device()
	}
}
impl Eq for DeviceTrait { }


impl<'a> Device<'a> {
	pub fn is_extension_present(&self, ext: ext::Alc) -> bool {
		match ext {
			ext::Alc::Dedicated => self.exts.ALC_EXT_DEDICATED().is_ok(),
			ext::Alc::Disconnect => self.exts.ALC_EXT_DISCONNECT().is_ok(),
			ext::Alc::Efx => self.exts.ALC_EXT_EFX().is_ok(),
			ext::Alc::SoftHrtf => self.exts.ALC_SOFT_HRTF().is_ok(),
			ext::Alc::SoftPauseDevice => self.exts.ALC_SOFT_pause_device().is_ok(),
		}
	}


	pub fn new_context(&self, attrs: Option<ContextAttrs>) -> AlcResult<Context> {
		let attrs_vec = attrs.map(|a| {
			let mut attrs_vec = Vec::with_capacity(9);

			if let Some(freq) = a.frequency {
				attrs_vec.extend(&[sys::ALC_FREQUENCY, freq]);
			}
			if let Some(refresh) = a.refresh {
				attrs_vec.extend(&[sys::ALC_REFRESH, refresh]);
			}
			if let Some(mono) = a.mono_sources {
				attrs_vec.extend(&[sys::ALC_MONO_SOURCES, mono]);
			}
			if let Some(stereo) = a.stereo_sources {
				attrs_vec.extend(&[sys::ALC_STEREO_SOURCES, stereo]);
			}

			attrs_vec.push(0);
			attrs_vec
		});

		let ctx = unsafe { self.alto.api.owner().alcCreateContext()(self.dev, attrs_vec.map(|a| a.as_slice().as_ptr()).unwrap_or(ptr::null())) };
		self.alto.get_error(self.dev)?;

		Ok(unsafe { Context::new(self, &self.alto.api, &self.alto.ctx_lock, ctx) })
	}


	pub fn soft_pause<'d>(&'d self) -> AlcResult<SoftPauseLock<'a, 'd>> {
		SoftPauseLock::new(self)
	}
}


impl<'a> DeviceTrait for Device<'a> {
	#[inline(always)]
	fn alto(&self) -> &Alto { &self.alto }
	#[inline(always)]
	fn specifier(&self) -> &CStr { &self.spec }
	#[inline(always)]
	fn raw_device(&self) -> *mut sys::ALCdevice { self.dev }
	#[inline(always)]
	fn extensions(&self) -> &ext::AlcCache { &self.exts }


	fn connected(&self) -> AlcResult<bool> {
		let mut connected = 0;
		unsafe { self.alto.api.owner().alcGetIntegerv()(self.dev, self.exts.ALC_EXT_DISCONNECT()?.ALC_CONNECTED?, 1, &mut connected); }
		self.alto.get_error(self.dev).map(|_| connected == sys::ALC_TRUE as sys::ALCint)
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
	fn new(dev: &'d Device<'a>) -> AlcResult<SoftPauseLock<'a, 'd>> {
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
	pub fn is_extension_present(&self, ext: ext::Alc) -> bool {
		match ext {
			ext::Alc::Dedicated => self.exts.ALC_EXT_DEDICATED().is_ok(),
			ext::Alc::Disconnect => self.exts.ALC_EXT_DISCONNECT().is_ok(),
			ext::Alc::Efx => self.exts.ALC_EXT_EFX().is_ok(),
			ext::Alc::SoftHrtf => self.exts.ALC_SOFT_HRTF().is_ok(),
			ext::Alc::SoftPauseDevice => self.exts.ALC_SOFT_pause_device().is_ok(),
		}
	}


	pub fn new_context(&self, freq: sys::ALCint, attrs: Option<LoopbackAttrs>) -> AlcResult<Context> {
		self.alto.api.rent(move|exts| {
			let sl = exts.ALC_SOFT_loopback()?;

			let mut attrs_vec = Vec::with_capacity(11);
			attrs_vec.extend(&[sys::ALC_FREQUENCY, freq]);
			attrs_vec.extend(&[sl.ALC_FORMAT_CHANNELS_SOFT?, F::channels(&sl)?]);
			attrs_vec.extend(&[sl.ALC_FORMAT_TYPE_SOFT?, F::sample_ty(&sl)?]);
			if let Some(attrs) = attrs {
				if let Some(mono) = attrs.mono_sources {
					attrs_vec.extend(&[sys::ALC_MONO_SOURCES, mono]);
				}
				if let Some(stereo) = attrs.stereo_sources {
					attrs_vec.extend(&[sys::ALC_STEREO_SOURCES, stereo]);
				}
			}
			attrs_vec.push(0);

			let ctx = unsafe { self.alto.api.owner().alcCreateContext()(self.dev, attrs_vec.as_slice().as_ptr()) };
			self.alto.get_error(self.dev)?;

			Ok(unsafe { Context::new(self, &self.alto.api, &self.alto.ctx_lock, ctx) })
		})
	}
}


impl<'a, F: LoopbackFrame> DeviceTrait for LoopbackDevice<'a, F> {
	#[inline(always)]
	fn alto(&self) -> &Alto { &self.alto }
	#[inline(always)]
	fn specifier(&self) -> &CStr { &self.spec }
	#[inline(always)]
	fn raw_device(&self) -> *mut sys::ALCdevice { self.dev }
	#[inline(always)]
	fn extensions(&self) -> &ext::AlcCache { &self.exts }
	#[inline(always)]
	fn connected(&self) -> AlcResult<bool> { Ok(true) }
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


impl<'a> CaptureDevice<'a> {
	#[inline(always)]
	pub fn alto(&self) -> &Alto { &self.alto }
	#[inline(always)]
	pub fn specifier(&self) -> &CStr { &self.spec }
	#[inline(always)]
	pub fn raw_device(&self) -> *mut sys::ALCdevice { self.dev }
}


impl<'a> PartialEq for CaptureDevice<'a> {
	fn eq(&self, other: &CaptureDevice<'a>) -> bool {
		self.dev == other.dev
	}
}
impl<'a> Eq for CaptureDevice<'a> { }


unsafe impl<'a> Send for CaptureDevice<'a> { }
