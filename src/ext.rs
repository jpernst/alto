use std::collections::HashMap;
use std::sync::Mutex;

use ffi;


lazy_static! {
	static ref ALC_NULL_NAMES: HashMap<AlcNull, &'static [u8]> = {
		let mut map = HashMap::new();
		map.insert(AlcNull::EnumerateAll, "ALC_ENUMERATE_ALL_EXT\0".as_bytes());
		map.insert(AlcNull::SoftLoopback, "ALC_SOFT_loopback\0".as_bytes());
		map.insert(AlcNull::ThreadLocalContext, "ALC_EXT_thread_local_context\0".as_bytes());
		map
	};
	static ref ALC_NAMES: HashMap<Alc, &'static [u8]> = {
		let mut map = HashMap::new();
		map.insert(Alc::Dedicated, "ALC_EXT_DEDICATED\0".as_bytes());
		map.insert(Alc::Disconnect, "ALC_EXT_disconnect\0".as_bytes());
		map.insert(Alc::Efx, "ALC_EXT_EFX\0".as_bytes());
		map.insert(Alc::SoftHrtf, "ALC_SOFT_HRTF\0".as_bytes());
		map.insert(Alc::SoftPauseDevice, "ALC_SOFT_pause_device\0".as_bytes());
		map
	};
	static ref AL_NAMES: HashMap<Al, &'static [u8]> = {
		let mut map = HashMap::new();
		map.insert(Al::Alaw, "AL_EXT_ALAW\0".as_bytes());
		map.insert(Al::BFormat, "AL_EXT_BFORMAT\0".as_bytes());
		map.insert(Al::Double, "AL_EXT_double\0".as_bytes());
		map.insert(Al::Float32, "AL_EXT_float32\0".as_bytes());
		map.insert(Al::Ima4, "AL_EXT_IMA4\0".as_bytes());
		map.insert(Al::LokiQuadriphonic, "AL_LOKI_quadriphonic\0".as_bytes());
		map.insert(Al::McFormats, "AL_EXT_MCFORMATS\0".as_bytes());
		map.insert(Al::Mulaw, "AL_EXT_MULAW\0".as_bytes());
		map.insert(Al::MulawBFormat, "AL_EXT_MULAW_BFORMAT\0".as_bytes());
		map.insert(Al::MulawMcFormats, "AL_EXT_MULAW_MCFORMATS\0".as_bytes());
		map.insert(Al::SoftBlockAlignment, "AL_SOFT_block_alignment\0".as_bytes());
		map.insert(Al::SoftBufferSamples, "AL_SOFT_buffer_samples\0".as_bytes());
		map.insert(Al::SoftBufferSubData, "AL_SOFT_buffer_sub_data\0".as_bytes());
		map.insert(Al::SoftDeferredUpdates, "AL_SOFT_deferred_updates\0".as_bytes());
		map.insert(Al::SoftDirectChannels, "AL_SOFT_direct_channels\0".as_bytes());
		map.insert(Al::SoftLoopPoints, "AL_SOFT_loop_points\0".as_bytes());
		map.insert(Al::SoftMsadpcm, "AL_SOFT_MSADPCM\0".as_bytes());
		map.insert(Al::SoftSourceLatency, "AL_SOFT_source_latency\0".as_bytes());
		map.insert(Al::SoftSourceLength, "AL_SOFT_source_length\0".as_bytes());
		map.insert(Al::SourceDistanceModel, "AL_EXT_source_distance_model\0".as_bytes());
		map
	};
	static ref ALC_CACHE: Mutex<AlcNullCache> = Mutex::new(AlcNullCache::new());
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
	Alaw,
	BFormat,
	Double,
	Float32,
	Ima4,
	LokiQuadriphonic,
	McFormats,
	Mulaw,
	MulawBFormat,
	MulawMcFormats,
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
	pub enumerate_all: Option<Option<()>>,
	pub soft_loopback: Option<Option<SoftLoopbackFns>>,
	pub thread_local_context: Option<Option<ThreadLocalContextFns>>,
}


pub struct SoftLoopbackFns {
}


pub struct ThreadLocalContextFns {
}


pub struct AlcCache {
	pub dev: *mut ffi::ALCdevice,
	pub dedicated: Option<Option<()>>,
	pub disconnect: Option<Option<()>>,
	pub efx: Option<Option<EfxFns>>,
	pub soft_hrtf: Option<Option<()>>,
	pub soft_pause_device: Option<Option<()>>,
}


pub struct EfxFns {
}


pub struct AlCache {
	pub alaw: Option<Option<()>>,
	pub b_format: Option<Option<()>>,
	pub double: Option<Option<()>>,
	pub float32: Option<Option<()>>,
	pub ima4: Option<Option<()>>,
	pub loki_quadriphonic: Option<Option<()>>,
	pub mc_formats: Option<Option<()>>,
	pub mulaw: Option<Option<()>>,
	pub mulaw_b_format: Option<Option<()>>,
	pub mulaw_mc_formats: Option<Option<()>>,
	pub soft_block_alignment: Option<Option<()>>,
	pub soft_buffer_samples: Option<Option<()>>,
	pub soft_buffer_sub_data: Option<Option<()>>,
	pub soft_deferred_updates: Option<Option<()>>,
	pub soft_direct_channels: Option<Option<()>>,
	pub soft_loop_points: Option<Option<()>>,
	pub soft_msadpcm: Option<Option<()>>,
	pub soft_source_latency: Option<Option<()>>,
	pub soft_source_length: Option<Option<()>>,
	pub source_distance_model: Option<Option<()>>,
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


	pub fn query_soft_loopback(&mut self) -> Option<&SoftLoopbackFns> {
		None
	}


	pub fn query_thread_local_context(&mut self) -> Option<&ThreadLocalContextFns> {
		None
	}
}


impl AlcCache {
	pub fn new(dev: *mut ffi::ALCdevice) -> AlcCache {
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


	pub fn query_efx(&mut self) -> Option<&EfxFns> {
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
			alaw: None,
			b_format: None,
			double: None,
			float32: None,
			ima4: None,
			loki_quadriphonic: None,
			mc_formats: None,
			mulaw: None,
			mulaw_b_format: None,
			mulaw_mc_formats: None,
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
