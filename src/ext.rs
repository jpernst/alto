use al_sys::*;


macro_rules! alc_ext {
	{
		$(pub struct $struct_:ident {
			$(
				pub enum $enum_:ident,
			)*
			$(
				pub fn $fn_:ident: $fn_ty:ty,
			)*
		})*
	} => {
		$(#[allow(non_snake_case)]
		pub struct $struct_ {
			$(pub $enum_: Option<ALCenum>,)*
			$(pub $fn_: Option<$fn_ty>,)*
		})*
	};
}


macro_rules! al_ext {
	{
		$(pub struct $struct_:ident {
			$(
				pub enum $enum_:ident,
			)*
			$(
				pub fn $fn_:ident: $fn_ty:ty,
			)*
		})*
	} => {
		$(#[allow(non_snake_case)]
		pub struct $struct_ {
			$(pub $enum_: Option<ALenum>,)*
			$(pub $fn_: Option<$fn_ty>,)*
		})*
	};
}


//lazy_static! {
//	static ref ALC_NULL_NAMES: HashMap<AlcNull, &'static [u8]> = {
//		let mut map = HashMap::new();
//		map.insert(AlcNull::EnumerateAll, "ALC_ENUMERATE_ALL_EXT\0".as_bytes());
//		map.insert(AlcNull::SoftLoopback, "ALC_SOFT_loopback\0".as_bytes());
//		map.insert(AlcNull::ThreadLocalContext, "ALC_EXT_thread_local_context\0".as_bytes());
//		map
//	};
//	static ref ALC_NAMES: HashMap<Alc, &'static [u8]> = {
//		let mut map = HashMap::new();
//		map.insert(Alc::Dedicated, "ALC_EXT_DEDICATED\0".as_bytes());
//		map.insert(Alc::Disconnect, "ALC_EXT_disconnect\0".as_bytes());
//		map.insert(Alc::Efx, "ALC_EXT_EFX\0".as_bytes());
//		map.insert(Alc::SoftHrtf, "ALC_SOFT_HRTF\0".as_bytes());
//		map.insert(Alc::SoftPauseDevice, "ALC_SOFT_pause_device\0".as_bytes());
//		map
//	};
//	static ref AL_NAMES: HashMap<Al, &'static [u8]> = {
//		let mut map = HashMap::new();
//		map.insert(Al::ALaw, "AL_EXT_ALAW\0".as_bytes());
//		map.insert(Al::BFormat, "AL_EXT_BFORMAT\0".as_bytes());
//		map.insert(Al::Double, "AL_EXT_double\0".as_bytes());
//		map.insert(Al::Float32, "AL_EXT_float32\0".as_bytes());
//		map.insert(Al::Ima4, "AL_EXT_IMA4\0".as_bytes());
//		map.insert(Al::McFormats, "AL_EXT_MCFORMATS\0".as_bytes());
//		map.insert(Al::MuLaw, "AL_EXT_MULAW\0".as_bytes());
//		map.insert(Al::MuLawBFormat, "AL_EXT_MULAW_BFORMAT\0".as_bytes());
//		map.insert(Al::MuLawMcFormats, "AL_EXT_MULAW_MCFORMATS\0".as_bytes());
//		map.insert(Al::SoftBlockAlignment, "AL_SOFT_block_alignment\0".as_bytes());
//		map.insert(Al::SoftBufferSamples, "AL_SOFT_buffer_samples\0".as_bytes());
//		map.insert(Al::SoftBufferSubData, "AL_SOFT_buffer_sub_data\0".as_bytes());
//		map.insert(Al::SoftDeferredUpdates, "AL_SOFT_deferred_updates\0".as_bytes());
//		map.insert(Al::SoftDirectChannels, "AL_SOFT_direct_channels\0".as_bytes());
//		map.insert(Al::SoftLoopPoints, "AL_SOFT_loop_points\0".as_bytes());
//		map.insert(Al::SoftMsadpcm, "AL_SOFT_MSADPCM\0".as_bytes());
//		map.insert(Al::SoftSourceLatency, "AL_SOFT_source_latency\0".as_bytes());
//		map.insert(Al::SoftSourceLength, "AL_SOFT_source_length\0".as_bytes());
//		map.insert(Al::SourceDistanceModel, "AL_EXT_source_distance_model\0".as_bytes());
//		map
//	};
//	static ref ALC_CACHE: Mutex<AlcNullCache> = Mutex::new(AlcNullCache::new());
//}


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


pub struct AlcNullCache {
	pub enumerate_all: Option<Option<EnumerateAllSymbols>>,
	pub soft_loopback: Option<Option<SoftLoopbackSymbols>>,
	pub thread_local_context: Option<Option<ThreadLocalContextSymbols>>,
}


alc_ext! {
	pub struct EnumerateAllSymbols {
		pub enum ALC_ALL_DEVICES_SPECIFIER,
		pub enum ALC_DEFAULT_ALL_DEVICES_SPECIFIER,
	}


	pub struct SoftLoopbackSymbols {
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

		pub fn alcLoopbackOpenDeviceSOFT: unsafe extern "C" fn(device: *mut ALCdevice) -> *mut ALCdevice,
		pub fn alcIsRenderFormatSupportedSOFT: unsafe extern "C" fn(device: *mut ALCdevice, frequency: ALCsizei, channels: ALCenum, type_: ALCenum) -> ALCboolean,
		pub fn alcRenderSamplesSOFT: unsafe extern "C" fn(device: *mut ALCdevice, buffer: *mut ALvoid, samples: ALCsizei),
	}


	pub struct ThreadLocalContextSymbols {
		pub fn alcSetThreadContext: unsafe extern "C" fn(device: *mut ALCdevice) -> ALCboolean,
		pub fn alcGetThreadContext: unsafe extern "C" fn() -> *mut ALCcontext,
	}
}


pub struct AlcCache {
	pub dev: *mut ALCdevice,
	pub dedicated: Option<Option<DedicatedSymbols>>,
	pub disconnect: Option<Option<DisconnectSymbols>>,
	pub efx: Option<Option<EfxSymbols>>,
	pub soft_hrtf: Option<Option<SoftHrtfSymbols>>,
	pub soft_pause_device: Option<Option<SoftPauseDeviceSymbols>>,
}


alc_ext! {
	pub struct DedicatedSymbols {
		//pub enum AL_EFFECT_DEDICATED_LOW_FREQUENCY_EFFECT,
		//pub enum AL_EFFECT_DEDICATED_DIALOGUE,
		//pub enum AL_EFFECT_DEDICATED_GAIN,
	}


	pub struct DisconnectSymbols {
		pub enum ALC_CONNECTED,
	}


	pub struct EfxSymbols {
		// TODO
	}


	pub struct SoftHrtfSymbols {
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


	pub struct SoftPauseDeviceSymbols {
		pub fn alcDevicePauseSOFT: unsafe extern "C" fn(dev: *mut ALCdevice),
		pub fn alcDeviceResumeSOFT: unsafe extern "C" fn(dev: *mut ALCdevice),
	}
}


pub struct AlCache {
	pub a_law: Option<Option<ALawSymbols>>,
	pub b_format: Option<Option<BFormatSymbols>>,
	pub double: Option<Option<DoubleSymbols>>,
	pub float32: Option<Option<Float32Symbols>>,
	pub ima4: Option<Option<Ima4Symbols>>,
	pub mc_formats: Option<Option<McFormatsSymbols>>,
	pub mu_law: Option<Option<MuLawSymbols>>,
	pub mu_law_b_format: Option<Option<MuLawBFormatSymbols>>,
	pub mu_law_mc_formats: Option<Option<MuLawMcFormatsSymbols>>,
	pub soft_block_alignment: Option<Option<SoftBlockAlignmentSymbols>>,
	pub soft_buffer_samples: Option<Option<SoftBufferSamplesSymbols>>,
	pub soft_buffer_sub_data: Option<Option<SoftBufferSubDataSymbols>>,
	pub soft_deferred_updates: Option<Option<SoftDeferredUpdatesSymbols>>,
	pub soft_direct_channels: Option<Option<SoftDirectChannelsSymbols>>,
	pub soft_loop_points: Option<Option<SoftLoopPointsSymbols>>,
	pub soft_msadpcm: Option<Option<SoftMsadpcmSymbols>>,
	pub soft_source_latency: Option<Option<SoftSourceLatencySymbols>>,
	pub soft_source_length: Option<Option<SoftSourceLengthSymbols>>,
	pub source_distance_model: Option<Option<SourceDistanceModelSymbols>>,
}


pub type ALint64SOFT = i64;
pub type ALuint64SOFT = u64;


al_ext! {
	pub struct ALawSymbols {
		pub enum AL_FORMAT_MONO_ALAW_EXT,
		pub enum AL_FORMAT_STEREO_ALAW_EXT,
	}


	pub struct BFormatSymbols {
		pub enum AL_FORMAT_BFORMAT2D_8,
		pub enum AL_FORMAT_BFORMAT2D_16,
		pub enum AL_FORMAT_BFORMAT2D_FLOAT32,
		pub enum AL_FORMAT_BFORMAT3D_8,
		pub enum AL_FORMAT_BFORMAT3D_16,
		pub enum AL_FORMAT_BFORMAT3D_FLOAT32,
	}


	pub struct DoubleSymbols {
		pub enum AL_FORMAT_MONO_DOUBLE_EXT,
		pub enum AL_FORMAT_STEREO_DOUBLE_EXT,
	}


	pub struct Float32Symbols {
		pub enum AL_FORMAT_MONO_FLOAT32,
		pub enum AL_FORMAT_STEREO_FLOAT32,
	}


	pub struct Ima4Symbols {
		pub enum AL_FORMAT_MONO_IMA4,
		pub enum AL_FORMAT_STEREO_IMA4,
	}


	pub struct McFormatsSymbols {
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


	pub struct MuLawSymbols {
		pub enum AL_FORMAT_MONO_MULAW_EXT,
		pub enum AL_FORMAT_STEREO_MULAW_EXT,
	}


	pub struct MuLawBFormatSymbols {
		pub enum AL_FORMAT_BFORMAT2D_MULAW,
		pub enum AL_FORMAT_BFORMAT3D_MULAW,
	}


	pub struct MuLawMcFormatsSymbols {
		pub enum AL_FORMAT_MONO_MULAW,
		pub enum AL_FORMAT_STEREO_MULAW,
		pub enum AL_FORMAT_QUAD_MULAW,
		pub enum AL_FORMAT_REAR_MULAW,
		pub enum AL_FORMAT_51CHN_MULAW,
		pub enum AL_FORMAT_61CHN_MULAW,
		pub enum AL_FORMAT_71CHN_MULAW,
	}


	pub struct SoftBlockAlignmentSymbols {
		pub enum AL_UNPACK_BLOCK_ALIGNMENT_SOFT,
		pub enum AL_PACK_BLOCK_ALIGNMENT_SOFT,
	}


	pub struct SoftBufferSamplesSymbols {
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


	pub struct SoftBufferSubDataSymbols {
		pub enum AL_BYTE_RW_OFFSETS_SOFT,
		pub enum AL_SAMPLE_RW_OFFSETS_SOFT,

		pub fn alBufferSubDataSOFT: unsafe extern "C" fn(buffer: ALuint, format: ALenum, data: *const ALvoid, offset: ALsizei, length: ALsizei),
	}


	pub struct SoftDeferredUpdatesSymbols {
		pub enum AL_DEFERRED_UPDATES_SOFT,

		pub fn alDeferUpdatesSOFT: unsafe extern "C" fn(),
		pub fn alProcessUpdatesSOFT: unsafe extern "C" fn(),
	}


	pub struct SoftDirectChannelsSymbols {
		pub enum AL_DIRECT_CHANNELS_SOFT,
	}


	pub struct SoftLoopPointsSymbols {
		pub enum AL_LOOP_POINTS_SOFT,
	}


	pub struct SoftMsadpcmSymbols {
		pub enum AL_FORMAT_MONO_MSADPCM_SOFT,
		pub enum AL_FORMAT_STEREO_MSADPCM_SOFT,
	}


	pub struct SoftSourceLatencySymbols {
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


	pub struct SoftSourceLengthSymbols {
		pub enum AL_BYTE_LENGTH_SOFT,
		pub enum AL_SAMPLE_LENGTH_SOFT,
		pub enum AL_SEC_LENGTH_SOFT,
	}


	pub struct SourceDistanceModelSymbols {
		pub enum AL_SOURCE_DISTANCE_MODEL,
	}
}


impl AlcNullCache {
	pub fn new() -> AlcNullCache {
		AlcNullCache{
			enumerate_all: None,
			soft_loopback: None,
			thread_local_context: None,
		}
	}


	pub fn query_enumerate_all(&mut self) -> Option<()> {
		None
	}


	pub fn query_soft_loopback(&mut self) -> Option<&SoftLoopbackSymbols> {
		None
	}


	pub fn query_thread_local_context(&mut self) -> Option<&ThreadLocalContextSymbols> {
		None
	}
}


impl AlcCache {
	pub fn new(dev: *mut ALCdevice) -> AlcCache {
		AlcCache{
			dev: dev,
			dedicated: None,
			disconnect: None,
			efx: None,
			soft_hrtf: None,
			soft_pause_device: None,
		}
	}


	pub fn query_dedicated(&mut self) -> Option<()> {
		None
	}


	pub fn query_disconnect(&mut self) -> Option<()> {
		None
	}


	pub fn query_efx(&mut self) -> Option<&EfxSymbols> {
		None
	}


	pub fn query_soft_hrtf(&mut self) -> Option<()> {
		None
	}


	pub fn query_soft_pause_device(&mut self) -> Option<()> {
		None
	}
}


impl AlCache {
	pub fn new() -> AlCache {
		AlCache{
			a_law: None,
			b_format: None,
			double: None,
			float32: None,
			ima4: None,
			mc_formats: None,
			mu_law: None,
			mu_law_b_format: None,
			mu_law_mc_formats: None,
			soft_block_alignment: None,
			soft_buffer_samples: None,
			soft_buffer_sub_data: None,
			soft_deferred_updates: None,
			soft_direct_channels: None,
			soft_loop_points: None,
			soft_msadpcm: None,
			soft_source_latency: None,
			soft_source_length: None,
			source_distance_model: None,
		}
	}



}
