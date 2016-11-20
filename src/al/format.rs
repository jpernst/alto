use sys;
use al::*;


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Format {
	Standard(StandardFormat),
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


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum StandardFormat {
	MonoU8,
	MonoI16,
	StereoU8,
	StereoI16,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtALawFormat {
	Mono,
	Stereo,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtBFormat {
	B2DU8,
	B2DI16,
	B2DF32,
	B3DU8,
	B3DI16,
	B3DF32,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtDoubleFormat {
	Mono,
	Stereo,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtFloat32Format {
	Mono,
	Stereo,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtIma4Format {
	Mono,
	Stereo,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtMcFormat {
	QuadU8,
	QuadI16,
	QuadF32,
	RearU8,
	RearI16,
	RearF32,
	Mc51ChnU8,
	Mc51ChnI16,
	Mc51ChnF32,
	Mc61ChnU8,
	Mc61ChnI16,
	Mc61ChnF32,
	Mc71ChnU8,
	Mc71ChnI16,
	Mc71ChnF32,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtMuLawFormat {
	Mono,
	Stereo,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtMuLawBFormat {
	B2D,
	B3D,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtMuLawMcFormat {
	Mono,
	Stereo,
	Quad,
	Rear,
	Mc51Chn,
	Mc61Chn,
	Mc71Chn,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SoftMsadpcmFormat {
	Mono,
	Stereo,
}


pub unsafe trait SampleFrame: Copy {
	type Unit: Copy;

	fn len() -> usize;
	fn format() -> Format;
}


pub trait Block {
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ALawSample(pub u8);
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MuLawSample(pub u8);


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Mono<S: Copy> {
	center: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Stereo<S: Copy> {
	left: S,
	right: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct McRear<S: Copy> {
	rear: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct McQuad<S: Copy> {
	front_left: S,
	front_right: S,
	back_left: S,
	back_right: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Mc51Chn<S: Copy> {
	front_left: S,
	front_right: S,
	front_center: S,
	low_freq: S,
	back_left: S,
	back_right: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Mc61Chn<S: Copy> {
	front_left: S,
	front_right: S,
	front_center: S,
	low_freq: S,
	back_left: S,
	back_right: S,
	back_center: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Mc71Chn<S: Copy> {
	front_left: S,
	front_right: S,
	front_center: S,
	low_freq: S,
	back_left: S,
	back_right: S,
	side_left: S,
	side_right: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BFormat2D<S: Copy> {
	pub w: S,
	pub x: S,
	pub y: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BFormat3D<S: Copy> {
	pub w: S,
	pub x: S,
	pub y: S,
	pub z: S,
}


impl Format {
	pub fn into_raw<C: ContextTrait>(self, ctx: Option<&C>) -> AlResult<sys::ALint> {
		match self {
			Format::Standard(f) => Ok(f.into_raw()),
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


impl StandardFormat {
	pub fn into_raw(self) -> sys::ALint {
		match self {
			StandardFormat::MonoU8 => sys::AL_FORMAT_MONO8,
			StandardFormat::MonoI16 => sys::AL_FORMAT_MONO16,
			StandardFormat::StereoU8 => sys::AL_FORMAT_STEREO8,
			StandardFormat::StereoI16 => sys::AL_FORMAT_STEREO16,
		}
	}
}


impl ExtALawFormat {
	pub fn into_raw<C: ContextTrait>(self, ctx: Option<&C>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			ExtALawFormat::Mono => Ok(ctx.exts().AL_EXT_ALAW()?.AL_FORMAT_MONO_ALAW_EXT?),
			ExtALawFormat::Stereo => Ok(ctx.exts().AL_EXT_ALAW()?.AL_FORMAT_STEREO_ALAW_EXT?),
		})
	}
}


impl ExtBFormat {
	pub fn into_raw<C: ContextTrait>(self, ctx: Option<&C>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			ExtBFormat::B2DU8 => Ok(ctx.exts().AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT2D_8?),
			ExtBFormat::B2DI16 => Ok(ctx.exts().AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT2D_16?),
			ExtBFormat::B2DF32 => Ok(ctx.exts().AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT2D_FLOAT32?),
			ExtBFormat::B3DU8 => Ok(ctx.exts().AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT3D_8?),
			ExtBFormat::B3DI16 => Ok(ctx.exts().AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT3D_16?),
			ExtBFormat::B3DF32 => Ok(ctx.exts().AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT3D_FLOAT32?),
		})
	}
}


impl ExtDoubleFormat {
	pub fn into_raw<C: ContextTrait>(self, ctx: Option<&C>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			ExtDoubleFormat::Mono => Ok(ctx.exts().AL_EXT_double()?.AL_FORMAT_MONO_DOUBLE_EXT?),
			ExtDoubleFormat::Stereo => Ok(ctx.exts().AL_EXT_double()?.AL_FORMAT_STEREO_DOUBLE_EXT?),
		})
	}
}


impl ExtFloat32Format {
	pub fn into_raw<C: ContextTrait>(self, ctx: Option<&C>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			ExtFloat32Format::Mono => Ok(ctx.exts().AL_EXT_float32()?.AL_FORMAT_MONO_FLOAT32?),
			ExtFloat32Format::Stereo => Ok(ctx.exts().AL_EXT_float32()?.AL_FORMAT_STEREO_FLOAT32?),
		})
	}
}


impl ExtIma4Format {
	pub fn into_raw<C: ContextTrait>(self, ctx: Option<&C>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			ExtIma4Format::Mono => Ok(ctx.exts().AL_EXT_IMA4()?.AL_FORMAT_MONO_IMA4?),
			ExtIma4Format::Stereo => Ok(ctx.exts().AL_EXT_IMA4()?.AL_FORMAT_STEREO_IMA4?),
		})
	}
}


impl ExtMcFormat {
	pub fn into_raw<C: ContextTrait>(self, ctx: Option<&C>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			ExtMcFormat::QuadU8 => Ok(ctx.exts().AL_EXT_MCFORMATS()?.AL_FORMAT_QUAD8?),
			ExtMcFormat::QuadI16 => Ok(ctx.exts().AL_EXT_MCFORMATS()?.AL_FORMAT_QUAD16?),
			ExtMcFormat::QuadF32 => Ok(ctx.exts().AL_EXT_MCFORMATS()?.AL_FORMAT_QUAD32?),
			ExtMcFormat::RearU8 => Ok(ctx.exts().AL_EXT_MCFORMATS()?.AL_FORMAT_REAR8?),
			ExtMcFormat::RearI16 => Ok(ctx.exts().AL_EXT_MCFORMATS()?.AL_FORMAT_REAR16?),
			ExtMcFormat::RearF32 => Ok(ctx.exts().AL_EXT_MCFORMATS()?.AL_FORMAT_REAR32?),
			ExtMcFormat::Mc51ChnU8 => Ok(ctx.exts().AL_EXT_MCFORMATS()?.AL_FORMAT_51CHN8?),
			ExtMcFormat::Mc51ChnI16 => Ok(ctx.exts().AL_EXT_MCFORMATS()?.AL_FORMAT_51CHN16?),
			ExtMcFormat::Mc51ChnF32 => Ok(ctx.exts().AL_EXT_MCFORMATS()?.AL_FORMAT_51CHN32?),
			ExtMcFormat::Mc61ChnU8 => Ok(ctx.exts().AL_EXT_MCFORMATS()?.AL_FORMAT_61CHN8?),
			ExtMcFormat::Mc61ChnI16 => Ok(ctx.exts().AL_EXT_MCFORMATS()?.AL_FORMAT_61CHN16?),
			ExtMcFormat::Mc61ChnF32 => Ok(ctx.exts().AL_EXT_MCFORMATS()?.AL_FORMAT_61CHN32?),
			ExtMcFormat::Mc71ChnU8 => Ok(ctx.exts().AL_EXT_MCFORMATS()?.AL_FORMAT_71CHN8?),
			ExtMcFormat::Mc71ChnI16 => Ok(ctx.exts().AL_EXT_MCFORMATS()?.AL_FORMAT_71CHN16?),
			ExtMcFormat::Mc71ChnF32 => Ok(ctx.exts().AL_EXT_MCFORMATS()?.AL_FORMAT_71CHN32?),
		})
	}
}


impl ExtMuLawFormat {
	pub fn into_raw<C: ContextTrait>(self, ctx: Option<&C>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			ExtMuLawFormat::Mono => Ok(ctx.exts().AL_EXT_MULAW()?.AL_FORMAT_MONO_MULAW_EXT?),
			ExtMuLawFormat::Stereo => Ok(ctx.exts().AL_EXT_MULAW()?.AL_FORMAT_STEREO_MULAW_EXT?),
		})
	}
}


impl ExtMuLawBFormat {
	pub fn into_raw<C: ContextTrait>(self, ctx: Option<&C>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			ExtMuLawBFormat::B2D => Ok(ctx.exts().AL_EXT_MULAW_BFORMAT()?.AL_FORMAT_BFORMAT2D_MULAW?),
			ExtMuLawBFormat::B3D => Ok(ctx.exts().AL_EXT_MULAW_BFORMAT()?.AL_FORMAT_BFORMAT3D_MULAW?),
		})
	}
}


impl ExtMuLawMcFormat {
	pub fn into_raw<C: ContextTrait>(self, ctx: Option<&C>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			ExtMuLawMcFormat::Mono => Ok(ctx.exts().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_MONO_MULAW?),
			ExtMuLawMcFormat::Stereo => Ok(ctx.exts().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_STEREO_MULAW?),
			ExtMuLawMcFormat::Quad => Ok(ctx.exts().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_QUAD_MULAW?),
			ExtMuLawMcFormat::Rear => Ok(ctx.exts().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_REAR_MULAW?),
			ExtMuLawMcFormat::Mc51Chn => Ok(ctx.exts().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_51CHN_MULAW?),
			ExtMuLawMcFormat::Mc61Chn => Ok(ctx.exts().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_61CHN_MULAW?),
			ExtMuLawMcFormat::Mc71Chn => Ok(ctx.exts().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_71CHN_MULAW?),
		})
	}
}


impl SoftMsadpcmFormat {
	pub fn into_raw<C: ContextTrait>(self, ctx: Option<&C>) -> AlResult<sys::ALint> {
		ctx.ok_or(AlError::ExtensionNotPresent).and_then(|ctx| match self {
			SoftMsadpcmFormat::Mono => Ok(ctx.exts().AL_SOFT_MSADPCM()?.AL_FORMAT_MONO_MSADPCM_SOFT?),
			SoftMsadpcmFormat::Stereo => Ok(ctx.exts().AL_SOFT_MSADPCM()?.AL_FORMAT_STEREO_MSADPCM_SOFT?),
		})
	}
}


unsafe impl SampleFrame for Mono<u8> {
	type Unit = u8;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::Standard(StandardFormat::MonoU8) }
}


unsafe impl SampleFrame for Mono<i16> {
	type Unit = i16;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::Standard(StandardFormat::MonoI16) }
}


unsafe impl SampleFrame for Mono<f32> {
	type Unit = f32;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtFloat32(ExtFloat32Format::Mono) }
}


unsafe impl SampleFrame for Mono<f64> {
	type Unit = f64;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtDouble(ExtDoubleFormat::Mono) }
}


unsafe impl SampleFrame for Mono<ALawSample> {
	type Unit = ALawSample;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtALaw(ExtALawFormat::Mono) }
}


unsafe impl SampleFrame for Mono<MuLawSample> {
	type Unit = MuLawSample;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLaw(ExtMuLawFormat::Mono) }
}


unsafe impl SampleFrame for Stereo<u8> {
	type Unit = u8;

	#[inline(always)] fn len() -> usize { 2 }
	#[inline(always)] fn format() -> Format { Format::Standard(StandardFormat::StereoU8) }
}


unsafe impl SampleFrame for Stereo<i16> {
	type Unit = i16;

	#[inline(always)] fn len() -> usize { 2 }
	#[inline(always)] fn format() -> Format { Format::Standard(StandardFormat::StereoI16) }
}


unsafe impl SampleFrame for Stereo<f32> {
	type Unit = f32;

	#[inline(always)] fn len() -> usize { 2 }
	#[inline(always)] fn format() -> Format { Format::ExtFloat32(ExtFloat32Format::Stereo) }
}


unsafe impl SampleFrame for Stereo<f64> {
	type Unit = f64;

	#[inline(always)] fn len() -> usize { 2 }
	#[inline(always)] fn format() -> Format { Format::ExtDouble(ExtDoubleFormat::Stereo) }
}


unsafe impl SampleFrame for Stereo<ALawSample> {
	type Unit = ALawSample;

	#[inline(always)] fn len() -> usize { 2 }
	#[inline(always)] fn format() -> Format { Format::ExtALaw(ExtALawFormat::Stereo) }
}


unsafe impl SampleFrame for Stereo<MuLawSample> {
	type Unit = MuLawSample;

	#[inline(always)] fn len() -> usize { 2 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLaw(ExtMuLawFormat::Stereo) }
}


unsafe impl SampleFrame for McRear<u8> {
	type Unit = u8;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::RearU8) }
}


unsafe impl SampleFrame for McRear<i16> {
	type Unit = i16;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::RearI16)  }
}


unsafe impl SampleFrame for McRear<f32> {
	type Unit = f32;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::RearF32) }
}


unsafe impl SampleFrame for McRear<MuLawSample> {
	type Unit = MuLawSample;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawMcFormats(ExtMuLawMcFormat::Rear) }
}


unsafe impl SampleFrame for McQuad<u8> {
	type Unit = u8;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::QuadU8) }
}


unsafe impl SampleFrame for McQuad<i16> {
	type Unit = i16;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::QuadI16)  }
}


unsafe impl SampleFrame for McQuad<f32> {
	type Unit = f32;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::QuadF32) }
}


unsafe impl SampleFrame for McQuad<MuLawSample> {
	type Unit = MuLawSample;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawMcFormats(ExtMuLawMcFormat::Quad) }
}


unsafe impl SampleFrame for Mc51Chn<u8> {
	type Unit = u8;

	#[inline(always)] fn len() -> usize { 6 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc51ChnU8) }
}


unsafe impl SampleFrame for Mc51Chn<i16> {
	type Unit = i16;

	#[inline(always)] fn len() -> usize { 6 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc51ChnI16)  }
}


unsafe impl SampleFrame for Mc51Chn<f32> {
	type Unit = f32;

	#[inline(always)] fn len() -> usize { 6 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc51ChnF32) }
}


unsafe impl SampleFrame for Mc51Chn<MuLawSample> {
	type Unit = MuLawSample;

	#[inline(always)] fn len() -> usize { 6 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawMcFormats(ExtMuLawMcFormat::Mc51Chn) }
}


unsafe impl SampleFrame for Mc61Chn<u8> {
	type Unit = u8;

	#[inline(always)] fn len() -> usize { 7 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc61ChnU8) }
}


unsafe impl SampleFrame for Mc61Chn<i16> {
	type Unit = i16;

	#[inline(always)] fn len() -> usize { 7 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc61ChnI16)  }
}


unsafe impl SampleFrame for Mc61Chn<f32> {
	type Unit = f32;

	#[inline(always)] fn len() -> usize { 7 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc61ChnF32) }
}


unsafe impl SampleFrame for Mc61Chn<MuLawSample> {
	type Unit = MuLawSample;

	#[inline(always)] fn len() -> usize { 7 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawMcFormats(ExtMuLawMcFormat::Mc61Chn) }
}


unsafe impl SampleFrame for Mc71Chn<u8> {
	type Unit = u8;

	#[inline(always)] fn len() -> usize { 8 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc71ChnU8) }
}


unsafe impl SampleFrame for Mc71Chn<i16> {
	type Unit = i16;

	#[inline(always)] fn len() -> usize { 8 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc71ChnI16)  }
}


unsafe impl SampleFrame for Mc71Chn<f32> {
	type Unit = f32;

	#[inline(always)] fn len() -> usize { 8 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc71ChnF32) }
}


unsafe impl SampleFrame for Mc71Chn<MuLawSample> {
	type Unit = MuLawSample;

	#[inline(always)] fn len() -> usize { 8 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawMcFormats(ExtMuLawMcFormat::Mc71Chn) }
}


unsafe impl SampleFrame for BFormat2D<u8> {
	type Unit = u8;

	#[inline(always)] fn len() -> usize { 3 }
	#[inline(always)] fn format() -> Format { Format::ExtBFormat(ExtBFormat::B2DU8) }
}


unsafe impl SampleFrame for BFormat2D<i16> {
	type Unit = i16;

	#[inline(always)] fn len() -> usize { 3 }
	#[inline(always)] fn format() -> Format { Format::ExtBFormat(ExtBFormat::B2DI16) }
}


unsafe impl SampleFrame for BFormat2D<f32> {
	type Unit = f32;

	#[inline(always)] fn len() -> usize { 3 }
	#[inline(always)] fn format() -> Format { Format::ExtBFormat(ExtBFormat::B2DF32) }
}


unsafe impl SampleFrame for BFormat2D<MuLawSample> {
	type Unit = MuLawSample;

	#[inline(always)] fn len() -> usize { 3 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawBFormat(ExtMuLawBFormat::B2D) }
}


unsafe impl SampleFrame for BFormat3D<u8> {
	type Unit = u8;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtBFormat(ExtBFormat::B3DU8) }
}


unsafe impl SampleFrame for BFormat3D<i16> {
	type Unit = i16;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtBFormat(ExtBFormat::B3DI16) }
}


unsafe impl SampleFrame for BFormat3D<f32> {
	type Unit = f32;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtBFormat(ExtBFormat::B3DF32) }
}


unsafe impl SampleFrame for BFormat3D<MuLawSample> {
	type Unit = MuLawSample;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawBFormat(ExtMuLawBFormat::B3D) }
}


