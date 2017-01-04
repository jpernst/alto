use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::VecDeque;
use std::io::{self, Write};
use std::mem;
use std::ptr;

use ::{AltoError, AltoResult};
use sys;
use alc::*;
use ext;


mod format;
pub use self::format::*;



/// The shape of the volume curve for 3D positional audio.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DistanceModel {
	/// No distance rolloff.
	None,
	/// Gain is inversely proportional to distance.
	Inverse,
	/// Gain is inversely proportional to distance, but clamped at 1.0.
	InverseClamped,
	/// Gain rolls off linearly.
	Linear,
	/// Gain rolls off linearly, but clamps at 1.0.
	LinearClamped,
	/// Exponential rolloff.
	Exponent,
	/// Exponential rolloff with clamping.
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


/// An RAII lock that will suspend state updates while held.
/// When this lock is droopped, the context will apply all pending updates.
pub struct SuspendLock<'d: 'c, 'c>(&'c Context<'d>);


/// A buffer containing audio data of any supported format.
pub struct Buffer<'d: 'c, 'c> {
	ctx: &'c Context<'d>,
	buf: sys::ALuint, 
}


/// Capabilities common to both static and streaming sources.
pub trait SourceTrait<'d> {
	/// The context from which this source was created.
	fn context(&self) -> &Context<'d>;
	/// Raw handle as provided by OpenAL.
	fn raw_source(&self) -> sys::ALuint;

	/// Current playback state of the source.
	fn state(&self) -> AltoResult<SourceState>;
	/// Begin playing audio, or resume playing if previously paused.
	fn play(&mut self) -> AltoResult<()>;
	/// Pause playback while retaining playback position.
	fn pause(&mut self) -> AltoResult<()>;
	/// Stop playback and reset playback to the beginning of the audio data.
	fn stop(&mut self) -> AltoResult<()>;
	/// Reset playback to the beginning of the audio data.
	fn rewind(&mut self) -> AltoResult<()>;

	/// Whether the source has a listener-relative position.
	fn relative(&self) -> AltoResult<bool>;
	fn set_relative(&mut self, bool) -> AltoResult<()>;

	/// Minimum gain that will be applied by the distance model.
	fn min_gain(&self) -> AltoResult<f32>;
	fn set_min_gain(&mut self, f32) -> AltoResult<()>;

	/// Maximum gain that will be applied by the distance model.
	fn max_gain(&self) -> AltoResult<f32>;
	fn set_max_gain(&mut self, f32) -> AltoResult<()>;

	/// Distance at which the source will have unmodified gain.
	fn reference_distance(&self) -> AltoResult<f32>;
	fn set_reference_distance(&mut self, f32) -> AltoResult<()>;

	/// Rolloff factor of the distance model.
	fn rolloff_factor(&self) -> AltoResult<f32>;
	fn set_rolloff_factor(&mut self, f32) -> AltoResult<()>;

	/// Distance beyond which the source will no longer attenuate.
	fn max_distance(&self) -> AltoResult<f32>;
	fn set_max_distance(&mut self, f32) -> AltoResult<()>;

	/// Relative playback speed of the source.
	fn pitch(&self) -> AltoResult<f32>;
	fn set_pitch(&mut self, f32) -> AltoResult<()>;

	/// Direction vector of the source.
	fn direction<V: From<[f32; 3]>>(&self) -> AltoResult<V>;
	fn set_direction<V: Into<[f32; 3]>>(&mut self, V) -> AltoResult<()>;

	/// Angle from the direction vector within which the source will be fully heard.
	fn cone_inner_angle(&self) -> AltoResult<f32>;
	fn set_cone_inner_angle(&mut self, f32) -> AltoResult<()>;

	/// Angle from the direction vector within which the source will be heard at all.
	fn cone_outer_angle(&self) -> AltoResult<f32>;
	fn set_cone_outer_angle(&mut self, f32) -> AltoResult<()>;

	/// Gain factor to determine attenuation when the listener is within the outer cone but outiside the inner cone.
	fn cone_outer_gain(&self) -> AltoResult<f32>;
	fn set_cone_outer_gain(&mut self, f32) -> AltoResult<()>;

	/// Read cursor position in seconds.
	fn sec_offset(&self) -> AltoResult<f32>;
	fn set_sec_offset(&mut self, f32) -> AltoResult<()>;

	/// Read cursor position in samples.
	fn sample_offset(&self) -> AltoResult<sys::ALint>;
	fn set_sample_offset(&mut self, sys::ALint) -> AltoResult<()>;

	/// Read cursor position in bytes.
	fn byte_offset(&self) -> AltoResult<sys::ALint>;
	fn set_byte_offset(&mut self, sys::ALint) -> AltoResult<()>;

	/// A tuple of a playback position and the amount of time until that position is heard, in seconds.
	/// Requires `AL_SOFT_source_latency`.
	fn soft_sec_offset_latency(&self) -> AltoResult<(f64, f64)>;

	/// A tuple of a fixed point playback position in samples, and the time until that position is heard, in nanoseconds.
	/// Requires `AL_SOFT_source_latency`.
	fn soft_sample_offset_frac_latency(&self) -> AltoResult<(i32, i32, i64)>;

	/// Total length of the queued audio data in seconds.
	/// Requires `AL_SOFT_source_length`.
	fn soft_sec_length(&self) -> AltoResult<f32>;

	/// Total length of the queued audio data in samples.
	/// Requires `AL_SOFT_source_length`.
	fn soft_sample_length(&self) -> AltoResult<sys::ALint>;

	/// Total length of the queued audio data in bytes.
	/// Requires `AL_SOFT_source_length`.
	fn soft_byte_length(&self) -> AltoResult<sys::ALint>;

	/// Whether the audio data will be directly output to the corresponding output channels, bypassing any processing.
	/// Requires `AL_SOFT_direct_channels`.
	fn soft_direct_channels(&self) -> AltoResult<bool>;
	fn set_soft_direct_channels(&mut self, bool) -> AltoResult<()>;

	/// Distance model specific to this source.
	/// Requires `AL_EXT_source_distance_model`.
	fn distance_model(&self) -> AltoResult<DistanceModel>;
	fn set_distance_model(&mut self, DistanceModel) -> AltoResult<()>;
}


/// Playstack state of a source.
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


/// A source that can play shared static buffer.
pub struct StaticSource<'d: 'c, 'c> {
	src: Source<'d, 'c>,
	buf: Option<Arc<Buffer<'d, 'c>>>,
}


/// A source that plays a queue of owned buffers.
pub struct StreamingSource<'d: 'c, 'c> {
	src: Source<'d, 'c>,
	bufs: VecDeque<Buffer<'d, 'c>>,
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


	/// The device from which this context was created.
	pub fn device(&self) -> &(DeviceTrait + 'd) { self.dev }
	/// Raw context pointer as provided by OpenAL.
	pub fn raw_context(&self) -> *mut sys::ALCcontext { self.ctx }


	/// Query presence of an extension.
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


	#[doc(hidden)]
	pub fn extensions(&self) -> &ext::AlCache { &self.exts }


	fn make_current(&self, set: bool) -> AltoResult<Option<MutexGuard<()>>> {
		self.api.rent(|exts| {
			if let Ok(tlc) = exts.ALC_EXT_thread_local_context() {
				unsafe { tlc.alcSetThreadContext?(if set { self.ctx } else { ptr::null_mut() }); }
				self.dev.alto().get_error(self.dev.raw_device()).map(|_| None)
			} else {
				unsafe { self.api.owner().alcMakeContextCurrent()(if set { self.ctx } else { ptr::null_mut() }); }
				self.dev.alto().get_error(self.dev.raw_device()).map(|_| Some(self.ctx_lock.lock().unwrap()))
			}
		})
	}


	/// Distance model applied to all sources from this context.
	pub fn distance_model(&self) -> AltoResult<DistanceModel> {
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
			_ => Err(AltoError::AlInvalidValue),
		})
	}
	pub fn set_distance_model(&self, model: DistanceModel) -> AltoResult<()> {
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


	/// Using per-source distance model settings.
	/// Requires `AL_EXT_source_distance_model`.
	pub fn using_source_distance_model(&self) -> AltoResult<bool> {
		let _lock = self.make_current(true)?;
		let enabled = unsafe { self.api.owner().alIsEnabled()(self.exts.AL_EXT_source_distance_model()?.AL_SOURCE_DISTANCE_MODEL?) };
		self.get_error().map(|_| enabled == sys::AL_TRUE)
	}
	pub fn use_source_distance_model(&self, enable: bool) -> AltoResult<()> {
		let _lock = self.make_current(true)?;
		if enable {
			unsafe { self.api.owner().alEnable()(self.exts.AL_EXT_source_distance_model()?.AL_SOURCE_DISTANCE_MODEL?); }
		} else {
			unsafe { self.api.owner().alDisable()(self.exts.AL_EXT_source_distance_model()?.AL_SOURCE_DISTANCE_MODEL?); }
		}
		self.get_error()
	}


	/// Doppler factor applied based on relative velocities.
	pub fn doppler_factor(&self) -> AltoResult<f32> {
		let _lock = self.make_current(true)?;
		let doppler = unsafe { self.api.owner().alGetFloat()(sys::AL_DOPPLER_FACTOR) };
		self.get_error().map(|_| doppler)
	}
	pub fn set_doppler_factor(&self, doppler: f32) -> AltoResult<()> {
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alDopplerFactor()(doppler); }
		self.get_error()
	}


	/// Speed of sound, used for doppler calculations.
	pub fn speed_of_sound(&self) -> AltoResult<f32> {
		let _lock = self.make_current(true)?;
		let speed = unsafe { self.api.owner().alGetFloat()(sys::AL_SPEED_OF_SOUND) };
		self.get_error().map(|_| speed)
	}
	pub fn set_speed_of_sound(&self, speed: f32) -> AltoResult<()> {
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alSpeedOfSound()(speed); }
		self.get_error()
	}


	/// Global gain.
	pub fn gain(&self) -> AltoResult<f32> {
		let mut gain = 0.0;
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alGetListenerf()(sys::AL_GAIN, &mut gain); }
		self.get_error().map(|_| gain)
	}
	pub fn set_gain(&self, gain: f32) -> AltoResult<()> {
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alListenerf()(sys::AL_GAIN, gain); }
		self.get_error()
	}


	/// Position of the listener.
	pub fn position<V: From<[f32; 3]>>(&self) -> AltoResult<V> {
		let mut pos = [0.0, 0.0, 0.0];
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alGetListenerfv()(sys::AL_POSITION, &mut pos as *mut [f32; 3] as *mut sys::ALfloat); }
		self.get_error().map(|_| pos.into())
	}
	pub fn set_position<V: Into<[f32; 3]>>(&self, pos: V) -> AltoResult<()> {
		let pos = pos.into();
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alListenerfv()(sys::AL_POSITION, &pos as *const [f32; 3] as *const sys::ALfloat); }
		self.get_error()
	}


	/// Velocity of the listener.
	pub fn velocity<V: From<[f32; 3]>>(&self) -> AltoResult<V> {
		let mut vel = [0.0, 0.0, 0.0];
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alGetListenerfv()(sys::AL_VELOCITY, &mut vel as *mut [f32; 3] as *mut sys::ALfloat); }
		self.get_error().map(|_| vel.into())
	}
	pub fn set_velocity<V: Into<[f32; 3]>>(&self, vel: V) -> AltoResult<()> {
		let vel = vel.into();
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alListenerfv()(sys::AL_VELOCITY, &vel as *const [f32; 3] as *const sys::ALfloat); }
		self.get_error()
	}


	/// Orientation of the listener, consisting of a forward vector and an up vector.
	pub fn orientation<V: From<[f32; 3]>>(&self) -> AltoResult<(V, V)> {
		let mut or = [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alGetListenerfv()(sys::AL_ORIENTATION, &mut or as *mut [[f32; 3]; 2] as *mut sys::ALfloat); }
		self.get_error().map(|_| (or[0].into(), or[1].into()))
	}
	pub fn set_orientation<V: Into<[f32; 3]>>(&self, at: V, up: V) -> AltoResult<()> {
		let or = [at.into(), up.into()];
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alListenerfv()(sys::AL_ORIENTATION, &or as *const [[f32; 3]; 2] as *const sys::ALfloat); }
		self.get_error()
	}


	/// Create a new, empty buffer object.
	pub fn new_buffer<'c>(&'c self) -> AltoResult<Buffer<'d, 'c>> {
		let mut buf = 0;
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alGenBuffers()(1, &mut buf as *mut sys::ALuint); }
		self.get_error().map(|_| Buffer{ctx: self, buf: buf})
	}


	/// Create a new static source.
	pub fn new_static_source(&self) -> AltoResult<StaticSource> {
		let mut src = 0;
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alGenSources()(1, &mut src as *mut sys::ALuint); }
		self.get_error().map(|_| StaticSource{src: Source{ctx: self, src: src}, buf: None})
	}


	/// Create a new streaming source.
	pub fn new_streaming_source(&self) -> AltoResult<StreamingSource> {
		let mut src = 0;
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alGenSources()(1, &mut src as *mut sys::ALuint); }
		self.get_error().map(|_| StreamingSource{src: Source{ctx: self, src: src}, bufs: VecDeque::new()})
	}


	/// Begin playing all specified sources simultaneously.
	pub fn play_all<S, I>(&self, srcs: I) -> AltoResult<()> where
		S: SourceTrait<'d>,
		I: Iterator,
		<I as Iterator>::Item: AsRef<S> + AsMut<S>,
	{
		let v: Vec<_> = srcs.filter(|s| s.as_ref().context() == self).map(|s| s.as_ref().raw_source()).collect();
		if v.len() > sys::ALint::max_value() as usize { return Err(AltoError::AlInvalidValue) }

		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alSourcePlayv()(v.len() as i32, v.as_slice().as_ptr()); }
		self.get_error()
	}


	/// Pause all specified sources simultaneously.
	pub fn pause_all<S, I>(&self, srcs: I) -> AltoResult<()> where
		S: SourceTrait<'d>,
		I: Iterator,
		<I as Iterator>::Item: AsRef<S> + AsMut<S>,
	{
		let v: Vec<_> = srcs.filter(|s| s.as_ref().context() == self).map(|s| s.as_ref().raw_source()).collect();
		if v.len() > sys::ALint::max_value() as usize { return Err(AltoError::AlInvalidValue) }

		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alSourcePausev()(v.len() as i32, v.as_slice().as_ptr()); }
		self.get_error()
	}


	/// Stop all specified sources simultaneously.
	pub fn stop_all<S, I>(&self, srcs: I) -> AltoResult<()> where
		S: SourceTrait<'d>,
		I: Iterator,
		<I as Iterator>::Item: AsRef<S> + AsMut<S>,
	{
		let v: Vec<_> = srcs.filter(|s| s.as_ref().context() == self).map(|s| s.as_ref().raw_source()).collect();
		if v.len() > sys::ALint::max_value() as usize { return Err(AltoError::AlInvalidValue) }

		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alSourceStopv()(v.len() as i32, v.as_slice().as_ptr()); }
		self.get_error()
	}


	/// Rewind all specified sources simultaneously.
	pub fn rewind_all<S, I>(&self, srcs: I) -> AltoResult<()> where
		S: SourceTrait<'d>,
		I: Iterator,
		<I as Iterator>::Item: AsRef<S> + AsMut<S>,
	{
		let v: Vec<_> = srcs.filter(|s| s.as_ref().context() == self).map(|s| s.as_ref().raw_source()).collect();
		if v.len() > sys::ALint::max_value() as usize { return Err(AltoError::AlInvalidValue) }

		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alSourceRewindv()(v.len() as i32, v.as_slice().as_ptr()); }
		self.get_error()
	}


	/// Suspend state updates for the context, returning a guard object.
	/// Until the guard object is dropped, any state changes for the context
	/// are deferred. After the guard is dropped, all pending changes will
	/// be applied simultaneously.
	pub fn suspend<'c>(&'c self) -> AltoResult<SuspendLock<'d, 'c>> {
		SuspendLock::new(self)
	}


	#[doc(hidden)]
	pub fn get_error(&self) -> AltoResult<()> {
		match unsafe { self.api.owner().alGetError()() } {
			sys::AL_NO_ERROR => Ok(()),
			e => Err(AltoError::from_al(e))
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
	fn new(ctx: &'c Context<'d>) -> AltoResult<SuspendLock<'d, 'c>> {
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
	/// Context from which this buffer was created.
	pub fn context(&self) -> &Context<'d> { self.ctx }
	/// Raw handle as provided by OpenAL.
	pub fn raw_buffer(&self) -> sys::ALuint { self.buf }


	/// Upload sound data from a slice of sample frames.
	pub fn set_data<F: SampleFrame, R: AsBufferData<F>>(&mut self, data: R, freq: i32) -> AltoResult<()> {
		let data = data.as_buffer_data();
		let size = data.len() * mem::size_of::<F>();
		if sys::ALsizei::max_value() as usize / mem::size_of::<F>() < data.len() { return Err(AltoError::AlInvalidValue) }

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


	/// Sample-rate of the audio in the buffer.
	pub fn frequency(&self) -> AltoResult<sys::ALint> {
		let mut freq = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetBufferi()(self.buf, sys::AL_FREQUENCY, &mut freq); }
		self.ctx.get_error().map(|_| freq)
	}


	/// Bit-depth of the audio in the buffer.
	pub fn bits(&self) -> AltoResult<sys::ALint> {
		let mut bits = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetBufferi()(self.buf, sys::AL_BITS, &mut bits); }
		self.ctx.get_error().map(|_| bits)
	}


	/// Number of channels for the audio in the buffer.
	pub fn channels(&self) -> AltoResult<sys::ALint> {
		let mut chans = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetBufferi()(self.buf, sys::AL_CHANNELS, &mut chans); }
		self.ctx.get_error().map(|_| chans)
	}


	/// Size in bytes of the audio in the buffer.
	pub fn size(&self) -> AltoResult<sys::ALint> {
		let mut size = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetBufferi()(self.buf, sys::AL_SIZE, &mut size); }
		self.ctx.get_error().map(|_| size)
	}


	/// Loop points for the audio in the buffer, as a tuple of start and end samples.
	/// Requires `AL_SOFT_loop_points`.
	pub fn soft_loop_points(&self) -> AltoResult<(sys::ALint, sys::ALint)> {
		let mut points = [0, 0];
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetBufferiv()(self.buf, self.ctx.exts.AL_SOFT_loop_points()?.AL_LOOP_POINTS_SOFT?, &mut points as *mut [sys::ALint; 2] as *mut sys::ALint); }
		self.ctx.get_error().map(|_| (points[0], points[1]))
	}
	pub fn set_soft_loop_points(&self, start: sys::ALint, end: sys::ALint) -> AltoResult<()> {
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


	fn state(&self) -> AltoResult<SourceState> {
		let mut state = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_SOURCE_STATE, &mut state); }
		self.ctx.get_error().and_then(|_| match state {
			sys::AL_INITIAL => Ok(SourceState::Initial),
			sys::AL_PLAYING => Ok(SourceState::Playing),
			sys::AL_PAUSED => Ok(SourceState::Paused),
			sys::AL_STOPPED => Ok(SourceState::Stopped),
			_ => Err(AltoError::AlInvalidEnum),
		})
	}
	fn play(&mut self) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcePlay()(self.src); }
		self.ctx.get_error()
	}
	fn pause(&mut self) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcePause()(self.src); }
		self.ctx.get_error()
	}
	fn stop(&mut self) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourceStop()(self.src); }
		self.ctx.get_error()
	}
	fn rewind(&mut self) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourceRewind()(self.src); }
		self.ctx.get_error()
	}


	fn relative(&self) -> AltoResult<bool> {
		let mut rel = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_SOURCE_RELATIVE, &mut rel); }
		self.ctx.get_error().map(|_| rel == sys::AL_TRUE as sys::ALint)
	}
	fn set_relative(&mut self, rel: bool) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, sys::AL_SOURCE_RELATIVE, if rel { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.ctx.get_error()
	}


	fn min_gain(&self) -> AltoResult<f32> {
		let mut gain = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_MIN_GAIN, &mut gain); }
		self.ctx.get_error().map(|_| gain)
	}
	fn set_min_gain(&mut self, gain: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_MIN_GAIN, gain); }
		self.ctx.get_error()
	}


	fn max_gain(&self) -> AltoResult<f32> {
		let mut gain = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_MAX_GAIN, &mut gain); }
		self.ctx.get_error().map(|_| gain)
	}
	fn set_max_gain(&mut self, gain: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_MAX_GAIN, gain); }
		self.ctx.get_error()
	}


	fn reference_distance(&self) -> AltoResult<f32> {
		let mut dist = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_REFERENCE_DISTANCE, &mut dist); }
		self.ctx.get_error().map(|_| dist)
	}
	fn set_reference_distance(&mut self, dist: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_REFERENCE_DISTANCE, dist); }
		self.ctx.get_error()
	}


	fn rolloff_factor(&self) -> AltoResult<f32> {
		let mut rolloff = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_ROLLOFF_FACTOR, &mut rolloff); }
		self.ctx.get_error().map(|_| rolloff)
	}
	fn set_rolloff_factor(&mut self, rolloff: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_ROLLOFF_FACTOR, rolloff); }
		self.ctx.get_error()
	}


	fn max_distance(&self) -> AltoResult<f32> {
		let mut dist = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_MAX_DISTANCE, &mut dist); }
		self.ctx.get_error().map(|_| dist)
	}
	fn set_max_distance(&mut self, dist: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_MAX_DISTANCE, dist); }
		self.ctx.get_error()
	}


	fn pitch(&self) -> AltoResult<f32> {
		let mut pitch = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_PITCH, &mut pitch); }
		self.ctx.get_error().map(|_| pitch)
	}
	fn set_pitch(&mut self, pitch: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_PITCH, pitch); }
		self.ctx.get_error()
	}


	fn direction<V: From<[f32; 3]>>(&self) -> AltoResult<V> {
		let mut dir = [0.0, 0.0, 0.0];
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcefv()(self.src, sys::AL_DIRECTION, &mut dir as *mut [f32; 3] as *mut sys::ALfloat); }
		self.ctx.get_error().map(|_| dir.into())
	}
	fn set_direction<V: Into<[f32; 3]>>(&mut self, dir: V) -> AltoResult<()> {
		let dir = dir.into();
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcefv()(self.src, sys::AL_DIRECTION, &dir as *const [f32; 3] as *const sys::ALfloat); }
		self.ctx.get_error()
	}


	fn cone_inner_angle(&self) -> AltoResult<f32> {
		let mut angle = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_CONE_INNER_ANGLE, &mut angle); }
		self.ctx.get_error().map(|_| angle)
	}
	fn set_cone_inner_angle(&mut self, angle: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_CONE_INNER_ANGLE, angle); }
		self.ctx.get_error()
	}


	fn cone_outer_angle(&self) -> AltoResult<f32> {
		let mut angle = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_CONE_OUTER_ANGLE, &mut angle); }
		self.ctx.get_error().map(|_| angle)
	}
	fn set_cone_outer_angle(&mut self, angle: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_CONE_OUTER_ANGLE, angle); }
		self.ctx.get_error()
	}


	fn cone_outer_gain(&self) -> AltoResult<f32> {
		let mut gain = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_CONE_OUTER_GAIN, &mut gain); }
		self.ctx.get_error().map(|_| gain)
	}
	fn set_cone_outer_gain(&mut self, gain: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_CONE_OUTER_GAIN, gain); }
		self.ctx.get_error()
	}


	fn sec_offset(&self) -> AltoResult<f32> {
		let mut offset = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_SEC_OFFSET, &mut offset); }
		self.ctx.get_error().map(|_| offset)
	}
	fn set_sec_offset(&mut self, offset: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_SEC_OFFSET, offset); }
		self.ctx.get_error()
	}


	fn sample_offset(&self) -> AltoResult<sys::ALint> {
		let mut offset = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_SAMPLE_OFFSET, &mut offset); }
		self.ctx.get_error().map(|_| offset)
	}
	fn set_sample_offset(&mut self, offset: sys::ALint) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, sys::AL_SAMPLE_OFFSET, offset); }
		self.ctx.get_error()
	}


	fn byte_offset(&self) -> AltoResult<sys::ALint> {
		let mut offset = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_BYTE_OFFSET, &mut offset); }
		self.ctx.get_error().map(|_| offset)
	}
	fn set_byte_offset(&mut self, offset: sys::ALint) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, sys::AL_BYTE_OFFSET, offset); }
		self.ctx.get_error()
	}


	fn soft_sec_offset_latency(&self) -> AltoResult<(f64, f64)> {
		let mut offset_latency = [0.0, 0.0];
		let assl = self.ctx.exts.AL_SOFT_source_latency()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { assl.alGetSourcedvSOFT?(self.src, assl.AL_SEC_OFFSET_LATENCY_SOFT?, &mut offset_latency as *mut [f64; 2] as *mut f64); }
		self.ctx.get_error().map(|_| (offset_latency[0], offset_latency[1]))
	}


	fn soft_sample_offset_frac_latency(&self) -> AltoResult<(i32, i32, i64)> {
		let mut offset_latency = [0, 0];
		let assl = self.ctx.exts.AL_SOFT_source_latency()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { assl.alGetSourcei64vSOFT?(self.src, assl.AL_SAMPLE_OFFSET_LATENCY_SOFT?, &mut offset_latency as *mut [i64; 2] as *mut i64); }
		self.ctx.get_error().map(|_| ((offset_latency[0] >> 32) as i32, offset_latency[0] as i32, offset_latency[1]))
	}


	fn soft_direct_channels(&self) -> AltoResult<bool> {
		let mut direct = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, self.ctx.exts.AL_SOFT_direct_channels()?.AL_DIRECT_CHANNELS_SOFT?, &mut direct); }
		self.ctx.get_error().map(|_| direct == sys::AL_TRUE as sys::ALint)
	}
	fn set_soft_direct_channels(&mut self, direct: bool) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, self.ctx.exts.AL_SOFT_direct_channels()?.AL_DIRECT_CHANNELS_SOFT?, if direct { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.ctx.get_error()
	}


	fn soft_sec_length(&self) -> AltoResult<f32> {
		let mut length = 0.0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, self.ctx.exts.AL_SOFT_source_length()?.AL_SEC_LENGTH_SOFT?, &mut length); }
		self.ctx.get_error().map(|_| length)
	}


	fn soft_sample_length(&self) -> AltoResult<sys::ALint> {
		let mut length = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, self.ctx.exts.AL_SOFT_source_length()?.AL_SAMPLE_LENGTH_SOFT?, &mut length); }
		self.ctx.get_error().map(|_| length)
	}


	fn soft_byte_length(&self) -> AltoResult<sys::ALint> {
		let mut length = 0;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, self.ctx.exts.AL_SOFT_source_length()?.AL_BYTE_LENGTH_SOFT?, &mut length); }
		self.ctx.get_error().map(|_| length)
	}


	fn distance_model(&self) -> AltoResult<DistanceModel> {
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
			_ => Err(AltoError::AlInvalidValue),
		})
	}
	fn set_distance_model(&mut self, model: DistanceModel) -> AltoResult<()> {
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
	/// The shared buffer currently associated with this source.
	pub fn buffer(&self) -> Option<&Arc<Buffer<'d, 'c>>> { self.buf.as_ref() }


	/// Associate a shared buffer with the source.
	pub fn set_buffer<B: Into<Option<Arc<Buffer<'d, 'c>>>>>(&mut self, buf: B) -> AltoResult<()> {
		let buf = buf.into();

		{
			let _lock = self.src.ctx.make_current(true)?;
			unsafe { self.src.ctx.api.owner().alSourcei()(self.src.src, sys::AL_BUFFER, if let Some(ref buf) = buf { buf.buf as sys::ALint } else { 0 }); }
			self.src.ctx.get_error()?;
		}

		self.buf = buf;
		Ok(())
	}


	pub fn looping(&self) -> AltoResult<bool> {
		let mut loop_ = 0;
		let _lock = self.src.ctx.make_current(true)?;
		unsafe { self.src.ctx.api.owner().alGetSourcei()(self.src.src, sys::AL_LOOPING, &mut loop_); }
		self.src.ctx.get_error().map(|_| loop_ == sys::AL_TRUE as sys::ALint)
	}
	pub fn set_looping(&mut self, loop_: bool) -> AltoResult<()> {
		let _lock = self.src.ctx.make_current(true)?;
		unsafe { self.src.ctx.api.owner().alSourcei()(self.src.src, sys::AL_LOOPING, if loop_ { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.src.ctx.get_error()
	}


}


impl<'d: 'c, 'c> SourceTrait<'d> for StaticSource<'d, 'c> {
	fn context(&self) -> &Context<'d> { self.src.context() }
	fn raw_source(&self) -> sys::ALuint { self.src.raw_source() }

	fn state(&self) -> AltoResult<SourceState> { self.src.state() }
	fn play(&mut self) -> AltoResult<()> { self.src.play() }
	fn pause(&mut self) -> AltoResult<()> { self.src.pause() }
	fn stop(&mut self) -> AltoResult<()> { self.src.stop() }
	fn rewind(&mut self) -> AltoResult<()> { self.src.rewind() }

	fn relative(&self) -> AltoResult<bool> { self.src.relative() }
	fn set_relative(&mut self, rel: bool) -> AltoResult<()> { self.src.set_relative(rel) }

	fn min_gain(&self) -> AltoResult<f32> { self.src.min_gain() }
	fn set_min_gain(&mut self, gain: f32) -> AltoResult<()> { self.src.set_min_gain(gain) }

	fn max_gain(&self) -> AltoResult<f32> { self.src.max_gain() }
	fn set_max_gain(&mut self, gain: f32) -> AltoResult<()> { self.src.set_max_gain(gain) }

	fn reference_distance(&self) -> AltoResult<f32> { self.src.reference_distance() }
	fn set_reference_distance(&mut self, dist: f32) -> AltoResult<()> { self.src.set_reference_distance(dist) }

	fn rolloff_factor(&self) -> AltoResult<f32> { self.src.rolloff_factor() }
	fn set_rolloff_factor(&mut self, rolloff: f32) -> AltoResult<()> { self.src.set_rolloff_factor(rolloff) }

	fn max_distance(&self) -> AltoResult<f32> { self.src.max_distance() }
	fn set_max_distance(&mut self, dist: f32) -> AltoResult<()> { self.src.set_max_distance(dist) }

	fn pitch(&self) -> AltoResult<f32> { self.src.pitch() }
	fn set_pitch(&mut self, pitch: f32) -> AltoResult<()> { self.src.set_pitch(pitch) }

	fn direction<V: From<[f32; 3]>>(&self) -> AltoResult<V> { self.src.direction() }
	fn set_direction<V: Into<[f32; 3]>>(&mut self, dir: V) -> AltoResult<()> { self.src.set_direction(dir) }

	fn cone_inner_angle(&self) -> AltoResult<f32> { self.src.cone_inner_angle() }
	fn set_cone_inner_angle(&mut self, angle: f32) -> AltoResult<()> { self.src.set_cone_inner_angle(angle) }

	fn cone_outer_angle(&self) -> AltoResult<f32> { self.src.cone_outer_angle() }
	fn set_cone_outer_angle(&mut self, angle: f32) -> AltoResult<()> { self.src.set_cone_outer_angle(angle) }

	fn cone_outer_gain(&self) -> AltoResult<f32> { self.src.cone_outer_gain() }
	fn set_cone_outer_gain(&mut self, gain: f32) -> AltoResult<()> { self.src.set_cone_outer_gain(gain) }

	fn sec_offset(&self) -> AltoResult<f32> { self.src.sec_offset() }
	fn set_sec_offset(&mut self, offset: f32) -> AltoResult<()> { self.src.set_sec_offset(offset) }

	fn sample_offset(&self) -> AltoResult<sys::ALint> { self.src.sample_offset() }
	fn set_sample_offset(&mut self, offset: sys::ALint) -> AltoResult<()> { self.src.set_sample_offset(offset) }

	fn byte_offset(&self) -> AltoResult<sys::ALint> { self.src.byte_offset() }
	fn set_byte_offset(&mut self, offset: sys::ALint) -> AltoResult<()> { self.src.set_byte_offset(offset) }

	fn soft_sec_offset_latency(&self) -> AltoResult<(f64, f64)> { self.src.soft_sec_offset_latency() }

	fn soft_sample_offset_frac_latency(&self) -> AltoResult<(i32, i32, i64)> { self.src.soft_sample_offset_frac_latency() }

	fn soft_sec_length(&self) -> AltoResult<f32> { self.src.soft_sec_length() }

	fn soft_sample_length(&self) -> AltoResult<sys::ALint> { self.src.soft_sample_length() }

	fn soft_byte_length(&self) -> AltoResult<sys::ALint> { self.src.soft_byte_length() }

	fn soft_direct_channels(&self) -> AltoResult<bool> { self.src.soft_direct_channels() }
	fn set_soft_direct_channels(&mut self, direct: bool) -> AltoResult<()> { self.src.set_soft_direct_channels(direct) }

	fn distance_model(&self) -> AltoResult<DistanceModel> { self.src.distance_model() }
	fn set_distance_model(&mut self, model: DistanceModel) -> AltoResult<()> { self.src.set_distance_model(model) }
}


impl<'d: 'c, 'c> PartialEq for StaticSource<'d, 'c> {
	fn eq(&self, other: &StaticSource<'d, 'c>) -> bool {
		self.src.ctx == other.src.ctx && self.src.src == other.src.src
	}
}
impl<'d: 'c, 'c> Eq for StaticSource<'d, 'c> { }


impl<'d: 'c, 'c> StreamingSource<'d, 'c> {
	/// Number of buffers currently queued in this stream.
	pub fn buffers_queued(&self) -> AltoResult<sys::ALint> {
		Ok(self.bufs.len() as sys::ALint)
	}


	/// Number of buffers that have been fully processed by this stream.
	pub fn buffers_processed(&self) -> AltoResult<sys::ALint> {
		let mut bufs = 0;
		let _lock = self.src.ctx.make_current(true)?;
		unsafe { self.src.ctx.api.owner().alGetSourcei()(self.src.src, sys::AL_BUFFERS_PROCESSED, &mut bufs); }
		self.src.ctx.get_error().map(|_| bufs)
	}


	/// Enqueue a buffer to the stream.
	pub fn queue_buffer(&mut self, buf: Buffer<'d, 'c>) -> Result<(), (AltoError, Buffer<'d, 'c>)> {
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


	/// Remove a processed buffer from the queue.
	pub fn unqueue_buffer(&mut self) -> AltoResult<Buffer<'d, 'c>> {
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

	fn state(&self) -> AltoResult<SourceState> { self.src.state() }
	fn play(&mut self) -> AltoResult<()> { self.src.play() }
	fn pause(&mut self) -> AltoResult<()> { self.src.pause() }
	fn stop(&mut self) -> AltoResult<()> { self.src.stop() }
	fn rewind(&mut self) -> AltoResult<()> { self.src.rewind() }

	fn relative(&self) -> AltoResult<bool> { self.src.relative() }
	fn set_relative(&mut self, rel: bool) -> AltoResult<()> { self.src.set_relative(rel) }

	fn min_gain(&self) -> AltoResult<f32> { self.src.min_gain() }
	fn set_min_gain(&mut self, gain: f32) -> AltoResult<()> { self.src.set_min_gain(gain) }

	fn max_gain(&self) -> AltoResult<f32> { self.src.max_gain() }
	fn set_max_gain(&mut self, gain: f32) -> AltoResult<()> { self.src.set_max_gain(gain) }

	fn reference_distance(&self) -> AltoResult<f32> { self.src.reference_distance() }
	fn set_reference_distance(&mut self, dist: f32) -> AltoResult<()> { self.src.set_reference_distance(dist) }

	fn rolloff_factor(&self) -> AltoResult<f32> { self.src.rolloff_factor() }
	fn set_rolloff_factor(&mut self, rolloff: f32) -> AltoResult<()> { self.src.set_rolloff_factor(rolloff) }

	fn max_distance(&self) -> AltoResult<f32> { self.src.max_distance() }
	fn set_max_distance(&mut self, dist: f32) -> AltoResult<()> { self.src.set_max_distance(dist) }

	fn pitch(&self) -> AltoResult<f32> { self.src.pitch() }
	fn set_pitch(&mut self, pitch: f32) -> AltoResult<()> { self.src.set_pitch(pitch) }

	fn direction<V: From<[f32; 3]>>(&self) -> AltoResult<V> { self.src.direction() }
	fn set_direction<V: Into<[f32; 3]>>(&mut self, dir: V) -> AltoResult<()> { self.src.set_direction(dir) }

	fn cone_inner_angle(&self) -> AltoResult<f32> { self.src.cone_inner_angle() }
	fn set_cone_inner_angle(&mut self, angle: f32) -> AltoResult<()> { self.src.set_cone_inner_angle(angle) }

	fn cone_outer_angle(&self) -> AltoResult<f32> { self.src.cone_outer_angle() }
	fn set_cone_outer_angle(&mut self, angle: f32) -> AltoResult<()> { self.src.set_cone_outer_angle(angle) }

	fn cone_outer_gain(&self) -> AltoResult<f32> { self.src.cone_outer_gain() }
	fn set_cone_outer_gain(&mut self, gain: f32) -> AltoResult<()> { self.src.set_cone_outer_gain(gain) }

	fn sec_offset(&self) -> AltoResult<f32> { self.src.sec_offset() }
	fn set_sec_offset(&mut self, offset: f32) -> AltoResult<()> { self.src.set_sec_offset(offset) }

	fn sample_offset(&self) -> AltoResult<sys::ALint> { self.src.sample_offset() }
	fn set_sample_offset(&mut self, offset: sys::ALint) -> AltoResult<()> { self.src.set_sample_offset(offset) }

	fn byte_offset(&self) -> AltoResult<sys::ALint> { self.src.byte_offset() }
	fn set_byte_offset(&mut self, offset: sys::ALint) -> AltoResult<()> { self.src.set_byte_offset(offset) }

	fn soft_sec_offset_latency(&self) -> AltoResult<(f64, f64)> { self.src.soft_sec_offset_latency() }

	fn soft_sample_offset_frac_latency(&self) -> AltoResult<(i32, i32, i64)> { self.src.soft_sample_offset_frac_latency() }

	fn soft_sec_length(&self) -> AltoResult<f32> { self.src.soft_sec_length() }

	fn soft_sample_length(&self) -> AltoResult<sys::ALint> { self.src.soft_sample_length() }

	fn soft_byte_length(&self) -> AltoResult<sys::ALint> { self.src.soft_byte_length() }

	fn soft_direct_channels(&self) -> AltoResult<bool> { self.src.soft_direct_channels() }
	fn set_soft_direct_channels(&mut self, direct: bool) -> AltoResult<()> { self.src.set_soft_direct_channels(direct) }

	fn distance_model(&self) -> AltoResult<DistanceModel> { self.src.distance_model() }
	fn set_distance_model(&mut self, model: DistanceModel) -> AltoResult<()> { self.src.set_distance_model(model) }
}


impl<'d: 'c, 'c> PartialEq for StreamingSource<'d, 'c> {
	fn eq(&self, other: &StreamingSource<'d, 'c>) -> bool {
		self.src.ctx == other.src.ctx && self.src.src == other.src.src
	}
}
impl<'d: 'c, 'c> Eq for StreamingSource<'d, 'c> { }
