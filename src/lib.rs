//! #Overview
//! Alto is an idiomatic wrapper for the OpenAL 3D audio API and associated extensions (including EFX).
//! This documentation does not describe how to use the OpenAL API itself, but rather explains how
//! it has been adapted for rust and provides the native symbols associated with each function
//! so they can be cross-referenced with the official OpenAL documentation for full details.
//!
//! The core of the API is the [`Alto`](struct.Alto.html) struct. It has no analog in raw OpenAL and
//! represents an implementation of the API itself. From there, instances of familiar OpenAL objects
//! can be instantiated.


#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate enum_primitive;
extern crate parking_lot;
extern crate al_sys;

use std::error::Error as StdError;
use std::fmt;
use std::io;


mod alc;
pub use alc::*;


mod al;
pub use al::*;


pub mod ext;


pub mod efx;


pub mod sys {
	pub use al_sys::*;
}


/// An error as reported by `alcGetError` or `alGetError`.
#[derive(Debug)]
pub enum AltoError {
	AlcInvalidDevice,
	AlcInvalidContext,
	AlcInvalidEnum,
	AlcInvalidValue,
	AlcOutOfMemory,

	AlcUnsupportedVersion,
	AlcExtensionNotPresent,
	AlcUnknownError,

	AlInvalidName,
	AlInvalidEnum,
	AlInvalidValue,
	AlInvalidOperation,
	AlOutOfMemory,

	AlExtensionNotPresent,
	AlWrongDevice,
	AlUnknownError,

	Io(io::Error),
}


pub type AltoResult<T> = ::std::result::Result<T, AltoError>;


impl AltoError {
	fn from_alc(alc: sys::ALCenum) -> AltoError {
		match alc {
			sys::ALC_INVALID_DEVICE => AltoError::AlcInvalidDevice,
			sys::ALC_INVALID_CONTEXT => AltoError::AlcInvalidContext,
			sys::ALC_INVALID_ENUM => AltoError::AlcInvalidEnum,
			sys::ALC_INVALID_VALUE => AltoError::AlcInvalidValue,
			sys::ALC_OUT_OF_MEMORY => AltoError::AlcOutOfMemory,
			_ => AltoError::AlcUnknownError,
		}
	}


	fn from_al(al: sys::ALenum) -> AltoError {
		match al {
			sys::AL_INVALID_NAME => AltoError::AlInvalidName,
			sys::AL_INVALID_ENUM => AltoError::AlInvalidEnum,
			sys::AL_INVALID_VALUE => AltoError::AlInvalidValue,
			sys::AL_INVALID_OPERATION => AltoError::AlInvalidOperation,
			sys::AL_OUT_OF_MEMORY => AltoError::AlOutOfMemory,
			_ => AltoError::AlUnknownError,
		}
	}
}


impl fmt::Display for AltoError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.description())
	}
}


impl StdError for AltoError {
	fn description(&self) -> &str {
		match *self {
			AltoError::AlcInvalidDevice => "ALC ERROR: Invalid Device",
			AltoError::AlcInvalidContext => "ALC ERROR: Invalid Context",
			AltoError::AlcInvalidEnum => "ALC ERROR: Invalid Enum",
			AltoError::AlcInvalidValue => "ALC ERROR: Invalid Value",
			AltoError::AlcOutOfMemory => "ALC ERROR: Invalid Memory",

			AltoError::AlcUnsupportedVersion => "ALC ERROR: Unsupported Version",
			AltoError::AlcExtensionNotPresent => "ALC ERROR: Extension Not Present",
			AltoError::AlcUnknownError => "ALC ERROR: Unknown Error",

			AltoError::AlInvalidName => "AL ERROR: Invalid Name",
			AltoError::AlInvalidEnum => "AL ERROR: Invalid Enum",
			AltoError::AlInvalidValue => "AL ERROR: Invalid Value",
			AltoError::AlInvalidOperation => "AL ERROR: Invalid Operation",
			AltoError::AlOutOfMemory => "AL ERROR: Invalid Memory",

			AltoError::AlExtensionNotPresent => "AL ERROR: Extension Not Present",
			AltoError::AlWrongDevice => "AL ERROR: Wrong Device",
			AltoError::AlUnknownError => "AL ERROR: Unknown Error",

			AltoError::Io(ref io) => io.description(),
		}
	}
}


impl From<io::Error> for AltoError {
	fn from(io: io::Error) -> AltoError {
		AltoError::Io(io)
	}
}


impl From<ext::AlcExtensionError> for AltoError {
	fn from(_: ext::AlcExtensionError) -> AltoError {
		AltoError::AlcExtensionNotPresent
	}
}
impl From<ext::AlExtensionError> for AltoError {
	fn from(_: ext::AlExtensionError) -> AltoError {
		AltoError::AlExtensionNotPresent
	}
}
