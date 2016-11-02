extern crate openal_rs;

use std::ptr;
use openal_rs::ffi;


fn main() {
	unsafe {
		let dev = ffi::alcOpenDevice(ptr::null());
		let exts = ffi::alcGetString(dev, ffi::ALC_EXTENSIONS);
		println!("ALC: {}", std::ffi::CStr::from_ptr(exts).to_string_lossy());
		let ctx = ffi::alcCreateContext(dev, ptr::null());
		ffi::alcMakeContextCurrent(ctx);
		let exts = ffi::alGetString(ffi::AL_EXTENSIONS);
		println!("AL: {}", std::ffi::CStr::from_ptr(exts).to_string_lossy());
		ffi::alcDestroyContext(ctx);
		ffi::alcCloseDevice(dev);
	}
}
