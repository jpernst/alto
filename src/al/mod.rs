use std::sync::Mutex;
use std::fmt;
use std::error::Error as StdError;

use ::sys;
use ::alc::*;
use ::ext;


mod format;
pub use self::format::*;


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


pub type AlResult<T> = ::std::result::Result<T, AlError>;


pub struct Context<'d> {
	dev: &'d OutputDevice,
	ctx: *mut sys::ALCcontext,
	cache: ext::AlCache,
}


pub struct Buffer<'d>(sys::ALuint, &'d OutputDevice);


pub struct Source<'c>(sys::ALuint, &'c Context<'c>);


fn get_error() -> AlResult<()> {
	match unsafe { sys::alGetError() } {
		sys::AL_NO_ERROR => Ok(()),
		e => Err(e.into())
	}
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


impl<'d> Context<'d> {
	#[doc(hidden)]
	pub unsafe fn new(dev: &OutputDevice, ctx: *mut sys::ALCcontext) -> Context {
		Context{
			dev: dev,
			ctx: ctx,
			cache: ext::AlCache::new(),
		}
	}


	pub fn is_extension_present(&self, ext: ext::Al) -> bool {
		match ext {
			ext::Al::ALaw => self.cache.AL_EXT_ALAW().is_ok(),
			ext::Al::BFormat => self.cache.AL_EXT_BFORMAT().is_ok(),
			ext::Al::Double => self.cache.AL_EXT_double().is_ok(),
			ext::Al::Float32 => self.cache.AL_EXT_float32().is_ok(),
			ext::Al::Ima4 => self.cache.AL_EXT_IMA4().is_ok(),
			ext::Al::McFormats => self.cache.AL_EXT_MCFORMATS().is_ok(),
			ext::Al::MuLaw => self.cache.AL_EXT_MULAW().is_ok(),
			ext::Al::MuLawBFormat => self.cache.AL_EXT_MULAW_BFORMAT().is_ok(),
			ext::Al::MuLawMcFormats => self.cache.AL_EXT_MULAW_MCFORMATS().is_ok(),
			ext::Al::SoftBlockAlignment => self.cache.AL_SOFT_block_alignment().is_ok(),
//			ext::Al::SoftBufferSamples => self.cache.AL_SOFT_buffer_samples().is_ok(),
//			ext::Al::SoftBufferSubData => self.cache.AL_SOFT_buffer_sub_data().is_ok(),
			ext::Al::SoftDeferredUpdates => self.cache.AL_SOFT_deferred_updates().is_ok(),
			ext::Al::SoftDirectChannels => self.cache.AL_SOFT_direct_channels().is_ok(),
			ext::Al::SoftLoopPoints => self.cache.AL_SOFT_loop_points().is_ok(),
			ext::Al::SoftMsadpcm => self.cache.AL_SOFT_MSADPCM().is_ok(),
			ext::Al::SoftSourceLatency => self.cache.AL_SOFT_source_latency().is_ok(),
			ext::Al::SoftSourceLength => self.cache.AL_SOFT_source_length().is_ok(),
			ext::Al::SourceDistanceModel => self.cache.AL_EXT_source_distance_model().is_ok(),
		}
	}
}


unsafe impl<'d> Send for Context<'d> { }


