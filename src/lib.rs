//! # Overview
//! Alto is an idiomatic wrapper for the OpenAL 3D audio API and associated extensions (including EFX).
//! This documentation does not describe how to use the OpenAL API itself, but rather explains how
//! it has been adapted for rust and provides the native symbols associated with each function
//! so they can be cross-referenced with the official OpenAL documentation for full details.
//!
//! The core of the API is the [`Alto`](struct.Alto.html) struct. It has no analog in raw OpenAL and
//! represents an implementation of the API itself. From there, instances of familiar OpenAL objects
//! can be instantiated.
//!
//! # WARNING
//! Because Alto interacts with global C state via dynamic linking, having multiple versions of Alto in one project could lead to unsafetly.
//! Please make sure only one version of Alto is in your dependency tree at any given time.


#[macro_use]
extern crate lazy_static;
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


/// An error as reported by `alcGetError` or `alGetError`, plus some Alto specific variants.
#[derive(Debug)]
pub enum AltoError {
	/// `ALC_INVALID_DEVICE`
	AlcInvalidDevice,
	/// `ALC_INVALID_CONTEXT`
	AlcInvalidContext,
	/// `ALC_INVALID_ENUM`
	AlcInvalidEnum,
	/// `ALC_INVALID_VALUE`
	AlcInvalidValue,
	/// `ALC_OUT_OF_MEMORY`
	AlcOutOfMemory,

	/// The underlying implementation is not compatible with the 1.1 spec. Alto specific.
	AlcUnsupportedVersion{major: sys::ALCint, minor: sys::ALCint},
	/// The requested action can't be performed because the required extension is unavaiable. Alto specific.
	AlcExtensionNotPresent,
	/// Resource creation failed without setting an error code.
	AlcNullError,
	AlcUnknownError(sys::ALCint),

	/// `AL_INVALID_NAME`
	AlInvalidName,
	/// `AL_INVALID_ENUM`
	AlInvalidEnum,
	/// `AL_INVALID_VALUE`
	AlInvalidValue,
	/// `AL_INVALID_OPERATION`
	AlInvalidOperation,
	/// `AL_OUT_OF_MEMORY`
	AlOutOfMemory,

	/// The requested action can't be performed because the required extension is unavaiable. Alto specific.
	AlExtensionNotPresent,
	/// A resource belongs to another device and is not eligible.
	AlWrongDevice,
	/// A resource belongs to another context and is not eligible.
	AlWrongContext,
	/// Resource creation failed without setting an error code.
	AlNullError,
	AlUnknownError(sys::ALint),

	/// There was an underlying IO error, usually from a failure when loading the OpenAL dylib. Alto specific.
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
			e => AltoError::AlcUnknownError(e),
		}
	}


	fn from_al(al: sys::ALenum) -> AltoError {
		match al {
			sys::AL_INVALID_NAME => AltoError::AlInvalidName,
			sys::AL_INVALID_ENUM => AltoError::AlInvalidEnum,
			sys::AL_INVALID_VALUE => AltoError::AlInvalidValue,
			sys::AL_INVALID_OPERATION => AltoError::AlInvalidOperation,
			sys::AL_OUT_OF_MEMORY => AltoError::AlOutOfMemory,
			e => AltoError::AlUnknownError(e),
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

			AltoError::AlcUnsupportedVersion{..} => "ALC ERROR: Unsupported Version",
			AltoError::AlcExtensionNotPresent => "ALC ERROR: Extension Not Present",
			AltoError::AlcNullError => "ALC ERROR: Return value is NULL with no error code",
			AltoError::AlcUnknownError(..) => "AL ERROR: Unknown ALC error",

			AltoError::AlInvalidName => "AL ERROR: Invalid Name",
			AltoError::AlInvalidEnum => "AL ERROR: Invalid Enum",
			AltoError::AlInvalidValue => "AL ERROR: Invalid Value",
			AltoError::AlInvalidOperation => "AL ERROR: Invalid Operation",
			AltoError::AlOutOfMemory => "AL ERROR: Invalid Memory",

			AltoError::AlExtensionNotPresent => "AL ERROR: Extension Not Present",
			AltoError::AlWrongDevice => "AL ERROR: Resource used on wrong device",
			AltoError::AlWrongContext => "AL ERROR: Resource used on wrong device",
			AltoError::AlNullError => "AL ERROR: Return value is NULL with no error code",
			AltoError::AlUnknownError(..) => "AL ERROR: Unknown AL error",

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
