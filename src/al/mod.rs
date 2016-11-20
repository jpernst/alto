use std::sync::{Arc, Mutex};
use std::fmt;
use std::error::Error as StdError;

use ::sys;
use ::alc::*;
use ::ext;


mod format;
pub use self::format::*;


#[derive(Debug)]
pub enum AlError {
	InvalidName,
	InvalidEnum,
	InvalidValue,
	InvalidOperation,
	OutOfMemory,

	ExtensionNotPresent,
	UnknownError,
}


pub type AlResult<T> = ::std::result::Result<T, AlError>;


pub trait ContextTrait {
	fn exts(&self) -> &ext::AlCache;
}


pub struct Context<'d, D: DeviceTrait + 'd> {
	api: Arc<sys::AlApi>,
	dev: &'d D,
	ctx: *mut sys::ALCcontext,
	exts: ext::AlCache,
}


pub struct Buffer<'d>(sys::ALuint, &'d DeviceTrait);


pub struct Source<'c, C: ContextTrait + 'c>(sys::ALuint, &'c C);


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


impl From<ext::ExtensionError> for AlError {
	fn from(_: ext::ExtensionError) -> AlError {
		AlError::ExtensionNotPresent
	}
}


impl<'d, D: DeviceTrait> Context<'d, D> {
	#[doc(hidden)]
	pub unsafe fn new(dev: &D, api: Arc<sys::AlApi>, ctx: *mut sys::ALCcontext, exts: ext::AlCache) -> Context<D> {
		Context{
			api: api,
			dev: dev,
			ctx: ctx,
			exts: exts,
		}
	}


	pub fn is_extension_present(&self, ext: ext::Al) -> bool {
		match ext {
			ext::Al::ALaw => self.exts.AL_EXT_ALAW().is_ok(),
			ext::Al::BFormat => self.exts.AL_EXT_BFORMAT().is_ok(),
			ext::Al::Double => self.exts.AL_EXT_double().is_ok(),
			ext::Al::Float32 => self.exts.AL_EXT_float32().is_ok(),
			ext::Al::Ima4 => self.exts.AL_EXT_IMA4().is_ok(),
			ext::Al::McFormats => self.exts.AL_EXT_MCFORMATS().is_ok(),
			ext::Al::MuLaw => self.exts.AL_EXT_MULAW().is_ok(),
			ext::Al::MuLawBFormat => self.exts.AL_EXT_MULAW_BFORMAT().is_ok(),
			ext::Al::MuLawMcFormats => self.exts.AL_EXT_MULAW_MCFORMATS().is_ok(),
			ext::Al::SoftBlockAlignment => self.exts.AL_SOFT_block_alignment().is_ok(),
//			ext::Al::SoftBufferSamples => self.ext.AL_SOFT_buffer_samples().is_ok(),
//			ext::Al::SoftBufferSubData => self.ext.AL_SOFT_buffer_sub_data().is_ok(),
			ext::Al::SoftDeferredUpdates => self.exts.AL_SOFT_deferred_updates().is_ok(),
			ext::Al::SoftDirectChannels => self.exts.AL_SOFT_direct_channels().is_ok(),
			ext::Al::SoftLoopPoints => self.exts.AL_SOFT_loop_points().is_ok(),
			ext::Al::SoftMsadpcm => self.exts.AL_SOFT_MSADPCM().is_ok(),
			ext::Al::SoftSourceLatency => self.exts.AL_SOFT_source_latency().is_ok(),
			ext::Al::SoftSourceLength => self.exts.AL_SOFT_source_length().is_ok(),
			ext::Al::SourceDistanceModel => self.exts.AL_EXT_source_distance_model().is_ok(),
		}
	}


	fn get_error(&self) -> AlResult<()> {
		match unsafe { (*self.api.alGetError)() } {
			sys::AL_NO_ERROR => Ok(()),
			e => Err(e.into())
		}
	}
}


impl<'d, D: DeviceTrait> ContextTrait for Context<'d, D> {
	#[inline(always)]
	fn exts(&self) -> &ext::AlCache { &self.exts }
}


unsafe impl<'d, D: DeviceTrait> Send for Context<'d, D> { }


