extern crate alto;

use std::ptr;
use std::ffi::CStr;
use alto::sys::*;


fn main() {
	let alto = alto::Alto::load_default().unwrap();
	println!("{:?}", alto.enumerate_outputs());

	unsafe {
		let api = alto::sys::AlApi::load_default().unwrap();
		let dev = (*api.alcOpenDevice)(ptr::null());
		let exts = (*api.alcGetString)(dev, ALC_EXTENSIONS);
		println!("ALC: {}", CStr::from_ptr(exts).to_string_lossy());
		let ctx = (*api.alcCreateContext)(dev, ptr::null());
		(*api.alcMakeContextCurrent)(ctx);
		let exts = (*api.alGetString)(AL_EXTENSIONS);
		println!("AL: {}", CStr::from_ptr(exts).to_string_lossy());
		(*api.alcDestroyContext)(ctx);
		(*api.alcCloseDevice)(dev);
	}
}
