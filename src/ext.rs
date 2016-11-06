use std::ptr;
use std::mem;
use std::cell::{RefCell, Ref};
use std::sync::{Mutex, RwLock};
use owning_ref::RwLockReadGuardRef;
use al_sys::*;


macro_rules! alc_ext {
	{
		pub cache $cache:ident;


		$(pub ext $ext:ident {
			$(pub enum $enum_:ident,)*
			$(pub fn $fn_:ident: $fn_ty:ty,)*
		})*
	} => {
		#[allow(non_snake_case)]
		#[doc(hidden)]
		pub struct $cache {
			dev: *mut ALCdevice,
			$($ext: RwLock<Option<Option<$ext>>>,)*
		}


		impl $cache {
			pub fn new(dev: *mut ALCdevice) -> $cache {
				$cache{
					dev: dev,
					$($ext: RwLock::new(None),)*
				}
			}


			$(pub fn $ext(&self) -> Option<RwLockReadGuardRef<Option<Option<$ext>>, $ext>> {
				if let Ok(mut e) = self.$ext.try_write() {
					if e.is_none() {
						*e = Some($ext::load(self.dev));
					}
				}

				let e = RwLockReadGuardRef::new(self.$ext.read().unwrap()).map(|e| e.as_ref().unwrap());
				if e.is_some() {
					Some(e.map(|e| e.as_ref().unwrap()))
				} else {
					None
				}
			})*
		}


		unsafe impl Send for $cache { }
		unsafe impl Sync for $cache { }


		$(#[allow(non_snake_case)]
		#[doc(hidden)]
		pub struct $ext {
			$(pub $enum_: Option<ALCenum>,)*
			$(pub $fn_: Option<$fn_ty>,)*
		}


		impl $ext {
			pub fn load(dev: *mut ALCdevice) -> Option<$ext> {
				unsafe { alcGetError(dev); }
				if unsafe { alcIsExtensionPresent(dev, concat!(stringify!($ext), "\0").as_bytes().as_ptr() as *const ALCchar) } == ALC_TRUE {
					Some($ext{
						$($enum_: {
							let e = unsafe { alcGetEnumValue(dev, concat!(stringify!($enum_), "\0").as_bytes().as_ptr() as *const ALCchar) };
							if unsafe { alcGetError(dev) } == ALC_NO_ERROR {
								Some(e)
							} else {
								None
							}
						},)*
						$($fn_: {
							let p = unsafe { alcGetProcAddress(dev, concat!(stringify!($fn_), "\0").as_bytes().as_ptr() as *const ALCchar) };
							if unsafe { alcGetError(dev) } == ALC_NO_ERROR {
								unsafe { mem::transmute(p) }
							} else {
								None
							}
						},)*
					})
				} else {
					None
				}
			}
		})*
	};
}


macro_rules! al_ext {
	{
		pub cache $cache:ident;


		$(pub ext $ext:ident {
			$(pub enum $enum_:ident,)*
			$(pub fn $fn_:ident: $fn_ty:ty,)*
		})*
	} => {
		#[allow(non_snake_case)]
		#[doc(hidden)]
		pub struct $cache {
			$($ext: RefCell<Option<Option<$ext>>>,)*
		}


		impl $cache {
			pub fn new() -> $cache {
				$cache{
					$($ext: RefCell::new(None),)*
				}
			}


			$(pub fn $ext(&self) -> Option<Ref<$ext>> {
				if let Ok(mut e) = self.$ext.try_borrow_mut() {
					if e.is_none() {
						*e = Some($ext::load());
					}
				}

				let e = Ref::map(self.$ext.borrow(), |e| e.as_ref().unwrap());
				if e.is_some() {
					Some(Ref::map(e, |e| e.as_ref().unwrap()))
				} else {
					None
				}
			})*
		}


		unsafe impl Send for $cache { }


		$(#[allow(non_snake_case)]
		#[doc(hidden)]
		pub struct $ext {
			$(pub $enum_: Option<ALenum>,)*
			$(pub $fn_: Option<$fn_ty>,)*
		}


		impl $ext {
			pub fn load() -> Option<$ext> {
				unsafe { alGetError(); }
				if unsafe { alIsExtensionPresent(concat!(stringify!($ext), "\0").as_bytes().as_ptr() as *const ALchar) } == AL_TRUE {
					Some($ext{
						$($enum_: {
							let e = unsafe { alGetEnumValue(concat!(stringify!($enum_), "\0").as_bytes().as_ptr() as *const ALchar) };
							if unsafe { alGetError() } == AL_NO_ERROR {
								Some(e)
							} else {
								None
							}
						},)*
						$($fn_: {
							let p = unsafe { alGetProcAddress(concat!(stringify!($fn_), "\0").as_bytes().as_ptr() as *const ALchar) };
							if unsafe { alGetError() } == AL_NO_ERROR {
								unsafe { mem::transmute(p) }
							} else {
								None
							}
						},)*
					})
				} else {
					None
				}
			}
		})*
	};
}


lazy_static! {
	static ref ALC_CACHE: AlcNullCache = AlcNullCache::new(ptr::null_mut());
}


#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug)]
#[repr(isize)]
pub enum AlcNull {
	EnumerateAll,
	SoftLoopback,
	ThreadLocalContext,
}


#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug)]
#[repr(isize)]
pub enum Alc {
	Dedicated,
	Disconnect,
	Efx,
	SoftHrtf,
	SoftPauseDevice,
}


#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug)]
#[repr(isize)]
pub enum Al {
	ALaw,
	BFormat,
	Double,
	Float32,
	Ima4,
	McFormats,
	MuLaw,
	MuLawBFormat,
	MuLawMcFormats,
	SoftBlockAlignment,
	SoftBufferSamples,
	SoftBufferSubData,
	SoftDeferredUpdates,
	SoftDirectChannels,
	SoftLoopPoints,
	SoftMsadpcm,
	SoftSourceLatency,
	SoftSourceLength,
	SourceDistanceModel,
}


alc_ext! {
	pub cache AlcNullCache;


	pub ext ALC_ENUMERATE_ALL_EXT {
		pub enum ALC_ALL_DEVICES_SPECIFIER,
		pub enum ALC_DEFAULT_ALL_DEVICES_SPECIFIER,
	}


	pub ext ALC_SOFT_loopback {
		pub enum ALC_BYTE_SOFT,
		pub enum ALC_UNSIGNED_BYTE_SOFT,
		pub enum ALC_SHORT_SOFT,
		pub enum ALC_UNSIGNED_SHORT_SOFT,
		pub enum ALC_INT_SOFT,
		pub enum ALC_UNSIGNED_INT_SOFT,
		pub enum ALC_FLOAT_SOFT,
		pub enum ALC_MONO_SOFT,
		pub enum ALC_STEREO_SOFT,
		pub enum ALC_QUAD_SOFT,
		pub enum ALC_5POINT1_SOFT,
		pub enum ALC_6POINT1_SOFT,
		pub enum ALC_7POINT1_SOFT,
		pub enum ALC_FORMAT_CHANNELS_SOFT,
		pub enum ALC_FORMAT_TYPE_SOFT,

		pub fn alcLoopbackOpenDeviceSOFT: unsafe extern "C" fn(deviceName: *const ALCchar) -> *mut ALCdevice,
		pub fn alcIsRenderFormatSupportedSOFT: unsafe extern "C" fn(device: *mut ALCdevice, frequency: ALCsizei, channels: ALCenum, type_: ALCenum) -> ALCboolean,
		pub fn alcRenderSamplesSOFT: unsafe extern "C" fn(device: *mut ALCdevice, buffer: *mut ALvoid, samples: ALCsizei),
	}


	pub ext ALC_EXT_thread_local_context {
		pub fn alcSetThreadContext: unsafe extern "C" fn(device: *mut ALCdevice) -> ALCboolean,
		pub fn alcGetThreadContext: unsafe extern "C" fn() -> *mut ALCcontext,
	}
}


alc_ext! {
	pub cache AlcCache;


	pub ext ALC_EXT_DEDICATED {
		//pub enum AL_EFFECT_DEDICATED_LOW_FREQUENCY_EFFECT,
		//pub enum AL_EFFECT_DEDICATED_DIALOGUE,
		//pub enum AL_EFFECT_DEDICATED_GAIN,
	}


	pub ext ALC_EXT_DISCONNECT {
		pub enum ALC_CONNECTED,
	}


	pub ext ALC_EXT_EFX {
		// TODO
	}


	pub ext ALC_SOFT_HRTF {
		pub enum ALC_HRTF_SOFT,
		pub enum ALC_HRTF_ID_SOFT,
		pub enum ALC_DONT_CARE_SOFT,
		pub enum ALC_HRTF_STATUS_SOFT,
		pub enum ALC_NUM_HRTF_SPECIFIERS_SOFT,
		pub enum ALC_HRTF_SPECIFIER_SOFT,
		pub enum ALC_HRTF_DISABLED_SOFT,
		pub enum ALC_HRTF_ENABLED_SOFT,
		pub enum ALC_HRTF_DENIED_SOFT,
		pub enum ALC_HRTF_REQUIRED_SOFT,
		pub enum ALC_HRTF_HEADPHONES_DETECTED_SOFT,
		pub enum ALC_HRTF_UNSUPPORTED_FORMAT_SOFT,

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
		pub enum AL_FORMAT_MONO_ALAW_EXT,
		pub enum AL_FORMAT_STEREO_ALAW_EXT,
	}


	pub ext AL_EXT_BFORMAT {
		pub enum AL_FORMAT_BFORMAT2D_8,
		pub enum AL_FORMAT_BFORMAT2D_16,
		pub enum AL_FORMAT_BFORMAT2D_FLOAT32,
		pub enum AL_FORMAT_BFORMAT3D_8,
		pub enum AL_FORMAT_BFORMAT3D_16,
		pub enum AL_FORMAT_BFORMAT3D_FLOAT32,
	}


	pub ext AL_EXT_double {
		pub enum AL_FORMAT_MONO_DOUBLE_EXT,
		pub enum AL_FORMAT_STEREO_DOUBLE_EXT,
	}


	pub ext AL_EXT_float32 {
		pub enum AL_FORMAT_MONO_FLOAT32,
		pub enum AL_FORMAT_STEREO_FLOAT32,
	}


	pub ext AL_EXT_IMA4 {
		pub enum AL_FORMAT_MONO_IMA4,
		pub enum AL_FORMAT_STEREO_IMA4,
	}


	pub ext AL_EXT_MCFORMATS {
		pub enum AL_FORMAT_QUAD8,
		pub enum AL_FORMAT_QUAD16,
		pub enum AL_FORMAT_QUAD32,
		pub enum AL_FORMAT_REAR8,
		pub enum AL_FORMAT_REAR16,
		pub enum AL_FORMAT_REAR32,
		pub enum AL_FORMAT_51CHN8,
		pub enum AL_FORMAT_51CHN16,
		pub enum AL_FORMAT_51CHN32,
		pub enum AL_FORMAT_61CHN8,
		pub enum AL_FORMAT_61CHN16,
		pub enum AL_FORMAT_61CHN32,
		pub enum AL_FORMAT_71CHN8,
		pub enum AL_FORMAT_71CHN16,
		pub enum AL_FORMAT_71CHN32,
	}


	pub ext AL_EXT_MULAW {
		pub enum AL_FORMAT_MONO_MULAW_EXT,
		pub enum AL_FORMAT_STEREO_MULAW_EXT,
	}


	pub ext AL_EXT_MULAW_BFORMAT {
		pub enum AL_FORMAT_BFORMAT2D_MULAW,
		pub enum AL_FORMAT_BFORMAT3D_MULAW,
	}


	pub ext AL_EXT_MULAW_MCFORMATS {
		pub enum AL_FORMAT_MONO_MULAW,
		pub enum AL_FORMAT_STEREO_MULAW,
		pub enum AL_FORMAT_QUAD_MULAW,
		pub enum AL_FORMAT_REAR_MULAW,
		pub enum AL_FORMAT_51CHN_MULAW,
		pub enum AL_FORMAT_61CHN_MULAW,
		pub enum AL_FORMAT_71CHN_MULAW,
	}


	pub ext AL_SOFT_block_alignment {
		pub enum AL_UNPACK_BLOCK_ALIGNMENT_SOFT,
		pub enum AL_PACK_BLOCK_ALIGNMENT_SOFT,
	}


	pub ext AL_SOFT_buffer_samples {
		pub enum AL_MONO_SOFT,
		pub enum AL_STEREO_SOFT,
		pub enum AL_REAR_SOFT,
		pub enum AL_QUAD_SOFT,
		pub enum AL_5POINT1_SOFT,
		pub enum AL_6POINT1_SOFT,
		pub enum AL_7POINT1_SOFT,

		pub enum AL_BYTE_SOFT,
		pub enum AL_UNSIGNED_BYTE_SOFT,
		pub enum AL_SHORT_SOFT,
		pub enum AL_UNSIGNED_SHORT_SOFT,
		pub enum AL_INT_SOFT,
		pub enum AL_UNSIGNED_INT_SOFT,
		pub enum AL_FLOAT_SOFT,
		pub enum AL_DOUBLE_SOFT,
		pub enum AL_BYTE3_SOFT,
		pub enum AL_UNSIGNED_BYTE3_SOFT,

		pub enum AL_MONO8_SOFT,
		pub enum AL_MONO16_SOFT,
		pub enum AL_MONO32F_SOFT,
		pub enum AL_STEREO8_SOFT,
		pub enum AL_STEREO16_SOFT,
		pub enum AL_STEREO32F_SOFT,
		pub enum AL_QUAD8_SOFT,
		pub enum AL_QUAD16_SOFT,
		pub enum AL_QUAD32F_SOFT,
		pub enum AL_REAR8_SOFT,
		pub enum AL_REAR16_SOFT,
		pub enum AL_REAR32F_SOFT,
		pub enum AL_5POINT1_8_SOFT,
		pub enum AL_5POINT1_16_SOFT,
		pub enum AL_5POINT1_32F_SOFT,
		pub enum AL_6POINT1_8_SOFT,
		pub enum AL_6POINT1_16_SOFT,
		pub enum AL_6POINT1_32F_SOFT,
		pub enum AL_7POINT1_8_SOFT,
		pub enum AL_7POINT1_16_SOFT,
		pub enum AL_7POINT1_32F_SOFT,

		pub enum AL_INTERNAL_FORMAT_SOFT,
		pub enum AL_BYTE_LENGTH_SOFT,
		pub enum AL_SAMPLE_LENGTH_SOFT,
		pub enum AL_SEC_LENGTH_SOFT,

		pub fn alBufferSamplesSOFT: unsafe extern "C" fn(buffer: ALuint, samplerate: ALuint, internalformat: ALenum, samples: ALsizei, channels: ALenum, type_: ALenum, data: *const ALvoid),
		pub fn alBufferSubSamplesSOFT: unsafe extern "C" fn(buffer: ALuint, offset: ALsizei, samples: ALsizei, channels: ALenum, type_: ALenum, data: *const ALvoid),
		pub fn alGetBufferSamplesSOFT: unsafe extern "C" fn(buffer: ALuint, offset: ALsizei, samples: ALsizei, channels: ALenum, type_: ALenum, data: *mut ALvoid),
		pub fn alIsBufferFormatSupportedSOFT: unsafe extern "C" fn(format: ALenum) -> ALboolean,
	}


	pub ext AL_SOFT_buffer_sub_data {
		pub enum AL_BYTE_RW_OFFSETS_SOFT,
		pub enum AL_SAMPLE_RW_OFFSETS_SOFT,

		pub fn alBufferSubDataSOFT: unsafe extern "C" fn(buffer: ALuint, format: ALenum, data: *const ALvoid, offset: ALsizei, length: ALsizei),
	}


	pub ext AL_SOFT_deferred_updates {
		pub enum AL_DEFERRED_UPDATES_SOFT,

		pub fn alDeferUpdatesSOFT: unsafe extern "C" fn(),
		pub fn alProcessUpdatesSOFT: unsafe extern "C" fn(),
	}


	pub ext AL_SOFT_direct_channels {
		pub enum AL_DIRECT_CHANNELS_SOFT,
	}


	pub ext AL_SOFT_loop_points {
		pub enum AL_LOOP_POINTS_SOFT,
	}


	pub ext AL_SOFT_MSADPCM {
		pub enum AL_FORMAT_MONO_MSADPCM_SOFT,
		pub enum AL_FORMAT_STEREO_MSADPCM_SOFT,
	}


	pub ext AL_SOFT_source_latency {
		pub enum AL_SAMPLE_OFFSET_LATENCY_SOFT,
		pub enum AL_SEC_OFFSET_LATENCY_SOFT,

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
		pub enum AL_BYTE_LENGTH_SOFT,
		pub enum AL_SAMPLE_LENGTH_SOFT,
		pub enum AL_SEC_LENGTH_SOFT,
	}


	pub ext AL_EXT_source_distance_model {
		pub enum AL_SOURCE_DISTANCE_MODEL,
	}
}


