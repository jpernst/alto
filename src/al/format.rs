use sys;
use al::*;


pub enum Format {
	MonoU8,
	MonoI16,
	StereoU8,
	StereoI16,

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
	QuadU8,
	QuadI16,
	QuadF32,
	RearU8,
	RearI16,
	RearF32,
	Mc51U8,
	Mc51I16,
	Mc51F32,
	Mc61U8,
	Mc61I16,
	Mc61F32,
	Mc71U8,
	Mc71I16,
	Mc71F32,
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
	Mc51,
	Mc61,
	Mc71,
}


pub enum SoftMsadpcmFormat {
	Mono,
	Stereo,
}


pub unsafe trait SampleFrame: Copy {
	type Sample: Copy;

	fn format() -> Format;
}


pub struct Mono<S: Copy> {
	center: S,
}


pub struct Stereo<S: Copy> {
	left: S,
	right: S,
}


pub struct McRear<S: Copy> {
	rear: S,
}


pub struct McQuad<S: Copy> {
	front_left: S,
	front_right: S,
	back_left: S,
	back_right: S,
}


pub struct Mc51<S: Copy> {
	front_left: S,
	front_right: S,
	front_center: S,
	low_freq: S,
	back_left: S,
	back_right: S,
}


pub struct Mc61<S: Copy> {
	front_left: S,
	front_right: S,
	front_center: S,
	low_freq: S,
	back_left: S,
	back_right: S,
	back_center: S,
}


pub struct Mc71<S: Copy> {
	front_left: S,
	front_right: S,
	front_center: S,
	low_freq: S,
	back_left: S,
	back_right: S,
	side_left: S,
	side_right: S,
}


pub struct BFormat2D<S: Copy> {
	pub w: S,
	pub x: S,
	pub y: S,
}


pub struct BFormat3D<S: Copy> {
	pub w: S,
	pub x: S,
	pub y: S,
	pub z: S,
}


pub struct ALawMono {
	center: u8,
}


pub struct ALawStereo {
	left: u8,
	right: u8,
}


pub struct MuLawMono {
	center: u8,
}


pub struct MuLawStereo {
	left: u8,
	right: u8,
}


pub struct MuLawMcRear {
	rear: u8,
}


pub struct MuLawMcQuad {
	front_left: u8,
	front_right: u8,
	back_left: u8,
	back_right: u8,
}


pub struct MuLawMc51 {
	front_left: u8,
	front_right: u8,
	front_center: u8,
	low_freq: u8,
	back_left: u8,
	back_right: u8,
}


pub struct MuLawMc61 {
	front_left: u8,
	front_right: u8,
	front_center: u8,
	low_freq: u8,
	back_left: u8,
	back_right: u8,
	back_center: u8,
}


pub struct MuLawMc71 {
	front_left: u8,
	front_right: u8,
	front_center: u8,
	low_freq: u8,
	back_left: u8,
	back_right: u8,
	side_left: u8,
	side_right: u8,
}


impl Format {
	pub fn into_raw(self, ctx: Option<&Context>) -> AlResult<sys::ALint> {
		match self {
			Format::MonoU8 => Ok(sys::AL_FORMAT_MONO8),
			Format::MonoI16 => Ok(sys::AL_FORMAT_MONO16),
			Format::StereoU8 => Ok(sys::AL_FORMAT_STEREO8),
			Format::StereoI16 => Ok(sys::AL_FORMAT_STEREO16),

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
			ExtMcFormat::QuadU8 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_QUAD8?),
			ExtMcFormat::QuadI16 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_QUAD16?),
			ExtMcFormat::QuadF32 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_QUAD32?),
			ExtMcFormat::RearU8 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_REAR8?),
			ExtMcFormat::RearI16 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_REAR16?),
			ExtMcFormat::RearF32 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_REAR32?),
			ExtMcFormat::Mc51U8 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_51CHN8?),
			ExtMcFormat::Mc51I16 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_51CHN16?),
			ExtMcFormat::Mc51F32 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_51CHN32?),
			ExtMcFormat::Mc61U8 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_61CHN8?),
			ExtMcFormat::Mc61I16 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_61CHN16?),
			ExtMcFormat::Mc61F32 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_61CHN32?),
			ExtMcFormat::Mc71U8 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_71CHN8?),
			ExtMcFormat::Mc71I16 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_71CHN16?),
			ExtMcFormat::Mc71F32 => Ok(ctx.cache.AL_EXT_MCFORMATS()?.AL_FORMAT_71CHN32?),
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
			ExtMuLawMcFormat::Mc51 => Ok(ctx.cache.AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_51CHN_MULAW?),
			ExtMuLawMcFormat::Mc61 => Ok(ctx.cache.AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_61CHN_MULAW?),
			ExtMuLawMcFormat::Mc71 => Ok(ctx.cache.AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_71CHN_MULAW?),
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


