use std::sync::{Arc, Mutex, MutexGuard};
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
	WrongDevice,
	UnknownError,
}


pub type AlResult<T> = ::std::result::Result<T, AlError>;


pub struct Context<'d> {
	dev: &'d DeviceTrait,
	api: &'d AlApi<'static>,
	ctx_lock: &'d Mutex<()>,
	ctx: *mut sys::ALCcontext,
	exts: ext::AlCache<'d>,
}


pub struct Buffer<'d: 'c, 'c> {
	ctx: &'c Context<'d>,
	buf: sys::ALuint, 
}


pub trait SourceTrait<'d> {
	fn context(&self) -> &Context<'d>;
	fn raw_source(&self) -> sys::ALuint;
}


struct Source<'d: 'c, 'c> {
	ctx: &'c Context<'d>,
	src: sys::ALuint,
}


pub struct StaticSource<'d: 'c, 'c> {
	src: Source<'d, 'c>,
	buf: Option<Arc<Buffer<'d, 'c>>>,
}


pub struct StreamingSource<'d: 'c, 'c> {
	src: Source<'d, 'c>,
	bufs: VecDeque<Buffer<'d, 'c>>,
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
			AlError::WrongDevice => "AL ERROR: Wrong Device",
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


impl<'d> Context<'d> {
	#[doc(hidden)]
	pub unsafe fn new(dev: &'d DeviceTrait, api: &'d AlApi<'static>, ctx_lock: &'d Mutex<()>, ctx: *mut sys::ALCcontext) -> Context<'d> {
		Context{
			dev: dev,
			api: api,
			ctx_lock: ctx_lock,
			ctx: ctx,
			exts: ext::AlCache::new(api.owner()),
		}
	}


	pub fn device(&self) -> &(DeviceTrait + 'd) { self.dev }
	pub fn raw_context(&self) -> *mut sys::ALCcontext { self.ctx }


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


	fn make_current(&self) -> AlResult<Option<MutexGuard<()>>> {
		self.api.rent(|exts| {
			if let Ok(tlc) = exts.ALC_EXT_thread_local_context() {
				unsafe { tlc.alcSetThreadContext.unwrap()(self.ctx); }
				self.get_error().map(|_| None)
			} else {
				unsafe { self.api.owner().alcMakeContextCurrent()(self.ctx); }
				self.get_error().map(|_| Some(self.ctx_lock.lock().unwrap()))
			}
		})
	}


	pub fn gain(&self) -> AlResult<f32> {
		let _lock = self.make_current()?;
		let mut gain = 0.0;
		unsafe { self.api.owner().alGetListenerf()(sys::AL_GAIN, &mut gain); }
		self.get_error().map(|_| gain)
	}
	pub fn set_gain(&self, gain: f32) -> AlResult<()> {
		let _lock = self.make_current()?;
		unsafe { self.api.owner().alListenerf()(sys::AL_GAIN, gain); }
		self.get_error()
	}


	pub fn position<V: From<[f32; 3]>>(&self) -> AlResult<V> {
		let _lock = self.make_current()?;
		let mut pos = [0.0, 0.0, 0.0];
		unsafe { self.api.owner().alGetListenerfv()(sys::AL_POSITION, &mut pos as *mut [f32; 3] as *mut sys::ALfloat); }
		self.get_error().map(|_| pos.into())
	}
	pub fn set_position<V: Into<[f32; 3]>>(&self, pos: V) -> AlResult<()> {
		let _lock = self.make_current()?;
		let pos = pos.into();
		unsafe { self.api.owner().alListenerfv()(sys::AL_POSITION, &pos as *const [f32; 3] as *const sys::ALfloat); }
		self.get_error()
	}


	pub fn velocity<V: From<[f32; 3]>>(&self) -> AlResult<V> {
		let _lock = self.make_current()?;
		let mut vel = [0.0, 0.0, 0.0];
		unsafe { self.api.owner().alGetListenerfv()(sys::AL_VELOCITY, &mut vel as *mut [f32; 3] as *mut sys::ALfloat); }
		self.get_error().map(|_| vel.into())
	}
	pub fn set_velocity<V: Into<[f32; 3]>>(&self, vel: V) -> AlResult<()> {
		let _lock = self.make_current()?;
		let vel = vel.into();
		unsafe { self.api.owner().alListenerfv()(sys::AL_VELOCITY, &vel as *const [f32; 3] as *const sys::ALfloat); }
		self.get_error()
	}


	pub fn orientation<V: From<[f32; 3]>>(&self) -> AlResult<(V, V)> {
		let _lock = self.make_current()?;
		let mut or = [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
		unsafe { self.api.owner().alGetListenerfv()(sys::AL_ORIENTATION, &mut or as *mut [[f32; 3]; 2] as *mut sys::ALfloat); }
		self.get_error().map(|_| (or[0].into(), or[1].into()))
	}
	pub fn set_orientation<V: Into<[f32; 3]>>(&self, at: V, up: V) -> AlResult<()> {
		let _lock = self.make_current()?;
		let or = [at.into(), up.into()];
		unsafe { self.api.owner().alListenerfv()(sys::AL_ORIENTATION, &or as *const [[f32; 3]; 2] as *const sys::ALfloat); }
		self.get_error()
	}


	pub fn new_buffer<'c>(&'c self) -> AlResult<Buffer<'d, 'c>> {
		let _lock = self.make_current()?;
		let mut buf = 0;
		unsafe { self.api.owner().alGenBuffers()(1, &mut buf as *mut sys::ALuint); }
		self.get_error().map(|_| Buffer{ctx: self, buf: buf})
	}


	pub fn new_static_source(&self) -> AlResult<StaticSource> {
		let _lock = self.make_current()?;
		let mut src = 0;
		unsafe { self.api.owner().alGenSources()(1, &mut src as *mut sys::ALuint); }
		self.get_error().map(|_| StaticSource{src: Source{ctx: self, src: src}, buf: None})
	}


	pub fn new_streaming_source(&self) -> AlResult<StreamingSource> {
		let _lock = self.make_current()?;
		let mut src = 0;
		unsafe { self.api.owner().alGenSources()(1, &mut src as *mut sys::ALuint); }
		self.get_error().map(|_| StreamingSource{src: Source{ctx: self, src: src}, bufs: VecDeque::new()})
	}


	#[doc(hidden)]
	pub fn get_error(&self) -> AlResult<()> {
		match unsafe { self.api.owner().alGetError()() } {
			sys::AL_NO_ERROR => Ok(()),
			e => Err(e.into())
		}
	}
}


unsafe impl<'d> Send for Context<'d> { }
unsafe impl<'d> Sync for Context<'d> { }


impl<'d: 'c, 'c> Buffer<'d, 'c> {
	pub fn context(&self) -> &Context<'d> { self.ctx }
	pub fn raw_buffer(&self) -> sys::ALuint { self.buf }


	pub fn frequency(&self, ctx: &Context<'d>) -> AlResult<i32> {
		let _lock = self.ctx.make_current()?;
		let mut freq = 0;
		unsafe { ctx.api.owner().alGetBufferi()(self.buf, sys::AL_FREQUENCY, &mut freq); }
		ctx.get_error().map(|_| freq)
	}


	pub fn bits(&self, ctx: &Context<'d>) -> AlResult<i32> {
		let _lock = self.ctx.make_current()?;
		let mut bits = 0;
		unsafe { ctx.api.owner().alGetBufferi()(self.buf, sys::AL_BITS, &mut bits); }
		ctx.get_error().map(|_| bits)
	}


	pub fn channels(&self, ctx: &Context<'d>) -> AlResult<i32> {
		let _lock = self.ctx.make_current()?;
		let mut chans = 0;
		unsafe { ctx.api.owner().alGetBufferi()(self.buf, sys::AL_CHANNELS, &mut chans); }
		ctx.get_error().map(|_| chans)
	}


	pub fn size(&self, ctx: &Context<'d>) -> AlResult<usize> {
		let _lock = self.ctx.make_current()?;
		let mut size = 0;
		unsafe { ctx.api.owner().alGetBufferi()(self.buf, sys::AL_SIZE, &mut size); }
		ctx.get_error().map(|_| size as usize)
	}
}


impl<'d: 'c, 'c> Drop for Buffer<'d, 'c> {
	fn drop(&mut self) {
		let _lock = self.ctx.make_current().unwrap();
		unsafe { self.ctx.api.owner().alDeleteBuffers()(1, &mut self.buf as *mut sys::ALuint); }
	}
}


impl<'d: 'c, 'c> SourceTrait<'d> for Source<'d, 'c> {
	fn context(&self) -> &Context<'d> { self.ctx }
	fn raw_source(&self) -> sys::ALuint { self.src }
}


impl<'d: 'c, 'c> StaticSource<'d, 'c> {
	pub fn buffer(&self) -> Option<&Arc<Buffer<'d, 'c>>> { self.buf.as_ref() }
	pub fn set_buffer(&self, buf: Option<Arc<Buffer<'d, 'c>>>) -> AlResult<()> {
		panic!();
	}
}


impl<'d: 'c, 'c> SourceTrait<'d> for StaticSource<'d, 'c> {
	fn context(&self) -> &Context<'d> { self.src.context() }
	fn raw_source(&self) -> sys::ALuint { self.src.raw_source() }
}


impl<'d: 'c, 'c> StreamingSource<'d, 'c> {
}


impl<'d: 'c, 'c> SourceTrait<'d> for StreamingSource<'d, 'c> {
	fn context(&self) -> &Context<'d> { self.src.context() }
	fn raw_source(&self) -> sys::ALuint { self.src.raw_source() }
}
