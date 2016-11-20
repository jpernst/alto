use std::ptr;
use std::mem;
use std::cell::{RefCell, Ref};
use std::sync::{Arc, RwLock};
use std::ops::Deref;
use std::fmt;
use std::error::Error as StdError;

use owning_ref::RwLockReadGuardRef;
use ::sys::*;
use ::alc::*;
use ::al::*;


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
		pub struct $cache {
			api: Arc<AlApi>,
			dev: *mut ALCdevice,
			$(
				$ext: RwLock<Option<ExtResult<$ext>>>,
			)*
		}


		#[allow(non_snake_case)]
		impl $cache {
			pub unsafe fn new(api: Arc<AlApi>, dev: *mut ALCdevice) -> $cache {
				$cache{
					api: api,
					dev: dev,
					$($ext: RwLock::new(None),)*
				}
			}


			$(pub fn $ext(&self) -> ExtResult<RwLockReadGuardRef<Option<ExtResult<$ext>>, $ext>> {
				if let Ok(mut ext) = self.$ext.try_write() {
					if ext.is_none() {
						*ext = Some($ext::load(&self.api, self.dev));
					}
				}

				let ext = RwLockReadGuardRef::new(self.$ext.read().unwrap()).map(|ext| ext.as_ref().unwrap());
				match *ext {
					Ok(_) => Ok(ext.map(|ext| ext.as_ref().unwrap())),
					Err(e) => Err(e),
				}
			})*
		}


		unsafe impl Send for $cache { }
		unsafe impl Sync for $cache { }


		$(#[doc(hidden)]
		#[allow(non_camel_case_types, non_snake_case)]
		#[derive(Debug)]
		pub struct $ext {
			$(
				pub $const_: ExtResult<ALCenum>,
			)*
			$(pub $fn_: ExtResult<$fn_ty>,)*
		}


		impl $ext {
			pub fn load(api: &AlApi, dev: *mut ALCdevice) -> ExtResult<$ext> {
				unsafe { (*api.alcGetError)(dev); }
				if unsafe { (*api.alcIsExtensionPresent)(dev, concat!(stringify!($ext), "\0").as_bytes().as_ptr() as *const ALCchar) } == ALC_TRUE {
					Ok($ext{
						$($const_: {
							let e = unsafe { (*api.alcGetEnumValue)(dev, concat!(stringify!($const_), "\0").as_bytes().as_ptr() as *const ALCchar) };
							if unsafe { (*api.alcGetError)(dev) } == ALC_NO_ERROR {
								Ok(e)
							} else {
								Err(ExtensionError)
							}
						},)*
						$($fn_: {
							let p = unsafe { (*api.alcGetProcAddress)(dev, concat!(stringify!($fn_), "\0").as_bytes().as_ptr() as *const ALCchar) };
							if unsafe { (*api.alcGetError)(dev) } == ALC_NO_ERROR {
								Ok(unsafe { mem::transmute(p) })
							} else {
								Err(ExtensionError)
							}
						},)*
					})
				} else {
					Err(ExtensionError)
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
		pub struct $cache {
			api: Arc<AlApi>,
			$($ext: RefCell<Option<ExtResult<$ext>>>,)*
		}


		#[allow(non_snake_case)]
		impl $cache {
			pub unsafe fn new(api: Arc<AlApi>) -> $cache {
				$cache{
					api: api,
					$($ext: RefCell::new(None),)*
				}
			}


			$(pub fn $ext(&self) -> ExtResult<Ref<$ext>> {
				if let Ok(mut ext) = self.$ext.try_borrow_mut() {
					if ext.is_none() {
						*ext = Some($ext::load(&self.api));
					}
				}

				let ext = Ref::map(self.$ext.borrow(), |e| e.as_ref().unwrap());
				match *ext {
					Ok(_) => Ok(Ref::map(ext, |ext| ext.as_ref().unwrap())),
					Err(e) => Err(e),
				}
			})*
		}


		unsafe impl Send for $cache { }


		$(#[doc(hidden)]
		#[allow(non_camel_case_types, non_snake_case)]
		#[derive(Debug)]
		pub struct $ext {
			$(pub $const_: ExtResult<ALenum>,)*
			$(pub $fn_: ExtResult<$fn_ty>,)*
		}


		impl $ext {
			pub fn load(api: &AlApi) -> ExtResult<$ext> {
				unsafe { (*api.alGetError)(); }
				if unsafe { (*api.alIsExtensionPresent)(concat!(stringify!($ext), "\0").as_bytes().as_ptr() as *const ALchar) } == AL_TRUE {
					Ok($ext{
						$($const_: {
							let e = unsafe { (*api.alGetEnumValue)(concat!(stringify!($const_), "\0").as_bytes().as_ptr() as *const ALchar) };
							if unsafe { (*api.alGetError)() } == AL_NO_ERROR {
								Ok(e)
							} else {
								Err(ExtensionError)
							}
						},)*
						$($fn_: {
							let p = unsafe { (*api.alGetProcAddress)(concat!(stringify!($fn_), "\0").as_bytes().as_ptr() as *const ALchar) };
							if unsafe { (*api.alGetError)() } == AL_NO_ERROR {
								Ok(unsafe { mem::transmute(p) })
							} else {
								Err(ExtensionError)
							}
						},)*
					})
				} else {
					Err(ExtensionError)
				}
			}
		})*
	};
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExtensionError;


pub type ExtResult<T> = ::std::result::Result<T, ExtensionError>;


#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug)]
pub enum AlcNull {
	EnumerateAll,
	SoftLoopback,
	ThreadLocalContext,
}


#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug)]
pub enum Alc {
	Dedicated,
	Disconnect,
	Efx,
	SoftHrtf,
	SoftPauseDevice,
}


#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug)]
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
//	SoftBufferSamples,
//	SoftBufferSubData,
	SoftDeferredUpdates,
	SoftDirectChannels,
	SoftLoopPoints,
	SoftMsadpcm,
	SoftSourceLatency,
	SoftSourceLength,
	SourceDistanceModel,
}


impl fmt::Display for ExtensionError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.description())
	}
}


impl StdError for ExtensionError {
	fn description(&self) -> &str { "Extension Not Present" }
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
		pub fn alcSetThreadContext: unsafe extern "C" fn(device: *mut ALCdevice) -> ALCboolean,
		pub fn alcGetThreadContext: unsafe extern "C" fn() -> *mut ALCcontext,
	}
}


alc_ext! {
	pub cache AlcCache;


	pub ext ALC_EXT_DEDICATED {
		//pub const AL_EFFECT_DEDICATED_LOW_FREQUENCY_EFFECT,
		//pub const AL_EFFECT_DEDICATED_DIALOGUE,
		//pub const AL_EFFECT_DEDICATED_GAIN,
	}


	pub ext ALC_EXT_DISCONNECT {
		pub const ALC_CONNECTED,
	}


	pub ext ALC_EXT_EFX {
		// TODO
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


