use std::mem;
use std::sync::RwLock;
use std::ptr;

use rental::{self, RentRwLock};
use sys::*;


macro_rules! alc_ext {
	{
		pub cache $cache:ident;


		$(pub ext $ext:ident {
			$(pub const $const_:ident,)*
			$(pub fn $fn_:ident: $fn_ty:ty,)*
		})*
	} => {
		#[doc(hidden)]
		#[allow(non_snake_case)]
		pub struct $cache<'a> {
			api: &'a AlApi,
			dev: *mut ALCdevice,
			$($ext: RwLock<Option<AlcExtResult<$ext>>>,)*
		}


		#[allow(non_snake_case)]
		impl<'a> $cache<'a> {
			pub unsafe fn new(api: &'a AlApi, dev: *mut ALCdevice) -> $cache<'a> {
				$cache{
					api: api,
					dev: dev,
					$($ext: RwLock::new(None),)*
				}
			}


			$(pub fn $ext(&self) -> AlcExtResult<RentRwLock<Option<AlcExtResult<$ext>>, $ext>> {
				if let Ok(mut ext) = self.$ext.try_write() {
					if ext.is_none() {
						*ext = Some($ext::load(&self.api, self.dev));
					}
				}

				let ext = RentRwLock::new(self.$ext.read().unwrap(), |ext| ext.as_ref().unwrap());
				match *ext {
					Ok(_) => Ok(rental::MapRef::map(ext, |ext| ext.as_ref().unwrap())),
					Err(e) => Err(e),
				}
			})*
		}


		unsafe impl<'a> Send for $cache<'a> { }
		unsafe impl<'a> Sync for $cache<'a> { }


		$(#[doc(hidden)]
		#[allow(non_camel_case_types, non_snake_case)]
		#[derive(Debug)]
		pub struct $ext {
			$(pub $const_: AlcExtResult<ALCenum>,)*
			$(pub $fn_: AlcExtResult<$fn_ty>,)*
		}


		impl $ext {
			pub fn load(api: &AlApi, dev: *mut ALCdevice) -> AlcExtResult<$ext> {
				unsafe { api.alcGetError()(dev); }
				if unsafe { api.alcIsExtensionPresent()(dev, concat!(stringify!($ext), "\0").as_bytes().as_ptr() as *const ALCchar) } == ALC_TRUE {
					Ok($ext{
						$($const_: {
							let e = unsafe { api.alcGetEnumValue()(dev, concat!(stringify!($const_), "\0").as_bytes().as_ptr() as *const ALCchar) };
							if e != 0 && unsafe { api.alcGetError()(dev) } == ALC_NO_ERROR {
								Ok(e)
							} else {
								// Workaround for missing symbols in OpenAL-Soft
								match stringify!($const_) {
									"AL_EFFECTSLOT_EFFECT" => Ok(1),
									"AL_EFFECTSLOT_GAIN" => Ok(2),
									"AL_EFFECTSLOT_AUXILIARY_SEND_AUTO" => Ok(3),
									_ => Err(AlcExtensionError),
								}
							}
						},)*
						$($fn_: {
							let p = unsafe { api.alcGetProcAddress()(dev, concat!(stringify!($fn_), "\0").as_bytes().as_ptr() as *const ALCchar) };
							if p != ptr::null_mut() && unsafe { api.alcGetError()(dev) } == ALC_NO_ERROR {
								Ok(unsafe { mem::transmute(p) })
							} else {
								Err(AlcExtensionError)
							}
						},)*
					})
				} else {
					Err(AlcExtensionError)
				}
			}
		})*
	};
}


macro_rules! al_ext {
	{
		pub cache $cache:ident;


		$(pub ext $ext:ident {
			$(pub const $const_:ident,)*
			$(pub fn $fn_:ident: $fn_ty:ty,)*
		})*
	} => {
		#[doc(hidden)]
		#[allow(non_snake_case)]
		pub struct $cache<'a> {
			api: &'a AlApi,
			$($ext: RwLock<Option<AlExtResult<$ext>>>,)*
		}


		#[allow(non_snake_case)]
		impl<'a> $cache<'a> {
			pub unsafe fn new(api: &'a AlApi) -> $cache<'a> {
				$cache{
					api: api,
					$($ext: RwLock::new(None),)*
				}
			}


			$(pub fn $ext(&self) -> AlExtResult<RentRwLock<Option<AlExtResult<$ext>>, $ext>> {
				if let Ok(mut ext) = self.$ext.try_write() {
					if ext.is_none() {
						*ext = Some($ext::load(&self.api));
					}
				}

				let ext = RentRwLock::new(self.$ext.read().unwrap(), |ext| ext.as_ref().unwrap());
				match *ext {
					Ok(_) => Ok(rental::MapRef::map(ext, |ext| ext.as_ref().unwrap())),
					Err(e) => Err(e),
				}
			})*
		}


		unsafe impl<'a> Send for $cache<'a> { }


		$(#[doc(hidden)]
		#[allow(non_camel_case_types, non_snake_case)]
		#[derive(Debug)]
		pub struct $ext {
			$(pub $const_: AlExtResult<ALenum>,)*
			$(pub $fn_: AlExtResult<$fn_ty>,)*
		}


		impl $ext {
			pub fn load(api: &AlApi) -> AlExtResult<$ext> {
				unsafe { api.alGetError()(); }
				if unsafe { api.alIsExtensionPresent()(concat!(stringify!($ext), "\0").as_bytes().as_ptr() as *const ALchar) } == AL_TRUE {
					Ok($ext{
						$($const_: {
							let e = unsafe { api.alGetEnumValue()(concat!(stringify!($const_), "\0").as_bytes().as_ptr() as *const ALchar) };
							if e != 0 && unsafe { api.alGetError()() } == AL_NO_ERROR {
								Ok(e)
							} else {
								Err(AlExtensionError)
							}
						},)*
						$($fn_: {
							let p = unsafe { api.alGetProcAddress()(concat!(stringify!($fn_), "\0").as_bytes().as_ptr() as *const ALchar) };
							if p != ptr::null_mut() && unsafe { api.alGetError()() } == AL_NO_ERROR {
								Ok(unsafe { mem::transmute(p) })
							} else {
								Err(AlExtensionError)
							}
						},)*
					})
				} else {
					Err(AlExtensionError)
				}
			}
		})*
	};
}


#[doc(hidden)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AlcExtensionError;
#[doc(hidden)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AlExtensionError;


#[doc(hidden)]
pub type AlcExtResult<T> = ::std::result::Result<T, AlcExtensionError>;
#[doc(hidden)]
pub type AlExtResult<T> = ::std::result::Result<T, AlExtensionError>;


#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug)]
pub enum AlcNull {
	/// `ALC_ENUMERATE_ALL_EXT`
	EnumerateAll,
	/// `ALC_SOFT_loopback`
	SoftLoopback,
	/// `ALC_EXT_thread_local_context`
	ThreadLocalContext,
}


#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug)]
pub enum Alc {
	/// `ALC_EXT_DEDICATED`
	Dedicated,
	/// `ALC_EXT_disconnect`
	Disconnect,
	/// `ALC_EXT_EFX`
	Efx,
	/// `ALC_SOFT_HRTF`
	SoftHrtf,
	/// `ALC_SOFT_pause_device`
	SoftPauseDevice,
}


#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug)]
pub enum Al {
	/// `AL_EXT_ALAW`
	ALaw,
	/// `AL_EXT_BFORMAT`
	BFormat,
	/// `AL_EXT_double`
	Double,
	/// `AL_EXT_float32`
	Float32,
	/// `AL_EXT_IMA4`
	Ima4,
	/// `AL_EXT_MCFORMATS`
	McFormats,
	/// `AL_EXT_MULAW`
	MuLaw,
	/// `AL_EXT_MULAW_BFORMAT`
	MuLawBFormat,
	/// `AL_EXT_MULAW_MCFORMATS`
	MuLawMcFormats,
	/// `AL_SOFT_block_alignment`
	SoftBlockAlignment,
//	SoftBufferSamples,
//	SoftBufferSubData,
	/// `AL_SOFT_deferred_updates`
	SoftDeferredUpdates,
	/// `AL_SOFT_direct_channels`
	SoftDirectChannels,
	/// `AL_SOFT_loop_points`
	SoftLoopPoints,
	/// `AL_SOFT_MSADPCM`
	SoftMsadpcm,
	/// `AL_SOFT_source_latency`
	SoftSourceLatency,
	/// `AL_SOFT_source_length`
	SoftSourceLength,
	/// `AL_EXT_source_distance_model`
	SourceDistanceModel,
}


alc_ext! {
	pub cache AlcNullCache;


	pub ext ALC_ENUMERATE_ALL_EXT {
		pub const ALC_ALL_DEVICES_SPECIFIER,
		pub const ALC_DEFAULT_ALL_DEVICES_SPECIFIER,
	}


	pub ext ALC_SOFT_loopback {
		pub const ALC_BYTE_SOFT,
		pub const ALC_UNSIGNED_BYTE_SOFT,
		pub const ALC_SHORT_SOFT,
		pub const ALC_UNSIGNED_SHORT_SOFT,
		pub const ALC_INT_SOFT,
		pub const ALC_UNSIGNED_INT_SOFT,
		pub const ALC_FLOAT_SOFT,
		pub const ALC_MONO_SOFT,
		pub const ALC_STEREO_SOFT,
		pub const ALC_QUAD_SOFT,
		pub const ALC_5POINT1_SOFT,
		pub const ALC_6POINT1_SOFT,
		pub const ALC_7POINT1_SOFT,
		pub const ALC_FORMAT_CHANNELS_SOFT,
		pub const ALC_FORMAT_TYPE_SOFT,

		pub fn alcLoopbackOpenDeviceSOFT: unsafe extern "C" fn(deviceName: *const ALCchar) -> *mut ALCdevice,
		pub fn alcIsRenderFormatSupportedSOFT: unsafe extern "C" fn(device: *mut ALCdevice, frequency: ALCsizei, channels: ALCenum, type_: ALCenum) -> ALCboolean,
		pub fn alcRenderSamplesSOFT: unsafe extern "C" fn(device: *mut ALCdevice, buffer: *mut ALvoid, samples: ALCsizei),
	}


	pub ext ALC_EXT_thread_local_context {
		pub fn alcSetThreadContext: unsafe extern "C" fn(ctx: *mut ALCcontext) -> ALCboolean,
		pub fn alcGetThreadContext: unsafe extern "C" fn() -> *mut ALCcontext,
	}
}


alc_ext! {
	pub cache AlcCache;


	pub ext ALC_EXT_DEDICATED {
		pub const AL_EFFECT_DEDICATED_LOW_FREQUENCY_EFFECT,
		pub const AL_EFFECT_DEDICATED_DIALOGUE,
		pub const AL_EFFECT_DEDICATED_GAIN,
	}


	pub ext ALC_EXT_DISCONNECT {
		pub const ALC_CONNECTED,
	}


	pub ext ALC_EXT_EFX {
		pub const AL_EFFECTSLOT_EFFECT,
		pub const AL_EFFECTSLOT_GAIN,
		pub const AL_EFFECTSLOT_AUXILIARY_SEND_AUTO,

		pub const AL_EFFECT_TYPE,
		pub const AL_EFFECT_EAXREVERB,
		pub const AL_EAXREVERB_DENSITY,
		pub const AL_EAXREVERB_DIFFUSION,
		pub const AL_EAXREVERB_GAIN,
		pub const AL_EAXREVERB_GAINHF,
		pub const AL_EAXREVERB_GAINLF,
		pub const AL_EAXREVERB_DECAY_TIME,
		pub const AL_EAXREVERB_DECAY_HFRATIO,
		pub const AL_EAXREVERB_DECAY_LFRATIO,
		pub const AL_EAXREVERB_REFLECTIONS_GAIN,
		pub const AL_EAXREVERB_REFLECTIONS_DELAY,
		pub const AL_EAXREVERB_REFLECTIONS_PAN,
		pub const AL_EAXREVERB_LATE_REVERB_GAIN,
		pub const AL_EAXREVERB_LATE_REVERB_DELAY,
		pub const AL_EAXREVERB_LATE_REVERB_PAN,
		pub const AL_EAXREVERB_ECHO_TIME,
		pub const AL_EAXREVERB_ECHO_DEPTH,
		pub const AL_EAXREVERB_MODULATION_TIME,
		pub const AL_EAXREVERB_MODULATION_DEPTH,
		pub const AL_EAXREVERB_AIR_ABSORPTION_GAINHF,
		pub const AL_EAXREVERB_HFREFERENCE,
		pub const AL_EAXREVERB_LFREFERENCE,
		pub const AL_EAXREVERB_ROOM_ROLLOFF_FACTOR,
		pub const AL_EAXREVERB_DECAY_HFLIMIT,
		pub const AL_EFFECT_REVERB,
		pub const AL_REVERB_DENSITY,
		pub const AL_REVERB_DIFFUSION,
		pub const AL_REVERB_GAIN,
		pub const AL_REVERB_GAINHF,
		pub const AL_REVERB_DECAY_TIME,
		pub const AL_REVERB_DECAY_HFRATIO,
		pub const AL_REVERB_REFLECTIONS_GAIN,
		pub const AL_REVERB_REFLECTIONS_DELAY,
		pub const AL_REVERB_LATE_REVERB_GAIN,
		pub const AL_REVERB_LATE_REVERB_DELAY,
		pub const AL_REVERB_AIR_ABSORPTION_GAINHF,
		pub const AL_REVERB_ROOM_ROLLOFF_FACTOR,
		pub const AL_REVERB_DECAY_HFLIMIT,
		pub const AL_EFFECT_CHORUS,
		pub const AL_CHORUS_WAVEFORM,
		pub const AL_CHORUS_PHASE,
		pub const AL_CHORUS_RATE,
		pub const AL_CHORUS_DEPTH,
		pub const AL_CHORUS_FEEDBACK,
		pub const AL_CHORUS_DELAY,
		pub const AL_EFFECT_DISTORTION,
		pub const AL_DISTORTION_EDGE,
		pub const AL_DISTORTION_GAIN,
		pub const AL_DISTORTION_LOWPASS_CUTOFF,
		pub const AL_DISTORTION_EQCENTER,
		pub const AL_DISTORTION_EQBANDWIDTH,
		pub const AL_EFFECT_ECHO,
		pub const AL_ECHO_DELAY,
		pub const AL_ECHO_LRDELAY,
		pub const AL_ECHO_DAMPING,
		pub const AL_ECHO_FEEDBACK,
		pub const AL_ECHO_SPREAD,
		pub const AL_EFFECT_FLANGER,
		pub const AL_FLANGER_WAVEFORM,
		pub const AL_FLANGER_PHASE,
		pub const AL_FLANGER_RATE,
		pub const AL_FLANGER_DEPTH,
		pub const AL_FLANGER_FEEDBACK,
		pub const AL_FLANGER_DELAY,
		pub const AL_EFFECT_FREQUENCY_SHIFTER,
		pub const AL_FREQUENCY_SHIFTER_FREQUENCY,
		pub const AL_FREQUENCY_SHIFTER_LEFT_DIRECTION,
		pub const AL_FREQUENCY_SHIFTER_RIGHT_DIRECTION,
		pub const AL_EFFECT_VOCAL_MORPHER,
		pub const AL_VOCAL_MORPHER_PHONEMEA,
		pub const AL_VOCAL_MORPHER_PHONEMEB,
		pub const AL_VOCAL_MORPHER_PHONEMEA_COARSE_TUNING,
		pub const AL_VOCAL_MORPHER_PHONEMEB_COARSE_TUNING,
		pub const AL_VOCAL_MORPHER_WAVEFORM,
		pub const AL_VOCAL_MORPHER_RATE,
		pub const AL_EFFECT_PITCH_SHIFTER,
		pub const AL_PITCH_SHIFTER_COARSE_TUNE,
		pub const AL_PITCH_SHIFTER_FINE_TUNE,
		pub const AL_EFFECT_RING_MODULATOR,
		pub const AL_RING_MODULATOR_FREQUENCY,
		pub const AL_RING_MODULATOR_HIGHPASS_CUTOFF,
		pub const AL_RING_MODULATOR_WAVEFORM,
		pub const AL_EFFECT_AUTOWAH,
		pub const AL_AUTOWAH_ATTACK_TIME,
		pub const AL_AUTOWAH_RELEASE_TIME,
		pub const AL_AUTOWAH_RESONANCE,
		pub const AL_AUTOWAH_PEAK_GAIN,
		pub const AL_EFFECT_COMPRESSOR,
		pub const AL_COMPRESSOR_ONOFF,
		pub const AL_EFFECT_EQUALIZER,
		pub const AL_EQUALIZER_LOW_GAIN,
		pub const AL_EQUALIZER_LOW_CUTOFF,
		pub const AL_EQUALIZER_MID1_GAIN,
		pub const AL_EQUALIZER_MID1_CENTER,
		pub const AL_EQUALIZER_MID1_WIDTH,
		pub const AL_EQUALIZER_MID2_GAIN,
		pub const AL_EQUALIZER_MID2_CENTER,
		pub const AL_EQUALIZER_MID2_WIDTH,
		pub const AL_EQUALIZER_HIGH_GAIN,
		pub const AL_EQUALIZER_HIGH_CUTOFF,

		pub const AL_FILTER_TYPE,
		pub const AL_FILTER_LOWPASS,
		pub const AL_LOWPASS_GAIN,
		pub const AL_LOWPASS_GAINHF,
		pub const AL_FILTER_HIGHPASS,
		pub const AL_HIGHPASS_GAIN,
		pub const AL_HIGHPASS_GAINLF,
		pub const AL_FILTER_BANDPASS,
		pub const AL_BANDPASS_GAIN,
		pub const AL_BANDPASS_GAINLF,
		pub const AL_BANDPASS_GAINHF,

		pub const AL_DIRECT_FILTER,
		pub const AL_AUXILIARY_SEND_FILTER,
		pub const AL_AIR_ABSORPTION_FACTOR,
		pub const AL_ROOM_ROLLOFF_FACTOR,
		pub const AL_CONE_OUTER_GAINHF,
		pub const AL_DIRECT_FILTER_GAINHF_AUTO,
		pub const AL_AUXILIARY_SEND_FILTER_GAIN_AUTO,
		pub const AL_AUXILIARY_SEND_FILTER_GAINHF_AUTO,

		pub const AL_METERS_PER_UNIT,

		pub const ALC_EFX_MAJOR_VERSION,
		pub const ALC_EFX_MINOR_VERSION,
		pub const ALC_MAX_AUXILIARY_SENDS,


		pub fn alGenAuxiliaryEffectSlots: unsafe extern "C" fn(n: ALsizei, auxiliaryeffectslots: *mut ALuint),
		pub fn alDeleteAuxiliaryEffectSlots: unsafe extern "C" fn(n: ALsizei, auxiliaryeffectslots: *mut ALuint),
		pub fn alIsAuxiliaryEffectSlot: unsafe extern "C" fn(auxiliaryeffectslot: ALuint),
		pub fn alAuxiliaryEffectSloti: unsafe extern "C" fn(auxiliaryeffectslot: ALuint, param: ALenum, iValue: ALint),
		pub fn alAuxiliaryEffectSlotiv: unsafe extern "C" fn(auxiliaryeffectslot: ALuint, param: ALenum, piValues: *mut ALint),
		pub fn alAuxiliaryEffectSlotf: unsafe extern "C" fn(auxiliaryeffectslot: ALuint, param: ALenum, flValue: ALfloat),
		pub fn alAuxiliaryEffectSlotfv: unsafe extern "C" fn(auxiliaryeffectslot: ALuint, param: ALenum, pflValues: *mut ALfloat),
		pub fn alGetAuxiliaryEffectSloti: unsafe extern "C" fn(auxiliaryeffectslot: ALuint, param: ALenum, piValue: *mut ALint),
		pub fn alGetAuxiliaryEffectSlotiv: unsafe extern "C" fn(auxiliaryeffectslot: ALuint, param: ALenum, piValues: *mut ALint),
		pub fn alGetAuxiliaryEffectSlotf: unsafe extern "C" fn(auxiliaryeffectslot: ALuint, param: ALenum, pflValue: *mut ALfloat),
		pub fn alGetAuxiliaryEffectSlotfv: unsafe extern "C" fn(auxiliaryeffectslot: ALuint, param: ALenum, pflValues: *mut ALfloat),

		pub fn alGenEffects: unsafe extern "C" fn(n: ALsizei, effects: *mut ALuint),
		pub fn alDeleteEffects: unsafe extern "C" fn(n: ALsizei, effects: *mut ALuint),
		pub fn alIsEffect: unsafe extern "C" fn(effect: ALuint),
		pub fn alEffecti: unsafe extern "C" fn(effect: ALuint, param: ALenum, iValue: ALint),
		pub fn alEffectiv: unsafe extern "C" fn(effect: ALuint, param: ALenum, piValues: *mut ALint),
		pub fn alEffectf: unsafe extern "C" fn(effect: ALuint, param: ALenum, flValue: ALfloat),
		pub fn alEffectfv: unsafe extern "C" fn(effect: ALuint, param: ALenum, pflValues: *mut ALfloat),
		pub fn alGetEffecti: unsafe extern "C" fn(effect: ALuint, param: ALenum, piValue: *mut ALint),
		pub fn alGetEffectiv: unsafe extern "C" fn(effect: ALuint, param: ALenum, piValues: *mut ALint),
		pub fn alGetEffectf: unsafe extern "C" fn(effect: ALuint, param: ALenum, pflValue: *mut ALfloat),
		pub fn alGetEffectfv: unsafe extern "C" fn(effect: ALuint, param: ALenum, pflValues: *mut ALfloat),

		pub fn alGenFilters: unsafe extern "C" fn(n: ALsizei, filters: *mut ALuint),
		pub fn alDeleteFilters: unsafe extern "C" fn(n: ALsizei, filters: *mut ALuint),
		pub fn alIsFilter: unsafe extern "C" fn(filter: ALuint),
		pub fn alFilteri: unsafe extern "C" fn(filter: ALuint, param: ALenum, iValue: ALint),
		pub fn alFilteriv: unsafe extern "C" fn(filter: ALuint, param: ALenum, piValues: *mut ALint),
		pub fn alFilterf: unsafe extern "C" fn(filter: ALuint, param: ALenum, flValue: ALfloat),
		pub fn alFilterfv: unsafe extern "C" fn(filter: ALuint, param: ALenum, pflValues: *mut ALfloat),
		pub fn alGetFilteri: unsafe extern "C" fn(filter: ALuint, param: ALenum, piValue: *mut ALint),
		pub fn alGetFilteriv: unsafe extern "C" fn(filter: ALuint, param: ALenum, piValues: *mut ALint),
		pub fn alGetFilterf: unsafe extern "C" fn(filter: ALuint, param: ALenum, pflValue: *mut ALfloat),
		pub fn alGetFilterfv: unsafe extern "C" fn(filter: ALuint, param: ALenum, pflValues: *mut ALfloat),
	}


	pub ext ALC_SOFT_HRTF {
		pub const ALC_HRTF_SOFT,
		pub const ALC_HRTF_ID_SOFT,
		pub const ALC_DONT_CARE_SOFT,
		pub const ALC_HRTF_STATUS_SOFT,
		pub const ALC_NUM_HRTF_SPECIFIERS_SOFT,
		pub const ALC_HRTF_SPECIFIER_SOFT,
		pub const ALC_HRTF_DISABLED_SOFT,
		pub const ALC_HRTF_ENABLED_SOFT,
		pub const ALC_HRTF_DENIED_SOFT,
		pub const ALC_HRTF_REQUIRED_SOFT,
		pub const ALC_HRTF_HEADPHONES_DETECTED_SOFT,
		pub const ALC_HRTF_UNSUPPORTED_FORMAT_SOFT,

		pub fn alcGetStringiSOFT: unsafe extern "C" fn(dev: *mut ALCdevice, paramName: ALCenum, index: ALCsizei) -> *const ALCchar,
		pub fn alcResetDeviceSOFT: unsafe extern "C" fn(dev: *mut ALCdevice, attrList: *const ALCint) -> ALCboolean,
	}


	pub ext ALC_SOFT_pause_device {
		pub fn alcDevicePauseSOFT: unsafe extern "C" fn(dev: *mut ALCdevice),
		pub fn alcDeviceResumeSOFT: unsafe extern "C" fn(dev: *mut ALCdevice),
	}
}


pub type ALint64SOFT = i64;
pub type ALuint64SOFT = u64;


al_ext! {
	pub cache AlCache;


	pub ext AL_EXT_ALAW {
		pub const AL_FORMAT_MONO_ALAW_EXT,
		pub const AL_FORMAT_STEREO_ALAW_EXT,
	}


	pub ext AL_EXT_BFORMAT {
		pub const AL_FORMAT_BFORMAT2D_8,
		pub const AL_FORMAT_BFORMAT2D_16,
		pub const AL_FORMAT_BFORMAT2D_FLOAT32,
		pub const AL_FORMAT_BFORMAT3D_8,
		pub const AL_FORMAT_BFORMAT3D_16,
		pub const AL_FORMAT_BFORMAT3D_FLOAT32,
	}


	pub ext AL_EXT_double {
		pub const AL_FORMAT_MONO_DOUBLE_EXT,
		pub const AL_FORMAT_STEREO_DOUBLE_EXT,
	}


	pub ext AL_EXT_float32 {
		pub const AL_FORMAT_MONO_FLOAT32,
		pub const AL_FORMAT_STEREO_FLOAT32,
	}


	pub ext AL_EXT_IMA4 {
		pub const AL_FORMAT_MONO_IMA4,
		pub const AL_FORMAT_STEREO_IMA4,
	}


	pub ext AL_EXT_MCFORMATS {
		pub const AL_FORMAT_QUAD8,
		pub const AL_FORMAT_QUAD16,
		pub const AL_FORMAT_QUAD32,
		pub const AL_FORMAT_REAR8,
		pub const AL_FORMAT_REAR16,
		pub const AL_FORMAT_REAR32,
		pub const AL_FORMAT_51CHN8,
		pub const AL_FORMAT_51CHN16,
		pub const AL_FORMAT_51CHN32,
		pub const AL_FORMAT_61CHN8,
		pub const AL_FORMAT_61CHN16,
		pub const AL_FORMAT_61CHN32,
		pub const AL_FORMAT_71CHN8,
		pub const AL_FORMAT_71CHN16,
		pub const AL_FORMAT_71CHN32,
	}


	pub ext AL_EXT_MULAW {
		pub const AL_FORMAT_MONO_MULAW_EXT,
		pub const AL_FORMAT_STEREO_MULAW_EXT,
	}


	pub ext AL_EXT_MULAW_BFORMAT {
		pub const AL_FORMAT_BFORMAT2D_MULAW,
		pub const AL_FORMAT_BFORMAT3D_MULAW,
	}


	pub ext AL_EXT_MULAW_MCFORMATS {
		pub const AL_FORMAT_MONO_MULAW,
		pub const AL_FORMAT_STEREO_MULAW,
		pub const AL_FORMAT_QUAD_MULAW,
		pub const AL_FORMAT_REAR_MULAW,
		pub const AL_FORMAT_51CHN_MULAW,
		pub const AL_FORMAT_61CHN_MULAW,
		pub const AL_FORMAT_71CHN_MULAW,
	}


	pub ext AL_SOFT_block_alignment {
		pub const AL_UNPACK_BLOCK_ALIGNMENT_SOFT,
		pub const AL_PACK_BLOCK_ALIGNMENT_SOFT,
	}


//	pub ext AL_SOFT_buffer_samples {
//		pub const AL_MONO_SOFT,
//		pub const AL_STEREO_SOFT,
//		pub const AL_REAR_SOFT,
//		pub const AL_QUAD_SOFT,
//		pub const AL_5POINT1_SOFT,
//		pub const AL_6POINT1_SOFT,
//		pub const AL_7POINT1_SOFT,
//
//		pub const AL_BYTE_SOFT,
//		pub const AL_UNSIGNED_BYTE_SOFT,
//		pub const AL_SHORT_SOFT,
//		pub const AL_UNSIGNED_SHORT_SOFT,
//		pub const AL_INT_SOFT,
//		pub const AL_UNSIGNED_INT_SOFT,
//		pub const AL_FLOAT_SOFT,
//		pub const AL_DOUBLE_SOFT,
//		pub const AL_BYTE3_SOFT,
//		pub const AL_UNSIGNED_BYTE3_SOFT,
//
//		pub const AL_MONO8_SOFT,
//		pub const AL_MONO16_SOFT,
//		pub const AL_MONO32F_SOFT,
//		pub const AL_STEREO8_SOFT,
//		pub const AL_STEREO16_SOFT,
//		pub const AL_STEREO32F_SOFT,
//		pub const AL_QUAD8_SOFT,
//		pub const AL_QUAD16_SOFT,
//		pub const AL_QUAD32F_SOFT,
//		pub const AL_REAR8_SOFT,
//		pub const AL_REAR16_SOFT,
//		pub const AL_REAR32F_SOFT,
//		pub const AL_5POINT1_8_SOFT,
//		pub const AL_5POINT1_16_SOFT,
//		pub const AL_5POINT1_32F_SOFT,
//		pub const AL_6POINT1_8_SOFT,
//		pub const AL_6POINT1_16_SOFT,
//		pub const AL_6POINT1_32F_SOFT,
//		pub const AL_7POINT1_8_SOFT,
//		pub const AL_7POINT1_16_SOFT,
//		pub const AL_7POINT1_32F_SOFT,
//
//		pub const AL_INTERNAL_FORMAT_SOFT,
//		pub const AL_BYTE_LENGTH_SOFT,
//		pub const AL_SAMPLE_LENGTH_SOFT,
//		pub const AL_SEC_LENGTH_SOFT,
//
//		pub fn alBufferSamplesSOFT: unsafe extern "C" fn(buffer: ALuint, samplerate: ALuint, internalformat: ALenum, samples: ALsizei, channels: ALenum, type_: ALenum, data: *const ALvoid),
//		pub fn alBufferSubSamplesSOFT: unsafe extern "C" fn(buffer: ALuint, offset: ALsizei, samples: ALsizei, channels: ALenum, type_: ALenum, data: *const ALvoid),
//		pub fn alGetBufferSamplesSOFT: unsafe extern "C" fn(buffer: ALuint, offset: ALsizei, samples: ALsizei, channels: ALenum, type_: ALenum, data: *mut ALvoid),
//		pub fn alIsBufferFormatSupportedSOFT: unsafe extern "C" fn(format: ALenum) -> ALboolean,
//	}
//
//
//	pub ext AL_SOFT_buffer_sub_data {
//		pub const AL_BYTE_RW_OFFSETS_SOFT,
//		pub const AL_SAMPLE_RW_OFFSETS_SOFT,
//
//		pub fn alBufferSubDataSOFT: unsafe extern "C" fn(buffer: ALuint, format: ALenum, data: *const ALvoid, offset: ALsizei, length: ALsizei),
//	}


	pub ext AL_SOFT_deferred_updates {
		pub const AL_DEFERRED_UPDATES_SOFT,

		pub fn alDeferUpdatesSOFT: unsafe extern "C" fn(),
		pub fn alProcessUpdatesSOFT: unsafe extern "C" fn(),
	}


	pub ext AL_SOFT_direct_channels {
		pub const AL_DIRECT_CHANNELS_SOFT,
	}


	pub ext AL_SOFT_loop_points {
		pub const AL_LOOP_POINTS_SOFT,
	}


	pub ext AL_SOFT_MSADPCM {
		pub const AL_FORMAT_MONO_MSADPCM_SOFT,
		pub const AL_FORMAT_STEREO_MSADPCM_SOFT,
	}


	pub ext AL_SOFT_source_latency {
		pub const AL_SAMPLE_OFFSET_LATENCY_SOFT,
		pub const AL_SEC_OFFSET_LATENCY_SOFT,

		pub fn alSourcedSOFT: unsafe extern "C" fn(source: ALuint, param: ALenum, value: ALdouble),
		pub fn alSource3dSOFT: unsafe extern "C" fn(source: ALuint, param: ALenum, value1: ALdouble, value2: ALdouble, value3: ALdouble),
		pub fn alSourcedvSOFT: unsafe extern "C" fn(source: ALuint, param: ALenum, values: *const ALdouble),
		pub fn alGetSourcedSOFT: unsafe extern "C" fn(source: ALuint, param: ALenum, value: *mut ALdouble),
		pub fn alGetSource3dSOFT: unsafe extern "C" fn(source: ALuint, param: ALenum, value1: *mut ALdouble, value2: *mut ALdouble, value3: *mut ALdouble),
		pub fn alGetSourcedvSOFT: unsafe extern "C" fn(source: ALuint, param: ALenum, values: *mut ALdouble),
		pub fn alSourcei64SOFT: unsafe extern "C" fn(source: ALuint, param: ALenum, value: ALint64SOFT),
		pub fn alSource3i64SOFT: unsafe extern "C" fn(source: ALuint, param: ALenum, value1: ALint64SOFT, value2: ALint64SOFT, value3: ALint64SOFT),
		pub fn alSourcei64vSOFT: unsafe extern "C" fn(source: ALuint, param: ALenum, values: *const ALint64SOFT),
		pub fn alGetSourcei64SOFT: unsafe extern "C" fn(source: ALuint, param: ALenum, value: *mut ALint64SOFT),
		pub fn alGetSource3i64SOFT: unsafe extern "C" fn(source: ALuint, param: ALenum, value1: *mut ALint64SOFT, value2: *mut ALint64SOFT, value3: *mut ALint64SOFT),
		pub fn alGetSourcei64vSOFT: unsafe extern "C" fn(source: ALuint, param: ALenum, values: *mut ALint64SOFT),
	}


	pub ext AL_SOFT_source_length {
		pub const AL_BYTE_LENGTH_SOFT,
		pub const AL_SAMPLE_LENGTH_SOFT,
		pub const AL_SEC_LENGTH_SOFT,
	}


	pub ext AL_EXT_source_distance_model {
		pub const AL_SOURCE_DISTANCE_MODEL,
	}
}


