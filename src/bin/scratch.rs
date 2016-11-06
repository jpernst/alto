extern crate alto;

use std::ptr;
use std::ffi::CStr;
use alto::sys::*;


fn main() {
	unsafe {
		let sl = alto::ext::ALC_SOFT_loopback::load(ptr::null_mut()).unwrap();
		let dev = sl.alcLoopbackOpenDeviceSOFT.unwrap()(ptr::null());
		//let dev = alcOpenDevice(ptr::null());
		let exts = alcGetString(dev, ALC_EXTENSIONS);
		println!("ALC: {}", CStr::from_ptr(exts).to_string_lossy());
		let ctx = alcCreateContext(dev, (&[
			sl.ALC_FORMAT_CHANNELS_SOFT.unwrap(), sl.ALC_STEREO_SOFT.unwrap(),
			sl.ALC_FORMAT_TYPE_SOFT.unwrap(), sl.ALC_SHORT_SOFT.unwrap(),
			alto::sys::ALC_FREQUENCY, 48000,
			0
		]).as_ptr());
		alcMakeContextCurrent(ctx);
		let exts = alGetString(AL_EXTENSIONS);
		println!("AL: {}", CStr::from_ptr(exts).to_string_lossy());
		alcDestroyContext(ctx);
		alcCloseDevice(dev);
	}
}
