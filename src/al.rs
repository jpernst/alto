use std::mem;
use std::sync::Mutex;
use std::fmt;
use std::error::Error as StdError;

use ::sys;
use ::alc::*;
use ::ext;


lazy_static! {
	static ref AL_MUTEX: Mutex<()> = Mutex::new(());
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum AlError {
	InvalidName,
	InvalidEnum,
	InvalidValue,
	InvalidOperation,
	OutOfMemory,

	ExtensionNotPresent,
	UnknownError,
}


pub enum Format {
	Mono8,
	Mono16,
	Stereo8,
	Stereo16,
}


pub type AlResult<T> = ::std::result::Result<T, AlError>;


pub struct Context<'d>(*mut sys::ALCcontext, &'d OutputDevice);


pub struct Buffer<'d>(sys::ALuint, &'d OutputDevice);


pub struct Source<'c>(sys::ALuint, &'c Context<'c>);


impl fmt::Display for AlError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.description())
	}
}


impl StdError for AlError {
	fn description(&self) -> &str {
		match *self {
			AlError::InvalidName => "AL ERROR: Invalid Name",
			AlError::InvalidEnum => "AL ERROR: Invalid Enum",
			AlError::InvalidValue => "AL ERROR: Invalid Value",
			AlError::InvalidOperation => "AL ERROR: Invalid Operation",
			AlError::OutOfMemory => "AL ERROR: Invalid Memory",

			AlError::ExtensionNotPresent => "AL ERROR: Extension Not Present",
			AlError::UnknownError => "AL ERROR: Unknown Error",
		}
	}
}


impl From<sys::ALenum> for AlError {
	fn from(al: sys::ALenum) -> AlError {
		match al {
			sys::AL_INVALID_NAME => AlError::InvalidName,
			sys::AL_INVALID_ENUM => AlError::InvalidEnum,
			sys::AL_INVALID_VALUE => AlError::InvalidValue,
			sys::AL_INVALID_OPERATION => AlError::InvalidOperation,
			sys::AL_OUT_OF_MEMORY => AlError::OutOfMemory,
			_ => AlError::UnknownError,
		}
	}
}


impl Format {
	pub fn into_raw(self, ctx: Option<&Context>) -> AlResult<sys::ALint> {
		match self {
			Format::Mono8 => Ok(sys::AL_FORMAT_MONO8),
			Format::Mono16 => Ok(sys::AL_FORMAT_MONO16),
			Format::Stereo8 => Ok(sys::AL_FORMAT_STEREO8),
			Format::Stereo16 => Ok(sys::AL_FORMAT_STEREO16),
		}
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
		match unsafe { sys::alGetError() } {
			sys::AL_NO_ERROR => Ok(()),
			e => unsafe { Err(e.into()) }
		}
	}
}


unsafe impl<'d> Send for Context<'d> { }
unsafe impl<'d> Sync for Context<'d> { }


