use std::ptr;
use std::mem;
use std::ffi::{CString, CStr};
use std::sync::Mutex;

use ::sys;
use ::ext;


lazy_static! {
	static ref ALC_INIT: AlcResult<()> = {
		let mut major = 0;
		unsafe { sys::alcGetIntegerv(ptr::null_mut(), sys::ALC_MAJOR_VERSION, 1, &mut major); }
		let mut minor = 0;
		unsafe { sys::alcGetIntegerv(ptr::null_mut(), sys::ALC_MINOR_VERSION, 1, &mut minor); }

		if major == 1 && minor >= 1 {
			Ok(())
		} else {
			Err(AlcError::UnsupportedVersion)
		}
	};
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(isize)]
pub enum AlcError {
	InvalidDevice = sys::ALC_INVALID_DEVICE as isize,
	InvalidContext = sys::ALC_INVALID_CONTEXT as isize,
	InvalidEnum = sys::ALC_INVALID_ENUM as isize,
	InvalidValue = sys::ALC_INVALID_VALUE as isize,
	OutOfMemory = sys::ALC_OUT_OF_MEMORY as isize,

	UnsupportedVersion,
	ExtensionNotPresent,
}


pub type AlcResult<T> = ::std::result::Result<T, AlcError>;


pub trait OutputDevice {
}


pub struct Device {
	dev: *mut sys::ALCdevice,
	cache: Mutex<ext::AlcCache>,
}


pub struct LoopbackDevice {
	dev: *mut sys::ALCdevice,
	cache: Mutex<ext::AlcCache>,
}


pub struct CaptureDevice {
	dev: *mut sys::ALCdevice,
}


pub fn default_impl() -> AlcResult<CString> {
	default(false)
}


pub fn default_output() -> AlcResult<CString> {
	default(true)
}


fn default(all: bool) -> AlcResult<CString> {
	(*ALC_INIT)?;

	let spec = if let (true, Some(dds)) = (all, ext::ALC_CACHE.ALC_ENUMERATE_ALL_EXT().and_then(|ext| ext.ALC_DEFAULT_ALL_DEVICES_SPECIFIER)) {
		unsafe { sys::alcGetString(ptr::null_mut(), dds) }
	} else {
		unsafe { sys::alcGetString(ptr::null_mut(), sys::ALC_DEFAULT_DEVICE_SPECIFIER) }
	};
	get_error(ptr::null_mut())?;

	Ok(unsafe { CStr::from_ptr(spec).to_owned() })
}


pub fn enumerate_impls() -> AlcResult<Vec<CString>> {
	enumerate(false)
}


pub fn enumerate_outputs() -> AlcResult<Vec<CString>> {
	enumerate(true)
}


fn enumerate(all: bool) -> AlcResult<Vec<CString>> {
	(*ALC_INIT)?;

	let spec = if let (true, Some(ads)) = (all, ext::ALC_CACHE.ALC_ENUMERATE_ALL_EXT().and_then(|ext| ext.ALC_ALL_DEVICES_SPECIFIER)) {
		unsafe { sys::alcGetString(ptr::null_mut(), ads) }
	} else {
		unsafe { sys::alcGetString(ptr::null_mut(), sys::ALC_DEVICE_SPECIFIER) }
	};
	get_error(ptr::null_mut())?;

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


fn get_error(dev: *mut sys::ALCdevice) -> AlcResult<()> {
	match unsafe { sys::alcGetError(dev)} {
		sys::ALC_NO_ERROR => Ok(()),
		e => unsafe { Err(mem::transmute(e as isize)) }
	}
}


impl Device {
	pub fn open(spec: Option<&CStr>) -> AlcResult<Device> {
		(*ALC_INIT)?;

		let dev = if let Some(spec) = spec {
			unsafe { sys::alcOpenDevice(spec.as_ptr()) }
		} else {
			unsafe { sys::alcOpenDevice(ptr::null()) }
		};

		if dev == ptr::null_mut() {
			Err(AlcError::InvalidDevice)
		} else {
			get_error(dev)?;
			Ok(Device{dev: dev, cache: Mutex::new(ext::AlcCache::new(dev))})
		}
	}


	pub fn is_extension_present(&self, ext: ext::Alc) -> AlcResult<bool> {
		unimplemented!();
	}


	pub fn get_string() -> AlcResult<CString> {
		unimplemented!();
	}


}


impl Drop for Device {
	fn drop(&mut self) {
		unsafe { sys::alcCloseDevice(self.dev); }
	}
}


unsafe impl Send for Device { }
unsafe impl Sync for Device { }


