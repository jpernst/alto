use std::sync::{Arc, Mutex, MutexGuard};
use std::fmt;
use std::error::Error as StdError;
use std::collections::VecDeque;
use std::io::{self, Write};
use std::ptr;

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

	fn state(&self) -> AlResult<SourceState>;
	fn play(&self) -> AlResult<()>;
	fn pause(&self) -> AlResult<()>;
	fn stop(&self) -> AlResult<()>;
	fn rewind(&self) -> AlResult<()>;

	fn relative(&self) -> AlResult<bool>;
	fn set_relative(&self, bool) -> AlResult<()>;

	fn looping(&self) -> AlResult<bool>;
	fn set_looping(&self, bool) -> AlResult<()>;

	fn min_gain(&self) -> AlResult<f32>;
	fn set_min_gain(&self, f32) -> AlResult<()>;

	fn max_gain(&self) -> AlResult<f32>;
	fn set_max_gain(&self, f32) -> AlResult<()>;

	fn reference_distance(&self) -> AlResult<f32>;
	fn set_reference_distance(&self, f32) -> AlResult<()>;

	fn rolloff_factor(&self) -> AlResult<f32>;
	fn set_rolloff_factor(&self, f32) -> AlResult<()>;

	fn max_distance(&self) -> AlResult<f32>;
	fn set_max_distance(&self, f32) -> AlResult<()>;

	fn pitch(&self) -> AlResult<f32>;
	fn set_pitch(&self, f32) -> AlResult<()>;

	fn direction<V: From<[f32; 3]>>(&self) -> AlResult<V>;
	fn set_direction<V: Into<[f32; 3]>>(&self, V) -> AlResult<()>;

	fn cone_inner_angle(&self) -> AlResult<f32>;
	fn set_cone_inner_angle(&self, f32) -> AlResult<()>;

	fn cone_outer_angle(&self) -> AlResult<f32>;
	fn set_cone_outer_angle(&self, f32) -> AlResult<()>;

	fn cone_outer_gain(&self) -> AlResult<f32>;
	fn set_cone_outer_gain(&self, f32) -> AlResult<()>;

	fn sec_offset(&self) -> AlResult<f32>;
	fn set_sec_offset(&self, f32) -> AlResult<()>;

	fn sample_offset(&self) -> AlResult<u32>;
	fn set_sample_offset(&self, u32) -> AlResult<()>;

	fn byte_offset(&self) -> AlResult<u32>;
	fn set_byte_offset(&self, u32) -> AlResult<()>;
}


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SourceState {
	Initial,
	Playing,
	Paused,
	Stopped,
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


	fn extensions(&self) -> &ext::AlCache { &self.exts }


	fn make_current(&self, set: bool) -> AlResult<Option<MutexGuard<()>>> {
		self.api.rent(|exts| {
			if let Ok(tlc) = exts.ALC_EXT_thread_local_context() {
				unsafe { tlc.alcSetThreadContext.unwrap()(if set { self.ctx } else { ptr::null_mut() }); }
				self.get_error().map(|_| None)
			} else {
				unsafe { self.api.owner().alcMakeContextCurrent()(if set { self.ctx } else { ptr::null_mut() }); }
				self.get_error().map(|_| Some(self.ctx_lock.lock().unwrap()))
			}
		})
	}


	pub fn gain(&self) -> AlResult<f32> {
		let _lock = self.make_current(true)?;
		let mut gain = 0.0;
		unsafe { self.api.owner().alGetListenerf()(sys::AL_GAIN, &mut gain); }
		self.get_error().map(|_| gain)
	}
	pub fn set_gain(&self, gain: f32) -> AlResult<()> {
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alListenerf()(sys::AL_GAIN, gain); }
		self.get_error()
	}


	pub fn position<V: From<[f32; 3]>>(&self) -> AlResult<V> {
		let _lock = self.make_current(true)?;
		let mut pos = [0.0, 0.0, 0.0];
		unsafe { self.api.owner().alGetListenerfv()(sys::AL_POSITION, &mut pos as *mut [f32; 3] as *mut sys::ALfloat); }
		self.get_error().map(|_| pos.into())
	}
	pub fn set_position<V: Into<[f32; 3]>>(&self, pos: V) -> AlResult<()> {
		let _lock = self.make_current(true)?;
		let pos = pos.into();
		unsafe { self.api.owner().alListenerfv()(sys::AL_POSITION, &pos as *const [f32; 3] as *const sys::ALfloat); }
		self.get_error()
	}


	pub fn velocity<V: From<[f32; 3]>>(&self) -> AlResult<V> {
		let _lock = self.make_current(true)?;
		let mut vel = [0.0, 0.0, 0.0];
		unsafe { self.api.owner().alGetListenerfv()(sys::AL_VELOCITY, &mut vel as *mut [f32; 3] as *mut sys::ALfloat); }
		self.get_error().map(|_| vel.into())
	}
	pub fn set_velocity<V: Into<[f32; 3]>>(&self, vel: V) -> AlResult<()> {
		let _lock = self.make_current(true)?;
		let vel = vel.into();
		unsafe { self.api.owner().alListenerfv()(sys::AL_VELOCITY, &vel as *const [f32; 3] as *const sys::ALfloat); }
		self.get_error()
	}


	pub fn orientation<V: From<[f32; 3]>>(&self) -> AlResult<(V, V)> {
		let _lock = self.make_current(true)?;
		let mut or = [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
		unsafe { self.api.owner().alGetListenerfv()(sys::AL_ORIENTATION, &mut or as *mut [[f32; 3]; 2] as *mut sys::ALfloat); }
		self.get_error().map(|_| (or[0].into(), or[1].into()))
	}
	pub fn set_orientation<V: Into<[f32; 3]>>(&self, at: V, up: V) -> AlResult<()> {
		let _lock = self.make_current(true)?;
		let or = [at.into(), up.into()];
		unsafe { self.api.owner().alListenerfv()(sys::AL_ORIENTATION, &or as *const [[f32; 3]; 2] as *const sys::ALfloat); }
		self.get_error()
	}


	pub fn new_buffer<'c>(&'c self) -> AlResult<Buffer<'d, 'c>> {
		let _lock = self.make_current(true)?;
		let mut buf = 0;
		unsafe { self.api.owner().alGenBuffers()(1, &mut buf as *mut sys::ALuint); }
		self.get_error().map(|_| Buffer{ctx: self, buf: buf})
	}


	pub fn new_static_source(&self) -> AlResult<StaticSource> {
		let _lock = self.make_current(true)?;
		let mut src = 0;
		unsafe { self.api.owner().alGenSources()(1, &mut src as *mut sys::ALuint); }
		self.get_error().map(|_| StaticSource{src: Source{ctx: self, src: src}, buf: None})
	}


	pub fn new_streaming_source(&self) -> AlResult<StreamingSource> {
		let _lock = self.make_current(true)?;
		let mut src = 0;
		unsafe { self.api.owner().alGenSources()(1, &mut src as *mut sys::ALuint); }
		self.get_error().map(|_| StreamingSource{src: Source{ctx: self, src: src}, bufs: VecDeque::new()})
	}


	pub fn play_all<S: SourceTrait<'d>>(&self, srcs: &[S]) -> AlResult<()> {
		if srcs.len() > sys::ALint::max_value() as usize { return Err(AlError::InvalidValue) }
		if srcs.iter().find(|s| s.context() != self).is_some() { return Err(AlError::InvalidValue) }

		let _lock = self.make_current(true)?;
		let v: Vec<_> = srcs.iter().map(|s| s.raw_source()).collect();
		unsafe { self.api.owner().alSourcePlayv()(v.len() as i32, v.as_slice().as_ptr()); }
		self.get_error()
	}


	pub fn pause_all<S: SourceTrait<'d>>(&self, srcs: &[S]) -> AlResult<()> {
		if srcs.len() > sys::ALint::max_value() as usize { return Err(AlError::InvalidValue) }
		if srcs.iter().find(|s| s.context() != self).is_some() { return Err(AlError::InvalidValue) }

		let _lock = self.make_current(true)?;
		let v: Vec<_> = srcs.iter().map(|s| s.raw_source()).collect();
		unsafe { self.api.owner().alSourcePausev()(v.len() as i32, v.as_slice().as_ptr()); }
		self.get_error()
	}


	pub fn stop_all<S: SourceTrait<'d>>(&self, srcs: &[S]) -> AlResult<()> {
		if srcs.len() > sys::ALint::max_value() as usize { return Err(AlError::InvalidValue) }
		if srcs.iter().find(|s| s.context() != self).is_some() { return Err(AlError::InvalidValue) }

		let _lock = self.make_current(true)?;
		let v: Vec<_> = srcs.iter().map(|s| s.raw_source()).collect();
		unsafe { self.api.owner().alSourceStopv()(v.len() as i32, v.as_slice().as_ptr()); }
		self.get_error()
	}


	pub fn rewind_all<S: SourceTrait<'d>>(&self, srcs: &[S]) -> AlResult<()> {
		if srcs.len() > sys::ALint::max_value() as usize { return Err(AlError::InvalidValue) }
		if srcs.iter().find(|s| s.context() != self).is_some() { return Err(AlError::InvalidValue) }

		let _lock = self.make_current(true)?;
		let v: Vec<_> = srcs.iter().map(|s| s.raw_source()).collect();
		unsafe { self.api.owner().alSourceRewindv()(v.len() as i32, v.as_slice().as_ptr()); }
		self.get_error()
	}


	#[doc(hidden)]
	pub fn get_error(&self) -> AlResult<()> {
		match unsafe { self.api.owner().alGetError()() } {
			sys::AL_NO_ERROR => Ok(()),
			e => Err(e.into())
		}
	}
}


impl<'d> PartialEq for Context<'d> {
	fn eq(&self, other: &Context<'d>) -> bool {
		self.ctx == other.ctx
	}
}
impl<'d> Eq for Context<'d> { }


impl<'d> Drop for Context<'d> {
	fn drop(&mut self) {
		if self.make_current(false).is_ok() {
			unsafe { self.api.owner().alcDestroyContext()(self.ctx); }
			if let Err(_) = self.dev.alto().get_error(self.dev.raw_device()) {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alcDeleteContext` failed in Context drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in Context drop");
		}
	}
}


unsafe impl<'d> Send for Context<'d> { }
unsafe impl<'d> Sync for Context<'d> { }


impl<'d: 'c, 'c> Buffer<'d, 'c> {
	pub fn context(&self) -> &Context<'d> { self.ctx }
	pub fn raw_buffer(&self) -> sys::ALuint { self.buf }


	pub fn set_buffer_data<F: SampleFrame>(&self, data: &[F]) -> AlResult<()> {
		Ok(())
	}


	pub fn frequency(&self) -> AlResult<i32> {
		let _lock = self.ctx.make_current(true)?;
		let mut freq = 0;
		unsafe { self.ctx.api.owner().alGetBufferi()(self.buf, sys::AL_FREQUENCY, &mut freq); }
		self.ctx.get_error().map(|_| freq)
	}


	pub fn bits(&self) -> AlResult<i32> {
		let _lock = self.ctx.make_current(true)?;
		let mut bits = 0;
		unsafe { self.ctx.api.owner().alGetBufferi()(self.buf, sys::AL_BITS, &mut bits); }
		self.ctx.get_error().map(|_| bits)
	}


	pub fn channels(&self) -> AlResult<i32> {
		let _lock = self.ctx.make_current(true)?;
		let mut chans = 0;
		unsafe { self.ctx.api.owner().alGetBufferi()(self.buf, sys::AL_CHANNELS, &mut chans); }
		self.ctx.get_error().map(|_| chans)
	}


	pub fn size(&self) -> AlResult<u32> {
		let _lock = self.ctx.make_current(true)?;
		let mut size = 0;
		unsafe { self.ctx.api.owner().alGetBufferi()(self.buf, sys::AL_SIZE, &mut size); }
		self.ctx.get_error().map(|_| size as u32)
	}
}


impl<'d: 'c, 'c> Drop for Buffer<'d, 'c> {
	fn drop(&mut self) {
		if let Ok(_lock) = self.ctx.make_current(true) {
			unsafe { self.ctx.api.owner().alDeleteBuffers()(1, &mut self.buf as *mut sys::ALuint); }
			if let Err(_) = self.ctx.get_error() {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alDeleteBuffers` failed in Buffer drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in Buffer drop");
		}
	}
}


impl<'d: 'c, 'c> SourceTrait<'d> for Source<'d, 'c> {
	fn context(&self) -> &Context<'d> { self.ctx }
	fn raw_source(&self) -> sys::ALuint { self.src }


	fn state(&self) -> AlResult<SourceState> {
		let _lock = self.ctx.make_current(true)?;
		let mut state = 0;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_SOURCE_STATE, &mut state); }
		self.ctx.get_error().and_then(|_| match state {
			sys::AL_INITIAL => Ok(SourceState::Initial),
			sys::AL_PLAYING => Ok(SourceState::Playing),
			sys::AL_PAUSED => Ok(SourceState::Paused),
			sys::AL_STOPPED => Ok(SourceState::Stopped),
			_ => Err(AlError::InvalidEnum),
		})
	}
	fn play(&self) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcePlay()(self.src); }
		self.ctx.get_error()
	}
	fn pause(&self) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcePause()(self.src); }
		self.ctx.get_error()
	}
	fn stop(&self) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourceStop()(self.src); }
		self.ctx.get_error()
	}
	fn rewind(&self) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourceRewind()(self.src); }
		self.ctx.get_error()
	}


	fn relative(&self) -> AlResult<bool> {
		let _lock = self.ctx.make_current(true)?;
		let mut rel = 0;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_SOURCE_RELATIVE, &mut rel); }
		self.ctx.get_error().map(|_| if rel == (sys::AL_TRUE as i32) { true } else { false })
	}
	fn set_relative(&self, rel: bool) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, sys::AL_SOURCE_RELATIVE, if rel { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.ctx.get_error()
	}


	fn looping(&self) -> AlResult<bool> {
		let _lock = self.ctx.make_current(true)?;
		let mut loop_ = 0;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_LOOPING, &mut loop_); }
		self.ctx.get_error().map(|_| if loop_ == (sys::AL_TRUE as i32) { true } else { false })
	}
	fn set_looping(&self, loop_: bool) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, sys::AL_LOOPING, if loop_ { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.ctx.get_error()
	}


	fn min_gain(&self) -> AlResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut min_gain = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_MIN_GAIN, &mut min_gain); }
		self.ctx.get_error().map(|_| min_gain)
	}
	fn set_min_gain(&self, min_gain: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_MIN_GAIN, min_gain); }
		self.ctx.get_error()
	}


	fn max_gain(&self) -> AlResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut max_gain = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_MAX_GAIN, &mut max_gain); }
		self.ctx.get_error().map(|_| max_gain)
	}
	fn set_max_gain(&self, max_gain: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_MAX_GAIN, max_gain); }
		self.ctx.get_error()
	}


	fn reference_distance(&self) -> AlResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut ref_dist = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_REFERENCE_DISTANCE, &mut ref_dist); }
		self.ctx.get_error().map(|_| ref_dist)
	}
	fn set_reference_distance(&self, ref_dist: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_REFERENCE_DISTANCE, ref_dist); }
		self.ctx.get_error()
	}


	fn rolloff_factor(&self) -> AlResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut rolloff = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_ROLLOFF_FACTOR, &mut rolloff); }
		self.ctx.get_error().map(|_| rolloff)
	}
	fn set_rolloff_factor(&self, rolloff: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_ROLLOFF_FACTOR, rolloff); }
		self.ctx.get_error()
	}


	fn max_distance(&self) -> AlResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut max_dist = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_MAX_DISTANCE, &mut max_dist); }
		self.ctx.get_error().map(|_| max_dist)
	}
	fn set_max_distance(&self, max_dist: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_MAX_DISTANCE, max_dist); }
		self.ctx.get_error()
	}


	fn pitch(&self) -> AlResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut pitch = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_PITCH, &mut pitch); }
		self.ctx.get_error().map(|_| pitch)
	}
	fn set_pitch(&self, pitch: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_PITCH, pitch); }
		self.ctx.get_error()
	}


	fn direction<V: From<[f32; 3]>>(&self) -> AlResult<V> {
		let _lock = self.ctx.make_current(true)?;
		let mut dir = [0.0, 0.0, 0.0];
		unsafe { self.ctx.api.owner().alGetSourcefv()(self.src, sys::AL_DIRECTION, &mut dir as *mut [f32; 3] as *mut sys::ALfloat); }
		self.ctx.get_error().map(|_| dir.into())
	}
	fn set_direction<V: Into<[f32; 3]>>(&self, dir: V) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		let dir = dir.into();
		unsafe { self.ctx.api.owner().alSourcefv()(self.src, sys::AL_DIRECTION, &dir as *const [f32; 3] as *const sys::ALfloat); }
		self.ctx.get_error()
	}


	fn cone_inner_angle(&self) -> AlResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut in_angle = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_CONE_INNER_ANGLE, &mut in_angle); }
		self.ctx.get_error().map(|_| in_angle)
	}
	fn set_cone_inner_angle(&self, in_angle: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_CONE_INNER_ANGLE, in_angle); }
		self.ctx.get_error()
	}


	fn cone_outer_angle(&self) -> AlResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut out_angle = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_CONE_OUTER_ANGLE, &mut out_angle); }
		self.ctx.get_error().map(|_| out_angle)
	}
	fn set_cone_outer_angle(&self, out_angle: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_CONE_OUTER_ANGLE, out_angle); }
		self.ctx.get_error()
	}


	fn cone_outer_gain(&self) -> AlResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut out_gain = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_CONE_OUTER_GAIN, &mut out_gain); }
		self.ctx.get_error().map(|_| out_gain)
	}
	fn set_cone_outer_gain(&self, out_gain: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_CONE_OUTER_GAIN, out_gain); }
		self.ctx.get_error()
	}


	fn sec_offset(&self) -> AlResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut sec_offset = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_SEC_OFFSET, &mut sec_offset); }
		self.ctx.get_error().map(|_| sec_offset)
	}
	fn set_sec_offset(&self, sec_offset: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_SEC_OFFSET, sec_offset); }
		self.ctx.get_error()
	}


	fn sample_offset(&self) -> AlResult<u32> {
		let _lock = self.ctx.make_current(true)?;
		let mut samp_offset = 0;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_SAMPLE_OFFSET, &mut samp_offset); }
		self.ctx.get_error().map(|_| samp_offset as u32)
	}
	fn set_sample_offset(&self, samp_offset: u32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, sys::AL_SAMPLE_OFFSET, samp_offset as sys::ALint); }
		self.ctx.get_error()
	}


	fn byte_offset(&self) -> AlResult<u32> {
		let _lock = self.ctx.make_current(true)?;
		let mut byte_offset = 0;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_BYTE_OFFSET, &mut byte_offset); }
		self.ctx.get_error().map(|_| byte_offset as u32)
	}
	fn set_byte_offset(&self, byte_offset: u32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, sys::AL_BYTE_OFFSET, byte_offset as sys::ALint); }
		self.ctx.get_error()
	}
}


impl<'d: 'c, 'c> Drop for Source<'d, 'c> {
	fn drop(&mut self) {
		if let Ok(_lock) = self.ctx.make_current(true) {
			unsafe { self.ctx.api.owner().alDeleteSources()(1, &mut self.src as *mut sys::ALuint); }
			if let Err(_) = self.ctx.get_error() {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alDeleteSources` failed in Source drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in Source drop");
		}
	}
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

	fn state(&self) -> AlResult<SourceState> { self.src.state() }
	fn play(&self) -> AlResult<()> { self.src.play() }
	fn pause(&self) -> AlResult<()> { self.src.pause() }
	fn stop(&self) -> AlResult<()> { self.src.stop() }
	fn rewind(&self) -> AlResult<()> { self.src.rewind() }

	fn relative(&self) -> AlResult<bool> { self.src.relative() }
	fn set_relative(&self, rel: bool) -> AlResult<()> { self.src.set_relative(rel) }

	fn looping(&self) -> AlResult<bool> { self.src.looping() }
	fn set_looping(&self, loop_: bool) -> AlResult<()> { self.src.set_looping(loop_) }

	fn min_gain(&self) -> AlResult<f32> { self.src.min_gain() }
	fn set_min_gain(&self, min_gain: f32) -> AlResult<()> { self.src.set_min_gain(min_gain) }

	fn max_gain(&self) -> AlResult<f32> { self.src.max_gain() }
	fn set_max_gain(&self, max_gain: f32) -> AlResult<()> { self.src.set_max_gain(max_gain) }

	fn reference_distance(&self) -> AlResult<f32> { self.src.reference_distance() }
	fn set_reference_distance(&self, ref_dist: f32) -> AlResult<()> { self.src.set_reference_distance(ref_dist) }

	fn rolloff_factor(&self) -> AlResult<f32> { self.src.rolloff_factor() }
	fn set_rolloff_factor(&self, rolloff: f32) -> AlResult<()> { self.src.set_rolloff_factor(rolloff) }

	fn max_distance(&self) -> AlResult<f32> { self.src.max_distance() }
	fn set_max_distance(&self, max_dist: f32) -> AlResult<()> { self.src.set_max_distance(max_dist) }

	fn pitch(&self) -> AlResult<f32> { self.src.pitch() }
	fn set_pitch(&self, pitch: f32) -> AlResult<()> { self.src.set_pitch(pitch) }

	fn direction<V: From<[f32; 3]>>(&self) -> AlResult<V> { self.src.direction() }
	fn set_direction<V: Into<[f32; 3]>>(&self, dir: V) -> AlResult<()> { self.src.set_direction(dir) }

	fn cone_inner_angle(&self) -> AlResult<f32> { self.src.cone_inner_angle() }
	fn set_cone_inner_angle(&self, in_angle: f32) -> AlResult<()> { self.src.set_cone_inner_angle(in_angle) }

	fn cone_outer_angle(&self) -> AlResult<f32> { self.src.cone_outer_angle() }
	fn set_cone_outer_angle(&self, out_angle: f32) -> AlResult<()> { self.src.set_cone_outer_angle(out_angle) }

	fn cone_outer_gain(&self) -> AlResult<f32> { self.src.cone_outer_gain() }
	fn set_cone_outer_gain(&self, out_gain: f32) -> AlResult<()> { self.src.set_cone_outer_gain(out_gain) }

	fn sec_offset(&self) -> AlResult<f32> { self.src.sec_offset() }
	fn set_sec_offset(&self, sec_offset: f32) -> AlResult<()> { self.src.set_sec_offset(sec_offset) }

	fn sample_offset(&self) -> AlResult<u32> { self.src.sample_offset() }
	fn set_sample_offset(&self, samp_offset: u32) -> AlResult<()> { self.src.set_sample_offset(samp_offset) }

	fn byte_offset(&self) -> AlResult<u32> { self.src.byte_offset() }
	fn set_byte_offset(&self, byte_offset: u32) -> AlResult<()> { self.src.set_byte_offset(byte_offset) }
}


impl<'d: 'c, 'c> PartialEq for StaticSource<'d, 'c> {
	fn eq(&self, other: &StaticSource<'d, 'c>) -> bool {
		self.src.ctx == other.src.ctx && self.src.src == other.src.src
	}
}
impl<'d: 'c, 'c> Eq for StaticSource<'d, 'c> { }


impl<'d: 'c, 'c> StreamingSource<'d, 'c> {
}


impl<'d: 'c, 'c> SourceTrait<'d> for StreamingSource<'d, 'c> {
	fn context(&self) -> &Context<'d> { self.src.context() }
	fn raw_source(&self) -> sys::ALuint { self.src.raw_source() }

	fn state(&self) -> AlResult<SourceState> { self.src.state() }
	fn play(&self) -> AlResult<()> { self.src.play() }
	fn pause(&self) -> AlResult<()> { self.src.pause() }
	fn stop(&self) -> AlResult<()> { self.src.stop() }
	fn rewind(&self) -> AlResult<()> { self.src.rewind() }

	fn relative(&self) -> AlResult<bool> { self.src.relative() }
	fn set_relative(&self, rel: bool) -> AlResult<()> { self.src.set_relative(rel) }

	fn looping(&self) -> AlResult<bool> { self.src.looping() }
	fn set_looping(&self, loop_: bool) -> AlResult<()> { self.src.set_looping(loop_) }

	fn min_gain(&self) -> AlResult<f32> { self.src.min_gain() }
	fn set_min_gain(&self, min_gain: f32) -> AlResult<()> { self.src.set_min_gain(min_gain) }

	fn max_gain(&self) -> AlResult<f32> { self.src.max_gain() }
	fn set_max_gain(&self, max_gain: f32) -> AlResult<()> { self.src.set_max_gain(max_gain) }

	fn reference_distance(&self) -> AlResult<f32> { self.src.reference_distance() }
	fn set_reference_distance(&self, ref_dist: f32) -> AlResult<()> { self.src.set_reference_distance(ref_dist) }

	fn rolloff_factor(&self) -> AlResult<f32> { self.src.rolloff_factor() }
	fn set_rolloff_factor(&self, rolloff: f32) -> AlResult<()> { self.src.set_rolloff_factor(rolloff) }

	fn max_distance(&self) -> AlResult<f32> { self.src.max_distance() }
	fn set_max_distance(&self, max_dist: f32) -> AlResult<()> { self.src.set_max_distance(max_dist) }

	fn pitch(&self) -> AlResult<f32> { self.src.pitch() }
	fn set_pitch(&self, pitch: f32) -> AlResult<()> { self.src.set_pitch(pitch) }

	fn direction<V: From<[f32; 3]>>(&self) -> AlResult<V> { self.src.direction() }
	fn set_direction<V: Into<[f32; 3]>>(&self, dir: V) -> AlResult<()> { self.src.set_direction(dir) }

	fn cone_inner_angle(&self) -> AlResult<f32> { self.src.cone_inner_angle() }
	fn set_cone_inner_angle(&self, in_angle: f32) -> AlResult<()> { self.src.set_cone_inner_angle(in_angle) }

	fn cone_outer_angle(&self) -> AlResult<f32> { self.src.cone_outer_angle() }
	fn set_cone_outer_angle(&self, out_angle: f32) -> AlResult<()> { self.src.set_cone_outer_angle(out_angle) }

	fn cone_outer_gain(&self) -> AlResult<f32> { self.src.cone_outer_gain() }
	fn set_cone_outer_gain(&self, out_gain: f32) -> AlResult<()> { self.src.set_cone_outer_gain(out_gain) }

	fn sec_offset(&self) -> AlResult<f32> { self.src.sec_offset() }
	fn set_sec_offset(&self, sec_offset: f32) -> AlResult<()> { self.src.set_sec_offset(sec_offset) }

	fn sample_offset(&self) -> AlResult<u32> { self.src.sample_offset() }
	fn set_sample_offset(&self, samp_offset: u32) -> AlResult<()> { self.src.set_sample_offset(samp_offset) }

	fn byte_offset(&self) -> AlResult<u32> { self.src.byte_offset() }
	fn set_byte_offset(&self, byte_offset: u32) -> AlResult<()> { self.src.set_byte_offset(byte_offset) }
}


impl<'d: 'c, 'c> PartialEq for StreamingSource<'d, 'c> {
	fn eq(&self, other: &StreamingSource<'d, 'c>) -> bool {
		self.src.ctx == other.src.ctx && self.src.src == other.src.src
	}
}
impl<'d: 'c, 'c> Eq for StreamingSource<'d, 'c> { }
