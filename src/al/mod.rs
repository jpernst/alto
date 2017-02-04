use std::ops::Deref;
use std::iter;
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::VecDeque;
use std::io::{self, Write};
use std::mem;
use std::ptr;
use std::hash::{Hash, Hasher};

use ::{AltoError, AltoResult};
use sys;
use alc::*;
use efx::*;
use ext;


mod format;
pub use self::format::*;


lazy_static! {
	#[doc(hidden)]
	#[no_mangle]
    pub static ref ALTO_CTX_LOCK__: Mutex<()> = Mutex::new(());
}


/// The gain curve of sources as a function of distance to the listener.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum DistanceModel {
	/// `AL_NONE`
	None,
	/// `AL_INVERSE_DISTANCE`
	Inverse,
	/// `AL_INVERSE_DISTANCE_CLAMPED`
	InverseClamped,
	/// `AL_LINEAR_DISTANCE`
	Linear,
	/// `AL_LINEAR_DISTANCE_CLAMPED`
	LinearClamped,
	/// `AL_EXPONENT_DISTANCE`
	Exponent,
	/// `AL_EXPONENT_DISTANCE_CLAMPLED`
	ExponentClamped,
}


/// A listener context.
pub struct Context<'d> {
	dev: &'d DeviceTrait,
	api: &'d AlApi<'static>,
	ctx: *mut sys::ALCcontext,
	exts: ext::AlCache<'d>,
	defer_rc: Arc<AtomicUsize>,
}


/// An RAII lock that will suspend state updates while held.
/// When this lock is dropped, the context will apply all pending updates.
pub struct SuspendLock<'d: 'c, 'c>(&'c Context<'d>);


/// An audio buffer of any format.
pub struct Buffer<'d: 'c, 'c> {
	ctx: &'c Context<'d>,
	buf: sys::ALuint, 
}


/// Capabilities common to both static and streaming sources.
pub unsafe trait SourceTrait<'d: 'c, 'c> {
	/// The context from which this source was created.
	fn context(&self) -> &Context<'d>;
	/// Raw handle as provided by OpenAL.
	fn as_raw(&self) -> sys::ALuint;

	/// `alGetSourcei(AL_SOURCE_STATE)`
	fn state(&self) -> AltoResult<SourceState>;
	/// `alSourcePlay()`
	fn play(&mut self) -> AltoResult<()>;
	/// `alSourcePause()`
	fn pause(&mut self) -> AltoResult<()>;
	/// `alSourceStop()`
	fn stop(&mut self) -> AltoResult<()>;
	/// `alSourceRewind()`
	fn rewind(&mut self) -> AltoResult<()>;

	/// `alGetSourcei(AL_SOURCE_RELATIVE)`
	fn relative(&self) -> AltoResult<bool>;
	/// `alSourcei(AL_SOURCE_RELATIVE)`
	fn set_relative(&mut self, bool) -> AltoResult<()>;

	/// `alGetSourcef(AL_GAIN)`
	fn gain(&self) -> AltoResult<f32>;
	/// `alSourcef(AL_GAIN)`
	fn set_gain(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_MIN_GAIN)`
	fn min_gain(&self) -> AltoResult<f32>;
	/// `alSourcef(AL_MIN_GAIN)`
	fn set_min_gain(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_MAX_GAIN)`
	fn max_gain(&self) -> AltoResult<f32>;
	/// `alSourcef(AL_MAX_GAIN)`
	fn set_max_gain(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_REFERENCE_DISTANCE)`
	fn reference_distance(&self) -> AltoResult<f32>;
	/// `alSourcef(AL_REFERENCE_DISTANCE)`
	fn set_reference_distance(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_ROLLOFF_FACTOR)`
	fn rolloff_factor(&self) -> AltoResult<f32>;
	/// `alSourcef(AL_ROLLOFF_FACTOR)`
	fn set_rolloff_factor(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_MAX_DISTANCE)`
	fn max_distance(&self) -> AltoResult<f32>;
	/// `alSourcef(AL_MAX_DISTANCE)`
	fn set_max_distance(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_PITCH)`
	fn pitch(&self) -> AltoResult<f32>;
	/// `alSourcef(AL_PITCH)`
	fn set_pitch(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcefv(AL_POSITION)`
	fn position<V: From<[f32; 3]>>(&self) -> AltoResult<V>;
	/// `alSourcefv(AL_POSITION)`
	fn set_position<V: Into<[f32; 3]>>(&mut self, V) -> AltoResult<()>;

	/// `alGetSourcefv(AL_VELOCITY)`
	fn velocity<V: From<[f32; 3]>>(&self) -> AltoResult<V>;
	/// `alSourcefv(AL_VELOCITY)`
	fn set_velocity<V: Into<[f32; 3]>>(&mut self, V) -> AltoResult<()>;

	/// `alGetSourcefv(AL_DIRECTION)`
	fn direction<V: From<[f32; 3]>>(&self) -> AltoResult<V>;
	/// `alSourcefv(AL_DIRECTION)`
	fn set_direction<V: Into<[f32; 3]>>(&mut self, V) -> AltoResult<()>;

	/// `alGetSourcef(AL_CONE_INNER_ANGLE)`
	fn cone_inner_angle(&self) -> AltoResult<f32>;
	/// `alSourcef(AL_CONE_INNER_ANGLE)`
	fn set_cone_inner_angle(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_CONE_OUTER_ANGLE)`
	fn cone_outer_angle(&self) -> AltoResult<f32>;
	/// `alSourcef(AL_CONE_OUTER_ANGLE)`
	fn set_cone_outer_angle(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_CONE_OUTER_GAIN)`
	fn cone_outer_gain(&self) -> AltoResult<f32>;
	/// `alSourcef(AL_CONE_OUTER_GAIN)`
	fn set_cone_outer_gain(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_SEC_OFFSET)`
	fn sec_offset(&self) -> AltoResult<f32>;
	/// `alSourcef(AL_SEC_OFFSET)`
	fn set_sec_offset(&mut self, f32) -> AltoResult<()>;

	/// `alSetSourcei(AL_SAMPLE_OFFSET)`
	fn sample_offset(&self) -> AltoResult<sys::ALint>;
	/// `alSourcei(AL_SAMPLE_OFFSET)`
	fn set_sample_offset(&mut self, sys::ALint) -> AltoResult<()>;

	/// `alSetSourcei(AL_BYTE_OFFSET)`
	fn byte_offset(&self) -> AltoResult<sys::ALint>;
	/// `alSourcei(AL_BYTE_OFFSET)`
	fn set_byte_offset(&mut self, sys::ALint) -> AltoResult<()>;

	/// `alGetSourcedvSOFT(AL_SEC_OFFSET_LATENCY_SOFT)`
	/// Requires `AL_SOFT_source_latency`
	fn soft_sec_offset_latency(&self) -> AltoResult<(f64, f64)>;

	/// `alGetSourcei16vSOFT(AL_SAMPLE_OFFSET_LATENCY_SOFT)`
	/// Requires `AL_SOFT_source_latency`
	fn soft_sample_frac_offset_latency(&self) -> AltoResult<(i32, i32, i64)>;

	/// `alGetSourcef(AL_SEC_LENGTH_SOFT)`
	/// Requires `AL_SOFT_source_length`
	fn soft_sec_length(&self) -> AltoResult<f32>;

	/// `alGetSourcei(AL_SAMPLE_LENGTH_SOFT)`
	/// Requires `AL_SOFT_source_length`
	fn soft_sample_length(&self) -> AltoResult<sys::ALint>;

	/// `alGetSourcei(AL_BYTE_LENGTH_SOFT)`
	/// Requires `AL_SOFT_source_length`
	fn soft_byte_length(&self) -> AltoResult<sys::ALint>;

	/// `alGetSourcei(AL_DIRECT_CHANNELS_SOFT)`
	/// Requires `AL_SOFT_direct_channels`
	fn soft_direct_channels(&self) -> AltoResult<bool>;
	/// `alSourcei(AL_DIRECT_CHANNELS_SOFT)`
	/// Requires `AL_SOFT_direct_channels`
	fn set_soft_direct_channels(&mut self, bool) -> AltoResult<()>;

	/// `alGetSourcei(AL_DISTANCE_MODEL)`
	/// Requires `AL_EXT_source_distance_model`
	fn distance_model(&self) -> AltoResult<DistanceModel>;
	/// `alSourcei(AL_DISTANCE_MODEL)`
	/// Requires `AL_EXT_source_distance_model`
	fn set_distance_model(&mut self, DistanceModel) -> AltoResult<()>;

	/// `alSourcei(AL_DIRECT_FILTER)`
	/// Requires `ALC_EXT_EFX`
	fn set_direct_filter<F: FilterTrait<'d, 'c>>(&mut self, value: &F) -> AltoResult<()>;
	/// `alSourcei(AL_DIRECT_FILTER)`
	/// Requires `ALC_EXT_EFX`
	fn clear_direct_filter(&mut self) -> AltoResult<()>;

	/// `alSourceiv(AL_AUXILIARY_SEND_FILTER)`
	/// Requires `ALC_EXT_EFX`
	fn set_auxiliary_send(&mut self, send: sys::ALint, value: &mut AuxEffectSlot<'d, 'c>) -> AltoResult<()>;
	/// `alSourceiv(AL_AUXILIARY_SEND_FILTER)`
	/// Requires `ALC_EXT_EFX`
	fn set_auxiliary_send_filter<F: FilterTrait<'d, 'c>>(&mut self, send: sys::ALint, slot: &mut AuxEffectSlot<'d, 'c>, filter: &F) -> AltoResult<()>;
	/// `alSourceiv(AL_AUXILIARY_SEND_FILTER)`
	/// Requires `ALC_EXT_EFX`
	fn clear_auxiliary_send(&mut self, send: sys::ALint) -> AltoResult<()>;

	/// `alGetSourcef(AL_AIR_ABSORPTION_FACTOR)`
	/// Requires `ALC_EXT_EFX`
	fn air_absorption_factor(&self) -> AltoResult<f32>;
	/// `alSourcef(AL_AIR_ABSORPTION_FACTOR)`
	/// Requires `ALC_EXT_EFX`
	fn set_air_absorption_factor(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_ROOM_ROLLOFF_FACTOR)`
	/// Requires `ALC_EXT_EFX`
	fn room_rolloff_factor(&self) -> AltoResult<f32>;
	/// `alSourcef(AL_ROOM_ROLLOFF_FACTOR)`
	/// Requires `ALC_EXT_EFX`
	fn set_room_rolloff_factor(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_CONE_OUTER_GAINHF)`
	/// Requires `ALC_EXT_EFX`
	fn cone_outer_gainhf(&self) -> AltoResult<f32>;
	/// `alSourcef(AL_CONE_OUTER_GAINHF)`
	/// Requires `ALC_EXT_EFX`
	fn set_cone_outer_gainhf(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcei(AL_DIRECT_FILTER_GAINHF_AUTO)`
	/// Requires `ALC_EXT_EFX`
	fn direct_filter_gainhf_auto(&self) -> AltoResult<bool>;
	/// `alSourcei(AL_DIRECT_FILTER_GAINHF_AUTO)`
	/// Requires `ALC_EXT_EFX`
	fn set_direct_filter_gainhf_auto(&mut self, bool) -> AltoResult<()>;
}


/// The current playback state of a source.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SourceState {
	/// `AL_INITIAL`
	Initial,
	/// `AL_PLAYING`
	Playing,
	/// `AL_PAUSED`
	Paused,
	/// `AL_STOPPED`
	Stopped,
}


#[doc(hidden)]
pub struct SourceImpl<'d: 'c, 'c> {
	ctx: &'c Context<'d>,
	src: sys::ALuint,
	sends: Mutex<Vec<sys::ALuint>>,
}


/// A source that can play a shared static buffer.
pub struct StaticSource<'d: 'c, 'c> {
	src: Arc<SourceImpl<'d, 'c>>,
	buf: Option<Arc<Buffer<'d, 'c>>>,
}


/// A source that plays a queue of owned buffers.
pub struct StreamingSource<'d: 'c, 'c> {
	src: Arc<SourceImpl<'d, 'c>>,
	bufs: VecDeque<Buffer<'d, 'c>>,
}


impl<'d> Context<'d> {
	#[doc(hidden)]
	pub unsafe fn new(dev: &'d DeviceTrait, api: &'d AlApi<'static>, ctx: *mut sys::ALCcontext) -> Context<'d> {
		Context{
			dev: dev,
			api: api,
			ctx: ctx,
			exts: ext::AlCache::new(api.owner()),
			defer_rc: Arc::new(AtomicUsize::new(0)),
		}
	}


	/// The device from which this context was created.
	pub fn device(&self) -> &(DeviceTrait + 'd) { self.dev }
	/// Raw context pointer as provided by OpenAL.
	pub fn as_raw(&self) -> *mut sys::ALCcontext { self.ctx }


	/// `alIsExtensionPresent()`
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


	#[doc(hidden)]
	pub fn make_current(&self, set: bool) -> AltoResult<Option<MutexGuard<()>>> {
		self.api.rent(|exts| {
			if let Ok(tlc) = exts.ALC_EXT_thread_local_context() {
				unsafe { tlc.alcSetThreadContext?(if set { self.ctx } else { ptr::null_mut() }); }
				self.dev.alto().get_error(self.dev.as_raw()).map(|_| None)
			} else {
				unsafe { self.api.owner().alcMakeContextCurrent()(if set { self.ctx } else { ptr::null_mut() }); }
				self.dev.alto().get_error(self.dev.as_raw()).map(|_| Some(ALTO_CTX_LOCK__.lock().unwrap()))
			}
		})
	}


	/// `alGetInteger(AL_DISTANCE_MODEL)`
	pub fn distance_model(&self) -> AltoResult<DistanceModel> {
		let _lock = self.make_current(true)?;
		let value = unsafe { self.api.owner().alGetInteger()(sys::AL_DISTANCE_MODEL) };
		self.get_error().and_then(|_| match value {
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
	/// `alDistanceModel()`
	pub fn set_distance_model(&self, value: DistanceModel) -> AltoResult<()> {
		let _lock = self.make_current(true)?;
		unsafe {
			self.api.owner().alDistanceModel()(match value {
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


	/// `alIsEnabled(AL_SOURCE_DISTANCE_MODEL)`
	/// Requires `AL_EXT_source_distance_model`
	pub fn using_source_distance_model(&self) -> AltoResult<bool> {
		let _lock = self.make_current(true)?;
		let value = unsafe { self.api.owner().alIsEnabled()(self.exts.AL_EXT_source_distance_model()?.AL_SOURCE_DISTANCE_MODEL?) };
		self.get_error().map(|_| value == sys::AL_TRUE)
	}
	/// `alEnable/alDisable(AL_SOURCE_DISTANCE_MODEL)`
	/// Requires `AL_EXT_source_distance_model`
	pub fn use_source_distance_model(&self, value: bool) -> AltoResult<()> {
		let _lock = self.make_current(true)?;
		if value {
			unsafe { self.api.owner().alEnable()(self.exts.AL_EXT_source_distance_model()?.AL_SOURCE_DISTANCE_MODEL?); }
		} else {
			unsafe { self.api.owner().alDisable()(self.exts.AL_EXT_source_distance_model()?.AL_SOURCE_DISTANCE_MODEL?); }
		}
		self.get_error()
	}


	/// `alGetFloat(AL_DOPPLER_FACTOR)`
	pub fn doppler_factor(&self) -> AltoResult<f32> {
		let _lock = self.make_current(true)?;
		let value = unsafe { self.api.owner().alGetFloat()(sys::AL_DOPPLER_FACTOR) };
		self.get_error().map(|_| value)
	}
	/// `alDopplerFactor()`
	pub fn set_doppler_factor(&self, value: f32) -> AltoResult<()> {
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alDopplerFactor()(value); }
		self.get_error()
	}


	/// `alGetFloat(AL_SPEED_OF_SOUND)`
	pub fn speed_of_sound(&self) -> AltoResult<f32> {
		let _lock = self.make_current(true)?;
		let value = unsafe { self.api.owner().alGetFloat()(sys::AL_SPEED_OF_SOUND) };
		self.get_error().map(|_| value)
	}
	/// `alSpeedOfSound()`
	pub fn set_speed_of_sound(&self, value: f32) -> AltoResult<()> {
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alSpeedOfSound()(value); }
		self.get_error()
	}


	/// `alGetListenerv(AL_GAIN)`
	pub fn gain(&self) -> AltoResult<f32> {
		let _lock = self.make_current(true)?;
		let mut value = 0.0;
		unsafe { self.api.owner().alGetListenerf()(sys::AL_GAIN, &mut value); }
		self.get_error().map(|_| value)
	}
	/// `alListenerf(AL_GAIN)`
	pub fn set_gain(&self, value: f32) -> AltoResult<()> {
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alListenerf()(sys::AL_GAIN, value); }
		self.get_error()
	}


	/// `alGetListenerfv(AL_POSITION)`
	pub fn position<V: From<[f32; 3]>>(&self) -> AltoResult<V> {
		let _lock = self.make_current(true)?;
		let mut value = [0.0, 0.0, 0.0];
		unsafe { self.api.owner().alGetListenerfv()(sys::AL_POSITION, &mut value as *mut [f32; 3] as *mut sys::ALfloat); }
		self.get_error().map(|_| value.into())
	}
	/// `alListenerfv(AL_POSITION)`
	pub fn set_position<V: Into<[f32; 3]>>(&self, value: V) -> AltoResult<()> {
		let _lock = self.make_current(true)?;
		let value = value.into();
		unsafe { self.api.owner().alListenerfv()(sys::AL_POSITION, &value as *const [f32; 3] as *const sys::ALfloat); }
		self.get_error()
	}


	/// `alGetListenerfv(AL_VELOCITY)`
	pub fn velocity<V: From<[f32; 3]>>(&self) -> AltoResult<V> {
		let _lock = self.make_current(true)?;
		let mut value = [0.0, 0.0, 0.0];
		unsafe { self.api.owner().alGetListenerfv()(sys::AL_VELOCITY, &mut value as *mut [f32; 3] as *mut sys::ALfloat); }
		self.get_error().map(|_| value.into())
	}
	/// `alListenerfv(AL_VELOCITY)`
	pub fn set_velocity<V: Into<[f32; 3]>>(&self, value: V) -> AltoResult<()> {
		let _lock = self.make_current(true)?;
		let value = value.into();
		unsafe { self.api.owner().alListenerfv()(sys::AL_VELOCITY, &value as *const [f32; 3] as *const sys::ALfloat); }
		self.get_error()
	}


	/// `alGetListenerfv(AL_ORIENTATION)`
	pub fn orientation<V: From<[f32; 3]>>(&self) -> AltoResult<(V, V)> {
		let _lock = self.make_current(true)?;
		let mut value = [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
		unsafe { self.api.owner().alGetListenerfv()(sys::AL_ORIENTATION, &mut value as *mut [[f32; 3]; 2] as *mut sys::ALfloat); }
		self.get_error().map(|_| (value[0].into(), value[1].into()))
	}
	/// `alListenerfv(AL_ORIENTATION)`
	pub fn set_orientation<V: Into<[f32; 3]>>(&self, value: (V, V)) -> AltoResult<()> {
		let _lock = self.make_current(true)?;
		let value = [value.0.into(), value.1.into()];
		unsafe { self.api.owner().alListenerfv()(sys::AL_ORIENTATION, &value as *const [[f32; 3]; 2] as *const sys::ALfloat); }
		self.get_error()
	}


	/// `alGetListenerf(AL_METERS_PER_UNIT)`
	/// Requires `ALC_EXT_EFX`
	pub fn meters_per_unit(&self) -> AltoResult<f32> {
		let efx = self.dev.extensions().ALC_EXT_EFX()?;
		let _lock = self.make_current(true)?;
		let mut value = 0.0;
		unsafe { self.api.owner().alGetListenerf()(efx.AL_METERS_PER_UNIT?, &mut value); }
		self.get_error().map(|_| value)
	}
	/// `alListenerf(AL_METERS_PER_UNIT)`
	/// Requires `ALC_EXT_EFX`
	pub fn set_meters_per_unit(&self, value: f32) -> AltoResult<()> {
		let efx = self.dev.extensions().ALC_EXT_EFX()?;
		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alListenerf()(efx.AL_METERS_PER_UNIT?, value); }
		self.get_error()
	}


	/// `alGenBuffers()`
	pub fn new_buffer<'c>(&'c self) -> AltoResult<Buffer<'d, 'c>> {
		Buffer::new(self)
	}


	/// `alGenSources()`
	pub fn new_static_source(&self) -> AltoResult<StaticSource> {
		StaticSource::new(self)
	}


	/// `alGenSources()`
	pub fn new_streaming_source(&self) -> AltoResult<StreamingSource> {
		StreamingSource::new(self)
	}


	/// `alSourcePlayv()`
	pub fn play_all<'c, S, I>(&self, srcs: I) -> AltoResult<()> where
		'd: 'c,
		S: SourceTrait<'d, 'c>,
		I: Iterator,
		<I as Iterator>::Item: AsRef<S> + AsMut<S>,
	{
		let v: Vec<_> = srcs.filter(|s| s.as_ref().context() == self).map(|s| s.as_ref().as_raw()).collect();
		if v.len() > sys::ALint::max_value() as usize { return Err(AltoError::AlInvalidValue) }

		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alSourcePlayv()(v.len() as i32, v.as_slice().as_ptr()); }
		self.get_error()
	}


	/// `alSourcePausev()`
	pub fn pause_all<'c, S, I>(&self, srcs: I) -> AltoResult<()> where
		'd: 'c,
		S: SourceTrait<'d, 'c>,
		I: Iterator,
		<I as Iterator>::Item: AsRef<S> + AsMut<S>,
	{
		let v: Vec<_> = srcs.filter(|s| s.as_ref().context() == self).map(|s| s.as_ref().as_raw()).collect();
		if v.len() > sys::ALint::max_value() as usize { return Err(AltoError::AlInvalidValue) }

		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alSourcePausev()(v.len() as i32, v.as_slice().as_ptr()); }
		self.get_error()
	}


	/// `alSourceStopv()`
	pub fn stop_all<'c, S, I>(&self, srcs: I) -> AltoResult<()> where
		'd: 'c,
		S: SourceTrait<'d, 'c>,
		I: Iterator,
		<I as Iterator>::Item: AsRef<S> + AsMut<S>,
	{
		let v: Vec<_> = srcs.filter(|s| s.as_ref().context() == self).map(|s| s.as_ref().as_raw()).collect();
		if v.len() > sys::ALint::max_value() as usize { return Err(AltoError::AlInvalidValue) }

		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alSourceStopv()(v.len() as i32, v.as_slice().as_ptr()); }
		self.get_error()
	}


	/// `alSourceRewindv()`
	pub fn rewind_all<'c, S, I>(&self, srcs: I) -> AltoResult<()> where
		'd: 'c,
		S: SourceTrait<'d, 'c>,
		I: Iterator,
		<I as Iterator>::Item: AsRef<S> + AsMut<S>,
	{
		let v: Vec<_> = srcs.filter(|s| s.as_ref().context() == self).map(|s| s.as_ref().as_raw()).collect();
		if v.len() > sys::ALint::max_value() as usize { return Err(AltoError::AlInvalidValue) }

		let _lock = self.make_current(true)?;
		unsafe { self.api.owner().alSourceRewindv()(v.len() as i32, v.as_slice().as_ptr()); }
		self.get_error()
	}


	/// `alcSuspendContext()`
	/// or `alDeferUpdatesSOFT()` if `AL_SOFT_deferred_updates` is available.
	pub fn suspend<'c>(&'c self) -> AltoResult<SuspendLock<'d, 'c>> {
		SuspendLock::new(self)
	}


	/// `alGenAuxiliaryEffectSlots()`
	/// Requires `ALC_EXT_EFX`
	pub fn new_aux_effect_slot<'c>(&'c self) -> AltoResult<AuxEffectSlot<'d, 'c>> {
		AuxEffectSlot::new(self)
	}


	/// `alGenEffects()`
	/// Requires `ALC_EXT_EFX`
	pub fn new_effect<'c, E: EffectTrait<'d, 'c>>(&'c self) -> AltoResult<E> {
		E::new(self)
	}


	/// `alGenFilters()`
	/// Requires `ALC_EXT_EFX`
	pub fn new_filter<'c, F: FilterTrait<'d, 'c>>(&'c self) -> AltoResult<F> {
		F::new(self)
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
			if let Err(_) = self.dev.alto().get_error(self.dev.as_raw()) {
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
					if let Err(e) = ctx.dev.alto().get_error(ctx.dev.as_raw()) {
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
					if let Err(_) = self.0.dev.alto().get_error(self.0.dev.as_raw()) {
						let _ = writeln!(io::stderr(), "ALTO ERROR: `alcProcessContext` failed in SuspendLock drop");
					}
				},
			}
		}
	}
}


impl<'d: 'c, 'c> Buffer<'d, 'c> {
	#[doc(hidden)]
	pub fn new(ctx: &'c Context<'d>) -> AltoResult<Buffer<'d, 'c>> {
		let _lock = ctx.make_current(true)?;
		let mut buf = 0;
		unsafe { ctx.api.owner().alGenBuffers()(1, &mut buf as *mut sys::ALuint); }
		ctx.get_error().map(|_| Buffer{ctx: ctx, buf: buf})
	}


	/// Context from which this buffer was created.
	pub fn context(&self) -> &Context<'d> { self.ctx }
	/// Raw handle as provided by OpenAL.
	pub fn as_raw(&self) -> sys::ALuint { self.buf }


	/// `alBufferData()`
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


	/// `alGetBufferi(AL_FREQUENCY)`
	pub fn frequency(&self) -> AltoResult<sys::ALint> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { self.ctx.api.owner().alGetBufferi()(self.buf, sys::AL_FREQUENCY, &mut value); }
		self.ctx.get_error().map(|_| value)
	}


	/// `alGetBufferi(AL_BITS)`
	pub fn bits(&self) -> AltoResult<sys::ALint> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { self.ctx.api.owner().alGetBufferi()(self.buf, sys::AL_BITS, &mut value); }
		self.ctx.get_error().map(|_| value)
	}


	/// `alGetBufferi(AL_CHANNELS)`
	pub fn channels(&self) -> AltoResult<sys::ALint> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { self.ctx.api.owner().alGetBufferi()(self.buf, sys::AL_CHANNELS, &mut value); }
		self.ctx.get_error().map(|_| value)
	}


	/// `alGetBufferi(AL_SIZE)`
	pub fn size(&self) -> AltoResult<sys::ALint> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { self.ctx.api.owner().alGetBufferi()(self.buf, sys::AL_SIZE, &mut value); }
		self.ctx.get_error().map(|_| value)
	}


	/// `alGetBufferiv(AL_LOOP_POINTS_SOFT)`
	/// Requires `AL_SOFT_loop_points`
	pub fn soft_loop_points(&self) -> AltoResult<(sys::ALint, sys::ALint)> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = [0, 0];
		unsafe { self.ctx.api.owner().alGetBufferiv()(self.buf, self.ctx.exts.AL_SOFT_loop_points()?.AL_LOOP_POINTS_SOFT?, &mut value as *mut [sys::ALint; 2] as *mut sys::ALint); }
		self.ctx.get_error().map(|_| (value[0], value[1]))
	}
	/// `alBufferiv(AL_LOOP_POINTS_SOFT)`
	/// Requires `AL_SOFT_loop_points`
	pub fn set_soft_loop_points(&self, value: (sys::ALint, sys::ALint)) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alBufferiv()(self.buf, self.ctx.exts.AL_SOFT_loop_points()?.AL_LOOP_POINTS_SOFT?, &[value.0, value.1] as *const [sys::ALint; 2] as *const sys::ALint); }
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


impl<'d: 'c, 'c> SourceImpl<'d, 'c> {
	fn context(&self) -> &Context<'d> { self.ctx }
	pub fn as_raw(&self) -> sys::ALuint { self.src }


	fn state(&self) -> AltoResult<SourceState> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_SOURCE_STATE, &mut value); }
		self.ctx.get_error().and_then(|_| match value {
			sys::AL_INITIAL => Ok(SourceState::Initial),
			sys::AL_PLAYING => Ok(SourceState::Playing),
			sys::AL_PAUSED => Ok(SourceState::Paused),
			sys::AL_STOPPED => Ok(SourceState::Stopped),
			_ => Err(AltoError::AlInvalidEnum),
		})
	}
	fn play(&self) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcePlay()(self.src); }
		self.ctx.get_error()
	}
	fn pause(&self) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcePause()(self.src); }
		self.ctx.get_error()
	}
	fn stop(&self) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourceStop()(self.src); }
		self.ctx.get_error()
	}
	fn rewind(&self) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourceRewind()(self.src); }
		self.ctx.get_error()
	}


	fn relative(&self) -> AltoResult<bool> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_SOURCE_RELATIVE, &mut value); }
		self.ctx.get_error().map(|_| value == sys::AL_TRUE as sys::ALint)
	}
	fn set_relative(&self, value: bool) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, sys::AL_SOURCE_RELATIVE, if value { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.ctx.get_error()
	}


	fn gain(&self) -> AltoResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_GAIN, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	fn set_gain(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_GAIN, value); }
		self.ctx.get_error()
	}


	fn min_gain(&self) -> AltoResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_MIN_GAIN, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	fn set_min_gain(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_MIN_GAIN, value); }
		self.ctx.get_error()
	}


	fn max_gain(&self) -> AltoResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_MAX_GAIN, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	fn set_max_gain(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_MAX_GAIN, value); }
		self.ctx.get_error()
	}


	fn reference_distance(&self) -> AltoResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_REFERENCE_DISTANCE, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	fn set_reference_distance(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_REFERENCE_DISTANCE, value); }
		self.ctx.get_error()
	}


	fn rolloff_factor(&self) -> AltoResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_ROLLOFF_FACTOR, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	fn set_rolloff_factor(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_ROLLOFF_FACTOR, value); }
		self.ctx.get_error()
	}


	fn max_distance(&self) -> AltoResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_MAX_DISTANCE, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	fn set_max_distance(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_MAX_DISTANCE, value); }
		self.ctx.get_error()
	}


	fn pitch(&self) -> AltoResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_PITCH, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	fn set_pitch(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_PITCH, value); }
		self.ctx.get_error()
	}


	fn position<V: From<[f32; 3]>>(&self) -> AltoResult<V> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = [0.0, 0.0, 0.0];
		unsafe { self.ctx.api.owner().alGetSourcefv()(self.src, sys::AL_POSITION, &mut value as *mut [f32; 3] as *mut sys::ALfloat); }
		self.ctx.get_error().map(|_| value.into())
	}
	fn set_position<V: Into<[f32; 3]>>(&self, value: V) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		let value = value.into();
		unsafe { self.ctx.api.owner().alSourcefv()(self.src, sys::AL_POSITION, &value as *const [f32; 3] as *const sys::ALfloat); }
		self.ctx.get_error()
	}


	fn velocity<V: From<[f32; 3]>>(&self) -> AltoResult<V> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = [0.0, 0.0, 0.0];
		unsafe { self.ctx.api.owner().alGetSourcefv()(self.src, sys::AL_VELOCITY, &mut value as *mut [f32; 3] as *mut sys::ALfloat); }
		self.ctx.get_error().map(|_| value.into())
	}
	fn set_velocity<V: Into<[f32; 3]>>(&self, value: V) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		let value = value.into();
		unsafe { self.ctx.api.owner().alSourcefv()(self.src, sys::AL_VELOCITY, &value as *const [f32; 3] as *const sys::ALfloat); }
		self.ctx.get_error()
	}


	fn direction<V: From<[f32; 3]>>(&self) -> AltoResult<V> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = [0.0, 0.0, 0.0];
		unsafe { self.ctx.api.owner().alGetSourcefv()(self.src, sys::AL_DIRECTION, &mut value as *mut [f32; 3] as *mut sys::ALfloat); }
		self.ctx.get_error().map(|_| value.into())
	}
	fn set_direction<V: Into<[f32; 3]>>(&self, value: V) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		let value = value.into();
		unsafe { self.ctx.api.owner().alSourcefv()(self.src, sys::AL_DIRECTION, &value as *const [f32; 3] as *const sys::ALfloat); }
		self.ctx.get_error()
	}


	fn cone_inner_angle(&self) -> AltoResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_CONE_INNER_ANGLE, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	fn set_cone_inner_angle(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_CONE_INNER_ANGLE, value); }
		self.ctx.get_error()
	}


	fn cone_outer_angle(&self) -> AltoResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_CONE_OUTER_ANGLE, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	fn set_cone_outer_angle(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_CONE_OUTER_ANGLE, value); }
		self.ctx.get_error()
	}


	fn cone_outer_gain(&self) -> AltoResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_CONE_OUTER_GAIN, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	fn set_cone_outer_gain(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_CONE_OUTER_GAIN, value); }
		self.ctx.get_error()
	}


	fn sec_offset(&self) -> AltoResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, sys::AL_SEC_OFFSET, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	fn set_sec_offset(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, sys::AL_SEC_OFFSET, value); }
		self.ctx.get_error()
	}


	fn sample_offset(&self) -> AltoResult<sys::ALint> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_SAMPLE_OFFSET, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	fn set_sample_offset(&self, value: sys::ALint) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, sys::AL_SAMPLE_OFFSET, value); }
		self.ctx.get_error()
	}


	fn byte_offset(&self) -> AltoResult<sys::ALint> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_BYTE_OFFSET, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	fn set_byte_offset(&self, value: sys::ALint) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, sys::AL_BYTE_OFFSET, value); }
		self.ctx.get_error()
	}


	fn soft_sec_offset_latency(&self) -> AltoResult<(f64, f64)> {
		let assl = self.ctx.exts.AL_SOFT_source_latency()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = [0.0, 0.0];
		unsafe { assl.alGetSourcedvSOFT?(self.src, assl.AL_SEC_OFFSET_LATENCY_SOFT?, &mut value as *mut [f64; 2] as *mut f64); }
		self.ctx.get_error().map(|_| (value[0], value[1]))
	}


	fn soft_sample_offset_frac_latency(&self) -> AltoResult<(i32, i32, i64)> {
		let assl = self.ctx.exts.AL_SOFT_source_latency()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = [0, 0];
		unsafe { assl.alGetSourcei64vSOFT?(self.src, assl.AL_SAMPLE_OFFSET_LATENCY_SOFT?, &mut value as *mut [i64; 2] as *mut i64); }
		self.ctx.get_error().map(|_| ((value[0] >> 32) as i32, value[0] as i32, value[1]))
	}


	fn soft_direct_channels(&self) -> AltoResult<bool> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, self.ctx.exts.AL_SOFT_direct_channels()?.AL_DIRECT_CHANNELS_SOFT?, &mut value); }
		self.ctx.get_error().map(|_| value == sys::AL_TRUE as sys::ALint)
	}
	fn set_soft_direct_channels(&self, value: bool) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, self.ctx.exts.AL_SOFT_direct_channels()?.AL_DIRECT_CHANNELS_SOFT?, if value { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.ctx.get_error()
	}


	fn soft_sec_length(&self) -> AltoResult<f32> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, self.ctx.exts.AL_SOFT_source_length()?.AL_SEC_LENGTH_SOFT?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}


	fn soft_sample_length(&self) -> AltoResult<sys::ALint> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, self.ctx.exts.AL_SOFT_source_length()?.AL_SAMPLE_LENGTH_SOFT?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}


	fn soft_byte_length(&self) -> AltoResult<sys::ALint> {
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, self.ctx.exts.AL_SOFT_source_length()?.AL_BYTE_LENGTH_SOFT?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}


	fn distance_model(&self) -> AltoResult<DistanceModel> {
		self.ctx.exts.AL_EXT_source_distance_model()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, sys::AL_DISTANCE_MODEL, &mut value); }
		self.ctx.get_error().and_then(|_| match value {
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
	fn set_distance_model(&self, value: DistanceModel) -> AltoResult<()> {
		self.ctx.exts.AL_EXT_source_distance_model()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe {
			self.ctx.api.owner().alSourcei()(self.src, sys::AL_DISTANCE_MODEL, match value {
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


	fn set_direct_filter<F: FilterTrait<'d, 'c>>(&self, value: &F) -> AltoResult<()> {
		let efx = self.ctx.dev.extensions().ALC_EXT_EFX()?;
		if value.context() != self.ctx {
			return Err(AltoError::AlInvalidValue);
		}

		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, efx.AL_DIRECT_FILTER?, value.as_raw() as sys::ALint); }
		self.ctx.get_error()
	}
	fn clear_direct_filter(&self) -> AltoResult<()> {
		let efx = self.ctx.dev.extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, efx.AL_DIRECT_FILTER?, 0); }
		self.ctx.get_error()
	}


	fn set_auxiliary_send(arc_self: &Arc<SourceImpl<'d, 'c>>, send: sys::ALint, slot: &mut AuxEffectSlot<'d, 'c>) -> AltoResult<()> {
		SourceImpl::set_auxiliary_send_impl(arc_self, send, slot, 0)
	}
	fn set_auxiliary_send_filter<F: FilterTrait<'d, 'c>>(arc_self: &Arc<SourceImpl<'d, 'c>>, send: sys::ALint, slot: &mut AuxEffectSlot<'d, 'c>, filter: &F) -> AltoResult<()> {
		if filter.context() != arc_self.ctx {
			return Err(AltoError::AlInvalidValue);
		}

		SourceImpl::set_auxiliary_send_impl(arc_self, send, slot, filter.as_raw())
	}
	fn set_auxiliary_send_impl(arc_self: &Arc<SourceImpl<'d, 'c>>, send: sys::ALint, slot: &mut AuxEffectSlot<'d, 'c>, filter: sys::ALuint) -> AltoResult<()> {
		let efx = arc_self.ctx.dev.extensions().ALC_EXT_EFX()?;
		if send >= arc_self.ctx.device().max_auxiliary_sends()? || slot.context() != arc_self.ctx {
			return Err(AltoError::AlInvalidValue);
		}

		let _lock = arc_self.ctx.make_current(true)?;
		let mut sends = arc_self.sends.lock().unwrap();
		unsafe { arc_self.ctx.api.owner().alSourceiv()(arc_self.src, efx.AL_AUXILIARY_SEND_FILTER?, &mut [slot.as_raw() as sys::ALint, send, filter as sys::ALint] as *mut [sys::ALint; 3] as *mut sys::ALint); }
		arc_self.ctx.get_error()?;
		sends[send as usize] = 0;
		slot.add_input(Arc::downgrade(arc_self));
		Ok(())
	}
	fn clear_auxiliary_send(&self, send: sys::ALint) -> AltoResult<()> {
		let efx = self.ctx.dev.extensions().ALC_EXT_EFX()?;
		if send >= self.ctx.device().max_auxiliary_sends()? {
			return Err(AltoError::AlInvalidValue);
		}

		let _lock = self.ctx.make_current(true)?;
		let mut sends = self.sends.lock().unwrap();
		unsafe { self.ctx.api.owner().alSourceiv()(self.src, efx.AL_AUXILIARY_SEND_FILTER?, &mut [0, send, 0] as *mut [sys::ALint; 3] as *mut sys::ALint); }
		self.ctx.get_error()?;
		sends[send as usize] = 0;
		Ok(())
	}
	pub fn clear_auxiliary_effect_slot(&self, slot: sys::ALuint) -> AltoResult<()> {
		let efx = self.ctx.dev.extensions().ALC_EXT_EFX()?;
		for (i, s) in self.sends.lock().unwrap().iter_mut().enumerate() {
			if *s == slot {
				unsafe { self.ctx.api.owner().alSourceiv()(self.src, efx.AL_AUXILIARY_SEND_FILTER.unwrap(), &mut [0, i as sys::ALint, 0] as *mut [sys::ALint; 3] as *mut sys::ALint); }
				*s = 0;
			}
		}

		self.ctx.get_error()
	}


	fn air_absorption_factor(&self) -> AltoResult<f32> {
		let efx = self.ctx.dev.extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, efx.AL_AIR_ABSORPTION_FACTOR?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	fn set_air_absorption_factor(&self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.dev.extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, efx.AL_AIR_ABSORPTION_FACTOR?, value); }
		self.ctx.get_error()
	}


	fn room_rolloff_factor(&self) -> AltoResult<f32> {
		let efx = self.ctx.dev.extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, efx.AL_ROOM_ROLLOFF_FACTOR?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	fn set_room_rolloff_factor(&self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.dev.extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, efx.AL_ROOM_ROLLOFF_FACTOR?, value); }
		self.ctx.get_error()
	}


	fn cone_outer_gainhf(&self) -> AltoResult<f32> {
		let efx = self.ctx.dev.extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { self.ctx.api.owner().alGetSourcef()(self.src, efx.AL_CONE_OUTER_GAINHF?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	fn set_cone_outer_gainhf(&self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.dev.extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcef()(self.src, efx.AL_CONE_OUTER_GAINHF?, value); }
		self.ctx.get_error()
	}


	fn direct_filter_gainhf_auto(&self) -> AltoResult<bool> {
		let efx = self.ctx.dev.extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { self.ctx.api.owner().alGetSourcei()(self.src, efx.AL_CONE_OUTER_GAINHF?, &mut value); }
		self.ctx.get_error().map(|_| value == sys::AL_TRUE as sys::ALint)
	}
	fn set_direct_filter_gainhf_auto(&self, value: bool) -> AltoResult<()> {
		let efx = self.ctx.dev.extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { self.ctx.api.owner().alSourcei()(self.src, efx.AL_CONE_OUTER_GAINHF?, if value { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.ctx.get_error()
	}
}


impl<'d: 'c, 'c> PartialEq for SourceImpl<'d, 'c> {
	fn eq(&self, other: &SourceImpl<'d, 'c>) -> bool {
		self.ctx == other.ctx && self.src == other.src
	}
}
impl<'d: 'c, 'c> Eq for SourceImpl<'d, 'c> { }


impl<'d: 'c, 'c> Hash for SourceImpl<'d, 'c> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
		self.ctx.as_raw().hash(state);
		self.src.hash(state);
	}
}


impl<'d: 'c, 'c> Drop for SourceImpl<'d, 'c> {
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
	#[doc(hidden)]
	pub fn new(ctx: &'c Context<'d>) -> AltoResult<StaticSource<'d, 'c>> {
		let _lock = ctx.make_current(true)?;
		let mut src = 0;
		unsafe { ctx.api.owner().alGenSources()(1, &mut src as *mut sys::ALuint); }
		let sends = iter::repeat(0).take(ctx.dev.max_auxiliary_sends().unwrap_or(0) as usize).collect();
		ctx.get_error().map(|_| StaticSource{src: Arc::new(SourceImpl{ctx: ctx, src: src, sends: Mutex::new(sends)}), buf: None})
	}


	pub fn buffer(&self) -> Option<&Arc<Buffer<'d, 'c>>> { self.buf.as_ref() }


	/// `alSourcei(AL_BUFFER)`
	pub fn set_buffer(&mut self, buf: Arc<Buffer<'d, 'c>>) -> AltoResult<()> {
		if buf.ctx.device().as_raw() != self.src.ctx.device().as_raw() {
			return Err(AltoError::AlInvalidValue);
		}

		{
			let _lock = self.src.ctx.make_current(true)?;
			unsafe { self.src.ctx.api.owner().alSourcei()(self.src.src, sys::AL_BUFFER, buf.buf as sys::ALint); }
			self.src.ctx.get_error()?;
		}

		self.buf = Some(buf);
		Ok(())
	}
	/// `alSourcei(AL_BUFFER)`
	pub fn clear_buffer(&mut self) -> AltoResult<()> {
		{
			let _lock = self.src.ctx.make_current(true)?;
			unsafe { self.src.ctx.api.owner().alSourcei()(self.src.src, sys::AL_BUFFER, 0); }
			self.src.ctx.get_error()?;
		}

		self.buf = None;
		Ok(())
	}


	/// `alGetSourcei(AL_LOOPING)`
	pub fn looping(&self) -> AltoResult<bool> {
		let _lock = self.src.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { self.src.ctx.api.owner().alGetSourcei()(self.src.src, sys::AL_LOOPING, &mut value); }
		self.src.ctx.get_error().map(|_| value == sys::AL_TRUE as sys::ALint)
	}
	/// `alSourcei(AL_LOOPING)`
	pub fn set_looping(&mut self, value: bool) -> AltoResult<()> {
		let _lock = self.src.ctx.make_current(true)?;
		unsafe { self.src.ctx.api.owner().alSourcei()(self.src.src, sys::AL_LOOPING, if value { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.src.ctx.get_error()
	}


}


unsafe impl<'d: 'c, 'c> SourceTrait<'d, 'c> for StaticSource<'d, 'c> {
	fn context(&self) -> &Context<'d> { self.src.context() }
	fn as_raw(&self) -> sys::ALuint { self.src.as_raw() }

	fn state(&self) -> AltoResult<SourceState> { self.src.state() }
	fn play(&mut self) -> AltoResult<()> { self.src.play() }
	fn pause(&mut self) -> AltoResult<()> { self.src.pause() }
	fn stop(&mut self) -> AltoResult<()> { self.src.stop() }
	fn rewind(&mut self) -> AltoResult<()> { self.src.rewind() }

	fn relative(&self) -> AltoResult<bool> { self.src.relative() }
	fn set_relative(&mut self, value: bool) -> AltoResult<()> { self.src.set_relative(value) }

	fn gain(&self) -> AltoResult<f32> { self.src.gain() }
	fn set_gain(&mut self, value: f32) -> AltoResult<()> { self.src.set_gain(value) }

	fn min_gain(&self) -> AltoResult<f32> { self.src.min_gain() }
	fn set_min_gain(&mut self, value: f32) -> AltoResult<()> { self.src.set_min_gain(value) }

	fn max_gain(&self) -> AltoResult<f32> { self.src.max_gain() }
	fn set_max_gain(&mut self, value: f32) -> AltoResult<()> { self.src.set_max_gain(value) }

	fn reference_distance(&self) -> AltoResult<f32> { self.src.reference_distance() }
	fn set_reference_distance(&mut self, value: f32) -> AltoResult<()> { self.src.set_reference_distance(value) }

	fn rolloff_factor(&self) -> AltoResult<f32> { self.src.rolloff_factor() }
	fn set_rolloff_factor(&mut self, value: f32) -> AltoResult<()> { self.src.set_rolloff_factor(value) }

	fn max_distance(&self) -> AltoResult<f32> { self.src.max_distance() }
	fn set_max_distance(&mut self, value: f32) -> AltoResult<()> { self.src.set_max_distance(value) }

	fn pitch(&self) -> AltoResult<f32> { self.src.pitch() }
	fn set_pitch(&mut self, value: f32) -> AltoResult<()> { self.src.set_pitch(value) }

	fn position<V: From<[f32; 3]>>(&self) -> AltoResult<V> { self.src.position() }
	fn set_position<V: Into<[f32; 3]>>(&mut self, value: V) -> AltoResult<()> { self.src.set_position(value) }

	fn velocity<V: From<[f32; 3]>>(&self) -> AltoResult<V> { self.src.velocity() }
	fn set_velocity<V: Into<[f32; 3]>>(&mut self, value: V) -> AltoResult<()> { self.src.set_velocity(value) }

	fn direction<V: From<[f32; 3]>>(&self) -> AltoResult<V> { self.src.direction() }
	fn set_direction<V: Into<[f32; 3]>>(&mut self, value: V) -> AltoResult<()> { self.src.set_direction(value) }

	fn cone_inner_angle(&self) -> AltoResult<f32> { self.src.cone_inner_angle() }
	fn set_cone_inner_angle(&mut self, value: f32) -> AltoResult<()> { self.src.set_cone_inner_angle(value) }

	fn cone_outer_angle(&self) -> AltoResult<f32> { self.src.cone_outer_angle() }
	fn set_cone_outer_angle(&mut self, value: f32) -> AltoResult<()> { self.src.set_cone_outer_angle(value) }

	fn cone_outer_gain(&self) -> AltoResult<f32> { self.src.cone_outer_gain() }
	fn set_cone_outer_gain(&mut self, value: f32) -> AltoResult<()> { self.src.set_cone_outer_gain(value) }

	fn sec_offset(&self) -> AltoResult<f32> { self.src.sec_offset() }
	fn set_sec_offset(&mut self, value: f32) -> AltoResult<()> { self.src.set_sec_offset(value) }

	fn sample_offset(&self) -> AltoResult<sys::ALint> { self.src.sample_offset() }
	fn set_sample_offset(&mut self, value: sys::ALint) -> AltoResult<()> { self.src.set_sample_offset(value) }

	fn byte_offset(&self) -> AltoResult<sys::ALint> { self.src.byte_offset() }
	fn set_byte_offset(&mut self, value: sys::ALint) -> AltoResult<()> { self.src.set_byte_offset(value) }

	fn soft_sec_offset_latency(&self) -> AltoResult<(f64, f64)> { self.src.soft_sec_offset_latency() }

	fn soft_sample_frac_offset_latency(&self) -> AltoResult<(i32, i32, i64)> { self.src.soft_sample_offset_frac_latency() }

	fn soft_sec_length(&self) -> AltoResult<f32> { self.src.soft_sec_length() }

	fn soft_sample_length(&self) -> AltoResult<sys::ALint> { self.src.soft_sample_length() }

	fn soft_byte_length(&self) -> AltoResult<sys::ALint> { self.src.soft_byte_length() }

	fn soft_direct_channels(&self) -> AltoResult<bool> { self.src.soft_direct_channels() }
	fn set_soft_direct_channels(&mut self, value: bool) -> AltoResult<()> { self.src.set_soft_direct_channels(value) }

	fn distance_model(&self) -> AltoResult<DistanceModel> { self.src.distance_model() }
	fn set_distance_model(&mut self, value: DistanceModel) -> AltoResult<()> { self.src.set_distance_model(value) }

	fn set_direct_filter<F: FilterTrait<'d, 'c>>(&mut self, value: &F) -> AltoResult<()> { self.src.set_direct_filter(value) }
	fn clear_direct_filter(&mut self) -> AltoResult<()> { self.src.clear_direct_filter() }

	fn set_auxiliary_send(&mut self, send: sys::ALint, slot: &mut AuxEffectSlot<'d, 'c>) -> AltoResult<()> { SourceImpl::set_auxiliary_send(&self.src, send, slot) }
	fn set_auxiliary_send_filter<F: FilterTrait<'d, 'c>>(&mut self, send: sys::ALint, slot: &mut AuxEffectSlot<'d, 'c>, filter: &F) -> AltoResult<()> { SourceImpl::set_auxiliary_send_filter(&self.src, send, slot, filter) }
	fn clear_auxiliary_send(&mut self, send: sys::ALint) -> AltoResult<()> { self.src.clear_auxiliary_send(send) }

	fn air_absorption_factor(&self) -> AltoResult<f32> { self.src.air_absorption_factor() }
	fn set_air_absorption_factor(&mut self, value: f32) -> AltoResult<()> { self.src.set_air_absorption_factor(value) }

	fn room_rolloff_factor(&self) -> AltoResult<f32> { self.src.room_rolloff_factor() }
	fn set_room_rolloff_factor(&mut self, value: f32) -> AltoResult<()> { self.src.set_room_rolloff_factor(value) }

	fn cone_outer_gainhf(&self) -> AltoResult<f32> { self.src.cone_outer_gainhf() }
	fn set_cone_outer_gainhf(&mut self, value: f32) -> AltoResult<()> { self.src.set_cone_outer_gainhf(value) }

	fn direct_filter_gainhf_auto(&self) -> AltoResult<bool> { self.src.direct_filter_gainhf_auto() }
	fn set_direct_filter_gainhf_auto(&mut self, value: bool) -> AltoResult<()> { self.src.set_direct_filter_gainhf_auto(value) }
}


impl<'d: 'c, 'c> PartialEq for StaticSource<'d, 'c> {
	fn eq(&self, other: &StaticSource<'d, 'c>) -> bool {
		self.src == other.src
	}
}
impl<'d: 'c, 'c> Eq for StaticSource<'d, 'c> { }


impl<'d: 'c, 'c> StreamingSource<'d, 'c> {
	#[doc(hidden)]
	pub fn new(ctx: &'c Context<'d>) -> AltoResult<StreamingSource<'d, 'c>> {
		let _lock = ctx.make_current(true)?;
		let mut src = 0;
		unsafe { ctx.api.owner().alGenSources()(1, &mut src as *mut sys::ALuint); }
		let sends = iter::repeat(0).take(ctx.dev.max_auxiliary_sends().unwrap_or(0) as usize).collect();
		ctx.get_error().map(|_| StreamingSource{src: Arc::new(SourceImpl{ctx: ctx, src: src, sends: Mutex::new(sends)}), bufs: VecDeque::new()})
	}


	/// `alGetSourcei(AL_BUFFERS_QUEUED)`
	pub fn buffers_queued(&self) -> AltoResult<sys::ALint> {
		Ok(self.bufs.len() as sys::ALint)
	}


	/// `alGetSourcei(AL_BUFFERS_PROCESSED)`
	pub fn buffers_processed(&self) -> AltoResult<sys::ALint> {
		let _lock = self.src.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { self.src.ctx.api.owner().alGetSourcei()(self.src.src, sys::AL_BUFFERS_PROCESSED, &mut value); }
		self.src.ctx.get_error().map(|_| value)
	}


	/// `alSourceQueueBuffers()`
	pub fn queue_buffer(&mut self, buf: Buffer<'d, 'c>) -> Result<(), (AltoError, Buffer<'d, 'c>)> {
		{
			if buf.ctx.device().as_raw() != self.src.ctx.device().as_raw() {
				return Err((AltoError::AlInvalidValue, buf));
			}
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


	/// `alSourceUnqueueBuffers()`
	pub fn unqueue_buffer(&mut self) -> AltoResult<Buffer<'d, 'c>> {
		{
			let _lock = self.src.ctx.make_current(true)?;
			let mut buf = 0;
			unsafe { self.src.ctx.api.owner().alSourceUnqueueBuffers()(self.src.src, 1, &mut buf); }
			self.src.ctx.get_error()?;
		}

		Ok(self.bufs.pop_front().unwrap())
	}
}


unsafe impl<'d: 'c, 'c> SourceTrait<'d, 'c> for StreamingSource<'d, 'c> {
	fn context(&self) -> &Context<'d> { self.src.context() }
	fn as_raw(&self) -> sys::ALuint { self.src.as_raw() }

	fn state(&self) -> AltoResult<SourceState> { self.src.state() }
	fn play(&mut self) -> AltoResult<()> { self.src.play() }
	fn pause(&mut self) -> AltoResult<()> { self.src.pause() }
	fn stop(&mut self) -> AltoResult<()> { self.src.stop() }
	fn rewind(&mut self) -> AltoResult<()> { self.src.rewind() }

	fn relative(&self) -> AltoResult<bool> { self.src.relative() }
	fn set_relative(&mut self, value: bool) -> AltoResult<()> { self.src.set_relative(value) }

	fn gain(&self) -> AltoResult<f32> { self.src.gain() }
	fn set_gain(&mut self, value: f32) -> AltoResult<()> { self.src.set_gain(value) }

	fn min_gain(&self) -> AltoResult<f32> { self.src.min_gain() }
	fn set_min_gain(&mut self, value: f32) -> AltoResult<()> { self.src.set_min_gain(value) }

	fn max_gain(&self) -> AltoResult<f32> { self.src.max_gain() }
	fn set_max_gain(&mut self, value: f32) -> AltoResult<()> { self.src.set_max_gain(value) }

	fn reference_distance(&self) -> AltoResult<f32> { self.src.reference_distance() }
	fn set_reference_distance(&mut self, value: f32) -> AltoResult<()> { self.src.set_reference_distance(value) }

	fn rolloff_factor(&self) -> AltoResult<f32> { self.src.rolloff_factor() }
	fn set_rolloff_factor(&mut self, value: f32) -> AltoResult<()> { self.src.set_rolloff_factor(value) }

	fn max_distance(&self) -> AltoResult<f32> { self.src.max_distance() }
	fn set_max_distance(&mut self, value: f32) -> AltoResult<()> { self.src.set_max_distance(value) }

	fn pitch(&self) -> AltoResult<f32> { self.src.pitch() }
	fn set_pitch(&mut self, value: f32) -> AltoResult<()> { self.src.set_pitch(value) }

	fn position<V: From<[f32; 3]>>(&self) -> AltoResult<V> { self.src.position() }
	fn set_position<V: Into<[f32; 3]>>(&mut self, value: V) -> AltoResult<()> { self.src.set_position(value) }

	fn velocity<V: From<[f32; 3]>>(&self) -> AltoResult<V> { self.src.velocity() }
	fn set_velocity<V: Into<[f32; 3]>>(&mut self, value: V) -> AltoResult<()> { self.src.set_velocity(value) }

	fn direction<V: From<[f32; 3]>>(&self) -> AltoResult<V> { self.src.direction() }
	fn set_direction<V: Into<[f32; 3]>>(&mut self, value: V) -> AltoResult<()> { self.src.set_direction(value) }

	fn cone_inner_angle(&self) -> AltoResult<f32> { self.src.cone_inner_angle() }
	fn set_cone_inner_angle(&mut self, value: f32) -> AltoResult<()> { self.src.set_cone_inner_angle(value) }

	fn cone_outer_angle(&self) -> AltoResult<f32> { self.src.cone_outer_angle() }
	fn set_cone_outer_angle(&mut self, value: f32) -> AltoResult<()> { self.src.set_cone_outer_angle(value) }

	fn cone_outer_gain(&self) -> AltoResult<f32> { self.src.cone_outer_gain() }
	fn set_cone_outer_gain(&mut self, value: f32) -> AltoResult<()> { self.src.set_cone_outer_gain(value) }

	fn sec_offset(&self) -> AltoResult<f32> { self.src.sec_offset() }
	fn set_sec_offset(&mut self, value: f32) -> AltoResult<()> { self.src.set_sec_offset(value) }

	fn sample_offset(&self) -> AltoResult<sys::ALint> { self.src.sample_offset() }
	fn set_sample_offset(&mut self, value: sys::ALint) -> AltoResult<()> { self.src.set_sample_offset(value) }

	fn byte_offset(&self) -> AltoResult<sys::ALint> { self.src.byte_offset() }
	fn set_byte_offset(&mut self, value: sys::ALint) -> AltoResult<()> { self.src.set_byte_offset(value) }

	fn soft_sec_offset_latency(&self) -> AltoResult<(f64, f64)> { self.src.soft_sec_offset_latency() }

	fn soft_sample_frac_offset_latency(&self) -> AltoResult<(i32, i32, i64)> { self.src.soft_sample_offset_frac_latency() }

	fn soft_sec_length(&self) -> AltoResult<f32> { self.src.soft_sec_length() }

	fn soft_sample_length(&self) -> AltoResult<sys::ALint> { self.src.soft_sample_length() }

	fn soft_byte_length(&self) -> AltoResult<sys::ALint> { self.src.soft_byte_length() }

	fn soft_direct_channels(&self) -> AltoResult<bool> { self.src.soft_direct_channels() }
	fn set_soft_direct_channels(&mut self, value: bool) -> AltoResult<()> { self.src.set_soft_direct_channels(value) }

	fn distance_model(&self) -> AltoResult<DistanceModel> { self.src.distance_model() }
	fn set_distance_model(&mut self, value: DistanceModel) -> AltoResult<()> { self.src.set_distance_model(value) }

	fn set_direct_filter<F: FilterTrait<'d, 'c>>(&mut self, value: &F) -> AltoResult<()> { self.src.set_direct_filter(value) }
	fn clear_direct_filter(&mut self) -> AltoResult<()> { self.src.clear_direct_filter() }

	fn set_auxiliary_send(&mut self, send: sys::ALint, slot: &mut AuxEffectSlot<'d, 'c>) -> AltoResult<()> { SourceImpl::set_auxiliary_send(&self.src, send, slot) }
	fn set_auxiliary_send_filter<F: FilterTrait<'d, 'c>>(&mut self, send: sys::ALint, slot: &mut AuxEffectSlot<'d, 'c>, filter: &F) -> AltoResult<()> { SourceImpl::set_auxiliary_send_filter(&self.src, send, slot, filter) }
	fn clear_auxiliary_send(&mut self, send: sys::ALint) -> AltoResult<()> { self.src.clear_auxiliary_send(send) }

	fn air_absorption_factor(&self) -> AltoResult<f32> { self.src.air_absorption_factor() }
	fn set_air_absorption_factor(&mut self, value: f32) -> AltoResult<()> { self.src.set_air_absorption_factor(value) }

	fn room_rolloff_factor(&self) -> AltoResult<f32> { self.src.room_rolloff_factor() }
	fn set_room_rolloff_factor(&mut self, value: f32) -> AltoResult<()> { self.src.set_room_rolloff_factor(value) }

	fn cone_outer_gainhf(&self) -> AltoResult<f32> { self.src.cone_outer_gainhf() }
	fn set_cone_outer_gainhf(&mut self, value: f32) -> AltoResult<()> { self.src.set_cone_outer_gainhf(value) }

	fn direct_filter_gainhf_auto(&self) -> AltoResult<bool> { self.src.direct_filter_gainhf_auto() }
	fn set_direct_filter_gainhf_auto(&mut self, value: bool) -> AltoResult<()> { self.src.set_direct_filter_gainhf_auto(value) }
}


impl<'d: 'c, 'c> PartialEq for StreamingSource<'d, 'c> {
	fn eq(&self, other: &StreamingSource<'d, 'c>) -> bool {
		self.src == other.src
	}
}
impl<'d: 'c, 'c> Eq for StreamingSource<'d, 'c> { }
