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
//! Because Alto interacts with global C state via dynamic linking, having multiple versions of Alto in one project could lead to unsafety.
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
	InvalidDevice,
	/// `ALC_INVALID_CONTEXT`
	InvalidContext,
	/// `AL_INVALID_NAME`
	InvalidName,
	/// `ALC/AL_INVALID_ENUM`
	InvalidEnum,
	/// `ALC/AL_INVALID_VALUE`
	InvalidValue,
	/// `AL_INVALID_OPERATION`
	InvalidOperation,
	/// `ALC/AL_OUT_OF_MEMORY`
	OutOfMemory,
	UnknownAlcError(sys::ALCint),
	UnknownAlError(sys::ALint),

	/// The underlying implementation is not compatible with the 1.1 spec. Alto specific.
	UnsupportedVersion{major: sys::ALCint, minor: sys::ALCint},
	/// The requested action can't be performed because the required extension is unavaiable. Alto specific.
	ExtensionNotPresent,
	/// Resource creation failed without setting an error code.
	NullError,
	/// A resource belongs to another device and is not eligible.
	WrongDevice,
	/// A resource belongs to another context and is not eligible.
	WrongContext,
	/// There was an underlying IO error, usually from a failure when loading the OpenAL dylib. Alto specific.
	Io(io::Error),
}


pub type AltoResult<T> = ::std::result::Result<T, AltoError>;


impl AltoError {
	fn from_alc(alc: sys::ALCenum) -> AltoError {
		match alc {
			sys::ALC_INVALID_DEVICE => AltoError::InvalidDevice,
			sys::ALC_INVALID_CONTEXT => AltoError::InvalidContext,
			sys::ALC_INVALID_ENUM => AltoError::InvalidEnum,
			sys::ALC_INVALID_VALUE => AltoError::InvalidValue,
			sys::ALC_OUT_OF_MEMORY => AltoError::OutOfMemory,
			e => AltoError::UnknownAlcError(e),
		}
	}


	fn from_al(al: sys::ALenum) -> AltoError {
		match al {
			sys::AL_INVALID_NAME => AltoError::InvalidName,
			sys::AL_INVALID_ENUM => AltoError::InvalidEnum,
			sys::AL_INVALID_VALUE => AltoError::InvalidValue,
			sys::AL_INVALID_OPERATION => AltoError::InvalidOperation,
			sys::AL_OUT_OF_MEMORY => AltoError::OutOfMemory,
			e => AltoError::UnknownAlError(e),
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
			AltoError::InvalidDevice => "ALTO ERROR: ALC Invalid Device",
			AltoError::InvalidContext => "ALTO ERROR: ALC Invalid Context",
			AltoError::InvalidName => "ALTO ERROR: AL Invalid Name",
			AltoError::InvalidEnum => "ALTO ERROR: ALC Invalid Enum",
			AltoError::InvalidValue => "ALTO ERROR: ALC Invalid Value",
			AltoError::InvalidOperation => "ALTO ERROR: AL Invalid Operation",
			AltoError::OutOfMemory => "ALTO ERROR: ALC Out of Memory",
			AltoError::UnknownAlcError(..) => "ALTO ERROR: Unknown ALC error",
			AltoError::UnknownAlError(..) => "ALTO ERROR: Unknown AL error",

			AltoError::UnsupportedVersion{..} => "ALTO ERROR: Unsupported Version",
			AltoError::ExtensionNotPresent => "ALTO ERROR: Extension Not Present",
			AltoError::NullError => "ALTO ERROR: Return value is NULL with no error code",
			AltoError::WrongDevice => "ALTO ERROR: Resource used on wrong device",
			AltoError::WrongContext => "ALTO ERROR: Resource used on wrong device",
			AltoError::Io(ref io) => io.description(),
		}
	}
}


impl From<io::Error> for AltoError {
	fn from(io: io::Error) -> AltoError {
		AltoError::Io(io)
	}
}


impl From<ext::ExtensionError> for AltoError {
	fn from(_: ext::ExtensionError) -> AltoError {
		AltoError::ExtensionNotPresent
	}
}
