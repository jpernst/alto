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

	ExtALaw(ExtALawFormat),
	ExtBFormat(ExtBFormat),
	ExtDouble(ExtDoubleFormat),
	ExtFloat32(ExtFloat32Format),
	ExtIma4(ExtIma4Format),
	ExtMcFormats(ExtMcFormat),
	ExtMuLaw(ExtMuLawFormat),
	ExtMuLawBFormat(ExtMuLawBFormat),
	ExtMuLawMcFormats(ExtMuLawMcFormat),
	SoftMsadpcm(SoftMsadpcmFormat),
}


pub enum ExtALawFormat {
	Mono,
	Stereo,
}


pub enum ExtBFormat {
	B2D8,
	B2D16,
	B2DF32,
	B3D8,
	B3D16,
	B3DF32,
}


pub enum ExtDoubleFormat {
	Mono,
	Stereo,
}


pub enum ExtFloat32Format {
	Mono,
	Stereo,
}


pub enum ExtIma4Format {
	Mono,
	Stereo,
}


pub enum ExtMcFormat {
	Quad8,
	Quad16,
	Quad32,
	Rear8,
	Rear16,
	Rear32,
	Mc51Chn8,
	Mc51Chn16,
	Mc51Chn32,
	Mc61Chn8,
	Mc61Chn16,
	Mc61Chn32,
	Mc71Chn8,
	Mc71Chn16,
	Mc71Chn32,
}


pub enum ExtMuLawFormat {
	Mono,
	Stereo,
}


pub enum ExtMuLawBFormat {
	B2D,
	B3D,
}


pub enum ExtMuLawMcFormat {
	Mono,
	Stereo,
	Quad,
	Rear,
	Mc51Chn,
	Mc61Chn,
	Mc71Chn,
}


pub enum SoftMsadpcmFormat {
	Mono,
	Stereo,
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
		e => unsafe { Err(e.into()) }
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


impl Format {
	pub fn into_raw(self, ctx: Option<&Context>) -> AlResult<sys::ALint> {
		match self {
			Format::Mono8 => Ok(sys::AL_FORMAT_MONO8),
			Format::Mono16 => Ok(sys::AL_FORMAT_MONO16),
			Format::Stereo8 => Ok(sys::AL_FORMAT_STEREO8),
			Format::Stereo16 => Ok(sys::AL_FORMAT_STEREO16),

			Format::ExtALaw(f) => f.into_raw(ctx),
			Format::ExtBFormat(f) => f.into_raw(ctx),
			Format::ExtDouble(f) => f.into_raw(ctx),
			Format::ExtFloat32(f) => f.into_raw(ctx),
			Format::ExtIma4(f) => f.into_raw(ctx),
			Format::ExtMcFormats(f) => f.into_raw(ctx),
			Format::ExtMuLaw(f) => f.into_raw(ctx),
			Format::ExtMuLawBFormat(f) => f.into_raw(ctx),
			Format::ExtMuLawMcFormats(f) => f.into_raw(ctx),
			Format::SoftMsadpcm(f) => f.into_raw(ctx),
		}
	}
}


impl ExtALawFormat {
	pub fn into_raw(self, ctx: Option<&Context>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			ExtALawFormat::Mono => Ok(ctx.cache.AL_EXT_ALAW()?.AL_FORMAT_MONO_ALAW_EXT?),
			ExtALawFormat::Stereo => Ok(ctx.cache.AL_EXT_ALAW()?.AL_FORMAT_STEREO_ALAW_EXT?),
		})
	}
}


impl ExtBFormat {
	pub fn into_raw(self, ctx: Option<&Context>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			ExtBFormat::B2D8 => Ok(ctx.cache.AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT2D_8?),
			ExtBFormat::B2D16 => Ok(ctx.cache.AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT2D_16?),
			ExtBFormat::B2DF32 => Ok(ctx.cache.AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT2D_FLOAT32?),
			ExtBFormat::B3D8 => Ok(ctx.cache.AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT3D_8?),
			ExtBFormat::B3D16 => Ok(ctx.cache.AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT3D_16?),
			ExtBFormat::B3DF32 => Ok(ctx.cache.AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT3D_FLOAT32?),
		})
	}
}


impl ExtDoubleFormat {
	pub fn into_raw(self, ctx: Option<&Context>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			ExtDoubleFormat::Mono => Ok(ctx.cache.AL_EXT_double()?.AL_FORMAT_MONO_DOUBLE_EXT?),
			ExtDoubleFormat::Stereo => Ok(ctx.cache.AL_EXT_double()?.AL_FORMAT_STEREO_DOUBLE_EXT?),
		})
	}
}


impl ExtFloat32Format {
	pub fn into_raw(self, ctx: Option<&Context>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			ExtFloat32Format::Mono => Ok(ctx.cache.AL_EXT_float32()?.AL_FORMAT_MONO_FLOAT32?),
			ExtFloat32Format::Stereo => Ok(ctx.cache.AL_EXT_float32()?.AL_FORMAT_STEREO_FLOAT32?),
		})
	}
}


impl ExtIma4Format {
	pub fn into_raw(self, ctx: Option<&Context>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			ExtIma4Format::Mono => Ok(ctx.cache.AL_EXT_IMA4()?.AL_FORMAT_MONO_IMA4?),
			ExtIma4Format::Stereo => Ok(ctx.cache.AL_EXT_IMA4()?.AL_FORMAT_STEREO_IMA4?),
		})
	}
}


impl ExtMcFormat {
	pub fn into_raw(self, ctx: Option<&Context>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			ExtMcFormat::Quad8 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_QUAD8?),
			ExtMcFormat::Quad16 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_QUAD16?),
			ExtMcFormat::Quad32 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_QUAD32?),
			ExtMcFormat::Rear8 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_REAR8?),
			ExtMcFormat::Rear16 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_REAR16?),
			ExtMcFormat::Rear32 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_REAR32?),
			ExtMcFormat::Mc51Chn8 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_51CHN8?),
			ExtMcFormat::Mc51Chn16 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_51CHN16?),
			ExtMcFormat::Mc51Chn32 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_51CHN32?),
			ExtMcFormat::Mc61Chn8 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_61CHN8?),
			ExtMcFormat::Mc61Chn16 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_61CHN16?),
			ExtMcFormat::Mc61Chn32 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_61CHN32?),
			ExtMcFormat::Mc71Chn8 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_71CHN8?),
			ExtMcFormat::Mc71Chn16 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_71CHN16?),
			ExtMcFormat::Mc71Chn32 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_71CHN32?),
		})
	}
}


impl ExtMuLawFormat {
	pub fn into_raw(self, ctx: Option<&Context>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			ExtMuLawFormat::Mono => Ok(ctx.cache.AL_EXT_MULAW()?.AL_FORMAT_MONO_MULAW_EXT?),
			ExtMuLawFormat::Stereo => Ok(ctx.cache.AL_EXT_MULAW()?.AL_FORMAT_STEREO_MULAW_EXT?),
		})
	}
}


impl ExtMuLawBFormat {
	pub fn into_raw(self, ctx: Option<&Context>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			ExtMuLawBFormat::B2D => Ok(ctx.cache.AL_EXT_MULAW_BFORMAT()?.AL_FORMAT_BFORMAT2D_MULAW?),
			ExtMuLawBFormat::B3D => Ok(ctx.cache.AL_EXT_MULAW_BFORMAT()?.AL_FORMAT_BFORMAT3D_MULAW?),
		})
	}
}


impl ExtMuLawMcFormat {
	pub fn into_raw(self, ctx: Option<&Context>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			ExtMuLawMcFormat::Mono => Ok(ctx.cache.AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_MONO_MULAW?),
			ExtMuLawMcFormat::Stereo => Ok(ctx.cache.AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_STEREO_MULAW?),
			ExtMuLawMcFormat::Quad => Ok(ctx.cache.AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_QUAD_MULAW?),
			ExtMuLawMcFormat::Rear => Ok(ctx.cache.AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_REAR_MULAW?),
			ExtMuLawMcFormat::Mc51Chn => Ok(ctx.cache.AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_51CHN_MULAW?),
			ExtMuLawMcFormat::Mc61Chn => Ok(ctx.cache.AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_61CHN_MULAW?),
			ExtMuLawMcFormat::Mc71Chn => Ok(ctx.cache.AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_71CHN_MULAW?),
		})
	}
}


impl SoftMsadpcmFormat {
	pub fn into_raw(self, ctx: Option<&Context>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			SoftMsadpcmFormat::Mono => Ok(ctx.cache.AL_SOFT_MSADPCM()?.AL_FORMAT_MONO_MSADPCM_SOFT?),
			SoftMsadpcmFormat::Stereo => Ok(ctx.cache.AL_SOFT_MSADPCM()?.AL_FORMAT_STEREO_MSADPCM_SOFT?),
		})
	}
}


impl<'d> Context<'d> {
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
			ext::Al::SoftBufferSamples => self.cache.AL_SOFT_buffer_samples().is_ok(),
			ext::Al::SoftBufferSubData => self.cache.AL_SOFT_buffer_sub_data().is_ok(),
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


