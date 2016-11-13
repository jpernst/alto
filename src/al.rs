use std::mem;
use std::sync::Mutex;

use ::sys;
use ::alc::*;
use ::ext;


lazy_static! {
	static ref AL_MUTEX: Mutex<()> = Mutex::new(());
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(isize)]
pub enum AlError {
	InvalidName = sys::AL_INVALID_NAME as isize,
	InvalidEnum = sys::AL_INVALID_ENUM as isize,
	InvalidValue = sys::AL_INVALID_VALUE as isize,
	InvalidOperation = sys::AL_INVALID_OPERATION as isize,
	OutOfMemory = sys::AL_OUT_OF_MEMORY as isize,

	ExtensionNotPresent,
}


pub type AlResult<T> = ::std::result::Result<T, AlError>;


pub struct Context<'d>(*mut sys::ALCcontext, &'d OutputDevice);


pub struct Buffer<'d>(sys::ALuint, &'d OutputDevice);


pub struct Source<'c>(sys::ALuint, &'c Context<'c>);


impl<'d> Context<'d> {
	pub fn is_extension_present(&self, ext: ext::Al) -> AlcResult<bool> {
		unimplemented!();
	}


	pub fn get_string() -> AlResult<String> {
		unimplemented!();
	}


	fn get_error() -> AlResult<()> {
		match unsafe { sys::alGetError() } {
			sys::AL_NO_ERROR => Ok(()),
			e => unsafe { Err(mem::transmute(e as isize)) }
		}
	}
}


unsafe impl<'d> Send for Context<'d> { }
unsafe impl<'d> Sync for Context<'d> { }


