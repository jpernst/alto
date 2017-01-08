use std::io::{self, Write};
use enum_primitive::FromPrimitive;

use ::{AltoError, AltoResult};
use sys;
use al;


pub struct AuxEffectSlot<'d: 'c, 'c> {
	ctx: &'c al::Context<'d>,
	slot: sys::ALuint,
}


pub unsafe trait EffectTrait<'d> {
	fn context(&self) -> &al::Context<'d>;
	fn as_raw(&self) -> sys::ALuint;
}


pub struct EaxReverbEffect<'d: 'c, 'c> {
	ctx: &'c al::Context<'d>,
	effect: sys::ALuint,
}


pub struct ReverbEffect<'d: 'c, 'c> {
	ctx: &'c al::Context<'d>,
	effect: sys::ALuint,
}


pub struct ChorusEffect<'d: 'c, 'c> {
	ctx: &'c al::Context<'d>,
	effect: sys::ALuint,
}


enum_from_primitive! {
	#[derive(Copy, Clone, PartialEq, Eq, Debug)]
	pub enum ChorusWaveform {
		Sinusoid = 0,
		Triangle,
	}
}


pub struct DistortionEffect<'d: 'c, 'c> {
	ctx: &'c al::Context<'d>,
	effect: sys::ALuint,
}


pub struct EchoEffect<'d: 'c, 'c> {
	ctx: &'c al::Context<'d>,
	effect: sys::ALuint,
}


pub struct FlangerEffect<'d: 'c, 'c> {
	ctx: &'c al::Context<'d>,
	effect: sys::ALuint,
}


enum_from_primitive! {
	#[derive(Copy, Clone, PartialEq, Eq, Debug)]
	pub enum FlangerWaveform {
		Sinusoid = 0,
		Triangle,
	}
}


pub struct FrequencyShifterEffect<'d: 'c, 'c> {
	ctx: &'c al::Context<'d>,
	effect: sys::ALuint,
}


enum_from_primitive! {
	#[derive(Copy, Clone, PartialEq, Eq, Debug)]
	pub enum FrequencyShifterDirection {
		Down = 0,
		Up,
		Off,
	}
}


pub struct VocalMorpherEffect<'d: 'c, 'c> {
	ctx: &'c al::Context<'d>,
	effect: sys::ALuint,
}


enum_from_primitive! {
	#[derive(Copy, Clone, PartialEq, Eq, Debug)]
	pub enum VocalMorpherPhoneme {
		A = 0,
		E,
		I,
		O,
		U,
		AA,
		AE,
		AH,
		AO,
		EH,
		ER,
		IH,
		IY,
		UH,
		UW,
		B,
		D,
		F,
		G,
		J,
		K,
		L,
		M,
		N,
		P,
		R,
		S,
		T,
		V,
		Z,
	}
}


enum_from_primitive! {
	#[derive(Copy, Clone, PartialEq, Eq, Debug)]
	pub enum VocalMorpherWaveform {
		Sinusoid,
		Triangle,
		Sawtooth,
	}
}


pub struct PitchShifterEffect<'d: 'c, 'c> {
	ctx: &'c al::Context<'d>,
	effect: sys::ALuint,
}


pub struct RingModulatorEffect<'d: 'c, 'c> {
	ctx: &'c al::Context<'d>,
	effect: sys::ALuint,
}


enum_from_primitive! {
	#[derive(Copy, Clone, PartialEq, Eq, Debug)]
	pub enum RingModulatorWaveform {
		Sinusoid = 0,
		Sawtooth,
		Square,
	}
}


pub struct AutowahEffect<'d: 'c, 'c> {
	ctx: &'c al::Context<'d>,
	effect: sys::ALuint,
}


pub struct CompressorEffect<'d: 'c, 'c> {
	ctx: &'c al::Context<'d>,
	effect: sys::ALuint,
}


pub struct EqualizerEffect<'d: 'c, 'c> {
	ctx: &'c al::Context<'d>,
	effect: sys::ALuint,
}


pub unsafe trait FilterTrait<'d> {
	fn context(&self) -> &al::Context<'d>;
	fn as_raw(&self) -> sys::ALuint;
}


pub struct LowpassFilter<'d: 'c, 'c> {
	ctx: &'c al::Context<'d>,
	filter: sys::ALuint,
}


pub struct HighpassFilter<'d: 'c, 'c> {
	ctx: &'c al::Context<'d>,
	filter: sys::ALuint,
}


pub struct BandpassFilter<'d: 'c, 'c> {
	ctx: &'c al::Context<'d>,
	filter: sys::ALuint,
}


impl<'d: 'c, 'c> AuxEffectSlot<'d, 'c> {
	pub unsafe fn new(ctx: &'c al::Context<'d>) -> AltoResult<AuxEffectSlot<'d, 'c>> {
		let efx = ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = ctx.make_current(true)?;
		let mut slot = 0;
		efx.alGenAuxiliaryEffectSlots?(1, &mut slot);
		ctx.get_error()?;
		Ok(AuxEffectSlot{ctx: ctx, slot: slot})
	}


	#[inline]
	pub fn context(&self) -> &al::Context<'d> { self.ctx }
	#[inline]
	pub fn as_raw(&self) -> sys::ALuint { self.slot }


	pub fn set_effect<E: EffectTrait<'d>>(&mut self, value: &E) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alAuxiliaryEffectSloti?(self.slot, efx.AL_EFFECTSLOT_EFFECT?, value.as_raw() as sys::ALint); }
		self.ctx.get_error()
	}


	pub fn gain(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetAuxiliaryEffectSlotf?(self.slot, efx.AL_EFFECTSLOT_GAIN?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alAuxiliaryEffectSlotf?(self.slot, efx.AL_EFFECTSLOT_GAIN?, value); }
		self.ctx.get_error()
	}


	pub fn auxiliary_send_auto(&self) -> AltoResult<bool> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { efx.alGetAuxiliaryEffectSloti?(self.slot, efx.AL_EFFECTSLOT_AUXILIARY_SEND_AUTO?, &mut value); }
		self.ctx.get_error().map(|_| value == sys::AL_TRUE as sys::ALint)
	}
	pub fn set_auxiliary_send_auto(&mut self, value: bool) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alAuxiliaryEffectSloti?(self.slot, efx.AL_EFFECTSLOT_AUXILIARY_SEND_AUTO?, if value { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.ctx.get_error()
	}
}


unsafe impl<'d: 'c, 'c> EffectTrait<'d> for EaxReverbEffect<'d, 'c> {
	#[inline]
	fn context(&self) -> &al::Context<'d> { self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl<'d: 'c, 'c> EaxReverbEffect<'d, 'c> {
	#[doc(hidden)]
	pub unsafe fn new(ctx: &'c al::Context<'d>) -> AltoResult<EaxReverbEffect<'d, 'c>> {
		let efx = ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = ctx.make_current(true)?;
		let mut effect = 0;
		efx.alGenEffects?(1, &mut effect);
		ctx.get_error()?;
		let effect = EaxReverbEffect{ctx: ctx, effect: effect};
		efx.alEffecti?(effect.as_raw(), efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_EAXREVERB?);
		ctx.get_error().map(|_| effect)
	}


	pub fn density(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_DENSITY?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_density(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_DENSITY?, value); }
		self.ctx.get_error()
	}


	pub fn diffusion(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_DIFFUSION?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_diffusion(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_DIFFUSION?, value); }
		self.ctx.get_error()
	}


	pub fn gain(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_GAIN?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_GAIN?, value); }
		self.ctx.get_error()
	}


	pub fn gainhf(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_GAINHF?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_gainhf(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_GAINHF?, value); }
		self.ctx.get_error()
	}


	pub fn gainlf(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_GAINLF?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_gainlf(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_GAINLF?, value); }
		self.ctx.get_error()
	}


	pub fn decay_time(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_DECAY_TIME?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_decay_time(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_DECAY_TIME?, value); }
		self.ctx.get_error()
	}


	pub fn decay_hfratio(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_DECAY_HFRATIO?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_decay_hfratio(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_DECAY_HFRATIO?, value); }
		self.ctx.get_error()
	}


	pub fn decay_lfratio(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_DECAY_LFRATIO?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_decay_lfratio(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_DECAY_LFRATIO?, value); }
		self.ctx.get_error()
	}


	pub fn reflections_gain(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_REFLECTIONS_GAIN?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_reflections_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_REFLECTIONS_GAIN?, value); }
		self.ctx.get_error()
	}


	pub fn reflections_delay(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_REFLECTIONS_DELAY?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_reflections_delay(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_REFLECTIONS_DELAY?, value); }
		self.ctx.get_error()
	}


	pub fn reflections_pan<V: From<[f32; 3]>>(&self) -> AltoResult<V> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = [0.0, 0.0, 0.0];
		unsafe { efx.alGetEffectfv?(self.effect, efx.AL_EAXREVERB_REFLECTIONS_PAN?, &mut value as *mut [f32; 3] as *mut f32); }
		self.ctx.get_error().map(|_| value.into())
	}
	pub fn set_reflections_pan<V: Into<[f32; 3]>>(&mut self, value: V) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectfv?(self.effect, efx.AL_EAXREVERB_REFLECTIONS_PAN?, &mut value.into() as *mut [f32; 3] as *mut f32); }
		self.ctx.get_error()
	}


	pub fn late_reverb_gain(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_LATE_REVERB_GAIN?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_late_reverb_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_LATE_REVERB_GAIN?, value); }
		self.ctx.get_error()
	}


	pub fn late_reverb_delay(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_LATE_REVERB_DELAY?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_late_reverb_delay(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_LATE_REVERB_DELAY?, value); }
		self.ctx.get_error()
	}


	pub fn late_reverb_pan<V: From<[f32; 3]>>(&self) -> AltoResult<V> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = [0.0, 0.0, 0.0];
		unsafe { efx.alGetEffectfv?(self.effect, efx.AL_EAXREVERB_LATE_REVERB_PAN?, &mut value as *mut [f32; 3] as *mut f32); }
		self.ctx.get_error().map(|_| value.into())
	}
	pub fn set_late_reverb_pan<V: Into<[f32; 3]>>(&mut self, value: V) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectfv?(self.effect, efx.AL_EAXREVERB_LATE_REVERB_PAN?, &mut value.into() as *mut [f32; 3] as *mut f32); }
		self.ctx.get_error()
	}


	pub fn echo_time(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_ECHO_TIME?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_echo_time(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_ECHO_TIME?, value); }
		self.ctx.get_error()
	}


	pub fn echo_depth(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_ECHO_DEPTH?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_echo_depth(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_ECHO_DEPTH?, value); }
		self.ctx.get_error()
	}


	pub fn modulation_time(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_MODULATION_TIME?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_modulation_time(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_MODULATION_TIME?, value); }
		self.ctx.get_error()
	}


	pub fn modulation_depth(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_MODULATION_DEPTH?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_modulation_depth(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_MODULATION_DEPTH?, value); }
		self.ctx.get_error()
	}


	pub fn hfreference(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_HFREFERENCE?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_hfreference(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_HFREFERENCE?, value); }
		self.ctx.get_error()
	}


	pub fn lfreference(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_LFREFERENCE?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_lfreference(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_LFREFERENCE?, value); }
		self.ctx.get_error()
	}


	pub fn air_absorption_gainhf(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_AIR_ABSORPTION_GAINHF?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_air_absorption_gainhf(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_AIR_ABSORPTION_GAINHF?, value); }
		self.ctx.get_error()
	}


	pub fn room_rolloff_factor(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EAXREVERB_ROOM_ROLLOFF_FACTOR?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_room_rolloff_factor(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EAXREVERB_ROOM_ROLLOFF_FACTOR?, value); }
		self.ctx.get_error()
	}


	pub fn decay_hflimit(&self) -> AltoResult<bool> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { efx.alGetEffecti?(self.effect, efx.AL_EAXREVERB_DECAY_HFLIMIT?, &mut value); }
		self.ctx.get_error().map(|_| value == sys::AL_TRUE as sys::ALint)
	}
	pub fn set_decay_hflimit(&mut self, value: bool) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffecti?(self.effect, efx.AL_EAXREVERB_DECAY_HFLIMIT?, if value { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.ctx.get_error()
	}
}


impl<'d: 'c, 'c> Drop for EaxReverbEffect<'d, 'c> {
	fn drop(&mut self) {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX().unwrap();
		if let Ok(_lock) = self.ctx.make_current(true) {
			unsafe { efx.alDeleteEffects.unwrap()(1, &mut self.effect as *mut sys::ALuint); }
			if let Err(_) = self.ctx.get_error() {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alDeleteEffects` failed in EaxReverbEffect drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in EaxReverbEffect drop");
		}
	}
}


unsafe impl<'d: 'c, 'c> EffectTrait<'d> for ReverbEffect<'d, 'c> {
	#[inline]
	fn context(&self) -> &al::Context<'d> { self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl<'d: 'c, 'c> ReverbEffect<'d, 'c> {
	#[doc(hidden)]
	pub unsafe fn new(ctx: &'c al::Context<'d>) -> AltoResult<ReverbEffect<'d, 'c>> {
		let efx = ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = ctx.make_current(true)?;
		let mut effect = 0;
		efx.alGenEffects?(1, &mut effect);
		ctx.get_error()?;
		let effect = ReverbEffect{ctx: ctx, effect: effect};
		efx.alEffecti?(effect.as_raw(), efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_REVERB?);
		ctx.get_error().map(|_| effect)
	}


	pub fn density(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_REVERB_DENSITY?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_density(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_REVERB_DENSITY?, value); }
		self.ctx.get_error()
	}


	pub fn diffusion(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_REVERB_DIFFUSION?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_diffusion(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_REVERB_DIFFUSION?, value); }
		self.ctx.get_error()
	}


	pub fn gain(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_REVERB_GAIN?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_REVERB_GAIN?, value); }
		self.ctx.get_error()
	}


	pub fn gainhf(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_REVERB_GAINHF?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_gainhf(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_REVERB_GAINHF?, value); }
		self.ctx.get_error()
	}


	pub fn decay_time(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_REVERB_DECAY_TIME?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_decay_time(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_REVERB_DECAY_TIME?, value); }
		self.ctx.get_error()
	}


	pub fn decay_hfratio(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_REVERB_DECAY_HFRATIO?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_decay_hfratio(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_REVERB_DECAY_HFRATIO?, value); }
		self.ctx.get_error()
	}


	pub fn reflections_gain(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_REVERB_REFLECTIONS_GAIN?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_reflections_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_REVERB_REFLECTIONS_GAIN?, value); }
		self.ctx.get_error()
	}


	pub fn reflections_delay(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_REVERB_REFLECTIONS_DELAY?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_reflections_delay(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_REVERB_REFLECTIONS_DELAY?, value); }
		self.ctx.get_error()
	}


	pub fn late_reverb_gain(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_REVERB_LATE_REVERB_GAIN?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_late_reverb_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_REVERB_LATE_REVERB_GAIN?, value); }
		self.ctx.get_error()
	}


	pub fn late_reverb_delay(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_REVERB_LATE_REVERB_DELAY?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_late_reverb_delay(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_REVERB_LATE_REVERB_DELAY?, value); }
		self.ctx.get_error()
	}


	pub fn air_absorption_gainhf(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_REVERB_AIR_ABSORPTION_GAINHF?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_air_absorption_gainhf(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_REVERB_AIR_ABSORPTION_GAINHF?, value); }
		self.ctx.get_error()
	}


	pub fn room_rolloff_factor(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_REVERB_ROOM_ROLLOFF_FACTOR?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_room_rolloff_factor(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_REVERB_ROOM_ROLLOFF_FACTOR?, value); }
		self.ctx.get_error()
	}


	pub fn decay_hflimit(&self) -> AltoResult<bool> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { efx.alGetEffecti?(self.effect, efx.AL_REVERB_DECAY_HFLIMIT?, &mut value); }
		self.ctx.get_error().map(|_| value == sys::AL_TRUE as sys::ALint)
	}
	pub fn set_decay_hflimit(&mut self, value: bool) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffecti?(self.effect, efx.AL_REVERB_DECAY_HFLIMIT?, if value { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.ctx.get_error()
	}
}


impl<'d: 'c, 'c> Drop for ReverbEffect<'d, 'c> {
	fn drop(&mut self) {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX().unwrap();
		if let Ok(_lock) = self.ctx.make_current(true) {
			unsafe { efx.alDeleteEffects.unwrap()(1, &mut self.effect as *mut sys::ALuint); }
			if let Err(_) = self.ctx.get_error() {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alDeleteEffects` failed in ReverbEffect drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in ReverbEffect drop");
		}
	}
}


unsafe impl<'d: 'c, 'c> EffectTrait<'d> for ChorusEffect<'d, 'c> {
	#[inline]
	fn context(&self) -> &al::Context<'d> { self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl<'d: 'c, 'c> ChorusEffect<'d, 'c> {
	#[doc(hidden)]
	pub unsafe fn new(ctx: &'c al::Context<'d>) -> AltoResult<ChorusEffect<'d, 'c>> {
		let efx = ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = ctx.make_current(true)?;
		let mut effect = 0;
		efx.alGenEffects?(1, &mut effect);
		ctx.get_error()?;
		let effect = ChorusEffect{ctx: ctx, effect: effect};
		efx.alEffecti?(effect.as_raw(), efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_CHORUS?);
		ctx.get_error().map(|_| effect)
	}


	pub fn waveform(&self) -> AltoResult<ChorusWaveform> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { efx.alGetEffecti?(self.effect, efx.AL_CHORUS_WAVEFORM?, &mut value); }
		self.ctx.get_error().and_then(|_| ChorusWaveform::from_i32(value as i32).ok_or(AltoError::AlInvalidValue))
	}
	pub fn set_waveform(&mut self, value: ChorusWaveform) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffecti?(self.effect, efx.AL_CHORUS_WAVEFORM?, value as sys::ALint) };
		self.ctx.get_error()
	}


	pub fn phase(&self) -> AltoResult<sys::ALint> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { efx.alGetEffecti?(self.effect, efx.AL_CHORUS_PHASE?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_phase(&mut self, value: sys::ALint) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffecti?(self.effect, efx.AL_CHORUS_PHASE?, value); }
		self.ctx.get_error()
	}


	pub fn rate(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_CHORUS_RATE?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_rate(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_CHORUS_RATE?, value); }
		self.ctx.get_error()
	}


	pub fn depth(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_CHORUS_DEPTH?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_depth(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_CHORUS_DEPTH?, value); }
		self.ctx.get_error()
	}


	pub fn feedback(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_CHORUS_FEEDBACK?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_feedback(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_CHORUS_FEEDBACK?, value); }
		self.ctx.get_error()
	}


	pub fn delay(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_CHORUS_DELAY?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_delay(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_CHORUS_DELAY?, value); }
		self.ctx.get_error()
	}
}


impl<'d: 'c, 'c> Drop for ChorusEffect<'d, 'c> {
	fn drop(&mut self) {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX().unwrap();
		if let Ok(_lock) = self.ctx.make_current(true) {
			unsafe { efx.alDeleteEffects.unwrap()(1, &mut self.effect as *mut sys::ALuint); }
			if let Err(_) = self.ctx.get_error() {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alDeleteEffects` failed in ChorusEffect drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in ChorusEffect drop");
		}
	}
}


unsafe impl<'d: 'c, 'c> EffectTrait<'d> for DistortionEffect<'d, 'c> {
	#[inline]
	fn context(&self) -> &al::Context<'d> { self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl<'d: 'c, 'c> DistortionEffect<'d, 'c> {
	#[doc(hidden)]
	pub unsafe fn new(ctx: &'c al::Context<'d>) -> AltoResult<DistortionEffect<'d, 'c>> {
		let efx = ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = ctx.make_current(true)?;
		let mut effect = 0;
		efx.alGenEffects?(1, &mut effect);
		ctx.get_error()?;
		let effect = DistortionEffect{ctx: ctx, effect: effect};
		efx.alEffecti?(effect.as_raw(), efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_DISTORTION?);
		ctx.get_error().map(|_| effect)
	}


	pub fn edge(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_DISTORTION_EDGE?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_edge(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_DISTORTION_EDGE?, value); }
		self.ctx.get_error()
	}


	pub fn lowpass_cutoff(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_DISTORTION_LOWPASS_CUTOFF?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_lowpass_cutoff(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_DISTORTION_LOWPASS_CUTOFF?, value); }
		self.ctx.get_error()
	}


	pub fn eqcenter(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_DISTORTION_EQCENTER?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_eqcenter(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_DISTORTION_EQCENTER?, value); }
		self.ctx.get_error()
	}


	pub fn eqbandwidth(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_DISTORTION_EQBANDWIDTH?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_eqbandwidth(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_DISTORTION_EQBANDWIDTH?, value); }
		self.ctx.get_error()
	}
}


impl<'d: 'c, 'c> Drop for DistortionEffect<'d, 'c> {
	fn drop(&mut self) {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX().unwrap();
		if let Ok(_lock) = self.ctx.make_current(true) {
			unsafe { efx.alDeleteEffects.unwrap()(1, &mut self.effect as *mut sys::ALuint); }
			if let Err(_) = self.ctx.get_error() {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alDeleteEffects` failed in DistortionEffect drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in DistortionEffect drop");
		}
	}
}


unsafe impl<'d: 'c, 'c> EffectTrait<'d> for EchoEffect<'d, 'c> {
	#[inline]
	fn context(&self) -> &al::Context<'d> { self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl<'d: 'c, 'c> EchoEffect<'d, 'c> {
	#[doc(hidden)]
	pub unsafe fn new(ctx: &'c al::Context<'d>) -> AltoResult<EchoEffect<'d, 'c>> {
		let efx = ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = ctx.make_current(true)?;
		let mut effect = 0;
		efx.alGenEffects?(1, &mut effect);
		ctx.get_error()?;
		let effect = EchoEffect{ctx: ctx, effect: effect};
		efx.alEffecti?(effect.as_raw(), efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_ECHO?);
		ctx.get_error().map(|_| effect)
	}


	pub fn delay(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_ECHO_DELAY?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_delay(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_ECHO_DELAY?, value); }
		self.ctx.get_error()
	}


	pub fn lrdelay(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_ECHO_LRDELAY?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_lrdelay(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_ECHO_LRDELAY?, value); }
		self.ctx.get_error()
	}


	pub fn damping(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_ECHO_DAMPING?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_damping(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_ECHO_DAMPING?, value); }
		self.ctx.get_error()
	}


	pub fn feedback(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_ECHO_FEEDBACK?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_feedback(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_ECHO_FEEDBACK?, value); }
		self.ctx.get_error()
	}


	pub fn spread(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_ECHO_SPREAD?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_spread(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_ECHO_SPREAD?, value); }
		self.ctx.get_error()
	}
}


impl<'d: 'c, 'c> Drop for EchoEffect<'d, 'c> {
	fn drop(&mut self) {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX().unwrap();
		if let Ok(_lock) = self.ctx.make_current(true) {
			unsafe { efx.alDeleteEffects.unwrap()(1, &mut self.effect as *mut sys::ALuint); }
			if let Err(_) = self.ctx.get_error() {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alDeleteEffects` failed in EchoEffect drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in EchoEffect drop");
		}
	}
}


unsafe impl<'d: 'c, 'c> EffectTrait<'d> for FlangerEffect<'d, 'c> {
	#[inline]
	fn context(&self) -> &al::Context<'d> { self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl<'d: 'c, 'c> FlangerEffect<'d, 'c> {
	#[doc(hidden)]
	pub unsafe fn new(ctx: &'c al::Context<'d>) -> AltoResult<FlangerEffect<'d, 'c>> {
		let efx = ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = ctx.make_current(true)?;
		let mut effect = 0;
		efx.alGenEffects?(1, &mut effect);
		ctx.get_error()?;
		let effect = FlangerEffect{ctx: ctx, effect: effect};
		efx.alEffecti?(effect.as_raw(), efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_FLANGER?);
		ctx.get_error().map(|_| effect)
	}


	pub fn waveform(&self) -> AltoResult<FlangerWaveform> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { efx.alGetEffecti?(self.effect, efx.AL_FLANGER_WAVEFORM?, &mut value); }
		self.ctx.get_error().and_then(|_| FlangerWaveform::from_i32(value as i32).ok_or(AltoError::AlInvalidValue))
	}
	pub fn set_waveform(&mut self, value: FlangerWaveform) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffecti?(self.effect, efx.AL_FLANGER_WAVEFORM?, value as sys::ALint) };
		self.ctx.get_error()
	}


	pub fn phase(&self) -> AltoResult<sys::ALint> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { efx.alGetEffecti?(self.effect, efx.AL_FLANGER_PHASE?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_phase(&mut self, value: sys::ALint) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffecti?(self.effect, efx.AL_FLANGER_PHASE?, value); }
		self.ctx.get_error()
	}


	pub fn rate(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_FLANGER_RATE?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_rate(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_FLANGER_RATE?, value); }
		self.ctx.get_error()
	}


	pub fn depth(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_FLANGER_DEPTH?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_depth(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_FLANGER_DEPTH?, value); }
		self.ctx.get_error()
	}


	pub fn feedback(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_FLANGER_FEEDBACK?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_feedback(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_FLANGER_FEEDBACK?, value); }
		self.ctx.get_error()
	}


	pub fn delay(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_FLANGER_DELAY?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_delay(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_FLANGER_DELAY?, value); }
		self.ctx.get_error()
	}
}


impl<'d: 'c, 'c> Drop for FlangerEffect<'d, 'c> {
	fn drop(&mut self) {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX().unwrap();
		if let Ok(_lock) = self.ctx.make_current(true) {
			unsafe { efx.alDeleteEffects.unwrap()(1, &mut self.effect as *mut sys::ALuint); }
			if let Err(_) = self.ctx.get_error() {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alDeleteEffects` failed in FlangerEffect drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in FlangerEffect drop");
		}
	}
}


unsafe impl<'d: 'c, 'c> EffectTrait<'d> for FrequencyShifterEffect<'d, 'c> {
	#[inline]
	fn context(&self) -> &al::Context<'d> { self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl<'d: 'c, 'c> FrequencyShifterEffect<'d, 'c> {
	#[doc(hidden)]
	pub unsafe fn new(ctx: &'c al::Context<'d>) -> AltoResult<FrequencyShifterEffect<'d, 'c>> {
		let efx = ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = ctx.make_current(true)?;
		let mut effect = 0;
		efx.alGenEffects?(1, &mut effect);
		ctx.get_error()?;
		let effect = FrequencyShifterEffect{ctx: ctx, effect: effect};
		efx.alEffecti?(effect.as_raw(), efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_FREQUENCY_SHIFTER?);
		ctx.get_error().map(|_| effect)
	}


	pub fn frequency(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_FREQUENCY_SHIFTER_FREQUENCY?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_frequency(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_FREQUENCY_SHIFTER_FREQUENCY?, value); }
		self.ctx.get_error()
	}


	pub fn left_direction(&self) -> AltoResult<FrequencyShifterDirection> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { efx.alGetEffecti?(self.effect, efx.AL_FREQUENCY_SHIFTER_LEFT_DIRECTION?, &mut value); }
		self.ctx.get_error().and_then(|_| FrequencyShifterDirection::from_i32(value as i32).ok_or(AltoError::AlInvalidValue))
	}
	pub fn set_left_direction(&mut self, value: FrequencyShifterDirection) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffecti?(self.effect, efx.AL_FREQUENCY_SHIFTER_LEFT_DIRECTION?, value as sys::ALint); }
		self.ctx.get_error()
	}


	pub fn right_direction(&self) -> AltoResult<FrequencyShifterDirection> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { efx.alGetEffecti?(self.effect, efx.AL_FREQUENCY_SHIFTER_RIGHT_DIRECTION?, &mut value); }
		self.ctx.get_error().and_then(|_| FrequencyShifterDirection::from_i32(value as i32).ok_or(AltoError::AlInvalidValue))
	}
	pub fn set_right_direction(&mut self, value: FrequencyShifterDirection) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffecti?(self.effect, efx.AL_FREQUENCY_SHIFTER_RIGHT_DIRECTION?, value as sys::ALint); }
		self.ctx.get_error()
	}
}


impl<'d: 'c, 'c> Drop for FrequencyShifterEffect<'d, 'c> {
	fn drop(&mut self) {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX().unwrap();
		if let Ok(_lock) = self.ctx.make_current(true) {
			unsafe { efx.alDeleteEffects.unwrap()(1, &mut self.effect as *mut sys::ALuint); }
			if let Err(_) = self.ctx.get_error() {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alDeleteEffects` failed in FrequencyShifterEffect drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in FrequencyShifterEffect drop");
		}
	}
}


unsafe impl<'d: 'c, 'c> EffectTrait<'d> for VocalMorpherEffect<'d, 'c> {
	#[inline]
	fn context(&self) -> &al::Context<'d> { self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl<'d: 'c, 'c> VocalMorpherEffect<'d, 'c> {
	#[doc(hidden)]
	pub unsafe fn new(ctx: &'c al::Context<'d>) -> AltoResult<VocalMorpherEffect<'d, 'c>> {
		let efx = ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = ctx.make_current(true)?;
		let mut effect = 0;
		efx.alGenEffects?(1, &mut effect);
		ctx.get_error()?;
		let effect = VocalMorpherEffect{ctx: ctx, effect: effect};
		efx.alEffecti?(effect.as_raw(), efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_VOCAL_MORPHER?);
		ctx.get_error().map(|_| effect)
	}


	pub fn phonemea(&self) -> AltoResult<VocalMorpherPhoneme> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { efx.alGetEffecti?(self.effect, efx.AL_VOCAL_MORPHER_PHONEMEA?, &mut value); }
		self.ctx.get_error().and_then(|_| VocalMorpherPhoneme::from_i32(value as i32).ok_or(AltoError::AlInvalidValue))
	}
	pub fn set_phonemea(&mut self, value: VocalMorpherPhoneme) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffecti?(self.effect, efx.AL_VOCAL_MORPHER_PHONEMEA?, value as sys::ALint); }
		self.ctx.get_error()
	}


	pub fn phonemeb(&self) -> AltoResult<VocalMorpherPhoneme> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { efx.alGetEffecti?(self.effect, efx.AL_VOCAL_MORPHER_PHONEMEB?, &mut value); }
		self.ctx.get_error().and_then(|_| VocalMorpherPhoneme::from_i32(value as i32).ok_or(AltoError::AlInvalidValue))
	}
	pub fn set_phonemeb(&mut self, value: VocalMorpherPhoneme) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffecti?(self.effect, efx.AL_VOCAL_MORPHER_PHONEMEB?, value as sys::ALint); }
		self.ctx.get_error()
	}


	pub fn phonemea_coarse_tuning(&self) -> AltoResult<sys::ALint> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { efx.alGetEffecti?(self.effect, efx.AL_VOCAL_MORPHER_PHONEMEA_COARSE_TUNING?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_phonemea_coarse_tuning(&mut self, value: sys::ALint) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffecti?(self.effect, efx.AL_VOCAL_MORPHER_PHONEMEA_COARSE_TUNING?, value); }
		self.ctx.get_error()
	}


	pub fn phonemeb_coarse_tuning(&self) -> AltoResult<sys::ALint> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { efx.alGetEffecti?(self.effect, efx.AL_VOCAL_MORPHER_PHONEMEB_COARSE_TUNING?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_phonemeb_coarse_tuning(&mut self, value: sys::ALint) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffecti?(self.effect, efx.AL_VOCAL_MORPHER_PHONEMEB_COARSE_TUNING?, value); }
		self.ctx.get_error()
	}


	pub fn waveform(&self) -> AltoResult<VocalMorpherWaveform> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { efx.alGetEffecti?(self.effect, efx.AL_VOCAL_MORPHER_WAVEFORM?, &mut value); }
		self.ctx.get_error().and_then(|_| VocalMorpherWaveform::from_i32(value as i32).ok_or(AltoError::AlInvalidValue))
	}
	pub fn set_waveform(&mut self, value: VocalMorpherWaveform) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffecti?(self.effect, efx.AL_VOCAL_MORPHER_WAVEFORM?, value as sys::ALint); }
		self.ctx.get_error()
	}


	pub fn rate(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_VOCAL_MORPHER_RATE?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_rate(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_VOCAL_MORPHER_RATE?, value); }
		self.ctx.get_error()
	}
}


impl<'d: 'c, 'c> Drop for VocalMorpherEffect<'d, 'c> {
	fn drop(&mut self) {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX().unwrap();
		if let Ok(_lock) = self.ctx.make_current(true) {
			unsafe { efx.alDeleteEffects.unwrap()(1, &mut self.effect as *mut sys::ALuint); }
			if let Err(_) = self.ctx.get_error() {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alDeleteEffects` failed in VocalMorpherEffect drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in VocalMorpherEffect drop");
		}
	}
}


unsafe impl<'d: 'c, 'c> EffectTrait<'d> for PitchShifterEffect<'d, 'c> {
	#[inline]
	fn context(&self) -> &al::Context<'d> { self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl<'d: 'c, 'c> PitchShifterEffect<'d, 'c> {
	#[doc(hidden)]
	pub unsafe fn new(ctx: &'c al::Context<'d>) -> AltoResult<PitchShifterEffect<'d, 'c>> {
		let efx = ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = ctx.make_current(true)?;
		let mut effect = 0;
		efx.alGenEffects?(1, &mut effect);
		ctx.get_error()?;
		let effect = PitchShifterEffect{ctx: ctx, effect: effect};
		efx.alEffecti?(effect.as_raw(), efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_PITCH_SHIFTER?);
		ctx.get_error().map(|_| effect)
	}


	pub fn coarse_tune(&self) -> AltoResult<sys::ALint> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { efx.alGetEffecti?(self.effect, efx.AL_PITCH_SHIFTER_COARSE_TUNE?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_coarse_tune(&mut self, value: sys::ALint) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffecti?(self.effect, efx.AL_PITCH_SHIFTER_COARSE_TUNE?, value); }
		self.ctx.get_error()
	}


	pub fn fine_tune(&self) -> AltoResult<sys::ALint> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { efx.alGetEffecti?(self.effect, efx.AL_PITCH_SHIFTER_FINE_TUNE?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_fine_tune(&mut self, value: sys::ALint) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffecti?(self.effect, efx.AL_PITCH_SHIFTER_FINE_TUNE?, value); }
		self.ctx.get_error()
	}
}


impl<'d: 'c, 'c> Drop for PitchShifterEffect<'d, 'c> {
	fn drop(&mut self) {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX().unwrap();
		if let Ok(_lock) = self.ctx.make_current(true) {
			unsafe { efx.alDeleteEffects.unwrap()(1, &mut self.effect as *mut sys::ALuint); }
			if let Err(_) = self.ctx.get_error() {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alDeleteEffects` failed in PitchShifterEffect drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in PitchShifterEffect drop");
		}
	}
}


unsafe impl<'d: 'c, 'c> EffectTrait<'d> for RingModulatorEffect<'d, 'c> {
	#[inline]
	fn context(&self) -> &al::Context<'d> { self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl<'d: 'c, 'c> RingModulatorEffect<'d, 'c> {
	#[doc(hidden)]
	pub unsafe fn new(ctx: &'c al::Context<'d>) -> AltoResult<RingModulatorEffect<'d, 'c>> {
		let efx = ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = ctx.make_current(true)?;
		let mut effect = 0;
		efx.alGenEffects?(1, &mut effect);
		ctx.get_error()?;
		let effect = RingModulatorEffect{ctx: ctx, effect: effect};
		efx.alEffecti?(effect.as_raw(), efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_RING_MODULATOR?);
		ctx.get_error().map(|_| effect)
	}


	pub fn frequency(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_RING_MODULATOR_FREQUENCY?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_frequency(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_RING_MODULATOR_FREQUENCY?, value); }
		self.ctx.get_error()
	}


	pub fn highpass_cutoff(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_RING_MODULATOR_HIGHPASS_CUTOFF?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_highpass_cutoff(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_RING_MODULATOR_HIGHPASS_CUTOFF?, value); }
		self.ctx.get_error()
	}


	pub fn waveform(&self) -> AltoResult<RingModulatorWaveform> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { efx.alGetEffecti?(self.effect, efx.AL_RING_MODULATOR_WAVEFORM?, &mut value); }
		self.ctx.get_error().and_then(|_| RingModulatorWaveform::from_i32(value as i32).ok_or(AltoError::AlInvalidValue))
	}
	pub fn set_waveform(&mut self, value: ChorusWaveform) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffecti?(self.effect, efx.AL_RING_MODULATOR_WAVEFORM?, value as sys::ALint) };
		self.ctx.get_error()
	}
}


impl<'d: 'c, 'c> Drop for RingModulatorEffect<'d, 'c> {
	fn drop(&mut self) {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX().unwrap();
		if let Ok(_lock) = self.ctx.make_current(true) {
			unsafe { efx.alDeleteEffects.unwrap()(1, &mut self.effect as *mut sys::ALuint); }
			if let Err(_) = self.ctx.get_error() {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alDeleteEffects` failed in RingModulatorEffect drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in RingModulatorEffect drop");
		}
	}
}


unsafe impl<'d: 'c, 'c> EffectTrait<'d> for AutowahEffect<'d, 'c> {
	#[inline]
	fn context(&self) -> &al::Context<'d> { self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl<'d: 'c, 'c> AutowahEffect<'d, 'c> {
	#[doc(hidden)]
	pub unsafe fn new(ctx: &'c al::Context<'d>) -> AltoResult<AutowahEffect<'d, 'c>> {
		let efx = ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = ctx.make_current(true)?;
		let mut effect = 0;
		efx.alGenEffects?(1, &mut effect);
		ctx.get_error()?;
		let effect = AutowahEffect{ctx: ctx, effect: effect};
		efx.alEffecti?(effect.as_raw(), efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_AUTOWAH?);
		ctx.get_error().map(|_| effect)
	}


	pub fn attack_time(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_AUTOWAH_ATTACK_TIME?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_attack_time(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_AUTOWAH_ATTACK_TIME?, value); }
		self.ctx.get_error()
	}


	pub fn release_time(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_AUTOWAH_RELEASE_TIME?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_release_time(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_AUTOWAH_RELEASE_TIME?, value); }
		self.ctx.get_error()
	}


	pub fn resonance(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_AUTOWAH_RESONANCE?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_resonance(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_AUTOWAH_RESONANCE?, value); }
		self.ctx.get_error()
	}


	pub fn peak_gain(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_AUTOWAH_PEAK_GAIN?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_peak_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_AUTOWAH_PEAK_GAIN?, value); }
		self.ctx.get_error()
	}
}


impl<'d: 'c, 'c> Drop for AutowahEffect<'d, 'c> {
	fn drop(&mut self) {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX().unwrap();
		if let Ok(_lock) = self.ctx.make_current(true) {
			unsafe { efx.alDeleteEffects.unwrap()(1, &mut self.effect as *mut sys::ALuint); }
			if let Err(_) = self.ctx.get_error() {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alDeleteEffects` failed in AutowahEffect drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in AutowahEffect drop");
		}
	}
}


unsafe impl<'d: 'c, 'c> EffectTrait<'d> for CompressorEffect<'d, 'c> {
	#[inline]
	fn context(&self) -> &al::Context<'d> { self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl<'d: 'c, 'c> CompressorEffect<'d, 'c> {
	#[doc(hidden)]
	pub unsafe fn new(ctx: &'c al::Context<'d>) -> AltoResult<CompressorEffect<'d, 'c>> {
		let efx = ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = ctx.make_current(true)?;
		let mut effect = 0;
		efx.alGenEffects?(1, &mut effect);
		ctx.get_error()?;
		let effect = CompressorEffect{ctx: ctx, effect: effect};
		efx.alEffecti?(effect.as_raw(), efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_COMPRESSOR?);
		ctx.get_error().map(|_| effect)
	}


	pub fn onoff(&self) -> AltoResult<bool> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0;
		unsafe { efx.alGetEffecti?(self.effect, efx.AL_COMPRESSOR_ONOFF?, &mut value); }
		self.ctx.get_error().map(|_| value == 1 as sys::ALint)
	}
	pub fn set_onoff(&mut self, value: bool) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffecti?(self.effect, efx.AL_COMPRESSOR_ONOFF?, if value { 1 } else { 0 } as sys::ALint); }
		self.ctx.get_error()
	}
}


impl<'d: 'c, 'c> Drop for CompressorEffect<'d, 'c> {
	fn drop(&mut self) {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX().unwrap();
		if let Ok(_lock) = self.ctx.make_current(true) {
			unsafe { efx.alDeleteEffects.unwrap()(1, &mut self.effect as *mut sys::ALuint); }
			if let Err(_) = self.ctx.get_error() {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alDeleteEffects` failed in CompressorEffect drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in CompressorEffect drop");
		}
	}
}


unsafe impl<'d: 'c, 'c> EffectTrait<'d> for EqualizerEffect<'d, 'c> {
	#[inline]
	fn context(&self) -> &al::Context<'d> { self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl<'d: 'c, 'c> EqualizerEffect<'d, 'c> {
	#[doc(hidden)]
	pub unsafe fn new(ctx: &'c al::Context<'d>) -> AltoResult<EqualizerEffect<'d, 'c>> {
		let efx = ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = ctx.make_current(true)?;
		let mut effect = 0;
		efx.alGenEffects?(1, &mut effect);
		ctx.get_error()?;
		let effect = EqualizerEffect{ctx: ctx, effect: effect};
		efx.alEffecti?(effect.as_raw(), efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_EQUALIZER?);
		ctx.get_error().map(|_| effect)
	}


	pub fn low_gain(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EQUALIZER_LOW_GAIN?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_low_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EQUALIZER_LOW_GAIN?, value); }
		self.ctx.get_error()
	}


	pub fn low_cutoff(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EQUALIZER_LOW_CUTOFF?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_low_cutoff(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EQUALIZER_LOW_CUTOFF?, value); }
		self.ctx.get_error()
	}


	pub fn mid1_gain(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EQUALIZER_MID1_GAIN?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_mid1_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EQUALIZER_MID1_GAIN?, value); }
		self.ctx.get_error()
	}


	pub fn mid1_center(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EQUALIZER_MID1_CENTER?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_mid1_center(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EQUALIZER_MID1_CENTER?, value); }
		self.ctx.get_error()
	}


	pub fn mid1_width(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EQUALIZER_MID1_WIDTH?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_mid1_width(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EQUALIZER_MID1_WIDTH?, value); }
		self.ctx.get_error()
	}


	pub fn mid2_gain(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EQUALIZER_MID2_GAIN?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_mid2_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EQUALIZER_MID2_GAIN?, value); }
		self.ctx.get_error()
	}


	pub fn mid2_center(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EQUALIZER_MID2_CENTER?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_mid2_center(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EQUALIZER_MID2_CENTER?, value); }
		self.ctx.get_error()
	}


	pub fn mid2_width(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EQUALIZER_MID2_WIDTH?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_mid2_width(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EQUALIZER_MID2_WIDTH?, value); }
		self.ctx.get_error()
	}


	pub fn high_gain(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EQUALIZER_HIGH_GAIN?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_high_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EQUALIZER_HIGH_GAIN?, value); }
		self.ctx.get_error()
	}


	pub fn high_cutoff(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetEffectf?(self.effect, efx.AL_EQUALIZER_HIGH_CUTOFF?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_high_cutoff(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alEffectf?(self.effect, efx.AL_EQUALIZER_HIGH_CUTOFF?, value); }
		self.ctx.get_error()
	}
}


impl<'d: 'c, 'c> Drop for EqualizerEffect<'d, 'c> {
	fn drop(&mut self) {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX().unwrap();
		if let Ok(_lock) = self.ctx.make_current(true) {
			unsafe { efx.alDeleteEffects.unwrap()(1, &mut self.effect as *mut sys::ALuint); }
			if let Err(_) = self.ctx.get_error() {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alDeleteEffects` failed in EqualizerEffect drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in EqualizerEffect drop");
		}
	}
}


unsafe impl<'d: 'c, 'c> FilterTrait<'d> for LowpassFilter<'d, 'c> {
	#[inline]
	fn context(&self) -> &al::Context<'d> { self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.filter }
}


impl<'d: 'c, 'c> LowpassFilter<'d, 'c> {
	#[doc(hidden)]
	pub unsafe fn new(ctx: &'c al::Context<'d>) -> AltoResult<LowpassFilter<'d, 'c>> {
		let efx = ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = ctx.make_current(true)?;
		let mut filter = 0;
		efx.alGenFilters?(1, &mut filter);
		ctx.get_error()?;
		let filter = LowpassFilter{ctx: ctx, filter: filter};
		efx.alFilteri?(filter.as_raw(), efx.AL_FILTER_TYPE?, efx.AL_FILTER_LOWPASS?);
		ctx.get_error().map(|_| filter)
	}


	pub fn gain(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetFilterf?(self.filter, efx.AL_LOWPASS_GAIN?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alFilterf?(self.filter, efx.AL_LOWPASS_GAIN?, value); }
		self.ctx.get_error()
	}


	pub fn gainhf(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetFilterf?(self.filter, efx.AL_LOWPASS_GAINHF?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_gainhf(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alFilterf?(self.filter, efx.AL_LOWPASS_GAINHF?, value); }
		self.ctx.get_error()
	}
}


impl<'d: 'c, 'c> Drop for LowpassFilter<'d, 'c> {
	fn drop(&mut self) {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX().unwrap();
		if let Ok(_lock) = self.ctx.make_current(true) {
			unsafe { efx.alDeleteFilters.unwrap()(1, &mut self.filter as *mut sys::ALuint); }
			if let Err(_) = self.ctx.get_error() {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alDeleteFilters` failed in LowpassFilter drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in LowpassFilter drop");
		}
	}
}


unsafe impl<'d: 'c, 'c> FilterTrait<'d> for HighpassFilter<'d, 'c> {
	#[inline]
	fn context(&self) -> &al::Context<'d> { self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.filter }
}


impl<'d: 'c, 'c> HighpassFilter<'d, 'c> {
	#[doc(hidden)]
	pub unsafe fn new(ctx: &'c al::Context<'d>) -> AltoResult<HighpassFilter<'d, 'c>> {
		let efx = ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = ctx.make_current(true)?;
		let mut filter = 0;
		efx.alGenFilters?(1, &mut filter);
		ctx.get_error()?;
		let filter = HighpassFilter{ctx: ctx, filter: filter};
		efx.alFilteri?(filter.as_raw(), efx.AL_FILTER_TYPE?, efx.AL_FILTER_HIGHPASS?);
		ctx.get_error().map(|_| filter)
	}


	pub fn gain(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetFilterf?(self.filter, efx.AL_HIGHPASS_GAIN?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alFilterf?(self.filter, efx.AL_HIGHPASS_GAIN?, value); }
		self.ctx.get_error()
	}


	pub fn gainlf(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetFilterf?(self.filter, efx.AL_HIGHPASS_GAINLF?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_gainlf(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alFilterf?(self.filter, efx.AL_HIGHPASS_GAINLF?, value); }
		self.ctx.get_error()
	}
}


impl<'d: 'c, 'c> Drop for HighpassFilter<'d, 'c> {
	fn drop(&mut self) {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX().unwrap();
		if let Ok(_lock) = self.ctx.make_current(true) {
			unsafe { efx.alDeleteFilters.unwrap()(1, &mut self.filter as *mut sys::ALuint); }
			if let Err(_) = self.ctx.get_error() {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alDeleteFilters` failed in HighpassFilter drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in HighpassFilter drop");
		}
	}
}


unsafe impl<'d: 'c, 'c> FilterTrait<'d> for BandpassFilter<'d, 'c> {
	#[inline]
	fn context(&self) -> &al::Context<'d> { self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.filter }
}


impl<'d: 'c, 'c> BandpassFilter<'d, 'c> {
	#[doc(hidden)]
	pub unsafe fn new(ctx: &'c al::Context<'d>) -> AltoResult<BandpassFilter<'d, 'c>> {
		let efx = ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = ctx.make_current(true)?;
		let mut filter = 0;
		efx.alGenFilters?(1, &mut filter);
		ctx.get_error()?;
		let filter = BandpassFilter{ctx: ctx, filter: filter};
		efx.alFilteri?(filter.as_raw(), efx.AL_FILTER_TYPE?, efx.AL_FILTER_BANDPASS?);
		ctx.get_error().map(|_| filter)
	}


	pub fn gain(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetFilterf?(self.filter, efx.AL_BANDPASS_GAIN?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alFilterf?(self.filter, efx.AL_BANDPASS_GAIN?, value); }
		self.ctx.get_error()
	}


	pub fn gainlf(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetFilterf?(self.filter, efx.AL_BANDPASS_GAINLF?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_gainlf(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alFilterf?(self.filter, efx.AL_BANDPASS_GAINLF?, value); }
		self.ctx.get_error()
	}


	pub fn gainhf(&self) -> AltoResult<f32> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		let mut value = 0.0;
		unsafe { efx.alGetFilterf?(self.filter, efx.AL_BANDPASS_GAINHF?, &mut value); }
		self.ctx.get_error().map(|_| value)
	}
	pub fn set_gainhf(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX()?;
		let _lock = self.ctx.make_current(true)?;
		unsafe { efx.alFilterf?(self.filter, efx.AL_BANDPASS_GAINHF?, value); }
		self.ctx.get_error()
	}
}


impl<'d: 'c, 'c> Drop for BandpassFilter<'d, 'c> {
	fn drop(&mut self) {
		let efx = self.ctx.device().extensions().ALC_EXT_EFX().unwrap();
		if let Ok(_lock) = self.ctx.make_current(true) {
			unsafe { efx.alDeleteFilters.unwrap()(1, &mut self.filter as *mut sys::ALuint); }
			if let Err(_) = self.ctx.get_error() {
				let _ = writeln!(io::stderr(), "ALTO ERROR: `alDeleteFilters` failed in BandpassFilter drop");
			}
		} else {
			let _ = writeln!(io::stderr(), "ALTO ERROR: `alcMakeContextCurrent` failed in BandpassFilter drop");
		}
	}
}
