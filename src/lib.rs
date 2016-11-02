#[macro_use]
extern crate lazy_static;

use std::sync;
use std::ptr;
use std::mem;
use std::ffi::CStr;
use std::os::raw::c_void;
use std::collections::{hash_map, HashMap};
use std::sync::Mutex;

pub mod ffi {
	mod alc;
	mod al;
	mod alext;
	mod efx_presets;

	pub use self::al::*;
	pub use self::alc::*;
	pub use self::alext::*;
	pub use self::efx_presets::*;
}

pub mod ext;


lazy_static! {
	static ref ALC_INIT: AlcResult<()> = {
		let mut major = 0;
		unsafe { ffi::alcGetIntegerv(ptr::null_mut(), ffi::ALC_MAJOR_VERSION, 1, &mut major); }
		let mut minor = 0;
		unsafe { ffi::alcGetIntegerv(ptr::null_mut(), ffi::ALC_MINOR_VERSION, 1, &mut minor); }

		if major == 1 && minor >= 1 {
			Ok(())
		} else {
			Err(AlcError::UnsupportedVersion)
		}
	};
	static ref AL_MUTEX: sync::Mutex<()> = sync::Mutex::new(());
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(isize)]
pub enum AlcError {
	InvalidDevice = ffi::ALC_INVALID_DEVICE as isize,
	InvalidContext = ffi::ALC_INVALID_CONTEXT as isize,
	InvalidEnum = ffi::ALC_INVALID_ENUM as isize,
	InvalidValue = ffi::ALC_INVALID_VALUE as isize,
	OutOfMemory = ffi::ALC_OUT_OF_MEMORY as isize,

	UnsupportedVersion,
	ExtensionNotPresent,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(isize)]
pub enum AlError {
	InvalidName = ffi::AL_INVALID_NAME as isize,
	InvalidEnum = ffi::AL_INVALID_ENUM as isize,
	InvalidValue = ffi::AL_INVALID_VALUE as isize,
	InvalidOperation = ffi::AL_INVALID_OPERATION as isize,
	OutOfMemory = ffi::AL_OUT_OF_MEMORY as isize,

	ExtensionNotPresent,
}


pub type AlcResult<T> = std::result::Result<T, AlcError>;
pub type AlResult<T> = std::result::Result<T, AlError>;


pub struct Device {
	dev: *mut ffi::ALCdevice,
	cache: Mutex<ext::AlcCache>,
}


pub struct Context<'d>(*mut ffi::ALCcontext, &'d Device);


pub struct Buffer<'d>(ffi::ALuint, &'d Device);


pub struct Source<'c>(ffi::ALuint, &'c Context<'c>);


pub fn is_extension_present(ext: ext::Alc) -> AlcResult<bool> {
	unimplemented!();
}


fn get_error(dev: *mut ffi::ALCdevice) -> AlcResult<()> {
	match unsafe { ffi::alcGetError(dev)} {
		ffi::ALC_NO_ERROR => Ok(()),
		e => unsafe { Err(mem::transmute(e as isize)) }
	}
}


impl Device {
	pub fn enumerate() -> AlcResult<Vec<String>> {
		ALC_INIT.and_then(|_| {
			//let specs = unsafe { ffi::alcGetString(ptr::null(), ffi::ALC_DEV) };
			unimplemented!();
		})
	}


	pub fn open(spec: Option<&str>) -> AlcResult<Device> {
		let dev = if let Some(spec) = spec {
			unsafe { ffi::alcOpenDevice(ptr::null()) }
		} else {
			unsafe { ffi::alcOpenDevice(ptr::null()) }
		};
		get_error(dev).and_then(|_| Ok(Device{dev: dev, cache: Mutex::new(ext::AlcCache::new(dev))}))
	}


	pub fn is_extension_present(&self, ext: ext::Alc) -> AlcResult<bool> {
		unimplemented!();
	}


	pub fn get_string() -> AlcResult<String> {
		unimplemented!();
	}
}


impl Drop for Device {
	fn drop(&mut self) {
		unsafe { ffi::alcCloseDevice(self.dev); }
	}
}


impl<'d> Context<'d> {
	pub fn is_extension_present(&self, ext: ext::Al) -> AlcResult<bool> {
		unimplemented!();
	}


	pub fn get_string() -> AlResult<String> {
		unimplemented!();
	}


	fn get_error() -> AlResult<()> {
		match unsafe { ffi::alGetError() } {
			ffi::AL_NO_ERROR => Ok(()),
			e => unsafe { Err(mem::transmute(e as isize)) }
		}
	}
}


unsafe impl Send for Device { }
unsafe impl Sync for Device { }

unsafe impl<'d> Send for Context<'d> { }
unsafe impl<'d> Sync for Context<'d> { }


