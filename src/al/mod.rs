use std::sync::{Arc, Mutex};
use std::fmt;
use std::error::Error as StdError;
use std::collections::VecDeque;

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


pub struct Context<'d, D: DeviceTrait + 'd> {
	dev: &'d D,
	api: &'d AlApi<'static>,
	ctx_lock: &'d Mutex<()>,
	ctx: *mut sys::ALCcontext,
	exts: ext::AlCache<'d>,
}


pub struct Buffer<'d, D: DeviceTrait + 'd> {
	dev: &'d D,
	hnd: sys::ALuint, 
}


pub struct StaticSource<'d: 'c, 'c, D: DeviceTrait + 'd> {
	ctx: &'c Context<'d, D>,
	hnd: sys::ALuint,
	buf: Option<Arc<Buffer<'d, D>>>,
}


pub struct StreamingSource<'d: 'c, 'c, D: DeviceTrait + 'd> {
	ctx: &'c Context<'d, D>,
	hnd: sys::ALuint,
	buf: VecDeque<Buffer<'d, D>>,
}


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
	pub unsafe fn new(dev: &'d D, api: &'d AlApi<'static>, ctx_lock: &'d Mutex<()>, ctx: *mut sys::ALCcontext) -> Context<'d, D> {
		Context{
			dev: dev,
			api: api,
			ctx_lock: ctx_lock,
			ctx: ctx,
			exts: ext::AlCache::new(api.owner()),
		}
	}


	pub fn new_source() {
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


	#[inline(always)]
	fn extensions(&self) -> &ext::AlCache { &self.exts }


	fn get_error(&self) -> AlResult<()> {
		match unsafe { self.api.owner().alGetError()() } {
			sys::AL_NO_ERROR => Ok(()),
			e => Err(e.into())
		}
	}
}


unsafe impl<'d, D: DeviceTrait> Send for Context<'d, D> { }
unsafe impl<'d, D: DeviceTrait> Sync for Context<'d, D> { }


impl<'d, D: DeviceTrait + 'd> Buffer<'d, D> {
}


impl<'d: 'c, 'c, D: DeviceTrait + 'd> StaticSource<'d, 'c, D> {
	pub fn set_buffer(self, buf: Option<Arc<Buffer<'d, D>>>) -> AlResult<()> {
		panic!();
	}
}


impl<'d: 'c, 'c, D: DeviceTrait + 'd> StreamingSource<'d, 'c, D> {
}
