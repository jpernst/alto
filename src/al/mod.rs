use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::fmt;
use std::error::Error as StdError;
use std::collections::VecDeque;
use std::io::{self, Write};
use std::mem;
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


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DistanceModel {
	None,
	Inverse,
	InverseClamped,
	Linear,
	LinearClamped,
	Exponent,
	ExponentClamped,
}


pub struct Context<'d> {
	dev: &'d DeviceTrait,
	api: &'d AlApi<'static>,
	ctx_lock: &'d Mutex<()>,
	ctx: *mut sys::ALCcontext,
	exts: ext::AlCache<'d>,
	defer_rc: Arc<AtomicUsize>,
}


pub struct SuspendLock<'d: 'c, 'c>(&'c Context<'d>);


pub struct Buffer<'d: 'c, 'c> {
	ctx: &'c Context<'d>,
	buf: sys::ALuint, 
}


pub trait SourceTrait<'d> {
	fn context(&self) -> &Context<'d>;
	fn raw_source(&self) -> sys::ALuint;

	fn state(&self) -> AlResult<SourceState>;
	fn play(&mut self) -> AlResult<()>;
	fn pause(&mut self) -> AlResult<()>;
	fn stop(&mut self) -> AlResult<()>;
	fn rewind(&mut self) -> AlResult<()>;

	fn relative(&self) -> AlResult<bool>;
	fn set_relative(&mut self, bool) -> AlResult<()>;

	fn looping(&self) -> AlResult<bool>;
	fn set_looping(&mut self, bool) -> AlResult<()>;

	fn min_gain(&self) -> AlResult<f32>;
	fn set_min_gain(&mut self, f32) -> AlResult<()>;

	fn max_gain(&self) -> AlResult<f32>;
	fn set_max_gain(&mut self, f32) -> AlResult<()>;

	fn reference_distance(&self) -> AlResult<f32>;
	fn set_reference_distance(&mut self, f32) -> AlResult<()>;

	fn rolloff_factor(&self) -> AlResult<f32>;
	fn set_rolloff_factor(&mut self, f32) -> AlResult<()>;

	fn max_distance(&self) -> AlResult<f32>;
	fn set_max_distance(&mut self, f32) -> AlResult<()>;

	fn pitch(&self) -> AlResult<f32>;
	fn set_pitch(&mut self, f32) -> AlResult<()>;

	fn direction<V: From<[f32; 3]>>(&self) -> AlResult<V>;
	fn set_direction<V: Into<[f32; 3]>>(&mut self, V) -> AlResult<()>;

	fn cone_inner_angle(&self) -> AlResult<f32>;
	fn set_cone_inner_angle(&mut self, f32) -> AlResult<()>;

	fn cone_outer_angle(&self) -> AlResult<f32>;
	fn set_cone_outer_angle(&mut self, f32) -> AlResult<()>;

	fn cone_outer_gain(&self) -> AlResult<f32>;
	fn set_cone_outer_gain(&mut self, f32) -> AlResult<()>;

	fn sec_offset(&self) -> AlResult<f32>;
	fn set_sec_offset(&mut self, f32) -> AlResult<()>;

	fn sample_offset(&self) -> AlResult<sys::ALint>;
	fn set_sample_offset(&mut self, sys::ALint) -> AlResult<()>;

	fn byte_offset(&self) -> AlResult<sys::ALint>;
	fn set_byte_offset(&mut self, sys::ALint) -> AlResult<()>;

	fn soft_sec_offset_latency(&self) -> AlResult<(f64, f64)>;

	fn soft_sample_offset_latency(&self) -> AlResult<(i32, i32, i64)>;

	fn soft_sec_length(&self) -> AlResult<f32>;

	fn soft_sample_length(&self) -> AlResult<sys::ALint>;

	fn soft_byte_length(&self) -> AlResult<sys::ALint>;

	fn soft_direct_channels(&self) -> AlResult<bool>;
	fn set_soft_direct_channels(&mut self, bool) -> AlResult<()>;

	fn distance_model(&self) -> AlResult<DistanceModel>;
	fn set_distance_model(&mut self, DistanceModel) -> AlResult<()>;
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
			defer_rc: Arc::new(AtomicUsize::new(0)),
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


	pub fn distance_model(&self) -> AlResult<DistanceModel> {
		let _lock = self.make_current(true)?;
		let model = unsafe { self.api.owner().alGetInteger()(sys::AL_DISTANCE_MODEL) };
		self.get_error().and_then(|_| match model {
			sys::AL_NONE => Ok(DistanceModel::None),
			sys::AL_INVERSE_DISTANCE => Ok(DistanceModel::Inverse),
			sys::AL_INVERSE_DISTANCE_CLAMPED => Ok(DistanceModel::InverseClamped),
			sys::AL_LINEAR_DISTANCE => Ok(DistanceModel::Linear),
			sys::AL_LINEAR_DISTANCE_CLAMPED => Ok(DistanceModel::LinearClamped),
			sys::AL_EXPONENT_DISTANCE => Ok(DistanceModel::Exponent),
			sys::AL_EXPONENT_DISTANCE_CLAMPED => Ok(DistanceModel::ExponentClamped),
			_ => Err(AlError::InvalidValue),
		})
	}
	pub fn set_distance_model(&self, model: DistanceModel) -> AlResult<()> {
		let _lock = self.make_current(true)?;
		unsafe {
			self.api.owner().alDistanceModel()(match model {
				DistanceModel::None => sys::AL_NONE,
				DistanceModel::Inverse => sys::AL_INVERSE_DISTANCE,
				DistanceModel::InverseClamped => sys::AL_INVERSE_DISTANCE_CLAMPED,
				DistanceModel::Linear => sys::AL_LINEAR_DISTANCE,
				DistanceModel::LinearClamped => sys::AL_LINEAR_DISTANCE_CLAMPED,
				DistanceModel::Exponent => sys::AL_EXPONENT_DISTANCE,
				DistanceModel::ExponentClamped => sys::AL_EXPONENT_DISTANCE_CLAMPED,
			})
		};
		self.get_error()
	}


	pub fn using_source_distance_model(&self) -> AlResult<bool> {
		let _lock = self.make_current(true)?;
		let enabled = unsafe { self.api.owner().alIsEnabled()(self.exts.AL_EXT_source_distance_model()?.AL_SOURCE_DISTANCE_MODEL?) };
		self.get_error().map(|_| enabled == sys::AL_TRUE)
	}
	pub fn use_source_distance_model(&self, enable: bool) -> AlResult<()> {
		let _lock = self.make_current(true)?;
		if enable {
			unsafe { self.api.owner().alEnable()(self.exts.AL_EXT_source_distance_model()?.AL_SOURCE_DISTANCE_MODEL?); }
		} else {
			unsafe { self.api.owner().alDisable()(self.exts.AL_EXT_source_distance_model()?.AL_SOURCE_DISTANCE_MODEL?); }
		}
		self.get_error()
	}


	pub fn doppler_factor(&self) -> AlResult<f32> {
		let _lock = self.make_current(true)?;
		let doppler = unsafe { self.api.owner().alGetFloat()(sys::AL_DOPPLER_FACTOR) };
		self.get_error().map(|_| doppler)
	}
	pub fn set_doppler_factor(&self, doppler: f32) -> AlResult<()> {
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alDopplerFactor()(doppler); }
		self.get_error()
	}


	pub fn speed_of_sound(&self) -> AlResult<f32> {
		let _lock = self.make_current(true)?;
		let speed = unsafe { self.api.owner().alGetFloat()(sys::AL_SPEED_OF_SOUND) };
		self.get_error().map(|_| speed)
	}
	pub fn set_speed_of_sound(&self, speed: f32) -> AlResult<()> {
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alSpeedOfSound()(speed); }
		self.get_error()
	}


	pub fn gain(&self) -> AlResult<f32> {
		let mut gain = 0.0;
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alGetListenerf()(sys::AL_GAIN, &mut gain); }
		self.get_error().map(|_| gain)
	}
	pub fn set_gain(&self, gain: f32) -> AlResult<()> {
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alListenerf()(sys::AL_GAIN, gain); }
		self.get_error()
	}


	pub fn position<V: From<[f32; 3]>>(&self) -> AlResult<V> {
		let mut pos = [0.0, 0.0, 0.0];
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alGetListenerfv()(sys::AL_POSITION, &mut pos as *mut [f32; 3] as *mut sys::ALfloat); }
		self.get_error().map(|_| pos.into())
	}
	pub fn set_position<V: Into<[f32; 3]>>(&self, pos: V) -> AlResult<()> {
		let pos = pos.into();
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alListenerfv()(sys::AL_POSITION, &pos as *const [f32; 3] as *const sys::ALfloat); }
		self.get_error()
	}


	pub fn velocity<V: From<[f32; 3]>>(&self) -> AlResult<V> {
		let mut vel = [0.0, 0.0, 0.0];
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alGetListenerfv()(sys::AL_VELOCITY, &mut vel as *mut [f32; 3] as *mut sys::ALfloat); }
		self.get_error().map(|_| vel.into())
	}
	pub fn set_velocity<V: Into<[f32; 3]>>(&self, vel: V) -> AlResult<()> {
		let vel = vel.into();
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alListenerfv()(sys::AL_VELOCITY, &vel as *const [f32; 3] as *const sys::ALfloat); }
		self.get_error()
	}


	pub fn orientation<V: From<[f32; 3]>>(&self) -> AlResult<(V, V)> {
		let mut or = [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alGetListenerfv()(sys::AL_ORIENTATION, &mut or as *mut [[f32; 3]; 2] as *mut sys::ALfloat); }
		self.get_error().map(|_| (or[0].into(), or[1].into()))
	}
	pub fn set_orientation<V: Into<[f32; 3]>>(&self, at: V, up: V) -> AlResult<()> {
		let or = [at.into(), up.into()];
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alListenerfv()(sys::AL_ORIENTATION, &or as *const [[f32; 3]; 2] as *const sys::ALfloat); }
		self.get_error()
	}


	pub fn new_buffer<'c>(&'c self) -> AlResult<Buffer<'d, 'c>> {
		let mut buf = 0;
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alGenBuffers()(1, &mut buf as *mut sys::ALuint); }
		self.get_error().map(|_| Buffer{ctx: self, buf: buf})
	}


	pub fn new_static_source(&self) -> AlResult<StaticSource> {
		let mut src = 0;
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alGenSources()(1, &mut src as *mut sys::ALuint); }
		self.get_error().map(|_| StaticSource{src: Source{ctx: self, src: src}, buf: None})
	}


	pub fn new_streaming_source(&self) -> AlResult<StreamingSource> {
		let mut src = 0;
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alGenSources()(1, &mut src as *mut sys::ALuint); }
		self.get_error().map(|_| StreamingSource{src: Source{ctx: self, src: src}, bufs: VecDeque::new()})
	}


	pub fn play_all<S, I>(&self, srcs: I) -> AlResult<()> where
		S: SourceTrait<'d>,
		I: Iterator,
		<I as Iterator>::Item: AsRef<S> + AsMut<S>,
	{
		let v: Vec<_> = srcs.filter(|s| s.as_ref().context() == self).map(|s| s.as_ref().raw_source()).collect();
		if v.len() > sys::ALint::max_value() as usize { return Err(AlError::InvalidValue) }

		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alSourcePlayv()(v.len() as i32, v.as_slice().as_ptr()); }
		self.get_error()
	}


	pub fn pause_all<S, I>(&self, srcs: I) -> AlResult<()> where
		S: SourceTrait<'d>,
		I: Iterator,
		<I as Iterator>::Item: AsRef<S> + AsMut<S>,
	{
		let v: Vec<_> = srcs.filter(|s| s.as_ref().context() == self).map(|s| s.as_ref().raw_source()).collect();
		if v.len() > sys::ALint::max_value() as usize { return Err(AlError::InvalidValue) }

		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alSourcePausev()(v.len() as i32, v.as_slice().as_ptr()); }
		self.get_error()
	}


	pub fn stop_all<S, I>(&self, srcs: I) -> AlResult<()> where
		S: SourceTrait<'d>,
		I: Iterator,
		<I as Iterator>::Item: AsRef<S> + AsMut<S>,
	{
		let v: Vec<_> = srcs.filter(|s| s.as_ref().context() == self).map(|s| s.as_ref().raw_source()).collect();
		if v.len() > sys::ALint::max_value() as usize { return Err(AlError::InvalidValue) }

		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alSourceStopv()(v.len() as i32, v.as_slice().as_ptr()); }
		self.get_error()
	}


	pub fn rewind_all<S, I>(&self, srcs: I) -> AlResult<()> where
		S: SourceTrait<'d>,
		I: Iterator,
		<I as Iterator>::Item: AsRef<S> + AsMut<S>,
	{
		let v: Vec<_> = srcs.filter(|s| s.as_ref().context() == self).map(|s| s.as_ref().raw_source()).collect();
		if v.len() > sys::ALint::max_value() as usize { return Err(AlError::InvalidValue) }

		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alSourceRewindv()(v.len() as i32, v.as_slice().as_ptr()); }
		self.get_error()
	}


	pub fn suspend<'c>(&'c self) -> AlcResult<SuspendLock<'d, 'c>> {
		SuspendLock::new(self)
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


impl<'d: 'c, 'c> SuspendLock<'d, 'c> {
	fn new(ctx: &'c Context<'d>) -> AlcResult<SuspendLock<'d, 'c>> {
		let adus = ctx.exts.AL_SOFT_deferred_updates()?.alDeferUpdatesSOFT;
		let _lock = ctx.make_current(true)?;

		let old = ctx.defer_rc.fetch_add(1, Ordering::SeqCst);
		if old == 0 {
			match adus {
				Ok(adus) => {
					unsafe { adus(); }
					if let Err(e) = ctx.get_error() {
						ctx.defer_rc.fetch_sub(1, Ordering::SeqCst);
						return Err(e.into());
					}
				},
				Err(_) => {
					unsafe { ctx.api.owner().alcSuspendContext()(ctx.ctx); }
					if let Err(e) = ctx.dev.alto().get_error(ctx.dev.raw_device()) {
						ctx.defer_rc.fetch_sub(1, Ordering::SeqCst);
						return Err(e);
					}
				},
			}
		}

		Ok(SuspendLock(ctx))
	}
}


impl<'d: 'c, 'c> Deref for SuspendLock<'d, 'c> {
	type Target = Context<'d>;

	fn deref(&self) -> &Context<'d> { self.0 }
}


impl<'d: 'c, 'c> Drop for SuspendLock<'d, 'c> {
	fn drop(&mut self) {
		let old = self.0.defer_rc.fetch_sub(1, Ordering::SeqCst);
		if old == 1 {
			match self.0.exts.AL_SOFT_deferred_updates().and_then(|asdu| asdu.alProcessUpdatesSOFT) {
				Ok(apus) => {
					if let Ok(_lock) = self.0.make_current(true) {
						unsafe { apus(); }
						if let Err(_) = self.0.get_error() {
							let _ = writeln!(io::stderr(), "ALTO ERROR: `alProcessUpdatesSOFT` failed in SuspendLock drop");
						}
					} else {
						let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in SuspendLock drop");
					}
				},
				Err(_) => {
					unsafe { self.0.api.owner().alcProcessContext()(self.0.ctx); }
					if let Err(_) = self.0.dev.alto().get_error(self.0.dev.raw_device()) {
						let _ = writeln!(io::stderr(), "ALTO ERROR: `alcProcessContext` failed in SuspendLock drop");
					}
				},
			}
		}
	}
}


impl<'d: 'c, 'c> Buffer<'d, 'c> {
	pub fn context(&self) -> &Context<'d> { self.ctx }
	pub fn raw_buffer(&self) -> sys::ALuint { self.buf }


	pub fn set_data<F: SampleFrame, R: AsRef<[F]>>(&mut self, data: R, freq: i32) -> AlResult<()> {
		let data = data.as_ref();
		let size = data.len() * mem::size_of::<F>();
		if sys::ALsizei::max_value() as usize / mem::size_of::<F>() < data.len() { return Err(AlError::InvalidValue) }

		let _lock = self.ctx.make_current(true)?;
		unsafe {
			self.ctx.api.owner().alBufferData()(
				self.buf,
				F::format().into_raw(Some(self.ctx))?,
				data.as_ptr() as *const sys::ALvoid,
				size as sys::ALsizei,
				freq as sys::ALint,
			);
		}
		self.ctx.get_error()
	}


	pub fn frequency(&self) -> AlResult<sys::ALint> {
		let mut freq = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetBufferi()(self.buf, sys::AL_FREQUENCY, &mut freq); }
		self.ctx.get_error().map(|_| freq)
	}


	pub fn bits(&self) -> AlResult<sys::ALint> {
		let mut bits = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetBufferi()(self.buf, sys::AL_BITS, &mut bits); }
		self.ctx.get_error().map(|_| bits)
	}


	pub fn channels(&self) -> AlResult<sys::ALint> {
		let mut chans = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetBufferi()(self.buf, sys::AL_CHANNELS, &mut chans); }
		self.ctx.get_error().map(|_| chans)
	}


	pub fn size(&self) -> AlResult<sys::ALint> {
		let mut size = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetBufferi()(self.buf, sys::AL_SIZE, &mut size); }
		self.ctx.get_error().map(|_| size)
	}


	pub fn soft_loop_points(&self) -> AlResult<(sys::ALint, sys::ALint)> {
		let mut points = [0, 0];
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetBufferiv()(self.buf, self.ctx.exts.AL_SOFT_loop_points()?.AL_LOOP_POINTS_SOFT?, &mut points as *mut [sys::ALint; 2] as *mut sys::ALint); }
		self.ctx.get_error().map(|_| (points[0], points[1]))
	}
	pub fn set_soft_loop_points(&self, start: sys::ALint, end: sys::ALint) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alBufferiv()(self.buf, self.ctx.exts.AL_SOFT_loop_points()?.AL_LOOP_POINTS_SOFT?, &[start, end] as *const [sys::ALint; 2] as *const sys::ALint); }
		self.ctx.get_error()
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
		let mut state = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_SOURCE_STATE, &mut state); }
		self.ctx.get_error().and_then(|_| match state {
			sys::AL_INITIAL => Ok(SourceState::Initial),
			sys::AL_PLAYING => Ok(SourceState::Playing),
			sys::AL_PAUSED => Ok(SourceState::Paused),
			sys::AL_STOPPED => Ok(SourceState::Stopped),
			_ => Err(AlError::InvalidEnum),
		})
	}
	fn play(&mut self) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcePlay()(self.src); }
		self.ctx.get_error()
	}
	fn pause(&mut self) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcePause()(self.src); }
		self.ctx.get_error()
	}
	fn stop(&mut self) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourceStop()(self.src); }
		self.ctx.get_error()
	}
	fn rewind(&mut self) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourceRewind()(self.src); }
		self.ctx.get_error()
	}


	fn relative(&self) -> AlResult<bool> {
		let mut rel = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_SOURCE_RELATIVE, &mut rel); }
		self.ctx.get_error().map(|_| rel == sys::AL_TRUE as sys::ALint)
	}
	fn set_relative(&mut self, rel: bool) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, sys::AL_SOURCE_RELATIVE, if rel { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.ctx.get_error()
	}


	fn looping(&self) -> AlResult<bool> {
		let mut loop_ = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_LOOPING, &mut loop_); }
		self.ctx.get_error().map(|_| loop_ == sys::AL_TRUE as sys::ALint)
	}
	fn set_looping(&mut self, loop_: bool) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, sys::AL_LOOPING, if loop_ { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.ctx.get_error()
	}


	fn min_gain(&self) -> AlResult<f32> {
		let mut gain = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_MIN_GAIN, &mut gain); }
		self.ctx.get_error().map(|_| gain)
	}
	fn set_min_gain(&mut self, gain: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_MIN_GAIN, gain); }
		self.ctx.get_error()
	}


	fn max_gain(&self) -> AlResult<f32> {
		let mut gain = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_MAX_GAIN, &mut gain); }
		self.ctx.get_error().map(|_| gain)
	}
	fn set_max_gain(&mut self, gain: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_MAX_GAIN, gain); }
		self.ctx.get_error()
	}


	fn reference_distance(&self) -> AlResult<f32> {
		let mut dist = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_REFERENCE_DISTANCE, &mut dist); }
		self.ctx.get_error().map(|_| dist)
	}
	fn set_reference_distance(&mut self, dist: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_REFERENCE_DISTANCE, dist); }
		self.ctx.get_error()
	}


	fn rolloff_factor(&self) -> AlResult<f32> {
		let mut rolloff = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_ROLLOFF_FACTOR, &mut rolloff); }
		self.ctx.get_error().map(|_| rolloff)
	}
	fn set_rolloff_factor(&mut self, rolloff: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_ROLLOFF_FACTOR, rolloff); }
		self.ctx.get_error()
	}


	fn max_distance(&self) -> AlResult<f32> {
		let mut dist = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_MAX_DISTANCE, &mut dist); }
		self.ctx.get_error().map(|_| dist)
	}
	fn set_max_distance(&mut self, dist: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_MAX_DISTANCE, dist); }
		self.ctx.get_error()
	}


	fn pitch(&self) -> AlResult<f32> {
		let mut pitch = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_PITCH, &mut pitch); }
		self.ctx.get_error().map(|_| pitch)
	}
	fn set_pitch(&mut self, pitch: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_PITCH, pitch); }
		self.ctx.get_error()
	}


	fn direction<V: From<[f32; 3]>>(&self) -> AlResult<V> {
		let mut dir = [0.0, 0.0, 0.0];
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcefv()(self.src, sys::AL_DIRECTION, &mut dir as *mut [f32; 3] as *mut sys::ALfloat); }
		self.ctx.get_error().map(|_| dir.into())
	}
	fn set_direction<V: Into<[f32; 3]>>(&mut self, dir: V) -> AlResult<()> {
		let dir = dir.into();
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcefv()(self.src, sys::AL_DIRECTION, &dir as *const [f32; 3] as *const sys::ALfloat); }
		self.ctx.get_error()
	}


	fn cone_inner_angle(&self) -> AlResult<f32> {
		let mut angle = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_CONE_INNER_ANGLE, &mut angle); }
		self.ctx.get_error().map(|_| angle)
	}
	fn set_cone_inner_angle(&mut self, angle: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_CONE_INNER_ANGLE, angle); }
		self.ctx.get_error()
	}


	fn cone_outer_angle(&self) -> AlResult<f32> {
		let mut angle = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_CONE_OUTER_ANGLE, &mut angle); }
		self.ctx.get_error().map(|_| angle)
	}
	fn set_cone_outer_angle(&mut self, angle: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_CONE_OUTER_ANGLE, angle); }
		self.ctx.get_error()
	}


	fn cone_outer_gain(&self) -> AlResult<f32> {
		let mut gain = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_CONE_OUTER_GAIN, &mut gain); }
		self.ctx.get_error().map(|_| gain)
	}
	fn set_cone_outer_gain(&mut self, gain: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_CONE_OUTER_GAIN, gain); }
		self.ctx.get_error()
	}


	fn sec_offset(&self) -> AlResult<f32> {
		let mut offset = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_SEC_OFFSET, &mut offset); }
		self.ctx.get_error().map(|_| offset)
	}
	fn set_sec_offset(&mut self, offset: f32) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_SEC_OFFSET, offset); }
		self.ctx.get_error()
	}


	fn sample_offset(&self) -> AlResult<sys::ALint> {
		let mut offset = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_SAMPLE_OFFSET, &mut offset); }
		self.ctx.get_error().map(|_| offset)
	}
	fn set_sample_offset(&mut self, offset: sys::ALint) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, sys::AL_SAMPLE_OFFSET, offset); }
		self.ctx.get_error()
	}


	fn byte_offset(&self) -> AlResult<sys::ALint> {
		let mut offset = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_BYTE_OFFSET, &mut offset); }
		self.ctx.get_error().map(|_| offset)
	}
	fn set_byte_offset(&mut self, offset: sys::ALint) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, sys::AL_BYTE_OFFSET, offset); }
		self.ctx.get_error()
	}


	fn soft_sec_offset_latency(&self) -> AlResult<(f64, f64)> {
		let mut offset_latency = [0.0, 0.0];
		let assl = self.ctx.exts.AL_SOFT_source_latency()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { assl.alGetSourcedvSOFT?(self.src, assl.AL_SEC_OFFSET_LATENCY_SOFT?, &mut offset_latency as *mut [f64; 2] as *mut f64); }
		self.ctx.get_error().map(|_| (offset_latency[0], offset_latency[1]))
	}


	fn soft_sample_offset_latency(&self) -> AlResult<(i32, i32, i64)> {
		let mut offset_latency = [0, 0];
		let assl = self.ctx.exts.AL_SOFT_source_latency()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { assl.alGetSourcei64vSOFT?(self.src, assl.AL_SAMPLE_OFFSET_LATENCY_SOFT?, &mut offset_latency as *mut [i64; 2] as *mut i64); }
		self.ctx.get_error().map(|_| ((offset_latency[0] >> 32) as i32, offset_latency[0] as i32, offset_latency[1]))
	}


	fn soft_direct_channels(&self) -> AlResult<bool> {
		let mut direct = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, self.ctx.exts.AL_SOFT_direct_channels()?.AL_DIRECT_CHANNELS_SOFT?, &mut direct); }
		self.ctx.get_error().map(|_| direct == sys::AL_TRUE as sys::ALint)
	}
	fn set_soft_direct_channels(&mut self, direct: bool) -> AlResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, self.ctx.exts.AL_SOFT_direct_channels()?.AL_DIRECT_CHANNELS_SOFT?, if direct { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.ctx.get_error()
	}


	fn soft_sec_length(&self) -> AlResult<f32> {
		let mut length = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, self.ctx.exts.AL_SOFT_source_length()?.AL_SEC_LENGTH_SOFT?, &mut length); }
		self.ctx.get_error().map(|_| length)
	}


	fn soft_sample_length(&self) -> AlResult<sys::ALint> {
		let mut length = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, self.ctx.exts.AL_SOFT_source_length()?.AL_SAMPLE_LENGTH_SOFT?, &mut length); }
		self.ctx.get_error().map(|_| length)
	}


	fn soft_byte_length(&self) -> AlResult<sys::ALint> {
		let mut length = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, self.ctx.exts.AL_SOFT_source_length()?.AL_BYTE_LENGTH_SOFT?, &mut length); }
		self.ctx.get_error().map(|_| length)
	}


	fn distance_model(&self) -> AlResult<DistanceModel> {
		self.ctx.exts.AL_EXT_source_distance_model()?;
		let mut model = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_DISTANCE_MODEL, &mut model); }
		self.ctx.get_error().and_then(|_| match model {
			sys::AL_NONE => Ok(DistanceModel::None),
			sys::AL_INVERSE_DISTANCE => Ok(DistanceModel::Inverse),
			sys::AL_INVERSE_DISTANCE_CLAMPED => Ok(DistanceModel::InverseClamped),
			sys::AL_LINEAR_DISTANCE => Ok(DistanceModel::Linear),
			sys::AL_LINEAR_DISTANCE_CLAMPED => Ok(DistanceModel::LinearClamped),
			sys::AL_EXPONENT_DISTANCE => Ok(DistanceModel::Exponent),
			sys::AL_EXPONENT_DISTANCE_CLAMPED => Ok(DistanceModel::ExponentClamped),
			_ => Err(AlError::InvalidValue),
		})
	}
	fn set_distance_model(&mut self, model: DistanceModel) -> AlResult<()> {
		self.ctx.exts.AL_EXT_source_distance_model()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe {
			self.ctx.api.owner().alSourcei()(self.src, sys::AL_DISTANCE_MODEL, match model {
				DistanceModel::None => sys::AL_NONE,
				DistanceModel::Inverse => sys::AL_INVERSE_DISTANCE,
				DistanceModel::InverseClamped => sys::AL_INVERSE_DISTANCE_CLAMPED,
				DistanceModel::Linear => sys::AL_LINEAR_DISTANCE,
				DistanceModel::LinearClamped => sys::AL_LINEAR_DISTANCE_CLAMPED,
				DistanceModel::Exponent => sys::AL_EXPONENT_DISTANCE,
				DistanceModel::ExponentClamped => sys::AL_EXPONENT_DISTANCE_CLAMPED,
			});
		}
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


	pub fn set_buffer(&mut self, buf: Option<Arc<Buffer<'d, 'c>>>) -> AlResult<()> {
		{
			let _lock = self.src.ctx.make_current(true)?;
			unsafe { self.src.ctx.api.owner().alSourcei()(self.src.src, sys::AL_BUFFER, if let Some(ref buf) = buf { buf.buf as sys::ALint } else { 0 }); }
			self.src.ctx.get_error()?;
		}

		self.buf = buf;
		Ok(())
	}
}


impl<'d: 'c, 'c> SourceTrait<'d> for StaticSource<'d, 'c> {
	fn context(&self) -> &Context<'d> { self.src.context() }
	fn raw_source(&self) -> sys::ALuint { self.src.raw_source() }

	fn state(&self) -> AlResult<SourceState> { self.src.state() }
	fn play(&mut self) -> AlResult<()> { self.src.play() }
	fn pause(&mut self) -> AlResult<()> { self.src.pause() }
	fn stop(&mut self) -> AlResult<()> { self.src.stop() }
	fn rewind(&mut self) -> AlResult<()> { self.src.rewind() }

	fn relative(&self) -> AlResult<bool> { self.src.relative() }
	fn set_relative(&mut self, rel: bool) -> AlResult<()> { self.src.set_relative(rel) }

	fn looping(&self) -> AlResult<bool> { self.src.looping() }
	fn set_looping(&mut self, loop_: bool) -> AlResult<()> { self.src.set_looping(loop_) }

	fn min_gain(&self) -> AlResult<f32> { self.src.min_gain() }
	fn set_min_gain(&mut self, gain: f32) -> AlResult<()> { self.src.set_min_gain(gain) }

	fn max_gain(&self) -> AlResult<f32> { self.src.max_gain() }
	fn set_max_gain(&mut self, gain: f32) -> AlResult<()> { self.src.set_max_gain(gain) }

	fn reference_distance(&self) -> AlResult<f32> { self.src.reference_distance() }
	fn set_reference_distance(&mut self, dist: f32) -> AlResult<()> { self.src.set_reference_distance(dist) }

	fn rolloff_factor(&self) -> AlResult<f32> { self.src.rolloff_factor() }
	fn set_rolloff_factor(&mut self, rolloff: f32) -> AlResult<()> { self.src.set_rolloff_factor(rolloff) }

	fn max_distance(&self) -> AlResult<f32> { self.src.max_distance() }
	fn set_max_distance(&mut self, dist: f32) -> AlResult<()> { self.src.set_max_distance(dist) }

	fn pitch(&self) -> AlResult<f32> { self.src.pitch() }
	fn set_pitch(&mut self, pitch: f32) -> AlResult<()> { self.src.set_pitch(pitch) }

	fn direction<V: From<[f32; 3]>>(&self) -> AlResult<V> { self.src.direction() }
	fn set_direction<V: Into<[f32; 3]>>(&mut self, dir: V) -> AlResult<()> { self.src.set_direction(dir) }

	fn cone_inner_angle(&self) -> AlResult<f32> { self.src.cone_inner_angle() }
	fn set_cone_inner_angle(&mut self, angle: f32) -> AlResult<()> { self.src.set_cone_inner_angle(angle) }

	fn cone_outer_angle(&self) -> AlResult<f32> { self.src.cone_outer_angle() }
	fn set_cone_outer_angle(&mut self, angle: f32) -> AlResult<()> { self.src.set_cone_outer_angle(angle) }

	fn cone_outer_gain(&self) -> AlResult<f32> { self.src.cone_outer_gain() }
	fn set_cone_outer_gain(&mut self, gain: f32) -> AlResult<()> { self.src.set_cone_outer_gain(gain) }

	fn sec_offset(&self) -> AlResult<f32> { self.src.sec_offset() }
	fn set_sec_offset(&mut self, offset: f32) -> AlResult<()> { self.src.set_sec_offset(offset) }

	fn sample_offset(&self) -> AlResult<sys::ALint> { self.src.sample_offset() }
	fn set_sample_offset(&mut self, offset: sys::ALint) -> AlResult<()> { self.src.set_sample_offset(offset) }

	fn byte_offset(&self) -> AlResult<sys::ALint> { self.src.byte_offset() }
	fn set_byte_offset(&mut self, offset: sys::ALint) -> AlResult<()> { self.src.set_byte_offset(offset) }

	fn soft_sec_offset_latency(&self) -> AlResult<(f64, f64)> { self.src.soft_sec_offset_latency() }

	fn soft_sample_offset_latency(&self) -> AlResult<(i32, i32, i64)> { self.src.soft_sample_offset_latency() }

	fn soft_sec_length(&self) -> AlResult<f32> { self.src.soft_sec_length() }

	fn soft_sample_length(&self) -> AlResult<sys::ALint> { self.src.soft_sample_length() }

	fn soft_byte_length(&self) -> AlResult<sys::ALint> { self.src.soft_byte_length() }

	fn soft_direct_channels(&self) -> AlResult<bool> { self.src.soft_direct_channels() }
	fn set_soft_direct_channels(&mut self, direct: bool) -> AlResult<()> { self.src.set_soft_direct_channels(direct) }

	fn distance_model(&self) -> AlResult<DistanceModel> { self.src.distance_model() }
	fn set_distance_model(&mut self, model: DistanceModel) -> AlResult<()> { self.src.set_distance_model(model) }
}


impl<'d: 'c, 'c> PartialEq for StaticSource<'d, 'c> {
	fn eq(&self, other: &StaticSource<'d, 'c>) -> bool {
		self.src.ctx == other.src.ctx && self.src.src == other.src.src
	}
}
impl<'d: 'c, 'c> Eq for StaticSource<'d, 'c> { }


impl<'d: 'c, 'c> StreamingSource<'d, 'c> {
	pub fn buffers_queued(&self) -> AlResult<sys::ALint> {
		Ok(self.bufs.len() as sys::ALint)
	}


	pub fn buffers_processed(&self) -> AlResult<sys::ALint> {
		let mut bufs = 0;
		let _lock = self.src.ctx.make_current(true)?;
		unsafe { self.src.ctx.api.owner().alGetSourcei()(self.src.src, sys::AL_BUFFERS_PROCESSED, &mut bufs); }
		self.src.ctx.get_error().map(|_| bufs)
	}


	pub fn queue_buffer(&mut self, buf: Buffer<'d, 'c>) -> Result<(), (AlError, Buffer<'d, 'c>)> {
		{
			let _lock = match self.src.ctx.make_current(true) {
				Ok(lock) => lock,
				Err(e) => return Err((e, buf)),
			};
			unsafe { self.src.ctx.api.owner().alSourceQueueBuffers()(self.src.src, 1, &buf.buf); }
			match self.src.ctx.get_error() {
				Ok(_) => (),
				Err(e) => return Err((e, buf)),
			};
		}

		self.bufs.push_back(buf);
		Ok(())
	}


	pub fn unqueue_buffer(&mut self) -> AlResult<Buffer<'d, 'c>> {
		{
			let mut buf = 0;
			let _lock = self.src.ctx.make_current(true)?;
			unsafe { self.src.ctx.api.owner().alSourceUnqueueBuffers()(self.src.src, 1, &mut buf); }
			self.src.ctx.get_error()?;
		}

		Ok(self.bufs.pop_front().unwrap())
	}
}


impl<'d: 'c, 'c> SourceTrait<'d> for StreamingSource<'d, 'c> {
	fn context(&self) -> &Context<'d> { self.src.context() }
	fn raw_source(&self) -> sys::ALuint { self.src.raw_source() }

	fn state(&self) -> AlResult<SourceState> { self.src.state() }
	fn play(&mut self) -> AlResult<()> { self.src.play() }
	fn pause(&mut self) -> AlResult<()> { self.src.pause() }
	fn stop(&mut self) -> AlResult<()> { self.src.stop() }
	fn rewind(&mut self) -> AlResult<()> { self.src.rewind() }

	fn relative(&self) -> AlResult<bool> { self.src.relative() }
	fn set_relative(&mut self, rel: bool) -> AlResult<()> { self.src.set_relative(rel) }

	fn looping(&self) -> AlResult<bool> { self.src.looping() }
	fn set_looping(&mut self, loop_: bool) -> AlResult<()> { self.src.set_looping(loop_) }

	fn min_gain(&self) -> AlResult<f32> { self.src.min_gain() }
	fn set_min_gain(&mut self, gain: f32) -> AlResult<()> { self.src.set_min_gain(gain) }

	fn max_gain(&self) -> AlResult<f32> { self.src.max_gain() }
	fn set_max_gain(&mut self, gain: f32) -> AlResult<()> { self.src.set_max_gain(gain) }

	fn reference_distance(&self) -> AlResult<f32> { self.src.reference_distance() }
	fn set_reference_distance(&mut self, dist: f32) -> AlResult<()> { self.src.set_reference_distance(dist) }

	fn rolloff_factor(&self) -> AlResult<f32> { self.src.rolloff_factor() }
	fn set_rolloff_factor(&mut self, rolloff: f32) -> AlResult<()> { self.src.set_rolloff_factor(rolloff) }

	fn max_distance(&self) -> AlResult<f32> { self.src.max_distance() }
	fn set_max_distance(&mut self, dist: f32) -> AlResult<()> { self.src.set_max_distance(dist) }

	fn pitch(&self) -> AlResult<f32> { self.src.pitch() }
	fn set_pitch(&mut self, pitch: f32) -> AlResult<()> { self.src.set_pitch(pitch) }

	fn direction<V: From<[f32; 3]>>(&self) -> AlResult<V> { self.src.direction() }
	fn set_direction<V: Into<[f32; 3]>>(&mut self, dir: V) -> AlResult<()> { self.src.set_direction(dir) }

	fn cone_inner_angle(&self) -> AlResult<f32> { self.src.cone_inner_angle() }
	fn set_cone_inner_angle(&mut self, angle: f32) -> AlResult<()> { self.src.set_cone_inner_angle(angle) }

	fn cone_outer_angle(&self) -> AlResult<f32> { self.src.cone_outer_angle() }
	fn set_cone_outer_angle(&mut self, angle: f32) -> AlResult<()> { self.src.set_cone_outer_angle(angle) }

	fn cone_outer_gain(&self) -> AlResult<f32> { self.src.cone_outer_gain() }
	fn set_cone_outer_gain(&mut self, gain: f32) -> AlResult<()> { self.src.set_cone_outer_gain(gain) }

	fn sec_offset(&self) -> AlResult<f32> { self.src.sec_offset() }
	fn set_sec_offset(&mut self, offset: f32) -> AlResult<()> { self.src.set_sec_offset(offset) }

	fn sample_offset(&self) -> AlResult<sys::ALint> { self.src.sample_offset() }
	fn set_sample_offset(&mut self, offset: sys::ALint) -> AlResult<()> { self.src.set_sample_offset(offset) }

	fn byte_offset(&self) -> AlResult<sys::ALint> { self.src.byte_offset() }
	fn set_byte_offset(&mut self, offset: sys::ALint) -> AlResult<()> { self.src.set_byte_offset(offset) }

	fn soft_sec_offset_latency(&self) -> AlResult<(f64, f64)> { self.src.soft_sec_offset_latency() }

	fn soft_sample_offset_latency(&self) -> AlResult<(i32, i32, i64)> { self.src.soft_sample_offset_latency() }

	fn soft_sec_length(&self) -> AlResult<f32> { self.src.soft_sec_length() }

	fn soft_sample_length(&self) -> AlResult<sys::ALint> { self.src.soft_sample_length() }

	fn soft_byte_length(&self) -> AlResult<sys::ALint> { self.src.soft_byte_length() }

	fn soft_direct_channels(&self) -> AlResult<bool> { self.src.soft_direct_channels() }
	fn set_soft_direct_channels(&mut self, direct: bool) -> AlResult<()> { self.src.set_soft_direct_channels(direct) }

	fn distance_model(&self) -> AlResult<DistanceModel> { self.src.distance_model() }
	fn set_distance_model(&mut self, model: DistanceModel) -> AlResult<()> { self.src.set_distance_model(model) }
}


impl<'d: 'c, 'c> PartialEq for StreamingSource<'d, 'c> {
	fn eq(&self, other: &StreamingSource<'d, 'c>) -> bool {
		self.src.ctx == other.src.ctx && self.src.src == other.src.src
	}
}
impl<'d: 'c, 'c> Eq for StreamingSource<'d, 'c> { }
