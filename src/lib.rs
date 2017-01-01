//! #Overview
//! Alto is an idiomatic wrapper for the OpenAL 3D audio API and associated extensions
//! (EFX support is still WIP). This documentation will describe how the API was adapted
//! for rust, but for more general information about OpenAL, the official documentation
//! should be consulted.
//!
//! The core of the API is the [`Alto`](struct.Alto.html) struct. From this struct audio
//! devices can be enumerated and opened. Once a [`Device`](struct.Device.html) or
//! [`LoopbackDevice`](struct.LoopbackDevice.html) is opened, a [`Context`](struct.Context.html)
//! can be created from it. The context governs properties of the listener and allows you to
//! allocate audio [`Buffer`](struct.Buffer.html)s. These buffers can then be played with either
//! a [`StaticSource`](struct.StaticSource.html) or [`StreamingSource`](struct.StreamingSource.html),
//! which are also allocated from the context.


#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rental;
extern crate al_sys;

use std::error::Error as StdError;
use std::fmt;
use std::io;


mod alc;
pub use alc::*;


mod al;
pub use al::*;


pub mod ext;


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
