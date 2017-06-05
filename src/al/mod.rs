use std::ops::Deref;
use std::iter;
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::VecDeque;
use std::mem;
use std::ptr;
use std::hash::{Hash, Hasher};
use std::ffi::{CString, CStr};

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


/// The spatialization mode of a source.
/// Requires `ALC_SOFT_source_spatialization`
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SoftSourceSpatialize {
	/// `AL_FALSE`
	Disabled,
	/// `AL_TRUE`
	Enabled,
	/// `AL_AUTO_SOFT`
	Auto,
}


/// A listener context.
pub struct Context(pub(crate) Arc<ContextInner>);


pub(crate) struct ContextInner {
	pub dev: Device,
	pub ctx: *mut sys::ALCcontext,
	pub exts: ext::AlCache,
	defer_rc: Arc<AtomicUsize>,
}


/// An RAII lock that will suspend state updates while held.
/// When this lock is dropped, the context will apply all pending updates.
pub struct DeferLock<'c> {
	ctx: &'c Context
}


/// An audio buffer of any format.
pub struct Buffer {
	ctx: Context,
	buf: sys::ALuint, 
	len: sys::ALsizei,
}


/// Capabilities common to both static and streaming sources.
pub unsafe trait SourceTrait {
	/// The context from which this source was created.
	fn context(&self) -> &Context;
	/// Raw handle as provided by OpenAL.
	fn as_raw(&self) -> sys::ALuint;

	/// `alGetSourcei(AL_SOURCE_STATE)`
	fn state(&self) -> SourceState;
	/// `alSourcePlay()`
	fn play(&mut self);
	/// `alSourcePause()`
	fn pause(&mut self);
	/// `alSourceStop()`
	fn stop(&mut self);
	/// `alSourceRewind()`
	fn rewind(&mut self);

	/// `alGetSourcei(AL_SOURCE_RELATIVE)`
	fn relative(&self) -> bool;
	/// `alSourcei(AL_SOURCE_RELATIVE)`
	fn set_relative(&mut self, bool);

	/// `alGetSourcef(AL_GAIN)`
	fn gain(&self) -> f32;
	/// `alSourcef(AL_GAIN)`
	fn set_gain(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_MIN_GAIN)`
	fn min_gain(&self) -> f32;
	/// `alSourcef(AL_MIN_GAIN)`
	fn set_min_gain(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_MAX_GAIN)`
	fn max_gain(&self) -> f32;
	/// `alSourcef(AL_MAX_GAIN)`
	fn set_max_gain(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_REFERENCE_DISTANCE)`
	fn reference_distance(&self) -> f32;
	/// `alSourcef(AL_REFERENCE_DISTANCE)`
	fn set_reference_distance(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_ROLLOFF_FACTOR)`
	fn rolloff_factor(&self) -> f32;
	/// `alSourcef(AL_ROLLOFF_FACTOR)`
	fn set_rolloff_factor(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_MAX_DISTANCE)`
	fn max_distance(&self) -> f32;
	/// `alSourcef(AL_MAX_DISTANCE)`
	fn set_max_distance(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_PITCH)`
	fn pitch(&self) -> f32;
	/// `alSourcef(AL_PITCH)`
	fn set_pitch(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcefv(AL_POSITION)`
	fn position<V: From<[f32; 3]>>(&self) -> V;
	/// `alSourcefv(AL_POSITION)`
	fn set_position<V: Into<[f32; 3]>>(&mut self, V) -> AltoResult<()>;

	/// `alGetSourcefv(AL_VELOCITY)`
	fn velocity<V: From<[f32; 3]>>(&self) -> V;
	/// `alSourcefv(AL_VELOCITY)`
	fn set_velocity<V: Into<[f32; 3]>>(&mut self, V) -> AltoResult<()>;

	/// `alGetSourcefv(AL_DIRECTION)`
	fn direction<V: From<[f32; 3]>>(&self) -> V;
	/// `alSourcefv(AL_DIRECTION)`
	fn set_direction<V: Into<[f32; 3]>>(&mut self, V) -> AltoResult<()>;

	/// `alGetSourcef(AL_CONE_INNER_ANGLE)`
	fn cone_inner_angle(&self) -> f32;
	/// `alSourcef(AL_CONE_INNER_ANGLE)`
	fn set_cone_inner_angle(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_CONE_OUTER_ANGLE)`
	fn cone_outer_angle(&self) -> f32;
	/// `alSourcef(AL_CONE_OUTER_ANGLE)`
	fn set_cone_outer_angle(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_CONE_OUTER_GAIN)`
	fn cone_outer_gain(&self) -> f32;
	/// `alSourcef(AL_CONE_OUTER_GAIN)`
	fn set_cone_outer_gain(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_SEC_OFFSET)`
	fn sec_offset(&self) -> f32;
	/// `alSourcef(AL_SEC_OFFSET)`
	fn set_sec_offset(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcei(AL_SAMPLE_OFFSET)`
	fn sample_offset(&self) -> sys::ALint;
	/// `alSourcei(AL_SAMPLE_OFFSET)`
	fn set_sample_offset(&mut self, sys::ALint) -> AltoResult<()>;

	/// `alGetSourcei(AL_BYTE_OFFSET)`
	fn byte_offset(&self) -> sys::ALint;
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
	fn soft_direct_channels(&self) -> bool;
	/// `alSourcei(AL_DIRECT_CHANNELS_SOFT)`
	/// Requires `AL_SOFT_direct_channels`
	fn set_soft_direct_channels(&mut self, bool) -> AltoResult<()>;

	/// `alGetSourcei(AL_DISTANCE_MODEL)`
	/// Requires `AL_EXT_source_distance_model`
	fn distance_model(&self) -> DistanceModel;
	/// `alSourcei(AL_DISTANCE_MODEL)`
	/// Requires `AL_EXT_source_distance_model`
	fn set_distance_model(&mut self, DistanceModel) -> AltoResult<()>;

	/// `alGetSourcei(AL_SOURCE_SPATIALIZATION_SOFT)`
	/// Requires `AL_SOFT_source_spatialization`
	fn soft_spatialization(&self) -> SoftSourceSpatialize;
	/// `alSourcei(AL_SOURCE_SPATIALIZATION_SOFT)`
	/// Requires `AL_SOFT_source_spatialization`
	fn set_soft_spatialization(&mut self, value: SoftSourceSpatialize) -> AltoResult<()>;

	/// `alGetSourcei(AL_SOURCE_RESAMPLER_SOFT)`
	/// Requires `AL_SOFT_source_resampler`
	fn soft_resampler(&self) -> AltoResult<sys::ALint>;
	/// `alSourcei(AL_SOURCE_RESAMPLER_SOFT)`
	/// Requires `AL_SOFT_source_resampler`
	fn set_soft_resampler(&mut self, value: sys::ALint) -> AltoResult<()>;

	/// `alGetSourcefv(AL_SOURCE_SPATIALIZATION_SOFT)`
	/// Requires `AL_SOFT_source_spatialization`
	fn stereo_angles<V: From<[f32; 2]>>(&self) -> AltoResult<V>;
	/// `alSourcefv(AL_SOURCE_SPATIALIZATION_SOFT)`
	/// Requires `AL_SOFT_source_spatialization`
	fn set_stereo_angles<V: Into<[f32; 2]>>(&mut self, value: V) -> AltoResult<()>;

	/// `alGetSourcef(AL_SOURCE_RADIUS)`
	/// Requires `AL_EXT_SOURCE_RADIUS`
	fn radius(&self) -> f32;
	/// `alSourcef(AL_SOURCE_RADIUS)`
	/// Requires `AL_EXT_SOURCE_RADIUS`
	fn set_radius(&self, value: f32) -> AltoResult<()>;

	/// `alSourcei(AL_DIRECT_FILTER)`
	/// Requires `ALC_EXT_EFX`
	fn set_direct_filter<F: FilterTrait>(&mut self, value: &F) -> AltoResult<()>;
	/// `alSourcei(AL_DIRECT_FILTER)`
	/// Requires `ALC_EXT_EFX`
	fn clear_direct_filter(&mut self);

	/// `alSourceiv(AL_AUXILIARY_SEND_FILTER)`
	/// Requires `ALC_EXT_EFX`
	fn set_aux_send(&mut self, send: sys::ALint, value: &mut AuxEffectSlot) -> AltoResult<()>;
	/// `alSourceiv(AL_AUXILIARY_SEND_FILTER)`
	/// Requires `ALC_EXT_EFX`
	fn set_aux_send_filter<F: FilterTrait>(&mut self, send: sys::ALint, slot: &mut AuxEffectSlot, filter: &F) -> AltoResult<()>;
	/// `alSourceiv(AL_AUXILIARY_SEND_FILTER)`
	/// Requires `ALC_EXT_EFX`
	fn clear_aux_send(&mut self, send: sys::ALint);

	/// `alGetSourcef(AL_AIR_ABSORPTION_FACTOR)`
	/// Requires `ALC_EXT_EFX`
	fn air_absorption_factor(&self) -> f32;
	/// `alSourcef(AL_AIR_ABSORPTION_FACTOR)`
	/// Requires `ALC_EXT_EFX`
	fn set_air_absorption_factor(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_ROOM_ROLLOFF_FACTOR)`
	/// Requires `ALC_EXT_EFX`
	fn room_rolloff_factor(&self) -> f32;
	/// `alSourcef(AL_ROOM_ROLLOFF_FACTOR)`
	/// Requires `ALC_EXT_EFX`
	fn set_room_rolloff_factor(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcef(AL_CONE_OUTER_GAINHF)`
	/// Requires `ALC_EXT_EFX`
	fn cone_outer_gainhf(&self) -> f32;
	/// `alSourcef(AL_CONE_OUTER_GAINHF)`
	/// Requires `ALC_EXT_EFX`
	fn set_cone_outer_gainhf(&mut self, f32) -> AltoResult<()>;

	/// `alGetSourcei(AL_DIRECT_FILTER_GAINHF_AUTO)`
	/// Requires `ALC_EXT_EFX`
	fn direct_filter_gainhf_auto(&self) -> bool;
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


pub(crate) struct SourceInner {
	ctx: Context,
	src: sys::ALuint,
	sends: Mutex<Vec<sys::ALuint>>,
}


/// A source that can play a shared static buffer.
pub struct StaticSource {
	src: Arc<SourceInner>,
	buf: Option<Arc<Buffer>>,
}


/// A source that plays a queue of owned buffers.
pub struct StreamingSource {
	src: Arc<SourceInner>,
	bufs: VecDeque<Buffer>,
}


impl Context {
	pub(crate) unsafe fn new(dev: Device, ctx: *mut sys::ALCcontext) -> Context {
		let exts = {
			let _lock = Context::make_raw_current(&dev, ctx);
			ext::AlCache::new(&dev.0.alto.0.api)
		};

		Context(Arc::new(ContextInner{
			dev: dev,
			ctx: ctx,
			exts: exts,
			defer_rc: Arc::new(AtomicUsize::new(0)),
		}))
	}


	/// The device from which this context was created.
	pub fn device(&self) -> &Device { &self.0.dev }
	/// Raw context pointer as provided by OpenAL.
	pub fn as_raw(&self) -> *mut sys::ALCcontext { self.0.ctx }


	/// `alIsExtensionPresent()`
	pub fn is_extension_present(&self, ext: ext::Al) -> bool {
		match ext {
			ext::Al::ALaw => self.0.exts.AL_EXT_ALAW().is_ok(),
			ext::Al::BFormat => self.0.exts.AL_EXT_BFORMAT().is_ok(),
			ext::Al::Double => self.0.exts.AL_EXT_double().is_ok(),
			ext::Al::Float32 => self.0.exts.AL_EXT_float32().is_ok(),
			ext::Al::Ima4 => self.0.exts.AL_EXT_IMA4().is_ok(),
			ext::Al::McFormats => self.0.exts.AL_EXT_MCFORMATS().is_ok(),
			ext::Al::MuLaw => self.0.exts.AL_EXT_MULAW().is_ok(),
			ext::Al::MuLawBFormat => self.0.exts.AL_EXT_MULAW_BFORMAT().is_ok(),
			ext::Al::MuLawMcFormats => self.0.exts.AL_EXT_MULAW_MCFORMATS().is_ok(),
			ext::Al::SoftBlockAlignment => self.0.exts.AL_SOFT_block_alignment().is_ok(),
//			ext::Al::SoftBufferSamples => self.0.ext.AL_SOFT_buffer_samples().is_ok(),
//			ext::Al::SoftBufferSubData => self.0.ext.AL_SOFT_buffer_sub_data().is_ok(),
			ext::Al::SoftDeferredUpdates => self.0.exts.AL_SOFT_deferred_updates().is_ok(),
			ext::Al::SoftDirectChannels => self.0.exts.AL_SOFT_direct_channels().is_ok(),
			ext::Al::SoftLoopPoints => self.0.exts.AL_SOFT_loop_points().is_ok(),
			ext::Al::SoftMsadpcm => self.0.exts.AL_SOFT_MSADPCM().is_ok(),
			ext::Al::SoftSourceLatency => self.0.exts.AL_SOFT_source_latency().is_ok(),
			ext::Al::SoftSourceLength => self.0.exts.AL_SOFT_source_length().is_ok(),
			ext::Al::SourceDistanceModel => self.0.exts.AL_EXT_source_distance_model().is_ok(),
			ext::Al::SoftSourceSpatialize => self.0.exts.AL_SOFT_source_spatialize().is_ok(),
			ext::Al::SoftSourceResampler => self.0.exts.AL_SOFT_source_resampler().is_ok(),
			ext::Al::SoftGainClampEx => self.0.exts.AL_SOFT_gain_clamp_ex().is_ok(),
			ext::Al::StereoAngles => self.0.exts.AL_EXT_STEREO_ANGLES().is_ok(),
			ext::Al::SourceRadius => self.0.exts.AL_EXT_SOURCE_RADIUS().is_ok(),
		}
	}


	/// `alGetInteger(AL_DISTANCE_MODEL)`
	pub fn distance_model(&self) -> DistanceModel {
		let _lock = self.make_current(true);
		match unsafe { self.0.dev.0.alto.0.api.alGetInteger(sys::AL_DISTANCE_MODEL) } {
			sys::AL_NONE => DistanceModel::None,
			sys::AL_INVERSE_DISTANCE => DistanceModel::Inverse,
			sys::AL_INVERSE_DISTANCE_CLAMPED => DistanceModel::InverseClamped,
			sys::AL_LINEAR_DISTANCE => DistanceModel::Linear,
			sys::AL_LINEAR_DISTANCE_CLAMPED => DistanceModel::LinearClamped,
			sys::AL_EXPONENT_DISTANCE => DistanceModel::Exponent,
			sys::AL_EXPONENT_DISTANCE_CLAMPED => DistanceModel::ExponentClamped,
			_ => panic!("ALTO ERROR: Unknown distance model"),
		}
	}
	/// `alDistanceModel()`
	pub fn set_distance_model(&self, value: DistanceModel) {
		let _lock = self.make_current(true);
		unsafe {
			self.0.dev.0.alto.0.api.alDistanceModel(match value {
				DistanceModel::None => sys::AL_NONE,
				DistanceModel::Inverse => sys::AL_INVERSE_DISTANCE,
				DistanceModel::InverseClamped => sys::AL_INVERSE_DISTANCE_CLAMPED,
				DistanceModel::Linear => sys::AL_LINEAR_DISTANCE,
				DistanceModel::LinearClamped => sys::AL_LINEAR_DISTANCE_CLAMPED,
				DistanceModel::Exponent => sys::AL_EXPONENT_DISTANCE,
				DistanceModel::ExponentClamped => sys::AL_EXPONENT_DISTANCE_CLAMPED,
			})
		};
	}


	/// `alIsEnabled(AL_SOURCE_DISTANCE_MODEL)`
	/// Requires `AL_EXT_source_distance_model`
	pub fn using_source_distance_model(&self) -> bool {
		let _lock = self.make_current(true);
		(|| -> AltoResult<_> {
			unsafe { Ok(self.0.dev.0.alto.0.api.alIsEnabled(self.0.exts.AL_EXT_source_distance_model()?.AL_SOURCE_DISTANCE_MODEL?)) }
		})().unwrap_or(sys::AL_FALSE) == sys::AL_TRUE
	}
	/// `alEnable/alDisable(AL_SOURCE_DISTANCE_MODEL)`
	/// Requires `AL_EXT_source_distance_model`
	pub fn use_source_distance_model(&self, value: bool) -> AltoResult<()> {
		let _lock = self.make_current(true);
		if value {
			unsafe { self.0.dev.0.alto.0.api.alEnable(self.0.exts.AL_EXT_source_distance_model()?.AL_SOURCE_DISTANCE_MODEL?); }
		} else {
			unsafe { self.0.dev.0.alto.0.api.alDisable(self.0.exts.AL_EXT_source_distance_model()?.AL_SOURCE_DISTANCE_MODEL?); }
		}
		self.get_error()
	}


	/// `alGetFloat(AL_DOPPLER_FACTOR)`
	pub fn doppler_factor(&self) -> f32 {
		let _lock = self.make_current(true);
		unsafe { self.0.dev.0.alto.0.api.alGetFloat(sys::AL_DOPPLER_FACTOR) }
	}
	/// `alDopplerFactor()`
	pub fn set_doppler_factor(&self, value: f32) -> AltoResult<()> {
		let _lock = self.make_current(true);
		unsafe { self.0.dev.0.alto.0.api.alDopplerFactor(value); }
		self.get_error()
	}


	/// `alGetFloat(AL_SPEED_OF_SOUND)`
	pub fn speed_of_sound(&self) -> f32 {
		let _lock = self.make_current(true);
		unsafe { self.0.dev.0.alto.0.api.alGetFloat(sys::AL_SPEED_OF_SOUND) }
	}
	/// `alSpeedOfSound()`
	pub fn set_speed_of_sound(&self, value: f32) -> AltoResult<()> {
		let _lock = self.make_current(true);
		unsafe { self.0.dev.0.alto.0.api.alSpeedOfSound(value); }
		self.get_error()
	}


	/// `alGetListenerv(AL_GAIN)`
	pub fn gain(&self) -> f32 {
		let _lock = self.make_current(true);
		let mut value = 0.0;
		unsafe { self.0.dev.0.alto.0.api.alGetListenerf(sys::AL_GAIN, &mut value); }
		value
	}
	/// `alListenerf(AL_GAIN)`
	pub fn set_gain(&self, value: f32) -> AltoResult<()> {
		let _lock = self.make_current(true);
		unsafe { self.0.dev.0.alto.0.api.alListenerf(sys::AL_GAIN, value); }
		self.get_error()
	}


	/// `alGetListenerfv(AL_POSITION)`
	pub fn position<V: From<[f32; 3]>>(&self) -> V {
		let _lock = self.make_current(true);
		let mut value = [0.0, 0.0, 0.0];
		unsafe { self.0.dev.0.alto.0.api.alGetListenerfv(sys::AL_POSITION, &mut value as *mut [f32; 3] as *mut sys::ALfloat); }
		value.into()
	}
	/// `alListenerfv(AL_POSITION)`
	pub fn set_position<V: Into<[f32; 3]>>(&self, value: V) -> AltoResult<()> {
		let _lock = self.make_current(true);
		let value = value.into();
		unsafe { self.0.dev.0.alto.0.api.alListenerfv(sys::AL_POSITION, &value as *const [f32; 3] as *const sys::ALfloat); }
		self.get_error()
	}


	/// `alGetListenerfv(AL_VELOCITY)`
	pub fn velocity<V: From<[f32; 3]>>(&self) -> V {
		let _lock = self.make_current(true);
		let mut value = [0.0, 0.0, 0.0];
		unsafe { self.0.dev.0.alto.0.api.alGetListenerfv(sys::AL_VELOCITY, &mut value as *mut [f32; 3] as *mut sys::ALfloat); }
		value.into()
	}
	/// `alListenerfv(AL_VELOCITY)`
	pub fn set_velocity<V: Into<[f32; 3]>>(&self, value: V) -> AltoResult<()> {
		let _lock = self.make_current(true);
		let value = value.into();
		unsafe { self.0.dev.0.alto.0.api.alListenerfv(sys::AL_VELOCITY, &value as *const [f32; 3] as *const sys::ALfloat); }
		self.get_error()
	}


	/// `alGetListenerfv(AL_ORIENTATION)`
	pub fn orientation<V: From<[f32; 3]>>(&self) -> (V, V) {
		let _lock = self.make_current(true);
		let mut value = [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
		unsafe { self.0.dev.0.alto.0.api.alGetListenerfv(sys::AL_ORIENTATION, &mut value as *mut [[f32; 3]; 2] as *mut sys::ALfloat); }
		(value[0].into(), value[1].into())
	}
	/// `alListenerfv(AL_ORIENTATION)`
	pub fn set_orientation<V: Into<[f32; 3]>>(&self, value: (V, V)) -> AltoResult<()> {
		let _lock = self.make_current(true);
		let value = [value.0.into(), value.1.into()];
		unsafe { self.0.dev.0.alto.0.api.alListenerfv(sys::AL_ORIENTATION, &value as *const [[f32; 3]; 2] as *const sys::ALfloat); }
		self.get_error()
	}


	/// `alGetListenerf(AL_GAIN_LIMIT_SOFT)`
	/// Requires `AL_SOFT_gain_clamp_ex`
	pub fn soft_gain_limit(&self) -> AltoResult<f32> {
		let asgce = self.0.exts.AL_SOFT_gain_clamp_ex()?;
		let _lock = self.make_current(true);
		Ok(unsafe { self.0.dev.0.alto.0.api.alGetFloat(asgce.AL_GAIN_LIMIT_SOFT?) })
	}


	/// `alGetStringiSOFT(AL_RESAMPLER_NAME_SOFT)`
	/// Requires `AL_SOFT_source_resampler`
	pub fn enumerate_soft_resamplers(&self) -> Vec<CString> {
		let mut name_vec = Vec::with_capacity(0);

		let _ = (|| -> AltoResult<_> {
			let assr = self.0.exts.AL_SOFT_source_resampler()?;
			let value = unsafe { self.0.dev.0.alto.0.api.alGetInteger(assr.AL_NUM_RESAMPLERS_SOFT?) };

			for i in 0 .. value {
				unsafe {
					let name = assr.alGetStringiSOFT?(assr.AL_RESAMPLER_NAME_SOFT?, i) as *mut _;
					name_vec.push(CStr::from_ptr(name).to_owned());
				}
			}

			Ok(())
		})();

		name_vec
	}


	/// `alGetListenerf(AL_METERS_PER_UNIT)`
	/// Requires `ALC_EXT_EFX`
	pub fn meters_per_unit(&self) -> f32 {
		(|| -> AltoResult<_> {
			let efx = self.0.dev.0.exts.ALC_EXT_EFX()?;
			let _lock = self.make_current(true);
			let mut value = 0.0;
			unsafe { self.0.dev.0.alto.0.api.alGetListenerf(efx.AL_METERS_PER_UNIT?, &mut value); }
			Ok(value)
		})().unwrap_or(1.0)
	}
	/// `alListenerf(AL_METERS_PER_UNIT)`
	/// Requires `ALC_EXT_EFX`
	pub fn set_meters_per_unit(&self, value: f32) -> AltoResult<()> {
		let efx = self.0.dev.0.exts.ALC_EXT_EFX()?;
		let _lock = self.make_current(true);
		unsafe { self.0.dev.0.alto.0.api.alListenerf(efx.AL_METERS_PER_UNIT?, value); }
		self.get_error()
	}


	/// `alGenBuffers()`
	pub fn new_buffer<F: SampleFrame, B: AsBufferData<F>>(&self, data: B, freq: i32) -> AltoResult<Buffer> {
		Buffer::new(self.clone(), data, freq)
	}


	/// `alGenSources()`
	pub fn new_static_source(&self) -> AltoResult<StaticSource> {
		StaticSource::new(self.clone())
	}


	/// `alGenSources()`
	pub fn new_streaming_source(&self) -> AltoResult<StreamingSource> {
		StreamingSource::new(self.clone())
	}


	/// `alDeferUpdatesSOFT()`
	/// requires `AL_SOFT_deferred_updates` is available.
	pub fn defer_updates(&self) -> AltoResult<DeferLock> {
		DeferLock::new(self)
	}


	/// `alGenAuxiliaryEffectSlots()`
	/// Requires `ALC_EXT_EFX`
	pub fn new_aux_effect_slot(&self) -> AltoResult<AuxEffectSlot> {
		AuxEffectSlot::new(self.clone())
	}


	/// `alGenEffects()`
	/// Requires `ALC_EXT_EFX`
	pub fn new_effect<E: EffectTrait>(&self) -> AltoResult<E> {
		E::new(self.clone())
	}


	/// `alGenFilters()`
	/// Requires `ALC_EXT_EFX`
	pub fn new_filter<F: FilterTrait>(&self) -> AltoResult<F> {
		F::new(self.clone())
	}


	pub(crate) fn make_current(&self, set: bool) -> Option<MutexGuard<()>> {
		Context::make_raw_current(&self.0.dev, if set { self.0.ctx } else { ptr::null_mut() })
	}


	fn make_raw_current(dev: &Device, ctx: *mut sys::ALCcontext) -> Option<MutexGuard<()>> {
		if let Ok(&ext::ALC_EXT_thread_local_context{alcSetThreadContext: Ok(astc), ..}) = dev.0.alto.0.exts.ALC_EXT_thread_local_context() {
			unsafe { astc(ctx); }
			None
		} else {
			unsafe { dev.0.alto.0.api.alcMakeContextCurrent(ctx); }
			Some(ALTO_CTX_LOCK__.lock().unwrap())
		}
	}


	pub(crate) fn get_error(&self) -> AltoResult<()> {
		match unsafe { self.0.dev.0.alto.0.api.alGetError() } {
			sys::AL_NO_ERROR => Ok(()),
			e => Err(AltoError::from_al(e))
		}
	}
}


impl Clone for Context {
	fn clone(&self) -> Context { Context(self.0.clone()) }
}


impl PartialEq for Context {
	fn eq(&self, other: &Context) -> bool {
		self.0.ctx == other.0.ctx
	}
}
impl Eq for Context { }


impl Drop for ContextInner {
	fn drop(&mut self) {
		let _lock = Context::make_raw_current(&self.dev, ptr::null_mut());
		unsafe { self.dev.0.alto.0.api.alcDestroyContext(self.ctx); }
	}
}


unsafe impl Send for Context { }
unsafe impl Sync for Context { }


impl<'c> DeferLock<'c> {
	fn new(ctx: &'c Context) -> AltoResult<DeferLock> {
		let asdu = ctx.0.exts.AL_SOFT_deferred_updates()?;
		let adus = asdu.alDeferUpdatesSOFT?;
		asdu.alProcessUpdatesSOFT?;
		let _lock = ctx.make_current(true);

		let old = ctx.0.defer_rc.fetch_add(1, Ordering::SeqCst);
		if old == 0 {
			unsafe { adus(); }
			if let Err(e) = ctx.get_error() {
				ctx.0.defer_rc.fetch_sub(1, Ordering::SeqCst);
				return Err(e.into());
			}
		}

		Ok(DeferLock{ctx: ctx})
	}
}


impl<'c> Deref for DeferLock<'c> {
	type Target = Context;

	fn deref(&self) -> &Context { &self.ctx }
}


impl<'c> Drop for DeferLock<'c> {
	fn drop(&mut self) {
		let old = self.0.defer_rc.fetch_sub(1, Ordering::SeqCst);
		if old == 1 {
			let apus = self.0.exts.AL_SOFT_deferred_updates().and_then(|asdu| asdu.alProcessUpdatesSOFT).unwrap();
			let _lock = self.ctx.make_current(true);
			unsafe { apus(); }
		}
	}
}


impl Buffer {
	pub(crate) fn new<F: SampleFrame, B: AsBufferData<F>>(ctx: Context, data: B, freq: i32) -> AltoResult<Buffer> {
		let mut buf = 0;
		{
			let _lock = ctx.make_current(true);
			unsafe { ctx.0.dev.0.alto.0.api.alGenBuffers(1, &mut buf as *mut sys::ALuint); }
			ctx.get_error()?;
		}
		let mut buf = Buffer{ctx: ctx, buf: buf, len: 0};
		buf.set_data(data, freq).map(|_| buf)
	}


	/// Context from which this buffer was created.
	pub fn context(&self) -> &Context { &self.ctx }
	/// Raw handle as provided by OpenAL.
	pub fn as_raw(&self) -> sys::ALuint { self.buf }


	/// `alBufferData()`
	pub fn set_data<F: SampleFrame, B: AsBufferData<F>>(&mut self, data: B, freq: i32) -> AltoResult<()> {
		let data = data.as_buffer_data();
		if data.len() == 0 { return Err(AltoError::AlInvalidValue) }
		if sys::ALsizei::max_value() as usize / mem::size_of::<F>() < data.len() { return Err(AltoError::AlInvalidValue) }
		let size = data.len() * mem::size_of::<F>();

		let _lock = self.ctx.make_current(true);
		unsafe {
			self.ctx.0.dev.0.alto.0.api.alBufferData(
				self.buf,
				F::format().into_raw(Some(&self.ctx))?,
				data.as_ptr() as *const sys::ALvoid,
				size as sys::ALsizei,
				freq as sys::ALint,
			);
		}
		self.ctx.get_error()?;

		self.len = data.len() as sys::ALsizei;
		Ok(())
	}


	/// `alGetBufferi(AL_FREQUENCY)`
	pub fn frequency(&self) -> sys::ALint {
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetBufferi(self.buf, sys::AL_FREQUENCY, &mut value); }
		value
	}


	/// `alGetBufferi(AL_BITS)`
	pub fn bits(&self) -> sys::ALint {
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetBufferi(self.buf, sys::AL_BITS, &mut value); }
		value
	}


	/// `alGetBufferi(AL_CHANNELS)`
	pub fn channels(&self) -> sys::ALint {
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetBufferi(self.buf, sys::AL_CHANNELS, &mut value); }
		value
	}


	/// `alGetBufferi(AL_SIZE)`
	pub fn size(&self) -> sys::ALint {
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetBufferi(self.buf, sys::AL_SIZE, &mut value); }
		value
	}


	/// `alGetBufferiv(AL_LOOP_POINTS_SOFT)`
	/// Requires `AL_SOFT_loop_points`
	pub fn soft_loop_points(&self) -> (sys::ALint, sys::ALint) {
		let _lock = self.ctx.make_current(true);
		(|| -> AltoResult<_> {
			let mut value = [0, self.len];
			unsafe { self.ctx.0.dev.0.alto.0.api.alGetBufferiv(self.buf, self.ctx.0.exts.AL_SOFT_loop_points()?.AL_LOOP_POINTS_SOFT?, &mut value as *mut [sys::ALint; 2] as *mut sys::ALint) }
			Ok((value[0], value[1]))
		})().unwrap_or((0, self.len))
	}
	/// `alBufferiv(AL_LOOP_POINTS_SOFT)`
	/// Requires `AL_SOFT_loop_points`
	pub fn set_soft_loop_points(&self, value: (sys::ALint, sys::ALint)) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alBufferiv(self.buf, self.ctx.0.exts.AL_SOFT_loop_points()?.AL_LOOP_POINTS_SOFT?, &[value.0, value.1] as *const [sys::ALint; 2] as *const sys::ALint); }
		self.ctx.get_error()
	}
}


impl Drop for Buffer {
	fn drop(&mut self) {
		if self.buf == 0 {
			return; 
		}

		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alDeleteBuffers(1, &mut self.buf as *mut sys::ALuint); }
	}
}


impl SourceInner {
	fn context(&self) -> &Context { &self.ctx }
	pub fn as_raw(&self) -> sys::ALuint { self.src }


	fn state(&self) -> SourceState {
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcei(self.src, sys::AL_SOURCE_STATE, &mut value); }
		match value {
			sys::AL_INITIAL => SourceState::Initial,
			sys::AL_PLAYING => SourceState::Playing,
			sys::AL_PAUSED => SourceState::Paused,
			sys::AL_STOPPED => SourceState::Stopped,
			_ => panic!("ALTO ERROR: Unknown source state"),
		}
	}
	fn play(&self) {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcePlay(self.src); }
	}
	fn pause(&self) {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcePause(self.src); }
	}
	fn stop(&self) {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourceStop(self.src); }
	}
	fn rewind(&self) {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourceRewind(self.src); }
	}


	fn relative(&self) -> bool {
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcei(self.src, sys::AL_SOURCE_RELATIVE, &mut value); }
		value == sys::AL_TRUE as sys::ALint
	}
	fn set_relative(&self, value: bool) {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcei(self.src, sys::AL_SOURCE_RELATIVE, if value { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
	}


	fn gain(&self) -> f32 {
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcef(self.src, sys::AL_GAIN, &mut value); }
		value
	}
	fn set_gain(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcef(self.src, sys::AL_GAIN, value); }
		self.ctx.get_error()
	}


	fn min_gain(&self) -> f32 {
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcef(self.src, sys::AL_MIN_GAIN, &mut value); }
		value
	}
	fn set_min_gain(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcef(self.src, sys::AL_MIN_GAIN, value); }
		self.ctx.get_error()
	}


	fn max_gain(&self) -> f32 {
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcef(self.src, sys::AL_MAX_GAIN, &mut value); }
		value
	}
	fn set_max_gain(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcef(self.src, sys::AL_MAX_GAIN, value); }
		self.ctx.get_error()
	}


	fn reference_distance(&self) -> f32 {
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcef(self.src, sys::AL_REFERENCE_DISTANCE, &mut value); }
		value
	}
	fn set_reference_distance(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcef(self.src, sys::AL_REFERENCE_DISTANCE, value); }
		self.ctx.get_error()
	}


	fn rolloff_factor(&self) -> f32 {
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcef(self.src, sys::AL_ROLLOFF_FACTOR, &mut value); }
		value
	}
	fn set_rolloff_factor(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcef(self.src, sys::AL_ROLLOFF_FACTOR, value); }
		self.ctx.get_error()
	}


	fn max_distance(&self) -> f32 {
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcef(self.src, sys::AL_MAX_DISTANCE, &mut value); }
		value
	}
	fn set_max_distance(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcef(self.src, sys::AL_MAX_DISTANCE, value); }
		self.ctx.get_error()
	}


	fn pitch(&self) -> f32 {
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcef(self.src, sys::AL_PITCH, &mut value); }
		value
	}
	fn set_pitch(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcef(self.src, sys::AL_PITCH, value); }
		self.ctx.get_error()
	}


	fn position<V: From<[f32; 3]>>(&self) -> V {
		let _lock = self.ctx.make_current(true);
		let mut value = [0.0, 0.0, 0.0];
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcefv(self.src, sys::AL_POSITION, &mut value as *mut [f32; 3] as *mut sys::ALfloat); }
		value.into()
	}
	fn set_position<V: Into<[f32; 3]>>(&self, value: V) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true);
		let value = value.into();
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcefv(self.src, sys::AL_POSITION, &value as *const [f32; 3] as *const sys::ALfloat); }
		self.ctx.get_error()
	}


	fn velocity<V: From<[f32; 3]>>(&self) -> V {
		let _lock = self.ctx.make_current(true);
		let mut value = [0.0, 0.0, 0.0];
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcefv(self.src, sys::AL_VELOCITY, &mut value as *mut [f32; 3] as *mut sys::ALfloat); }
		value.into()
	}
	fn set_velocity<V: Into<[f32; 3]>>(&self, value: V) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true);
		let value = value.into();
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcefv(self.src, sys::AL_VELOCITY, &value as *const [f32; 3] as *const sys::ALfloat); }
		self.ctx.get_error()
	}


	fn direction<V: From<[f32; 3]>>(&self) -> V {
		let _lock = self.ctx.make_current(true);
		let mut value = [0.0, 0.0, 0.0];
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcefv(self.src, sys::AL_DIRECTION, &mut value as *mut [f32; 3] as *mut sys::ALfloat); }
		value.into()
	}
	fn set_direction<V: Into<[f32; 3]>>(&self, value: V) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true);
		let value = value.into();
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcefv(self.src, sys::AL_DIRECTION, &value as *const [f32; 3] as *const sys::ALfloat); }
		self.ctx.get_error()
	}


	fn cone_inner_angle(&self) -> f32 {
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcef(self.src, sys::AL_CONE_INNER_ANGLE, &mut value); }
		value
	}
	fn set_cone_inner_angle(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcef(self.src, sys::AL_CONE_INNER_ANGLE, value); }
		self.ctx.get_error()
	}


	fn cone_outer_angle(&self) -> f32 {
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcef(self.src, sys::AL_CONE_OUTER_ANGLE, &mut value); }
		value
	}
	fn set_cone_outer_angle(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcef(self.src, sys::AL_CONE_OUTER_ANGLE, value); }
		self.ctx.get_error()
	}


	fn cone_outer_gain(&self) -> f32 {
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcef(self.src, sys::AL_CONE_OUTER_GAIN, &mut value); }
		value
	}
	fn set_cone_outer_gain(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcef(self.src, sys::AL_CONE_OUTER_GAIN, value); }
		self.ctx.get_error()
	}


	fn sec_offset(&self) -> f32 {
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcef(self.src, sys::AL_SEC_OFFSET, &mut value); }
		value
	}
	fn set_sec_offset(&self, value: f32) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcef(self.src, sys::AL_SEC_OFFSET, value); }
		self.ctx.get_error()
	}


	fn sample_offset(&self) -> sys::ALint {
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcei(self.src, sys::AL_SAMPLE_OFFSET, &mut value); }
		value
	}
	fn set_sample_offset(&self, value: sys::ALint) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcei(self.src, sys::AL_SAMPLE_OFFSET, value); }
		self.ctx.get_error()
	}


	fn byte_offset(&self) -> sys::ALint {
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcei(self.src, sys::AL_BYTE_OFFSET, &mut value); }
		value
	}
	fn set_byte_offset(&self, value: sys::ALint) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcei(self.src, sys::AL_BYTE_OFFSET, value); }
		self.ctx.get_error()
	}


	fn soft_sec_offset_latency(&self) -> AltoResult<(f64, f64)> {
		let assl = self.ctx.0.exts.AL_SOFT_source_latency()?;
		let _lock = self.ctx.make_current(true);
		let mut value = [0.0, 0.0];
		unsafe { assl.alGetSourcedvSOFT?(self.src, assl.AL_SEC_OFFSET_LATENCY_SOFT?, &mut value as *mut [f64; 2] as *mut f64); }
		self.ctx.get_error().map(|_| (value[0], value[1]))
	}


	fn soft_sample_offset_frac_latency(&self) -> AltoResult<(i32, i32, i64)> {
		let assl = self.ctx.0.exts.AL_SOFT_source_latency()?;
		let _lock = self.ctx.make_current(true);
		let mut value = [0, 0];
		unsafe { assl.alGetSourcei64vSOFT?(self.src, assl.AL_SAMPLE_OFFSET_LATENCY_SOFT?, &mut value as *mut [i64; 2] as *mut i64); }
		self.ctx.get_error().map(|_| ((value[0] >> 32) as i32, value[0] as i32, value[1]))
	}


	fn soft_direct_channels(&self) -> bool {
		let _lock = self.ctx.make_current(true);
		(|| -> AltoResult<_> {
			let mut value = 0;
			unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcei(self.src, self.ctx.0.exts.AL_SOFT_direct_channels()?.AL_DIRECT_CHANNELS_SOFT?, &mut value); }
			Ok(value)
		})().unwrap_or(sys::AL_FALSE as sys::ALint) == sys::AL_TRUE as sys::ALint
	}
	fn set_soft_direct_channels(&self, value: bool) -> AltoResult<()> {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcei(self.src, self.ctx.0.exts.AL_SOFT_direct_channels()?.AL_DIRECT_CHANNELS_SOFT?, if value { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.ctx.get_error()
	}


	fn soft_sec_length(&self) -> AltoResult<f32> {
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcef(self.src, self.ctx.0.exts.AL_SOFT_source_length()?.AL_SEC_LENGTH_SOFT?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}


	fn soft_sample_length(&self) -> AltoResult<sys::ALint> {
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcei(self.src, self.ctx.0.exts.AL_SOFT_source_length()?.AL_SAMPLE_LENGTH_SOFT?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}


	fn soft_byte_length(&self) -> AltoResult<sys::ALint> {
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcei(self.src, self.ctx.0.exts.AL_SOFT_source_length()?.AL_BYTE_LENGTH_SOFT?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}


	fn distance_model(&self) -> DistanceModel {
		(|| -> AltoResult<_> {
			self.ctx.0.exts.AL_EXT_source_distance_model()?;
			let _lock = self.ctx.make_current(true);
			let mut value = 0;
			unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcei(self.src, sys::AL_DISTANCE_MODEL, &mut value); }
			Ok(match value {
				sys::AL_NONE => DistanceModel::None,
				sys::AL_INVERSE_DISTANCE => DistanceModel::Inverse,
				sys::AL_INVERSE_DISTANCE_CLAMPED => DistanceModel::InverseClamped,
				sys::AL_LINEAR_DISTANCE => DistanceModel::Linear,
				sys::AL_LINEAR_DISTANCE_CLAMPED => DistanceModel::LinearClamped,
				sys::AL_EXPONENT_DISTANCE => DistanceModel::Exponent,
				sys::AL_EXPONENT_DISTANCE_CLAMPED => DistanceModel::ExponentClamped,
				_ => panic!("ALTO ERROR: Unknown distance model")
			})
		})().unwrap_or(DistanceModel::InverseClamped)
	}
	fn set_distance_model(&self, value: DistanceModel) -> AltoResult<()> {
		self.ctx.0.exts.AL_EXT_source_distance_model()?;
		let _lock = self.ctx.make_current(true);
		unsafe {
			self.ctx.0.dev.0.alto.0.api.alSourcei(self.src, sys::AL_DISTANCE_MODEL, match value {
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


	fn soft_spatialization(&self) -> SoftSourceSpatialize {
		(|| -> AltoResult<_> {
			let assp = self.ctx.0.exts.AL_SOFT_source_spatialize()?;
			let _lock = self.ctx.make_current(true);
			let mut value = 0;
			unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcei(self.src, assp.AL_SOURCE_SPATIALIZE_SOFT?, &mut value); }
			Ok(match value as sys::ALboolean {
				sys::AL_FALSE => SoftSourceSpatialize::Disabled,
				sys::AL_TRUE => SoftSourceSpatialize::Enabled,
				v if v == assp.AL_AUTO_SOFT? as sys::ALboolean => SoftSourceSpatialize::Auto,
				_ => panic!("ALTO ERROR: Unknown source spatialization")
			})
		})().unwrap_or(SoftSourceSpatialize::Auto)
	}
	fn set_soft_spatialization(&self, value: SoftSourceSpatialize) -> AltoResult<()> {
		let assp = self.ctx.0.exts.AL_SOFT_source_spatialize()?;
		let _lock = self.ctx.make_current(true);
		unsafe {
			self.ctx.0.dev.0.alto.0.api.alSourcei(self.src, assp.AL_SOURCE_SPATIALIZE_SOFT?, match value {
				SoftSourceSpatialize::Disabled => sys::AL_FALSE as sys::ALint,
				SoftSourceSpatialize::Enabled => sys::AL_TRUE as sys::ALint,
				SoftSourceSpatialize::Auto => assp.AL_AUTO_SOFT?,
			});
		}
		self.ctx.get_error()
	}


	fn soft_resampler(&self) -> AltoResult<sys::ALint> {
		let assr = self.ctx.0.exts.AL_SOFT_source_resampler()?;
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourceiv(self.src, assr.AL_SOURCE_RESAMPLER_SOFT?, &mut value); }
		Ok(value.into())
	}
	fn set_soft_resampler(&self, value: sys::ALint) -> AltoResult<()> {
		let assr = self.ctx.0.exts.AL_SOFT_source_resampler()?;
		let _lock = self.ctx.make_current(true);
		unsafe {
			let value = value.into();
			self.ctx.0.dev.0.alto.0.api.alSourceiv(self.src, assr.AL_SOURCE_RESAMPLER_SOFT?, &value);
		}
		self.ctx.get_error()
	}


	fn stereo_angles<V: From<[f32; 2]>>(&self) -> AltoResult<V> {
		let aesa = self.ctx.0.exts.AL_EXT_STEREO_ANGLES()?;
		let _lock = self.ctx.make_current(true);
		let mut value = [0.0, 0.0];
		unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcefv(self.src, aesa.AL_STEREO_ANGLES?, &mut value as *mut [f32; 2] as *mut f32); }
		Ok(value.into())
	}
	fn set_stereo_angles<V: Into<[f32; 2]>>(&self, value: V) -> AltoResult<()> {
		let aesa = self.ctx.0.exts.AL_EXT_STEREO_ANGLES()?;
		let _lock = self.ctx.make_current(true);
		unsafe {
			let value = value.into();
			self.ctx.0.dev.0.alto.0.api.alSourcefv(self.src, aesa.AL_STEREO_ANGLES?, &value as *const [f32; 2] as *const f32);
		}
		self.ctx.get_error()
	}


	fn radius(&self) -> f32 {
		(|| -> AltoResult<_> {
			let aesr = self.ctx.0.exts.AL_EXT_SOURCE_RADIUS()?;
			let _lock = self.ctx.make_current(true);
			let mut value = 0.0;
			unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcefv(self.src, aesr.AL_SOURCE_RADIUS?, &mut value); }
			Ok(value.into())
		})().unwrap_or(0.0)
	}
	fn set_radius(&self, value: f32) -> AltoResult<()> {
		let aesr = self.ctx.0.exts.AL_EXT_SOURCE_RADIUS()?;
		let _lock = self.ctx.make_current(true);
		unsafe {
			let value = value.into();
			self.ctx.0.dev.0.alto.0.api.alSourcefv(self.src, aesr.AL_SOURCE_RADIUS?, &value);
		}
		self.ctx.get_error()
	}


	fn set_direct_filter<F: FilterTrait>(&self, value: &F) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX()?;
		if *value.context() != self.ctx {
			panic!("ALTO ERROR: Filter used on wrong context");
		}

		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcei(self.src, efx.AL_DIRECT_FILTER?, value.as_raw() as sys::ALint); }
		self.ctx.get_error()
	}
	fn clear_direct_filter(&self) {
		let _ = (|| -> AltoResult<_> {
			let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			let _lock = self.ctx.make_current(true);
			unsafe { self.ctx.0.dev.0.alto.0.api.alSourcei(self.src, efx.AL_DIRECT_FILTER?, 0); }
			Ok(())
		})();
	}


	fn set_aux_send(arc_self: &Arc<SourceInner>, send: sys::ALint, slot: &mut AuxEffectSlot) -> AltoResult<()> {
		SourceInner::set_aux_send_impl(arc_self, send, slot, 0)
	}
	fn set_aux_send_filter<F: FilterTrait>(arc_self: &Arc<SourceInner>, send: sys::ALint, slot: &mut AuxEffectSlot, filter: &F) -> AltoResult<()> {
		if *filter.context() != arc_self.ctx {
			panic!("ALTO ERROR: Filter used on wrong context");
		}

		SourceInner::set_aux_send_impl(arc_self, send, slot, filter.as_raw())
	}
	fn set_aux_send_impl(arc_self: &Arc<SourceInner>, send: sys::ALint, slot: &mut AuxEffectSlot, filter: sys::ALuint) -> AltoResult<()> {
		let efx = arc_self.ctx.0.dev.0.exts.ALC_EXT_EFX()?;
		if send >= arc_self.ctx.0.dev.max_aux_sends() || *slot.context() != arc_self.ctx {
			panic!("ALTO ERROR: AuxEffectSlot used on wrong context");
		}

		let _lock = arc_self.ctx.make_current(true);
		let mut sends = arc_self.sends.lock().unwrap();
		unsafe { arc_self.ctx.0.dev.0.alto.0.api.alSourceiv(arc_self.src, efx.AL_AUXILIARY_SEND_FILTER?, &mut [slot.as_raw() as sys::ALint, send, filter as sys::ALint] as *mut [sys::ALint; 3] as *mut sys::ALint); }
		arc_self.ctx.get_error()?;
		sends[send as usize] = 0;
		slot.add_input(Arc::downgrade(arc_self));
		Ok(())
	}
	fn clear_aux_send(&self, send: sys::ALint) {
		let _ = (|| -> AltoResult<_> {
			let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			if send >= self.ctx.0.dev.max_aux_sends() {
				return Err(AltoError::AlInvalidValue);
			}

			let _lock = self.ctx.make_current(true);
			let mut sends = self.sends.lock().unwrap();
			unsafe { self.ctx.0.dev.0.alto.0.api.alSourceiv(self.src, efx.AL_AUXILIARY_SEND_FILTER?, &mut [0, send, 0] as *mut [sys::ALint; 3] as *mut sys::ALint); }
			sends[send as usize] = 0;
			Ok(())
		})();
	}
	pub fn clear_aux_effect_slot(&self, slot: sys::ALuint) {
		if let Ok(&ext::ALC_EXT_EFX{AL_AUXILIARY_SEND_FILTER: Ok(aasf), ..}) = self.ctx.0.dev.0.exts.ALC_EXT_EFX() {
			for (i, s) in self.sends.lock().unwrap().iter_mut().enumerate() {
				if *s == slot {
					unsafe { self.ctx.0.dev.0.alto.0.api.alSourceiv(self.src, aasf, &mut [0, i as sys::ALint, 0] as *mut [sys::ALint; 3] as *mut sys::ALint); }
					*s = 0;
				}
			}
		}
	}


	fn air_absorption_factor(&self) -> f32 {
		(|| -> AltoResult<_> {
			let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			let _lock = self.ctx.make_current(true);
			let mut value = 0.0;
			unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcef(self.src, efx.AL_AIR_ABSORPTION_FACTOR?, &mut value); }
			Ok(value)
		})().unwrap_or(0.0)
	}
	fn set_air_absorption_factor(&self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcef(self.src, efx.AL_AIR_ABSORPTION_FACTOR?, value); }
		self.ctx.get_error()
	}


	fn room_rolloff_factor(&self) -> f32 {
		(|| -> AltoResult<_> {
			let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			let _lock = self.ctx.make_current(true);
			let mut value = 0.0;
			unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcef(self.src, efx.AL_ROOM_ROLLOFF_FACTOR?, &mut value); }
			Ok(value)
		})().unwrap_or(0.0)
	}
	fn set_room_rolloff_factor(&self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcef(self.src, efx.AL_ROOM_ROLLOFF_FACTOR?, value); }
		self.ctx.get_error()
	}


	fn cone_outer_gainhf(&self) -> f32 {
		(|| -> AltoResult<_> {
			let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			let _lock = self.ctx.make_current(true);
			let mut value = 0.0;
			unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcef(self.src, efx.AL_CONE_OUTER_GAINHF?, &mut value); }
			Ok(value)
		})().unwrap_or(0.0)
	}
	fn set_cone_outer_gainhf(&self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcef(self.src, efx.AL_CONE_OUTER_GAINHF?, value); }
		self.ctx.get_error()
	}


	fn direct_filter_gainhf_auto(&self) -> bool {
		(|| -> AltoResult<_> {
			let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			let _lock = self.ctx.make_current(true);
			let mut value = 0;
			unsafe { self.ctx.0.dev.0.alto.0.api.alGetSourcei(self.src, efx.AL_CONE_OUTER_GAINHF?, &mut value); }
			Ok(value)
		})().unwrap_or(sys::AL_TRUE as sys::ALint) == sys::AL_TRUE as sys::ALint
	}
	fn set_direct_filter_gainhf_auto(&self, value: bool) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alSourcei(self.src, efx.AL_CONE_OUTER_GAINHF?, if value { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.ctx.get_error()
	}
}


impl PartialEq for SourceInner {
	fn eq(&self, other: &SourceInner) -> bool {
		self.ctx == other.ctx && self.src == other.src
	}
}
impl Eq for SourceInner { }


impl Hash for SourceInner {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
		self.ctx.as_raw().hash(state);
		self.src.hash(state);
	}
}


impl Drop for SourceInner {
	fn drop(&mut self) {
		let _lock = self.ctx.make_current(true);
		unsafe { self.ctx.0.dev.0.alto.0.api.alDeleteSources(1, &mut self.src as *mut sys::ALuint); }
	}
}


impl StaticSource {
	pub(crate) fn new(ctx: Context) -> AltoResult<StaticSource> {
		let mut src = 0;
		{
			let _lock = ctx.make_current(true);
			unsafe { ctx.0.dev.0.alto.0.api.alGenSources(1, &mut src as *mut sys::ALuint); }
			ctx.get_error()?;
		}
		let sends = iter::repeat(0).take(ctx.0.dev.0.max_aux_sends() as usize).collect();
		Ok(StaticSource{src: Arc::new(SourceInner{ctx: ctx, src: src, sends: Mutex::new(sends)}), buf: None})
	}


	pub fn buffer(&self) -> Option<&Arc<Buffer>> { self.buf.as_ref() }


	/// `alSourcei(AL_BUFFER)`
	pub fn set_buffer(&mut self, buf: Arc<Buffer>) {
		if buf.ctx.device().as_raw() != self.src.ctx.device().as_raw() {
			panic!("ALTO ERROR: Buffer used on wrong device");
		}

		{
			let _lock = self.src.ctx.make_current(true);
			unsafe { self.src.ctx.0.dev.0.alto.0.api.alSourcei(self.src.src, sys::AL_BUFFER, buf.buf as sys::ALint); }
		}

		self.buf = Some(buf);
	}
	/// `alSourcei(AL_BUFFER)`
	pub fn clear_buffer(&mut self) {
		{
			let _lock = self.src.ctx.make_current(true);
			unsafe { self.src.ctx.0.dev.0.alto.0.api.alSourcei(self.src.src, sys::AL_BUFFER, 0); }
		}

		self.buf = None;
	}


	/// `alGetSourcei(AL_LOOPING)`
	pub fn looping(&self) -> bool {
		let _lock = self.src.ctx.make_current(true);
		let mut value = 0;
		unsafe { self.src.ctx.0.dev.0.alto.0.api.alGetSourcei(self.src.src, sys::AL_LOOPING, &mut value); }
		value == sys::AL_TRUE as sys::ALint
	}
	/// `alSourcei(AL_LOOPING)`
	pub fn set_looping(&mut self, value: bool) {
		let _lock = self.src.ctx.make_current(true);
		unsafe { self.src.ctx.0.dev.0.alto.0.api.alSourcei(self.src.src, sys::AL_LOOPING, if value { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
	}
}


unsafe impl SourceTrait for StaticSource {
	#[inline] fn context(&self) -> &Context { self.src.context() }
	#[inline] fn as_raw(&self) -> sys::ALuint { self.src.as_raw() }

	#[inline] fn state(&self) -> SourceState { self.src.state() }
	#[inline] fn play(&mut self) -> () { self.src.play() }
	#[inline] fn pause(&mut self) -> () { self.src.pause() }
	#[inline] fn stop(&mut self) -> () { self.src.stop() }
	#[inline] fn rewind(&mut self) -> () { self.src.rewind() }

	#[inline] fn relative(&self) -> bool { self.src.relative() }
	#[inline] fn set_relative(&mut self, value: bool) { self.src.set_relative(value) }

	#[inline] fn gain(&self) -> f32 { self.src.gain() }
	#[inline] fn set_gain(&mut self, value: f32) -> AltoResult<()> { self.src.set_gain(value) }

	#[inline] fn min_gain(&self) -> f32 { self.src.min_gain() }
	#[inline] fn set_min_gain(&mut self, value: f32) -> AltoResult<()> { self.src.set_min_gain(value) }

	#[inline] fn max_gain(&self) -> f32 { self.src.max_gain() }
	#[inline] fn set_max_gain(&mut self, value: f32) -> AltoResult<()> { self.src.set_max_gain(value) }

	#[inline] fn reference_distance(&self) -> f32 { self.src.reference_distance() }
	#[inline] fn set_reference_distance(&mut self, value: f32) -> AltoResult<()> { self.src.set_reference_distance(value) }

	#[inline] fn rolloff_factor(&self) -> f32 { self.src.rolloff_factor() }
	#[inline] fn set_rolloff_factor(&mut self, value: f32) -> AltoResult<()> { self.src.set_rolloff_factor(value) }

	#[inline] fn max_distance(&self) -> f32 { self.src.max_distance() }
	#[inline] fn set_max_distance(&mut self, value: f32) -> AltoResult<()> { self.src.set_max_distance(value) }

	#[inline] fn pitch(&self) -> f32 { self.src.pitch() }
	#[inline] fn set_pitch(&mut self, value: f32) -> AltoResult<()> { self.src.set_pitch(value) }

	#[inline] fn position<V: From<[f32; 3]>>(&self) -> V { self.src.position() }
	#[inline] fn set_position<V: Into<[f32; 3]>>(&mut self, value: V) -> AltoResult<()> { self.src.set_position(value) }

	#[inline] fn velocity<V: From<[f32; 3]>>(&self) -> V { self.src.velocity() }
	#[inline] fn set_velocity<V: Into<[f32; 3]>>(&mut self, value: V) -> AltoResult<()> { self.src.set_velocity(value) }

	#[inline] fn direction<V: From<[f32; 3]>>(&self) -> V { self.src.direction() }
	#[inline] fn set_direction<V: Into<[f32; 3]>>(&mut self, value: V) -> AltoResult<()> { self.src.set_direction(value) }

	#[inline] fn cone_inner_angle(&self) -> f32 { self.src.cone_inner_angle() }
	#[inline] fn set_cone_inner_angle(&mut self, value: f32) -> AltoResult<()> { self.src.set_cone_inner_angle(value) }

	#[inline] fn cone_outer_angle(&self) -> f32 { self.src.cone_outer_angle() }
	#[inline] fn set_cone_outer_angle(&mut self, value: f32) -> AltoResult<()> { self.src.set_cone_outer_angle(value) }

	#[inline] fn cone_outer_gain(&self) -> f32 { self.src.cone_outer_gain() }
	#[inline] fn set_cone_outer_gain(&mut self, value: f32) -> AltoResult<()> { self.src.set_cone_outer_gain(value) }

	#[inline] fn sec_offset(&self) -> f32 { self.src.sec_offset() }
	#[inline] fn set_sec_offset(&mut self, value: f32) -> AltoResult<()> { self.src.set_sec_offset(value) }

	#[inline] fn sample_offset(&self) -> sys::ALint { self.src.sample_offset() }
	#[inline] fn set_sample_offset(&mut self, value: sys::ALint) -> AltoResult<()> { self.src.set_sample_offset(value) }

	#[inline] fn byte_offset(&self) -> sys::ALint { self.src.byte_offset() }
	#[inline] fn set_byte_offset(&mut self, value: sys::ALint) -> AltoResult<()> { self.src.set_byte_offset(value) }

	#[inline] fn soft_sec_offset_latency(&self) -> AltoResult<(f64, f64)> { self.src.soft_sec_offset_latency() }

	#[inline] fn soft_sample_frac_offset_latency(&self) -> AltoResult<(i32, i32, i64)> { self.src.soft_sample_offset_frac_latency() }

	#[inline] fn soft_sec_length(&self) -> AltoResult<f32> { self.src.soft_sec_length() }

	#[inline] fn soft_sample_length(&self) -> AltoResult<sys::ALint> { self.src.soft_sample_length() }

	#[inline] fn soft_byte_length(&self) -> AltoResult<sys::ALint> { self.src.soft_byte_length() }

	#[inline] fn soft_direct_channels(&self) -> bool { self.src.soft_direct_channels() }
	#[inline] fn set_soft_direct_channels(&mut self, value: bool) -> AltoResult<()> { self.src.set_soft_direct_channels(value) }

	#[inline] fn distance_model(&self) -> DistanceModel { self.src.distance_model() }
	#[inline] fn set_distance_model(&mut self, value: DistanceModel) -> AltoResult<()> { self.src.set_distance_model(value) }

	#[inline] fn soft_spatialization(&self) -> SoftSourceSpatialize { self.src.soft_spatialization() }
	#[inline] fn set_soft_spatialization(&mut self, value: SoftSourceSpatialize) -> AltoResult<()> { self.src.set_soft_spatialization(value) }

	#[inline] fn soft_resampler(&self) -> AltoResult<sys::ALint> { self.src.soft_resampler() }
	#[inline] fn set_soft_resampler(&mut self, value: sys::ALint) -> AltoResult<()> { self.src.set_soft_resampler(value) }

	#[inline] fn stereo_angles<V: From<[f32; 2]>>(&self) -> AltoResult<V> { self.src.stereo_angles() }
	#[inline] fn set_stereo_angles<V: Into<[f32; 2]>>(&mut self, value: V) -> AltoResult<()> { self.src.set_stereo_angles(value) }

	#[inline] fn radius(&self) -> f32 { self.src.radius() }
	#[inline] fn set_radius(&self, value: f32) -> AltoResult<()> { self.src.set_radius(value) }

	#[inline] fn set_direct_filter<F: FilterTrait>(&mut self, value: &F) -> AltoResult<()> { self.src.set_direct_filter(value) }
	#[inline] fn clear_direct_filter(&mut self) { self.src.clear_direct_filter() }

	#[inline] fn set_aux_send(&mut self, send: sys::ALint, slot: &mut AuxEffectSlot) -> AltoResult<()> { SourceInner::set_aux_send(&self.src, send, slot) }
	#[inline] fn set_aux_send_filter<F: FilterTrait>(&mut self, send: sys::ALint, slot: &mut AuxEffectSlot, filter: &F) -> AltoResult<()> { SourceInner::set_aux_send_filter(&self.src, send, slot, filter) }
	#[inline] fn clear_aux_send(&mut self, send: sys::ALint) { self.src.clear_aux_send(send) }

	#[inline] fn air_absorption_factor(&self) -> f32 { self.src.air_absorption_factor() }
	#[inline] fn set_air_absorption_factor(&mut self, value: f32) -> AltoResult<()> { self.src.set_air_absorption_factor(value) }

	#[inline] fn room_rolloff_factor(&self) -> f32 { self.src.room_rolloff_factor() }
	#[inline] fn set_room_rolloff_factor(&mut self, value: f32) -> AltoResult<()> { self.src.set_room_rolloff_factor(value) }

	#[inline] fn cone_outer_gainhf(&self) -> f32 { self.src.cone_outer_gainhf() }
	#[inline] fn set_cone_outer_gainhf(&mut self, value: f32) -> AltoResult<()> { self.src.set_cone_outer_gainhf(value) }

	#[inline] fn direct_filter_gainhf_auto(&self) -> bool { self.src.direct_filter_gainhf_auto() }
	#[inline] fn set_direct_filter_gainhf_auto(&mut self, value: bool) -> AltoResult<()> { self.src.set_direct_filter_gainhf_auto(value) }
}


impl PartialEq for StaticSource {
	fn eq(&self, other: &StaticSource) -> bool {
		self.src == other.src
	}
}
impl Eq for StaticSource { }


impl StreamingSource {
	pub(crate) fn new(ctx: Context) -> AltoResult<StreamingSource> {
		let mut src = 0;
		{
			let _lock = ctx.make_current(true);
			unsafe { ctx.0.dev.0.alto.0.api.alGenSources(1, &mut src as *mut sys::ALuint); }
			ctx.get_error()?;
		}
		let sends = iter::repeat(0).take(ctx.0.dev.0.max_aux_sends() as usize).collect();
		Ok(StreamingSource{src: Arc::new(SourceInner{ctx: ctx, src: src, sends: Mutex::new(sends)}), bufs: VecDeque::new() })
	}


	/// `alGetSourcei(AL_BUFFERS_QUEUED)`
	pub fn buffers_queued(&self) -> sys::ALint {
		self.bufs.len() as sys::ALint
	}


	/// `alGetSourcei(AL_BUFFERS_PROCESSED)`
	pub fn buffers_processed(&self) -> sys::ALint {
		let _lock = self.src.ctx.make_current(true);
		let mut value = 0;
		unsafe { self.src.ctx.0.dev.0.alto.0.api.alGetSourcei(self.src.src, sys::AL_BUFFERS_PROCESSED, &mut value); }
		value
	}


	/// `alSourceQueueBuffers()`
	pub fn queue_buffer(&mut self, buf: Buffer) {
		{
			if buf.ctx.device().as_raw() != self.src.ctx.device().as_raw() {
				panic!("ALTO ERROR: Buffer used on wrong device");
			}
			let _lock = self.src.ctx.make_current(true);

			unsafe { self.src.ctx.0.dev.0.alto.0.api.alSourceQueueBuffers(self.src.src, 1, &buf.buf); }
		}

		self.bufs.push_back(buf);
	}


	/// `alSourceUnqueueBuffers()`
	pub fn unqueue_buffer(&mut self) -> AltoResult<Buffer> {
		{
			let _lock = self.src.ctx.make_current(true);
			let mut buf = 0;
			unsafe { self.src.ctx.0.dev.0.alto.0.api.alSourceUnqueueBuffers(self.src.src, 1, &mut buf); }
			self.src.ctx.get_error()?;
		}

		Ok(self.bufs.pop_front().unwrap())
	}
}


unsafe impl SourceTrait for StreamingSource {
	#[inline] fn context(&self) -> &Context { self.src.context() }
	#[inline] fn as_raw(&self) -> sys::ALuint { self.src.as_raw() }

	#[inline] fn state(&self) -> SourceState { self.src.state() }
	#[inline] fn play(&mut self) -> () { self.src.play() }
	#[inline] fn pause(&mut self) -> () { self.src.pause() }
	#[inline] fn stop(&mut self) -> () { self.src.stop() }
	#[inline] fn rewind(&mut self) -> () { self.src.rewind() }

	#[inline] fn relative(&self) -> bool { self.src.relative() }
	#[inline] fn set_relative(&mut self, value: bool) { self.src.set_relative(value) }

	#[inline] fn gain(&self) -> f32 { self.src.gain() }
	#[inline] fn set_gain(&mut self, value: f32) -> AltoResult<()> { self.src.set_gain(value) }

	#[inline] fn min_gain(&self) -> f32 { self.src.min_gain() }
	#[inline] fn set_min_gain(&mut self, value: f32) -> AltoResult<()> { self.src.set_min_gain(value) }

	#[inline] fn max_gain(&self) -> f32 { self.src.max_gain() }
	#[inline] fn set_max_gain(&mut self, value: f32) -> AltoResult<()> { self.src.set_max_gain(value) }

	#[inline] fn reference_distance(&self) -> f32 { self.src.reference_distance() }
	#[inline] fn set_reference_distance(&mut self, value: f32) -> AltoResult<()> { self.src.set_reference_distance(value) }

	#[inline] fn rolloff_factor(&self) -> f32 { self.src.rolloff_factor() }
	#[inline] fn set_rolloff_factor(&mut self, value: f32) -> AltoResult<()> { self.src.set_rolloff_factor(value) }

	#[inline] fn max_distance(&self) -> f32 { self.src.max_distance() }
	#[inline] fn set_max_distance(&mut self, value: f32) -> AltoResult<()> { self.src.set_max_distance(value) }

	#[inline] fn pitch(&self) -> f32 { self.src.pitch() }
	#[inline] fn set_pitch(&mut self, value: f32) -> AltoResult<()> { self.src.set_pitch(value) }

	#[inline] fn position<V: From<[f32; 3]>>(&self) -> V { self.src.position() }
	#[inline] fn set_position<V: Into<[f32; 3]>>(&mut self, value: V) -> AltoResult<()> { self.src.set_position(value) }

	#[inline] fn velocity<V: From<[f32; 3]>>(&self) -> V { self.src.velocity() }
	#[inline] fn set_velocity<V: Into<[f32; 3]>>(&mut self, value: V) -> AltoResult<()> { self.src.set_velocity(value) }

	#[inline] fn direction<V: From<[f32; 3]>>(&self) -> V { self.src.direction() }
	#[inline] fn set_direction<V: Into<[f32; 3]>>(&mut self, value: V) -> AltoResult<()> { self.src.set_direction(value) }

	#[inline] fn cone_inner_angle(&self) -> f32 { self.src.cone_inner_angle() }
	#[inline] fn set_cone_inner_angle(&mut self, value: f32) -> AltoResult<()> { self.src.set_cone_inner_angle(value) }

	#[inline] fn cone_outer_angle(&self) -> f32 { self.src.cone_outer_angle() }
	#[inline] fn set_cone_outer_angle(&mut self, value: f32) -> AltoResult<()> { self.src.set_cone_outer_angle(value) }

	#[inline] fn cone_outer_gain(&self) -> f32 { self.src.cone_outer_gain() }
	#[inline] fn set_cone_outer_gain(&mut self, value: f32) -> AltoResult<()> { self.src.set_cone_outer_gain(value) }

	#[inline] fn sec_offset(&self) -> f32 { self.src.sec_offset() }
	#[inline] fn set_sec_offset(&mut self, value: f32) -> AltoResult<()> { self.src.set_sec_offset(value) }

	#[inline] fn sample_offset(&self) -> sys::ALint { self.src.sample_offset() }
	#[inline] fn set_sample_offset(&mut self, value: sys::ALint) -> AltoResult<()> { self.src.set_sample_offset(value) }

	#[inline] fn byte_offset(&self) -> sys::ALint { self.src.byte_offset() }
	#[inline] fn set_byte_offset(&mut self, value: sys::ALint) -> AltoResult<()> { self.src.set_byte_offset(value) }

	#[inline] fn soft_sec_offset_latency(&self) -> AltoResult<(f64, f64)> { self.src.soft_sec_offset_latency() }

	#[inline] fn soft_sample_frac_offset_latency(&self) -> AltoResult<(i32, i32, i64)> { self.src.soft_sample_offset_frac_latency() }

	#[inline] fn soft_sec_length(&self) -> AltoResult<f32> { self.src.soft_sec_length() }

	#[inline] fn soft_sample_length(&self) -> AltoResult<sys::ALint> { self.src.soft_sample_length() }

	#[inline] fn soft_byte_length(&self) -> AltoResult<sys::ALint> { self.src.soft_byte_length() }

	#[inline] fn soft_direct_channels(&self) -> bool { self.src.soft_direct_channels() }
	#[inline] fn set_soft_direct_channels(&mut self, value: bool) -> AltoResult<()> { self.src.set_soft_direct_channels(value) }

	#[inline] fn distance_model(&self) -> DistanceModel { self.src.distance_model() }
	#[inline] fn set_distance_model(&mut self, value: DistanceModel) -> AltoResult<()> { self.src.set_distance_model(value) }

	#[inline] fn soft_spatialization(&self) -> SoftSourceSpatialize { self.src.soft_spatialization() }
	#[inline] fn set_soft_spatialization(&mut self, value: SoftSourceSpatialize) -> AltoResult<()> { self.src.set_soft_spatialization(value) }

	#[inline] fn soft_resampler(&self) -> AltoResult<sys::ALint> { self.src.soft_resampler() }
	#[inline] fn set_soft_resampler(&mut self, value: sys::ALint) -> AltoResult<()> { self.src.set_soft_resampler(value) }

	#[inline] fn stereo_angles<V: From<[f32; 2]>>(&self) -> AltoResult<V> { self.src.stereo_angles() }
	#[inline] fn set_stereo_angles<V: Into<[f32; 2]>>(&mut self, value: V) -> AltoResult<()> { self.src.set_stereo_angles(value) }

	#[inline] fn radius(&self) -> f32 { self.src.radius() }
	#[inline] fn set_radius(&self, value: f32) -> AltoResult<()> { self.src.set_radius(value) }

	#[inline] fn set_direct_filter<F: FilterTrait>(&mut self, value: &F) -> AltoResult<()> { self.src.set_direct_filter(value) }
	#[inline] fn clear_direct_filter(&mut self) { self.src.clear_direct_filter() }

	#[inline] fn set_aux_send(&mut self, send: sys::ALint, slot: &mut AuxEffectSlot) -> AltoResult<()> { SourceInner::set_aux_send(&self.src, send, slot) }
	#[inline] fn set_aux_send_filter<F: FilterTrait>(&mut self, send: sys::ALint, slot: &mut AuxEffectSlot, filter: &F) -> AltoResult<()> { SourceInner::set_aux_send_filter(&self.src, send, slot, filter) }
	#[inline] fn clear_aux_send(&mut self, send: sys::ALint) { self.src.clear_aux_send(send) }

	#[inline] fn air_absorption_factor(&self) -> f32 { self.src.air_absorption_factor() }
	#[inline] fn set_air_absorption_factor(&mut self, value: f32) -> AltoResult<()> { self.src.set_air_absorption_factor(value) }

	#[inline] fn room_rolloff_factor(&self) -> f32 { self.src.room_rolloff_factor() }
	#[inline] fn set_room_rolloff_factor(&mut self, value: f32) -> AltoResult<()> { self.src.set_room_rolloff_factor(value) }

	#[inline] fn cone_outer_gainhf(&self) -> f32 { self.src.cone_outer_gainhf() }
	#[inline] fn set_cone_outer_gainhf(&mut self, value: f32) -> AltoResult<()> { self.src.set_cone_outer_gainhf(value) }

	#[inline] fn direct_filter_gainhf_auto(&self) -> bool { self.src.direct_filter_gainhf_auto() }
	#[inline] fn set_direct_filter_gainhf_auto(&mut self, value: bool) -> AltoResult<()> { self.src.set_direct_filter_gainhf_auto(value) }
}


impl PartialEq for StreamingSource {
	fn eq(&self, other: &StreamingSource) -> bool {
		self.src == other.src
	}
}
impl Eq for StreamingSource { }
