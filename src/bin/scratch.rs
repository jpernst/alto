extern crate alto;

use std::ptr;
use std::ffi::CStr;
use alto::sys::*;


fn main() {
	unsafe {
		let dev = alcOpenDevice(ptr::null());
		let exts = alcGetString(dev, ALC_EXTENSIONS);
		println!("ALC: {}", CStr::from_ptr(exts).to_string_lossy());
		let ctx = alcCreateContext(dev, ptr::null());
		alcMakeContextCurrent(ctx);
		let exts = alGetString(AL_EXTENSIONS);
		println!("AL: {}", CStr::from_ptr(exts).to_string_lossy());
		alcDestroyContext(ctx);
		alcCloseDevice(dev);
	}
}
