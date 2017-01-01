use std::slice;

use ::{AltoError, AltoResult};
use sys;
use alc::*;
use al::*;
use ext;


pub trait AsBufferData<S: SampleFrame> {
	fn as_buffer_data(&self) -> &[S];
}


/// Audio formats supported by OpenAL.
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


/// Standard formats defined in the base specification.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum StandardFormat {
	MonoU8,
	MonoI16,
	StereoU8,
	StereoI16,
}


/// Formats provided by `AL_EXT_ALAW`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtALawFormat {
	Mono,
	Stereo,
}


/// Formats provided by `AL_EXT_BFORMAT`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtBFormat {
	B2DU8,
	B2DI16,
	B2DF32,
	B3DU8,
	B3DI16,
	B3DF32,
}


/// Formats provided by `AL_EXT_double`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtDoubleFormat {
	Mono,
	Stereo,
}


/// Formats provided by `AL_EXT_float32`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtFloat32Format {
	Mono,
	Stereo,
}


/// Formats provided by `AL_EXT_IMA4`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtIma4Format {
	Mono,
	Stereo,
}


/// Formats provided by `AL_EXT_MCFORMATS`.
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


/// Formats provided by `AL_EXT_MULAW`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtMuLawFormat {
	Mono,
	Stereo,
}


/// Formats provided by `AL_EXT_MULAW_BFORMAT`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtMuLawBFormat {
	B2D,
	B3D,
}


/// Formats provided by `AL_EXT_MULAW_MCFORMATS`.
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


/// Formats provided by `AL_SOFT_MSADPCM`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SoftMsadpcmFormat {
	Mono,
	Stereo,
}


/// Implemented by structs that represent a frame of audio samples.
/// A sample frame is a grouping of audio samples from each channel
/// of an output format.
pub unsafe trait SampleFrame: Copy {
	/// Underlying sample type.
	type Sample: Copy;


	/// Length of the frame in samples.
	fn len() -> usize;
	/// The exact format described by this struct.
	fn format() -> Format;
}


pub trait Block {
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct ALawSample(pub u8);
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct MuLawSample(pub u8);


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct Mono<S: Copy> {
	center: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct Stereo<S: Copy> {
	left: S,
	right: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct McRear<S: Copy> {
	rear: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct McQuad<S: Copy> {
	front_left: S,
	front_right: S,
	back_left: S,
	back_right: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct Mc51Chn<S: Copy> {
	front_left: S,
	front_right: S,
	front_center: S,
	low_freq: S,
	back_left: S,
	back_right: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct BFormat2D<S: Copy> {
	pub w: S,
	pub x: S,
	pub y: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct BFormat3D<S: Copy> {
	pub w: S,
	pub x: S,
	pub y: S,
	pub z: S,
}


impl Format {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
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
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			ExtALawFormat::Mono => Ok(ctx.extensions().AL_EXT_ALAW()?.AL_FORMAT_MONO_ALAW_EXT?),
			ExtALawFormat::Stereo => Ok(ctx.extensions().AL_EXT_ALAW()?.AL_FORMAT_STEREO_ALAW_EXT?),
		})
	}
}


impl ExtBFormat {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			ExtBFormat::B2DU8 => Ok(ctx.extensions().AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT2D_8?),
			ExtBFormat::B2DI16 => Ok(ctx.extensions().AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT2D_16?),
			ExtBFormat::B2DF32 => Ok(ctx.extensions().AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT2D_FLOAT32?),
			ExtBFormat::B3DU8 => Ok(ctx.extensions().AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT3D_8?),
			ExtBFormat::B3DI16 => Ok(ctx.extensions().AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT3D_16?),
			ExtBFormat::B3DF32 => Ok(ctx.extensions().AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT3D_FLOAT32?),
		})
	}
}


impl ExtDoubleFormat {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			ExtDoubleFormat::Mono => Ok(ctx.extensions().AL_EXT_double()?.AL_FORMAT_MONO_DOUBLE_EXT?),
			ExtDoubleFormat::Stereo => Ok(ctx.extensions().AL_EXT_double()?.AL_FORMAT_STEREO_DOUBLE_EXT?),
		})
	}
}


impl ExtFloat32Format {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			ExtFloat32Format::Mono => Ok(ctx.extensions().AL_EXT_float32()?.AL_FORMAT_MONO_FLOAT32?),
			ExtFloat32Format::Stereo => Ok(ctx.extensions().AL_EXT_float32()?.AL_FORMAT_STEREO_FLOAT32?),
		})
	}
}


impl ExtIma4Format {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			ExtIma4Format::Mono => Ok(ctx.extensions().AL_EXT_IMA4()?.AL_FORMAT_MONO_IMA4?),
			ExtIma4Format::Stereo => Ok(ctx.extensions().AL_EXT_IMA4()?.AL_FORMAT_STEREO_IMA4?),
		})
	}
}


impl ExtMcFormat {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			ExtMcFormat::QuadU8 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_QUAD8?),
			ExtMcFormat::QuadI16 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_QUAD16?),
			ExtMcFormat::QuadF32 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_QUAD32?),
			ExtMcFormat::RearU8 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_REAR8?),
			ExtMcFormat::RearI16 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_REAR16?),
			ExtMcFormat::RearF32 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_REAR32?),
			ExtMcFormat::Mc51ChnU8 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_51CHN8?),
			ExtMcFormat::Mc51ChnI16 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_51CHN16?),
			ExtMcFormat::Mc51ChnF32 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_51CHN32?),
			ExtMcFormat::Mc61ChnU8 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_61CHN8?),
			ExtMcFormat::Mc61ChnI16 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_61CHN16?),
			ExtMcFormat::Mc61ChnF32 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_61CHN32?),
			ExtMcFormat::Mc71ChnU8 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_71CHN8?),
			ExtMcFormat::Mc71ChnI16 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_71CHN16?),
			ExtMcFormat::Mc71ChnF32 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_71CHN32?),
		})
	}
}


impl ExtMuLawFormat {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			ExtMuLawFormat::Mono => Ok(ctx.extensions().AL_EXT_MULAW()?.AL_FORMAT_MONO_MULAW_EXT?),
			ExtMuLawFormat::Stereo => Ok(ctx.extensions().AL_EXT_MULAW()?.AL_FORMAT_STEREO_MULAW_EXT?),
		})
	}
}


impl ExtMuLawBFormat {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			ExtMuLawBFormat::B2D => Ok(ctx.extensions().AL_EXT_MULAW_BFORMAT()?.AL_FORMAT_BFORMAT2D_MULAW?),
			ExtMuLawBFormat::B3D => Ok(ctx.extensions().AL_EXT_MULAW_BFORMAT()?.AL_FORMAT_BFORMAT3D_MULAW?),
		})
	}
}


impl ExtMuLawMcFormat {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			ExtMuLawMcFormat::Mono => Ok(ctx.extensions().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_MONO_MULAW?),
			ExtMuLawMcFormat::Stereo => Ok(ctx.extensions().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_STEREO_MULAW?),
			ExtMuLawMcFormat::Quad => Ok(ctx.extensions().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_QUAD_MULAW?),
			ExtMuLawMcFormat::Rear => Ok(ctx.extensions().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_REAR_MULAW?),
			ExtMuLawMcFormat::Mc51Chn => Ok(ctx.extensions().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_51CHN_MULAW?),
			ExtMuLawMcFormat::Mc61Chn => Ok(ctx.extensions().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_61CHN_MULAW?),
			ExtMuLawMcFormat::Mc71Chn => Ok(ctx.extensions().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_71CHN_MULAW?),
		})
	}
}


impl SoftMsadpcmFormat {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			SoftMsadpcmFormat::Mono => Ok(ctx.extensions().AL_SOFT_MSADPCM()?.AL_FORMAT_MONO_MSADPCM_SOFT?),
			SoftMsadpcmFormat::Stereo => Ok(ctx.extensions().AL_SOFT_MSADPCM()?.AL_FORMAT_STEREO_MSADPCM_SOFT?),
		})
	}
}


unsafe impl SampleFrame for Mono<u8> {
	type Sample = u8;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::Standard(StandardFormat::MonoU8) }
}
unsafe impl SampleFrame for Mono<i16> {
	type Sample = i16;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::Standard(StandardFormat::MonoI16) }
}
unsafe impl SampleFrame for Mono<f32> {
	type Sample = f32;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtFloat32(ExtFloat32Format::Mono) }
}
unsafe impl SampleFrame for Mono<f64> {
	type Sample = f64;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtDouble(ExtDoubleFormat::Mono) }
}
unsafe impl SampleFrame for Mono<ALawSample> {
	type Sample = ALawSample;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtALaw(ExtALawFormat::Mono) }
}
unsafe impl SampleFrame for Mono<MuLawSample> {
	type Sample = MuLawSample;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLaw(ExtMuLawFormat::Mono) }
}


unsafe impl SampleFrame for Stereo<u8> {
	type Sample = u8;

	#[inline(always)] fn len() -> usize { 2 }
	#[inline(always)] fn format() -> Format { Format::Standard(StandardFormat::StereoU8) }
}
unsafe impl SampleFrame for Stereo<i16> {
	type Sample = i16;

	#[inline(always)] fn len() -> usize { 2 }
	#[inline(always)] fn format() -> Format { Format::Standard(StandardFormat::StereoI16) }
}
unsafe impl SampleFrame for Stereo<f32> {
	type Sample = f32;

	#[inline(always)] fn len() -> usize { 2 }
	#[inline(always)] fn format() -> Format { Format::ExtFloat32(ExtFloat32Format::Stereo) }
}
unsafe impl SampleFrame for Stereo<f64> {
	type Sample = f64;

	#[inline(always)] fn len() -> usize { 2 }
	#[inline(always)] fn format() -> Format { Format::ExtDouble(ExtDoubleFormat::Stereo) }
}
unsafe impl SampleFrame for Stereo<ALawSample> {
	type Sample = ALawSample;

	#[inline(always)] fn len() -> usize { 2 }
	#[inline(always)] fn format() -> Format { Format::ExtALaw(ExtALawFormat::Stereo) }
}
unsafe impl SampleFrame for Stereo<MuLawSample> {
	type Sample = MuLawSample;

	#[inline(always)] fn len() -> usize { 2 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLaw(ExtMuLawFormat::Stereo) }
}


unsafe impl SampleFrame for McRear<u8> {
	type Sample = u8;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::RearU8) }
}
unsafe impl SampleFrame for McRear<i16> {
	type Sample = i16;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::RearI16)  }
}
unsafe impl SampleFrame for McRear<f32> {
	type Sample = f32;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::RearF32) }
}
unsafe impl SampleFrame for McRear<MuLawSample> {
	type Sample = MuLawSample;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawMcFormats(ExtMuLawMcFormat::Rear) }
}


unsafe impl SampleFrame for McQuad<u8> {
	type Sample = u8;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::QuadU8) }
}
unsafe impl SampleFrame for McQuad<i16> {
	type Sample = i16;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::QuadI16)  }
}
unsafe impl SampleFrame for McQuad<f32> {
	type Sample = f32;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::QuadF32) }
}
unsafe impl SampleFrame for McQuad<MuLawSample> {
	type Sample = MuLawSample;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawMcFormats(ExtMuLawMcFormat::Quad) }
}


unsafe impl SampleFrame for Mc51Chn<u8> {
	type Sample = u8;

	#[inline(always)] fn len() -> usize { 6 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc51ChnU8) }
}
unsafe impl SampleFrame for Mc51Chn<i16> {
	type Sample = i16;

	#[inline(always)] fn len() -> usize { 6 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc51ChnI16)  }
}
unsafe impl SampleFrame for Mc51Chn<f32> {
	type Sample = f32;

	#[inline(always)] fn len() -> usize { 6 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc51ChnF32) }
}
unsafe impl SampleFrame for Mc51Chn<MuLawSample> {
	type Sample = MuLawSample;

	#[inline(always)] fn len() -> usize { 6 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawMcFormats(ExtMuLawMcFormat::Mc51Chn) }
}


unsafe impl SampleFrame for Mc61Chn<u8> {
	type Sample = u8;

	#[inline(always)] fn len() -> usize { 7 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc61ChnU8) }
}
unsafe impl SampleFrame for Mc61Chn<i16> {
	type Sample = i16;

	#[inline(always)] fn len() -> usize { 7 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc61ChnI16)  }
}
unsafe impl SampleFrame for Mc61Chn<f32> {
	type Sample = f32;

	#[inline(always)] fn len() -> usize { 7 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc61ChnF32) }
}
unsafe impl SampleFrame for Mc61Chn<MuLawSample> {
	type Sample = MuLawSample;

	#[inline(always)] fn len() -> usize { 7 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawMcFormats(ExtMuLawMcFormat::Mc61Chn) }
}


unsafe impl SampleFrame for Mc71Chn<u8> {
	type Sample = u8;

	#[inline(always)] fn len() -> usize { 8 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc71ChnU8) }
}
unsafe impl SampleFrame for Mc71Chn<i16> {
	type Sample = i16;

	#[inline(always)] fn len() -> usize { 8 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc71ChnI16)  }
}
unsafe impl SampleFrame for Mc71Chn<f32> {
	type Sample = f32;

	#[inline(always)] fn len() -> usize { 8 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc71ChnF32) }
}
unsafe impl SampleFrame for Mc71Chn<MuLawSample> {
	type Sample = MuLawSample;

	#[inline(always)] fn len() -> usize { 8 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawMcFormats(ExtMuLawMcFormat::Mc71Chn) }
}


unsafe impl SampleFrame for BFormat2D<u8> {
	type Sample = u8;

	#[inline(always)] fn len() -> usize { 3 }
	#[inline(always)] fn format() -> Format { Format::ExtBFormat(ExtBFormat::B2DU8) }
}
unsafe impl SampleFrame for BFormat2D<i16> {
	type Sample = i16;

	#[inline(always)] fn len() -> usize { 3 }
	#[inline(always)] fn format() -> Format { Format::ExtBFormat(ExtBFormat::B2DI16) }
}
unsafe impl SampleFrame for BFormat2D<f32> {
	type Sample = f32;

	#[inline(always)] fn len() -> usize { 3 }
	#[inline(always)] fn format() -> Format { Format::ExtBFormat(ExtBFormat::B2DF32) }
}
unsafe impl SampleFrame for BFormat2D<MuLawSample> {
	type Sample = MuLawSample;

	#[inline(always)] fn len() -> usize { 3 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawBFormat(ExtMuLawBFormat::B2D) }
}


unsafe impl SampleFrame for BFormat3D<u8> {
	type Sample = u8;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtBFormat(ExtBFormat::B3DU8) }
}
unsafe impl SampleFrame for BFormat3D<i16> {
	type Sample = i16;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtBFormat(ExtBFormat::B3DI16) }
}
unsafe impl SampleFrame for BFormat3D<f32> {
	type Sample = f32;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtBFormat(ExtBFormat::B3DF32) }
}
unsafe impl SampleFrame for BFormat3D<MuLawSample> {
	type Sample = MuLawSample;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawBFormat(ExtMuLawBFormat::B3D) }
}


unsafe impl LoopbackFrame for Mono<u8>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_MONO_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_UNSIGNED_BYTE_SOFT?) }
}
unsafe impl LoopbackFrame for Mono<i16>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_MONO_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_SHORT_SOFT?) }
}
unsafe impl LoopbackFrame for Mono<f32>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_MONO_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_FLOAT_SOFT?) }
}


unsafe impl LoopbackFrame for Stereo<u8>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_STEREO_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_UNSIGNED_BYTE_SOFT?) }
}
unsafe impl LoopbackFrame for Stereo<i16>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_STEREO_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_SHORT_SOFT?) }
}
unsafe impl LoopbackFrame for Stereo<f32>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_STEREO_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_FLOAT_SOFT?) }
}


unsafe impl LoopbackFrame for McQuad<u8>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_QUAD_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_UNSIGNED_BYTE_SOFT?) }
}
unsafe impl LoopbackFrame for McQuad<i16>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_QUAD_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_SHORT_SOFT?) }
}
unsafe impl LoopbackFrame for McQuad<f32>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_QUAD_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_FLOAT_SOFT?) }
}


unsafe impl LoopbackFrame for Mc51Chn<u8>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_5POINT1_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_UNSIGNED_BYTE_SOFT?) }
}
unsafe impl LoopbackFrame for Mc51Chn<i16>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_5POINT1_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_SHORT_SOFT?) }
}
unsafe impl LoopbackFrame for Mc51Chn<f32>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_5POINT1_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_FLOAT_SOFT?) }
}


unsafe impl LoopbackFrame for Mc61Chn<u8>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_6POINT1_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_UNSIGNED_BYTE_SOFT?) }
}
unsafe impl LoopbackFrame for Mc61Chn<i16>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_6POINT1_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_SHORT_SOFT?) }
}
unsafe impl LoopbackFrame for Mc61Chn<f32>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_6POINT1_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_FLOAT_SOFT?) }
}


unsafe impl LoopbackFrame for Mc71Chn<u8>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_7POINT1_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_UNSIGNED_BYTE_SOFT?) }
}
unsafe impl LoopbackFrame for Mc71Chn<i16>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_7POINT1_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_SHORT_SOFT?) }
}
unsafe impl LoopbackFrame for Mc71Chn<f32>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_7POINT1_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_FLOAT_SOFT?) }
}


impl<T: AsRef<[F]>, F: SampleFrame> AsBufferData<F> for T {
	fn as_buffer_data(&self) -> &[F] { self.as_ref() }
}


impl<S> AsBufferData<Mono<S>> for [S] where
	S: Copy,
	Mono<S>: SampleFrame,
{
	fn as_buffer_data(&self) -> &[Mono<S>] {
		unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() / Mono::<S>::len()) }
	}
}
impl<S> AsBufferData<Stereo<S>> for [S] where
	S: Copy,
	Stereo<S>: SampleFrame,
{
	fn as_buffer_data(&self) -> &[Stereo<S>] {
		unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() / Stereo::<S>::len()) }
	}
}
impl<S> AsBufferData<McRear<S>> for [S] where
	S: Copy,
	McRear<S>: SampleFrame,
{
	fn as_buffer_data(&self) -> &[McRear<S>] {
		unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() / McRear::<S>::len()) }
	}
}
impl<S> AsBufferData<McQuad<S>> for [S] where
	S: Copy,
	McQuad<S>: SampleFrame,
{
	fn as_buffer_data(&self) -> &[McQuad<S>] {
		unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() / McQuad::<S>::len()) }
	}
}
impl<S> AsBufferData<Mc51Chn<S>> for [S] where
	S: Copy,
	Mc51Chn<S>: SampleFrame,
{
	fn as_buffer_data(&self) -> &[Mc51Chn<S>] {
		unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() / Mc51Chn::<S>::len()) }
	}
}
impl<S> AsBufferData<Mc61Chn<S>> for [S] where
	S: Copy,
	Mc61Chn<S>: SampleFrame,
{
	fn as_buffer_data(&self) -> &[Mc61Chn<S>] {
		unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() / Mc61Chn::<S>::len()) }
	}
}
impl<S> AsBufferData<Mc71Chn<S>> for [S] where
	S: Copy,
	Mc71Chn<S>: SampleFrame,
{
	fn as_buffer_data(&self) -> &[Mc71Chn<S>] {
		unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() / Mc71Chn::<S>::len()) }
	}
}
impl<S> AsBufferData<BFormat2D<S>> for [S] where
	S: Copy,
	BFormat2D<S>: SampleFrame,
{
	fn as_buffer_data(&self) -> &[BFormat2D<S>] {
		unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() / BFormat2D::<S>::len()) }
	}
}
impl<S> AsBufferData<BFormat3D<S>> for [S] where
	S: Copy,
	BFormat3D<S>: SampleFrame,
{
	fn as_buffer_data(&self) -> &[BFormat3D<S>] {
		unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() / BFormat3D::<S>::len()) }
	}
}
