use std::ptr;
use std::ffi::{CString, CStr};
use std::sync::{Arc, Mutex};
use std::fmt;
use std::error::Error as StdError;
use std::io;
use std::path::Path;

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
		use std::sync::Arc;
		use ::sys::AlApi;
		use ::ext::AlcNullCache;

		pub rental RentAlcNull<'rental>(Arc<AlApi>, AlcNullCache<'rental>);
	}
}


pub struct Alto {
	api: Arc<sys::AlApi>,
	exts: rent::RentAlcNull<'static>,
	ctx_lock: Mutex<()>,
}


pub trait DeviceTrait {
	fn alto(&self) -> &Alto;
	fn exts(&self) -> &ext::AlcCache;
}


pub struct Device<'a> {
	alto: &'a Alto,
	dev: *mut sys::ALCdevice,
	exts: ext::AlcCache<'a>,
}


pub struct LoopbackDevice<'a> {
	alto: &'a Alto,
	dev: *mut sys::ALCdevice,
	exts: ext::AlcCache<'a>,
}


pub struct CaptureDevice<'a> {
	alto: &'a Alto,
	dev: *mut sys::ALCdevice,
}


impl Alto {
	pub fn load_default() -> AlcResult<Alto> {
		let api = Arc::new(sys::AlApi::load_default()?);
		Ok(Alto{
			exts: rent::RentAlcNull::new(api.clone(), |a| unsafe { ext::AlcNullCache::new(a, ptr::null_mut()) }),
			api: api,
			ctx_lock: Mutex::new(()),
		}).and_then(|a| a.check_version())
	}


	pub fn load<P: AsRef<Path>>(path: P) -> AlcResult<Alto> {
		let api = Arc::new(sys::AlApi::load_default()?);
		Ok(Alto{
			exts: rent::RentAlcNull::new(api.clone(), |a| unsafe { ext::AlcNullCache::new(a, ptr::null_mut()) }),
			api: api,
			ctx_lock: Mutex::new(()),
		}).and_then(|a| a.check_version())
	}

	fn check_version(self) -> AlcResult<Alto> {
		let mut major = 0;
		unsafe { self.api.alcGetIntegerv()(ptr::null_mut(), sys::ALC_MAJOR_VERSION, 1, &mut major); }
		let mut minor = 0;
		unsafe { self.api.alcGetIntegerv()(ptr::null_mut(), sys::ALC_MINOR_VERSION, 1, &mut minor); }

		if major == 1 && minor >= 1 {
			Ok(self)
		} else {
			Err(AlcError::UnsupportedVersion)
		}
	}


	pub fn default_impl(&self) -> AlcResult<CString> {
		let spec = unsafe { CStr::from_ptr(self.api.alcGetString()(ptr::null_mut(), sys::ALC_DEFAULT_DEVICE_SPECIFIER)) };
		self.get_error(ptr::null_mut()).map(|_| spec.to_owned())
	}


	pub fn default_output(&self) -> AlcResult<CString> {
		self.exts.rent(|exts|
			if let Ok(ea) = exts.ALC_ENUMERATE_ALL_EXT() {
				let spec = unsafe { CStr::from_ptr(self.api.alcGetString()(ptr::null_mut(), ea.ALC_DEFAULT_ALL_DEVICES_SPECIFIER.unwrap())) };
				self.get_error(ptr::null_mut()).map(|_| spec.to_owned())
			} else {
				self.default_impl()
			}
		)
	}


	pub fn default_capture(&self) -> AlcResult<CString> {
		let spec = unsafe { CStr::from_ptr(self.api.alcGetString()(ptr::null_mut(), sys::ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER)) };
		self.get_error(ptr::null_mut()).map(|_| spec.to_owned())
	}


	pub fn enumerate_impls(&self) -> AlcResult<Vec<CString>> {
		let spec = unsafe { self.api.alcGetString()(ptr::null_mut(), sys::ALC_DEVICE_SPECIFIER) };
		self.get_error(ptr::null_mut()).and_then(|_| Alto::parse_enum_spec(spec as *const u8))
	}


	pub fn enumerate_outputs(&self) -> AlcResult<Vec<CString>> {
		self.exts.rent(|exts|
			if let Ok(ea) = exts.ALC_ENUMERATE_ALL_EXT() {
				let spec = unsafe { self.api.alcGetString()(ptr::null_mut(), ea.ALC_ALL_DEVICES_SPECIFIER.unwrap()) };
				self.get_error(ptr::null_mut()).and_then(|_| Alto::parse_enum_spec(spec as *const u8))
			} else {
				self.enumerate_impls()
			}
		)
	}


	pub fn enumerate_captures(&self) -> AlcResult<Vec<CString>> {
		let spec = unsafe { self.api.alcGetString()(ptr::null_mut(), sys::ALC_CAPTURE_DEVICE_SPECIFIER) };
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
		let dev = if let Some(spec) = spec {
			unsafe { self.api.alcOpenDevice()(spec.as_ptr()) }
		} else {
			unsafe { self.api.alcOpenDevice()(ptr::null()) }
		};
		self.get_error(ptr::null_mut())?;

		if dev == ptr::null_mut() {
			Err(AlcError::InvalidDevice)
		} else {
			Ok(Device{alto: self, dev: dev, exts: unsafe { ext::AlcCache::new(&self.api, dev) }})
		}
	}


	pub fn open_loopback(&self, spec: Option<&CStr>) -> AlcResult<LoopbackDevice> {
		self.exts.rent(|exts| {
			let sl = exts.ALC_SOFT_loopback()?;

			let dev = if let Some(spec) = spec {
				unsafe { sl.alcLoopbackOpenDeviceSOFT.unwrap()(spec.as_ptr()) }
			} else {
				unsafe { sl.alcLoopbackOpenDeviceSOFT.unwrap()(ptr::null()) }
			};
			self.get_error(ptr::null_mut())?;

			if dev == ptr::null_mut() {
				Err(AlcError::InvalidDevice)
			} else {
				Ok(LoopbackDevice{alto: self, dev: dev, exts: unsafe { ext::AlcCache::new(&self.api, dev) }})
			}
		})
	}


	pub fn open_capture(&self, spec: Option<&CStr>, freq: sys::ALCuint, format: StandardFormat, size: sys::ALCsizei) -> AlcResult<CaptureDevice> {
		let dev = if let Some(spec) = spec {
			unsafe { self.api.alcCaptureOpenDevice()(spec.as_ptr(), freq, format.into_raw(), size) }
		} else {
			unsafe { self.api.alcCaptureOpenDevice()(ptr::null(), freq, format.into_raw(), size) }
		};
		self.get_error(ptr::null_mut())?;

		if dev == ptr::null_mut() {
			Err(AlcError::InvalidDevice)
		} else {
			Ok(CaptureDevice{alto: self, dev: dev})
		}
	}


	fn get_error(&self, dev: *mut sys::ALCdevice) -> AlcResult<()> {
		match unsafe { self.api.alcGetError()(dev)} {
			sys::ALC_NO_ERROR => Ok(()),
			e => Err(e.into())
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


}


impl<'a> DeviceTrait for Device<'a> {
	#[inline(always)]
	fn alto(&self) -> &Alto { &self.alto }
	#[inline(always)]
	fn exts(&self) -> &ext::AlcCache { &self.exts }
}


impl<'a> Drop for Device<'a> {
	fn drop(&mut self) {
		unsafe { self.alto.api.alcCloseDevice()(self.dev); }
	}
}


unsafe impl<'a> Send for Device<'a> { }
unsafe impl<'a> Sync for Device<'a> { }


impl<'a> LoopbackDevice<'a> {
	pub fn is_extension_present(&self, ext: ext::Alc) -> bool {
		match ext {
			ext::Alc::Dedicated => self.exts.ALC_EXT_DEDICATED().is_ok(),
			ext::Alc::Disconnect => self.exts.ALC_EXT_DISCONNECT().is_ok(),
			ext::Alc::Efx => self.exts.ALC_EXT_EFX().is_ok(),
			ext::Alc::SoftHrtf => self.exts.ALC_SOFT_HRTF().is_ok(),
			ext::Alc::SoftPauseDevice => self.exts.ALC_SOFT_pause_device().is_ok(),
		}
	}


	pub fn new_context(&self, freq: sys::ALCint, chan: LoopbackFormatChannels, ty: LoopbackFormatType, attrs: Option<ContextAttrs>) -> AlcResult<Context<Self>> {
		self.alto.exts.rent(move|exts| {
			let sl = exts.ALC_SOFT_loopback()?;

			let mut attrs_vec = Vec::with_capacity(4);
			attrs_vec.extend(&[sys::ALC_FREQUENCY, freq]);
			attrs_vec.extend(&[sl.ALC_FORMAT_CHANNELS_SOFT?, match chan {
				LoopbackFormatChannels::Mono => sl.ALC_MONO_SOFT?,
				LoopbackFormatChannels::Stereo => sl.ALC_STEREO_SOFT?,
				LoopbackFormatChannels::Quad => sl.ALC_QUAD_SOFT?,
				LoopbackFormatChannels::Mc51 => sl.ALC_5POINT1_SOFT?,
				LoopbackFormatChannels::Mc61 => sl.ALC_6POINT1_SOFT?,
				LoopbackFormatChannels::Mc71 => sl.ALC_7POINT1_SOFT?,
			}]);
			attrs_vec.extend(&[sl.ALC_FORMAT_TYPE_SOFT?, match ty {
				LoopbackFormatType::U8 => sl.ALC_UNSIGNED_BYTE_SOFT?,
				LoopbackFormatType::I16 => sl.ALC_SHORT_SOFT?,
				LoopbackFormatType::F32 => sl.ALC_FLOAT_SOFT?,
			}]);
			if let Some(attrs) = attrs {
			}
			attrs_vec.push(0);

			let ctx = unsafe { self.alto.api.alcCreateContext()(self.dev, attrs_vec.as_slice().as_ptr()) };
			self.alto.get_error(self.dev)?;

			Ok(unsafe { Context::new(self, self.alto.api.clone(), &self.alto.ctx_lock, ctx, ext::AlCache::new(&self.alto.api)) })
		})
	}
}


impl<'a> DeviceTrait for LoopbackDevice<'a> {
	#[inline(always)]
	fn alto(&self) -> &Alto { &self.alto }
	#[inline(always)]
	fn exts(&self) -> &ext::AlcCache { &self.exts }
}


impl<'a> Drop for LoopbackDevice<'a> {
	fn drop(&mut self) {
		unsafe { self.alto.api.alcCloseDevice()(self.dev); }
	}
}


unsafe impl<'a> Send for LoopbackDevice<'a> { }
unsafe impl<'a> Sync for LoopbackDevice<'a> { }


impl<'a> CaptureDevice<'a> {
}


unsafe impl<'a> Send for CaptureDevice<'a> { }
