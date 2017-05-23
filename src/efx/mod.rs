use std::sync::Weak;
use enum_primitive::FromPrimitive;

use ::{AltoError, AltoResult};
use sys;
use al;
use ext;


mod presets;

pub use self::presets::*;


/// An aux effect slot as provided by EFX.
pub struct AuxEffectSlot {
	ctx: al::Context,
	slot: sys::ALuint,
	inputs: Vec<Weak<al::SourceInner>>,
}


/// Implemented for effects defined by EFX.
pub unsafe trait EffectTrait: Sized {
	#[doc(hidden)]
	fn new(ctx: al::Context) -> AltoResult<Self>;

	/// Context from which this effect was created.
	fn context(&self) -> &al::Context;
	/// Raw handle as provided by OpenAL.
	fn as_raw(&self) -> sys::ALuint;
}


/// `AL_EFFECT_EAXREVERB`
pub struct EaxReverbEffect {
	ctx: al::Context,
	effect: sys::ALuint,
}


/// `AL_EFFECT_REVERB`
pub struct ReverbEffect {
	ctx: al::Context,
	effect: sys::ALuint,
}


/// `AL_EFFECT_CHORUS`
pub struct ChorusEffect {
	ctx: al::Context,
	effect: sys::ALuint,
}


enum_from_primitive! {
	#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
	#[repr(C)]
	pub enum ChorusWaveform {
		Sinusoid = 0,
		Triangle,
	}
}


/// `AL_EFFECT_DISTORTION`
pub struct DistortionEffect {
	ctx: al::Context,
	effect: sys::ALuint,
}


/// `AL_EFFECT_ECHO`
pub struct EchoEffect {
	ctx: al::Context,
	effect: sys::ALuint,
}


/// `AL_EFFECT_FLANGER`
pub struct FlangerEffect {
	ctx: al::Context,
	effect: sys::ALuint,
}


enum_from_primitive! {
	#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
	#[repr(C)]
	pub enum FlangerWaveform {
		Sinusoid = 0,
		Triangle,
	}
}


/// `AL_EFFECT_FREQUENCY_SHIFTER`
pub struct FrequencyShifterEffect {
	ctx: al::Context,
	effect: sys::ALuint,
}


enum_from_primitive! {
	#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
	#[repr(C)]
	pub enum FrequencyShifterDirection {
		Down = 0,
		Up,
		Off,
	}
}


/// `AL_EFFECT_VOCAL_MORPHER`
pub struct VocalMorpherEffect {
	ctx: al::Context,
	effect: sys::ALuint,
}


enum_from_primitive! {
	#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
	#[repr(C)]
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
	#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
	#[repr(C)]
	pub enum VocalMorpherWaveform {
		Sinusoid = 0,
		Triangle,
		Sawtooth,
	}
}


/// `AL_EFFECT_PITCH_SHIFTER`
pub struct PitchShifterEffect {
	ctx: al::Context,
	effect: sys::ALuint,
}


/// `AL_EFFECT_RING_MODULATOR`
pub struct RingModulatorEffect {
	ctx: al::Context,
	effect: sys::ALuint,
}


enum_from_primitive! {
	#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
	#[repr(C)]
	pub enum RingModulatorWaveform {
		Sinusoid = 0,
		Sawtooth,
		Square,
	}
}


/// `AL_EFFECT_AUTOWAH`
pub struct AutowahEffect {
	ctx: al::Context,
	effect: sys::ALuint,
}


/// `AL_EFFECT_COMPRESSOR`
pub struct CompressorEffect {
	ctx: al::Context,
	effect: sys::ALuint,
}


/// `AL_EFFECT_EQUALIZER`
pub struct EqualizerEffect {
	ctx: al::Context,
	effect: sys::ALuint,
}


/// `AL_EFFECT_DEDICATED_LOW_FREQUENCY_EFFECT`
/// Requires `ALC_EXT_DEDICATED`
pub struct DedicatedLowFrequencyEffect {
	ctx: al::Context,
	effect: sys::ALuint,
}


/// `AL_EFFECT_DEDICATED_DIALOGUE`
/// Requires `ALC_EXT_DEDICATED`
pub struct DedicatedDialogueEffect {
	ctx: al::Context,
	effect: sys::ALuint,
}


/// Implemented for filters as defined by EFX.
pub unsafe trait FilterTrait: Sized {
	#[doc(hidden)]
	fn new(ctx: al::Context) -> AltoResult<Self>;

	/// Context from which this effect was created.
	fn context(&self) -> &al::Context;
	/// Raw handle as provided by OpenAL.
	fn as_raw(&self) -> sys::ALuint;
}


/// `AL_FILTER_LOWPASS`
pub struct LowpassFilter {
	ctx: al::Context,
	filter: sys::ALuint,
}


/// `AL_FILTER_HIGHPASS`
pub struct HighpassFilter {
	ctx: al::Context,
	filter: sys::ALuint,
}


/// `AL_FILTER_BANDPASS`
pub struct BandpassFilter {
	ctx: al::Context,
	filter: sys::ALuint,
}


impl AuxEffectSlot {
	pub(crate) fn new(ctx: al::Context) -> AltoResult<AuxEffectSlot> {
		let mut slot = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			efx.alAuxiliaryEffectSloti?;
			efx.alAuxiliaryEffectSlotf?;
			efx.alAuxiliaryEffectSlotiv?;
			efx.alAuxiliaryEffectSlotfv?;
			efx.alDeleteAuxiliaryEffectSlots?;
			efx.AL_EFFECTSLOT_EFFECT?;
			efx.AL_EFFECTSLOT_GAIN?;
			efx.AL_EFFECTSLOT_AUXILIARY_SEND_AUTO?;
			let _lock = ctx.make_current(true);
			unsafe { efx.alGenAuxiliaryEffectSlots?(1, &mut slot); }
			ctx.get_error()?;
		}
		Ok(AuxEffectSlot{ctx: ctx, slot: slot, inputs: Vec::new()})
	}


	pub(crate) fn add_input(&mut self, src: Weak<al::SourceInner>) {
		if self.inputs.len() == self.inputs.capacity() {
			self.inputs.retain(|s| s.upgrade().is_some());
		}

		self.inputs.push(src);
	}


	#[inline]
	pub fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	pub fn as_raw(&self) -> sys::ALuint { self.slot }


	/// `alAuxiliaryEffectSloti(AL_EFFECTSLOT_EFFECT)`
	pub fn set_effect<E: EffectTrait>(&mut self, value: &E) -> AltoResult<()> {
		if *value.context() != self.ctx {
			return Err(AltoError::AlInvalidValue);
		}
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alAuxiliaryEffectSloti.unwrap()(self.slot, efx.AL_EFFECTSLOT_EFFECT.unwrap(), value.as_raw() as sys::ALint); }
		self.ctx.get_error()
	}
	/// `alAuxiliaryEffectSloti(AL_EFFECTSLOT_EFFECT)`
	pub fn clear_effect(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alAuxiliaryEffectSloti.unwrap()(self.slot, efx.AL_EFFECTSLOT_EFFECT.unwrap(), 0); }
	}


	/// `alGetAuxiliaryEffectSloti(AL_EFFECTSLOT_GAIN)`
	pub fn gain(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetAuxiliaryEffectSlotf.unwrap()(self.slot, efx.AL_EFFECTSLOT_GAIN.unwrap(), &mut value); }
		value
	}
	/// `alAuxiliaryEffectSloti(AL_EFFECTSLOT_GAIN)`
	pub fn set_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alAuxiliaryEffectSlotf.unwrap()(self.slot, efx.AL_EFFECTSLOT_GAIN.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetAuxiliaryEffectSloti(AL_EFFECTSLOT_AUXILIARY_SEND_AUTO)`
	pub fn aux_send_auto(&self) -> bool {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { efx.alGetAuxiliaryEffectSloti.unwrap()(self.slot, efx.AL_EFFECTSLOT_AUXILIARY_SEND_AUTO.unwrap(), &mut value); }
		value == sys::AL_TRUE as sys::ALint
	}
	/// `alAuxiliaryEffectSloti(AL_EFFECTSLOT_AUXILIARY_SEND_AUTO)`
	pub fn set_aux_send_auto(&mut self, value: bool) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alAuxiliaryEffectSloti.unwrap()(self.slot, efx.AL_EFFECTSLOT_AUXILIARY_SEND_AUTO.unwrap(), if value { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.ctx.get_error()
	}
}


impl Drop for AuxEffectSlot {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		for src in self.inputs.drain(..) {
			if let Some(src) = src.upgrade() {
				src.clear_aux_effect_slot(self.slot);
			}
		}

		let adaes = efx.alDeleteAuxiliaryEffectSlots.unwrap();
		unsafe { adaes(1, &mut self.slot as *mut sys::ALuint); }
	}
}


fn check_effect_symbols(efx: &ext::ALC_EXT_EFX) -> AltoResult<()> {
	efx.alGetEffecti?;
	efx.alGetEffectf?;
	efx.alGetEffectiv?;
	efx.alGetEffectfv?;
	efx.alEffectf?;
	efx.alEffectiv?;
	efx.alEffectfv?;
	efx.alDeleteEffects?;

	Ok(())
}


unsafe impl EffectTrait for EaxReverbEffect {
	fn new(ctx: al::Context) -> AltoResult<EaxReverbEffect> {
		let mut effect = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			check_effect_symbols(&efx)?;
			efx.AL_EAXREVERB_DENSITY?;
			efx.AL_EAXREVERB_DIFFUSION?;
			efx.AL_EAXREVERB_GAIN?;
			efx.AL_EAXREVERB_GAINHF?;
			efx.AL_EAXREVERB_GAINLF?;
			efx.AL_EAXREVERB_DECAY_TIME?;
			efx.AL_EAXREVERB_DECAY_HFRATIO?;
			efx.AL_EAXREVERB_DECAY_LFRATIO?;
			efx.AL_EAXREVERB_REFLECTIONS_GAIN?;
			efx.AL_EAXREVERB_REFLECTIONS_DELAY?;
			efx.AL_EAXREVERB_REFLECTIONS_PAN?;
			efx.AL_EAXREVERB_LATE_REVERB_GAIN?;
			efx.AL_EAXREVERB_LATE_REVERB_DELAY?;
			efx.AL_EAXREVERB_LATE_REVERB_PAN?;
			efx.AL_EAXREVERB_ECHO_TIME?;
			efx.AL_EAXREVERB_ECHO_DEPTH?;
			efx.AL_EAXREVERB_MODULATION_TIME?;
			efx.AL_EAXREVERB_MODULATION_DEPTH?;
			efx.AL_EAXREVERB_AIR_ABSORPTION_GAINHF?;
			efx.AL_EAXREVERB_HFREFERENCE?;
			efx.AL_EAXREVERB_LFREFERENCE?;
			efx.AL_EAXREVERB_ROOM_ROLLOFF_FACTOR?;
			efx.AL_EAXREVERB_DECAY_HFLIMIT?;
			let _lock = ctx.make_current(true);
			unsafe {
				efx.alGenEffects?(1, &mut effect);
				efx.alEffecti?(effect, efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_EAXREVERB?);
			}
			ctx.get_error()?;
		}
		Ok(EaxReverbEffect{ctx: ctx, effect: effect})
	}


	#[inline]
	fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl EaxReverbEffect {
	/// Set all effect properties based on a reverb preset.
	pub fn set_preset(&mut self, preset: &EaxReverbProperties) -> AltoResult<()> {
		let mut r = Ok(());

		r = r.and(self.set_density(preset.density));
		r = r.and(self.set_diffusion(preset.diffusion));
		r = r.and(self.set_gain(preset.gain));
		r = r.and(self.set_gainhf(preset.gainhf));
		r = r.and(self.set_gainlf(preset.gainlf));
		r = r.and(self.set_decay_time(preset.decay_time));
		r = r.and(self.set_decay_hfratio(preset.decay_hfratio));
		r = r.and(self.set_decay_lfratio(preset.decay_lfratio));
		r = r.and(self.set_reflections_gain(preset.reflections_gain));
		r = r.and(self.set_reflections_delay(preset.reflections_delay));
		r = r.and(self.set_reflections_pan(preset.reflections_pan));
		r = r.and(self.set_late_reverb_gain(preset.late_reverb_gain));
		r = r.and(self.set_late_reverb_delay(preset.late_reverb_delay));
		r = r.and(self.set_late_reverb_pan(preset.late_reverb_pan));
		r = r.and(self.set_echo_time(preset.echo_time));
		r = r.and(self.set_echo_depth(preset.echo_depth));
		r = r.and(self.set_modulation_time(preset.modulation_time));
		r = r.and(self.set_modulation_depth(preset.modulation_depth));
		r = r.and(self.set_air_absorption_gainhf(preset.air_absorption_gainhf));
		r = r.and(self.set_hfreference(preset.hfreference));
		r = r.and(self.set_lfreference(preset.lfreference));
		r = r.and(self.set_room_rolloff_factor(preset.room_rolloff_factor));
		r = r.and(self.set_decay_hflimit(preset.decay_hflimit));

		r
	}


	/// `alGetEffectf(AL_EAXREVERB_DENSITY)`
	pub fn density(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_DENSITY.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_DENSITY)`
	pub fn set_density(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_DENSITY.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_DIFFUSION)`
	pub fn diffusion(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_DIFFUSION.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_DIFFUSION)`
	pub fn set_diffusion(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_DIFFUSION.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_GAIN)`
	pub fn gain(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_GAIN.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_GAIN)`
	pub fn set_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_GAIN.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_GAINHF)`
	pub fn gainhf(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_GAINHF.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_GAINHF)`
	pub fn set_gainhf(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_GAINHF.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_GAINLF)`
	pub fn gainlf(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_GAINLF.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_GAINLF)`
	pub fn set_gainlf(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_GAINLF.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_DECAY_TIME)`
	pub fn decay_time(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_DECAY_TIME.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_DECAY_TIME)`
	pub fn set_decay_time(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_DECAY_TIME.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_DECAY_HFRATIO)`
	pub fn decay_hfratio(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_DECAY_HFRATIO.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_DECAY_HFRATIO)`
	pub fn set_decay_hfratio(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_DECAY_HFRATIO.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_DECAY_LFRATIO)`
	pub fn decay_lfratio(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_DECAY_LFRATIO.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_DECAY_LFRATIO)`
	pub fn set_decay_lfratio(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_DECAY_LFRATIO.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_REFLECTIONS_GAIN)`
	pub fn reflections_gain(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_REFLECTIONS_GAIN.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_REFLECTIONS_GAIN)`
	pub fn set_reflections_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_REFLECTIONS_GAIN.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_REFLECTIONS_DELAY)`
	pub fn reflections_delay(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_REFLECTIONS_DELAY.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_REFLECTIONS_DELAY)`
	pub fn set_reflections_delay(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_REFLECTIONS_DELAY.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectfv(AL_EAXREVERB_REFLECTIONS_PAN)`
	pub fn reflections_pan<V: From<[f32; 3]>>(&self) -> V {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = [0.0, 0.0, 0.0];
		unsafe { efx.alGetEffectfv.unwrap()(self.effect, efx.AL_EAXREVERB_REFLECTIONS_PAN.unwrap(), &mut value as *mut [f32; 3] as *mut f32); }
		value.into()
	}
	/// `alEffectfv(AL_EAXREVERB_REFLECTIONS_PAN)`
	pub fn set_reflections_pan<V: Into<[f32; 3]>>(&mut self, value: V) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectfv.unwrap()(self.effect, efx.AL_EAXREVERB_REFLECTIONS_PAN.unwrap(), &mut value.into() as *mut [f32; 3] as *mut f32); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_LATE_REVERB_GAIN)`
	pub fn late_reverb_gain(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_LATE_REVERB_GAIN.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_LATE_REVERB_GAIN)`
	pub fn set_late_reverb_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_LATE_REVERB_GAIN.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_LATE_REVERB_DELAY)`
	pub fn late_reverb_delay(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_LATE_REVERB_DELAY.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_LATE_REVERB_DELAY)`
	pub fn set_late_reverb_delay(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_LATE_REVERB_DELAY.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectfv(AL_EAXREVERB_LATE_REVERB_PAN)`
	pub fn late_reverb_pan<V: From<[f32; 3]>>(&self) -> V {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = [0.0, 0.0, 0.0];
		unsafe { efx.alGetEffectfv.unwrap()(self.effect, efx.AL_EAXREVERB_LATE_REVERB_PAN.unwrap(), &mut value as *mut [f32; 3] as *mut f32); }
		value.into()
	}
	/// `alEffectfv(AL_EAXREVERB_LATE_REVERB_PAN)`
	pub fn set_late_reverb_pan<V: Into<[f32; 3]>>(&mut self, value: V) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectfv.unwrap()(self.effect, efx.AL_EAXREVERB_LATE_REVERB_PAN.unwrap(), &mut value.into() as *mut [f32; 3] as *mut f32); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_ECHO_TIME)`
	pub fn echo_time(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_ECHO_TIME.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_ECHO_TIME)`
	pub fn set_echo_time(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_ECHO_TIME.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_ECHO_DEPTH)`
	pub fn echo_depth(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_ECHO_DEPTH.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_ECHO_DEPTH)`
	pub fn set_echo_depth(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_ECHO_DEPTH.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_MODULATION_TIME)`
	pub fn modulation_time(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_MODULATION_TIME.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_MODULATION_TIME)`
	pub fn set_modulation_time(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_MODULATION_TIME.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_MODULATION_DEPTH)`
	pub fn modulation_depth(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_MODULATION_DEPTH.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_MODULATION_DEPTH)`
	pub fn set_modulation_depth(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_MODULATION_DEPTH.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_HFREFERENCE)`
	pub fn hfreference(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_HFREFERENCE.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_HFREFERENCE)`
	pub fn set_hfreference(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_HFREFERENCE.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_LFREFERENCE)`
	pub fn lfreference(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_LFREFERENCE.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_LFREFERENCE)`
	pub fn set_lfreference(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_LFREFERENCE.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_AIR_ABSORPTION_GAINHF)`
	pub fn air_absorption_gainhf(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_AIR_ABSORPTION_GAINHF.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_AIR_ABSORPTION_GAINHF)`
	pub fn set_air_absorption_gainhf(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_AIR_ABSORPTION_GAINHF.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EAXREVERB_ROOM_ROLLOFF_FACTOR)`
	pub fn room_rolloff_factor(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_ROOM_ROLLOFF_FACTOR.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EAXREVERB_ROOM_ROLLOFF_FACTOR)`
	pub fn set_room_rolloff_factor(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EAXREVERB_ROOM_ROLLOFF_FACTOR.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffecti(AL_EAXREVERB_DECAY_HFLIMIT)`
	pub fn decay_hflimit(&self) -> bool {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { efx.alGetEffecti.unwrap()(self.effect, efx.AL_EAXREVERB_DECAY_HFLIMIT.unwrap(), &mut value); }
		value == sys::AL_TRUE as sys::ALint
	}
	/// `alEffecti(AL_EAXREVERB_DECAY_HFLIMIT)`
	pub fn set_decay_hflimit(&mut self, value: bool) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffecti.unwrap()(self.effect, efx.AL_EAXREVERB_DECAY_HFLIMIT.unwrap(), if value { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.ctx.get_error()
	}
}


impl Drop for EaxReverbEffect {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let ade = efx.alDeleteEffects.unwrap();
		unsafe { ade(1, &mut self.effect as *mut sys::ALuint); }
	}
}


unsafe impl EffectTrait for ReverbEffect {
	fn new(ctx: al::Context) -> AltoResult<ReverbEffect> {
		let mut effect = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			check_effect_symbols(&efx)?;
			efx.AL_REVERB_DENSITY?;
			efx.AL_REVERB_DIFFUSION?;
			efx.AL_REVERB_GAIN?;
			efx.AL_REVERB_GAINHF?;
			efx.AL_REVERB_DECAY_TIME?;
			efx.AL_REVERB_DECAY_HFRATIO?;
			efx.AL_REVERB_REFLECTIONS_GAIN?;
			efx.AL_REVERB_REFLECTIONS_DELAY?;
			efx.AL_REVERB_LATE_REVERB_GAIN?;
			efx.AL_REVERB_LATE_REVERB_DELAY?;
			efx.AL_REVERB_AIR_ABSORPTION_GAINHF?;
			efx.AL_REVERB_ROOM_ROLLOFF_FACTOR?;
			efx.AL_REVERB_DECAY_HFLIMIT?;
			let _lock = ctx.make_current(true);
			unsafe {
				efx.alGenEffects?(1, &mut effect);
				efx.alEffecti?(effect, efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_REVERB?);
			}
			ctx.get_error()?;
		}
		Ok(ReverbEffect{ctx: ctx, effect: effect})
	}


	#[inline]
	fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl ReverbEffect {
	/// Set all effect properties based on a reverb preset.
	pub fn set_preset(&mut self, preset: &EaxReverbProperties) -> AltoResult<()> {
		let mut r = Ok(());

		r = r.and(self.set_density(preset.density));
		r = r.and(self.set_diffusion(preset.diffusion));
		r = r.and(self.set_gain(preset.gain));
		r = r.and(self.set_gainhf(preset.gainhf));
		r = r.and(self.set_decay_time(preset.decay_time));
		r = r.and(self.set_decay_hfratio(preset.decay_hfratio));
		r = r.and(self.set_reflections_gain(preset.reflections_gain));
		r = r.and(self.set_reflections_delay(preset.reflections_delay));
		r = r.and(self.set_late_reverb_gain(preset.late_reverb_gain));
		r = r.and(self.set_late_reverb_delay(preset.late_reverb_delay));
		r = r.and(self.set_air_absorption_gainhf(preset.air_absorption_gainhf));
		r = r.and(self.set_room_rolloff_factor(preset.room_rolloff_factor));
		r = r.and(self.set_decay_hflimit(preset.decay_hflimit));

		r
	}


	/// `alGetEffectf(AL_REVERB_DENSITY)`
	pub fn density(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_REVERB_DENSITY.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_REVERB_DENSITY)`
	pub fn set_density(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_REVERB_DENSITY.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_REVERB_DIFFUSION)`
	pub fn diffusion(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_REVERB_DIFFUSION.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_REVERB_DIFFUSION)`
	pub fn set_diffusion(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_REVERB_DIFFUSION.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_REVERB_GAIN)`
	pub fn gain(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_REVERB_GAIN.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_REVERB_GAIN)`
	pub fn set_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_REVERB_GAIN.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_REVERB_GAINHF)`
	pub fn gainhf(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_REVERB_GAINHF.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_REVERB_GAINHF)`
	pub fn set_gainhf(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_REVERB_GAINHF.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_REVERB_DECAY_TIME)`
	pub fn decay_time(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_REVERB_DECAY_TIME.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_REVERB_DECAY_TIME)`
	pub fn set_decay_time(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_REVERB_DECAY_TIME.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_REVERB_DECAY_HFRATIO)`
	pub fn decay_hfratio(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_REVERB_DECAY_HFRATIO.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_REVERB_DECAY_HFRATIO)`
	pub fn set_decay_hfratio(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_REVERB_DECAY_HFRATIO.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_REVERB_REFLECTIONS_GAIN)`
	pub fn reflections_gain(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_REVERB_REFLECTIONS_GAIN.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_REVERB_REFLECTIONS_GAIN)`
	pub fn set_reflections_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_REVERB_REFLECTIONS_GAIN.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_REVERB_REFLECTIONS_DELAY)`
	pub fn reflections_delay(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_REVERB_REFLECTIONS_DELAY.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_REVERB_REFLECTIONS_DELAY)`
	pub fn set_reflections_delay(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_REVERB_REFLECTIONS_DELAY.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_REVERB_LATE_REVERB_GAIN)`
	pub fn late_reverb_gain(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_REVERB_LATE_REVERB_GAIN.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_REVERB_LATE_REVERB_GAIN)`
	pub fn set_late_reverb_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_REVERB_LATE_REVERB_GAIN.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_REVERB_LATE_REVERB_DELAY)`
	pub fn late_reverb_delay(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_REVERB_LATE_REVERB_DELAY.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_REVERB_LATE_REVERB_DELAY)`
	pub fn set_late_reverb_delay(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_REVERB_LATE_REVERB_DELAY.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_REVERB_AIR_ABSORPTION_GAINHF)`
	pub fn air_absorption_gainhf(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_REVERB_AIR_ABSORPTION_GAINHF.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_REVERB_AIR_ABSORPTION_GAINHF)`
	pub fn set_air_absorption_gainhf(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_REVERB_AIR_ABSORPTION_GAINHF.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_REVERB_ROOM_ROLLOFF_FACTOR)`
	pub fn room_rolloff_factor(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_REVERB_ROOM_ROLLOFF_FACTOR.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_REVERB_ROOM_ROLLOFF_FACTOR)`
	pub fn set_room_rolloff_factor(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_REVERB_ROOM_ROLLOFF_FACTOR.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffecti(AL_REVERB_DECAY_HFLIMIT)`
	pub fn decay_hflimit(&self) -> bool {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { efx.alGetEffecti.unwrap()(self.effect, efx.AL_REVERB_DECAY_HFLIMIT.unwrap(), &mut value); }
		value == sys::AL_TRUE as sys::ALint
	}
	/// `alEffecti(AL_REVERB_DECAY_HFLIMIT)`
	pub fn set_decay_hflimit(&mut self, value: bool) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffecti.unwrap()(self.effect, efx.AL_REVERB_DECAY_HFLIMIT.unwrap(), if value { sys::AL_TRUE } else { sys::AL_FALSE } as sys::ALint); }
		self.ctx.get_error()
	}
}


impl Drop for ReverbEffect {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let ade = efx.alDeleteEffects.unwrap();
		unsafe { ade(1, &mut self.effect as *mut sys::ALuint); }
	}
}


unsafe impl EffectTrait for ChorusEffect {
	fn new(ctx: al::Context) -> AltoResult<ChorusEffect> {
		let mut effect = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			check_effect_symbols(&efx)?;
			efx.AL_CHORUS_WAVEFORM?;
			efx.AL_CHORUS_PHASE?;
			efx.AL_CHORUS_RATE?;
			efx.AL_CHORUS_DEPTH?;
			efx.AL_CHORUS_FEEDBACK?;
			efx.AL_CHORUS_DELAY?;
			let _lock = ctx.make_current(true);
			unsafe {
				efx.alGenEffects?(1, &mut effect);
				efx.alEffecti?(effect, efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_CHORUS?);
			}
			ctx.get_error()?;
		}
		Ok(ChorusEffect{ctx: ctx, effect: effect})
	}


	#[inline]
	fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl ChorusEffect {
	/// `alGetEffecti(AL_CHORUS_WAVEFORM)`
	pub fn waveform(&self) -> ChorusWaveform {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { efx.alGetEffecti.unwrap()(self.effect, efx.AL_CHORUS_WAVEFORM.unwrap(), &mut value); }
		ChorusWaveform::from_i32(value as i32).expect("ALTO ERROR: Unknown chorus waveform")
	}
	/// `alEffecti(AL_CHORUS_WAVEFORM)`
	pub fn set_waveform(&mut self, value: ChorusWaveform) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffecti.unwrap()(self.effect, efx.AL_CHORUS_WAVEFORM.unwrap(), value as sys::ALint) };
		self.ctx.get_error()
	}


	/// `alGetEffecti(AL_CHORUS_PHASE)`
	pub fn phase(&self) -> sys::ALint {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { efx.alGetEffecti.unwrap()(self.effect, efx.AL_CHORUS_PHASE.unwrap(), &mut value); }
		value
	}
	/// `alEffecti(AL_CHORUS_PHASE)`
	pub fn set_phase(&mut self, value: sys::ALint) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffecti.unwrap()(self.effect, efx.AL_CHORUS_PHASE.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_CHORUS_RATE)`
	pub fn rate(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_CHORUS_RATE.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_CHORUS_RATE)`
	pub fn set_rate(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_CHORUS_RATE.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_CHORUS_DEPTH)`
	pub fn depth(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_CHORUS_DEPTH.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_CHORUS_DEPTH)`
	pub fn set_depth(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_CHORUS_DEPTH.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_CHORUS_FEEDBACK)`
	pub fn feedback(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_CHORUS_FEEDBACK.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_CHORUS_FEEDBACK)`
	pub fn set_feedback(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_CHORUS_FEEDBACK.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_CHORUS_DELAY)`
	pub fn delay(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_CHORUS_DELAY.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_CHORUS_DELAY)`
	pub fn set_delay(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_CHORUS_DELAY.unwrap(), value); }
		self.ctx.get_error()
	}
}


impl Drop for ChorusEffect {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let ade = efx.alDeleteEffects.unwrap();
		unsafe { ade(1, &mut self.effect as *mut sys::ALuint); }
	}
}


unsafe impl EffectTrait for DistortionEffect {
	fn new(ctx: al::Context) -> AltoResult<DistortionEffect> {
		let mut effect = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			check_effect_symbols(&efx)?;
			efx.AL_DISTORTION_EDGE?;
			efx.AL_DISTORTION_GAIN?;
			efx.AL_DISTORTION_LOWPASS_CUTOFF?;
			efx.AL_DISTORTION_EQCENTER?;
			efx.AL_DISTORTION_EQBANDWIDTH?;
			let _lock = ctx.make_current(true);
			unsafe {
				efx.alGenEffects?(1, &mut effect);
				efx.alEffecti?(effect, efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_DISTORTION?);
			}
			ctx.get_error()?;
		}
		Ok(DistortionEffect{ctx: ctx, effect: effect})
	}


	#[inline]
	fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl DistortionEffect {
	/// `alGetEffectf(AL_DISTORTION_EDGE)`
	pub fn edge(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_DISTORTION_EDGE.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_DISTORTION_EDGE)`
	pub fn set_edge(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_DISTORTION_EDGE.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_DISTORTION_LOWPASS_CUTOFF)`
	pub fn lowpass_cutoff(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_DISTORTION_LOWPASS_CUTOFF.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_DISTORTION_LOWPASS_CUTOFF)`
	pub fn set_lowpass_cutoff(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_DISTORTION_LOWPASS_CUTOFF.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_DISTORTION_EQCENTER)`
	pub fn eqcenter(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_DISTORTION_EQCENTER.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_DISTORTION_EQCENTER)`
	pub fn set_eqcenter(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_DISTORTION_EQCENTER.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_DISTORTION_EQBANDWIDTH)`
	pub fn eqbandwidth(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_DISTORTION_EQBANDWIDTH.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_DISTORTION_EQBANDWIDTH)`
	pub fn set_eqbandwidth(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_DISTORTION_EQBANDWIDTH.unwrap(), value); }
		self.ctx.get_error()
	}
}


impl Drop for DistortionEffect {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let ade = efx.alDeleteEffects.unwrap();
		unsafe { ade(1, &mut self.effect as *mut sys::ALuint); }
	}
}


unsafe impl EffectTrait for EchoEffect {
	fn new(ctx: al::Context) -> AltoResult<EchoEffect> {
		let mut effect = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			check_effect_symbols(&efx)?;
			efx.AL_ECHO_DELAY?;
			efx.AL_ECHO_LRDELAY?;
			efx.AL_ECHO_DAMPING?;
			efx.AL_ECHO_FEEDBACK?;
			efx.AL_ECHO_SPREAD?;
			let _lock = ctx.make_current(true);
			unsafe {
				efx.alGenEffects?(1, &mut effect);
				efx.alEffecti?(effect, efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_ECHO?);
			}
			ctx.get_error()?;
		}
		Ok(EchoEffect{ctx: ctx, effect: effect})
	}


	#[inline]
	fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl EchoEffect {
	/// `alGetEffectf(AL_ECHO_DELAY)`
	pub fn delay(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_ECHO_DELAY.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_ECHO_DELAY)`
	pub fn set_delay(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_ECHO_DELAY.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_ECHO_LRDELAY)`
	pub fn lrdelay(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_ECHO_LRDELAY.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_ECHO_LRDELAY)`
	pub fn set_lrdelay(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_ECHO_LRDELAY.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_ECHO_DAMPING)`
	pub fn damping(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_ECHO_DAMPING.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_ECHO_DAMPING)`
	pub fn set_damping(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_ECHO_DAMPING.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_ECHO_FEEDBACK)`
	pub fn feedback(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_ECHO_FEEDBACK.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_ECHO_FEEDBACK)`
	pub fn set_feedback(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_ECHO_FEEDBACK.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_ECHO_SPREAD)`
	pub fn spread(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_ECHO_SPREAD.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_ECHO_SPREAD)`
	pub fn set_spread(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_ECHO_SPREAD.unwrap(), value); }
		self.ctx.get_error()
	}
}


impl Drop for EchoEffect {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let ade = efx.alDeleteEffects.unwrap();
		unsafe { ade(1, &mut self.effect as *mut sys::ALuint); }
	}
}


unsafe impl EffectTrait for FlangerEffect {
	fn new(ctx: al::Context) -> AltoResult<FlangerEffect> {
		let mut effect = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			check_effect_symbols(&efx)?;
			efx.AL_FLANGER_WAVEFORM?;
			efx.AL_FLANGER_PHASE?;
			efx.AL_FLANGER_RATE?;
			efx.AL_FLANGER_DEPTH?;
			efx.AL_FLANGER_FEEDBACK?;
			efx.AL_FLANGER_DELAY?;
			let _lock = ctx.make_current(true);
			unsafe {
				efx.alGenEffects?(1, &mut effect);
				efx.alEffecti?(effect, efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_FLANGER?);
			}
			ctx.get_error()?;
		}
		Ok(FlangerEffect{ctx: ctx, effect: effect})
	}


	#[inline]
	fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl FlangerEffect {
	/// `alGetEffecti(AL_FLANGER_WAVEFORM)`
	pub fn waveform(&self) -> FlangerWaveform {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { efx.alGetEffecti.unwrap()(self.effect, efx.AL_FLANGER_WAVEFORM.unwrap(), &mut value); }
		FlangerWaveform::from_i32(value as i32).expect("ALTO ERROR: Unknown flanger waveform")
	}
	/// `alEffecti(AL_FLANGER_WAVEFORM)`
	pub fn set_waveform(&mut self, value: FlangerWaveform) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffecti.unwrap()(self.effect, efx.AL_FLANGER_WAVEFORM.unwrap(), value as sys::ALint) };
		self.ctx.get_error()
	}


	/// `alGetEffecti(AL_FLANGER_PHASE)`
	pub fn phase(&self) -> sys::ALint {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { efx.alGetEffecti.unwrap()(self.effect, efx.AL_FLANGER_PHASE.unwrap(), &mut value); }
		value
	}
	/// `alEffecti(AL_FLANGER_PHASE)`
	pub fn set_phase(&mut self, value: sys::ALint) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffecti.unwrap()(self.effect, efx.AL_FLANGER_PHASE.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_FLANGER_RATE)`
	pub fn rate(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_FLANGER_RATE.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_FLANGER_RATE)`
	pub fn set_rate(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_FLANGER_RATE.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_FLANGER_DEPTH)`
	pub fn depth(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_FLANGER_DEPTH.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_FLANGER_DEPTH)`
	pub fn set_depth(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_FLANGER_DEPTH.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_FLANGER_FEEDBACK)`
	pub fn feedback(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_FLANGER_FEEDBACK.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_FLANGER_FEEDBACK)`
	pub fn set_feedback(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_FLANGER_FEEDBACK.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_FLANGER_DELAY)`
	pub fn delay(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_FLANGER_DELAY.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_FLANGER_DELAY)`
	pub fn set_delay(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_FLANGER_DELAY.unwrap(), value); }
		self.ctx.get_error()
	}
}


impl Drop for FlangerEffect {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let ade = efx.alDeleteEffects.unwrap();
		unsafe { ade(1, &mut self.effect as *mut sys::ALuint); }
	}
}


unsafe impl EffectTrait for FrequencyShifterEffect {
	fn new(ctx: al::Context) -> AltoResult<FrequencyShifterEffect> {
		let mut effect = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			check_effect_symbols(&efx)?;
			efx.AL_FREQUENCY_SHIFTER_FREQUENCY?;
			efx.AL_FREQUENCY_SHIFTER_LEFT_DIRECTION?;
			efx.AL_FREQUENCY_SHIFTER_RIGHT_DIRECTION?;
			let _lock = ctx.make_current(true);
			unsafe {
				efx.alGenEffects?(1, &mut effect);
				efx.alEffecti?(effect, efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_FREQUENCY_SHIFTER?);
			}
			ctx.get_error()?;
		}
		Ok(FrequencyShifterEffect{ctx: ctx, effect: effect})
	}


	#[inline]
	fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl FrequencyShifterEffect {
	/// `alGetEffectf(AL_FREQUENCY_SHIFTER_FREQUENCY)`
	pub fn frequency(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_FREQUENCY_SHIFTER_FREQUENCY.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_FREQUENCY_SHIFTER_FREQUENCY)`
	pub fn set_frequency(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_FREQUENCY_SHIFTER_FREQUENCY.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffecti(AL_FREQUENCY_SHIFTER_LEFT_DIRECTION)`
	pub fn left_direction(&self) -> FrequencyShifterDirection {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { efx.alGetEffecti.unwrap()(self.effect, efx.AL_FREQUENCY_SHIFTER_LEFT_DIRECTION.unwrap(), &mut value); }
		FrequencyShifterDirection::from_i32(value as i32).expect("ALTO ERROR: Unknown frequency shifter direction")
	}
	/// `alEffecti(AL_FREQUENCY_SHIFTER_LEFT_DIRECTION)`
	pub fn set_left_direction(&mut self, value: FrequencyShifterDirection) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffecti.unwrap()(self.effect, efx.AL_FREQUENCY_SHIFTER_LEFT_DIRECTION.unwrap(), value as sys::ALint); }
		self.ctx.get_error()
	}


	/// `alGetEffecti(AL_FREQUENCY_SHIFTER_RIGHT_DIRECTION)`
	pub fn right_direction(&self) -> FrequencyShifterDirection {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { efx.alGetEffecti.unwrap()(self.effect, efx.AL_FREQUENCY_SHIFTER_RIGHT_DIRECTION.unwrap(), &mut value); }
		FrequencyShifterDirection::from_i32(value as i32).expect("ALTO ERROR: Unknown frequency shifter direction")
	}
	/// `alEffecti(AL_FREQUENCY_SHIFTER_RIGHT_DIRECTION)`
	pub fn set_right_direction(&mut self, value: FrequencyShifterDirection) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffecti.unwrap()(self.effect, efx.AL_FREQUENCY_SHIFTER_RIGHT_DIRECTION.unwrap(), value as sys::ALint); }
		self.ctx.get_error()
	}
}


impl Drop for FrequencyShifterEffect {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let ade = efx.alDeleteEffects.unwrap();
		unsafe { ade(1, &mut self.effect as *mut sys::ALuint); }
	}
}


unsafe impl EffectTrait for VocalMorpherEffect {
	fn new(ctx: al::Context) -> AltoResult<VocalMorpherEffect> {
		let mut effect = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			check_effect_symbols(&efx)?;
			efx.AL_VOCAL_MORPHER_PHONEMEA?;
			efx.AL_VOCAL_MORPHER_PHONEMEB?;
			efx.AL_VOCAL_MORPHER_PHONEMEA_COARSE_TUNING?;
			efx.AL_VOCAL_MORPHER_PHONEMEB_COARSE_TUNING?;
			efx.AL_VOCAL_MORPHER_WAVEFORM?;
			efx.AL_VOCAL_MORPHER_RATE?;
			let _lock = ctx.make_current(true);
			unsafe {
				efx.alGenEffects?(1, &mut effect);
				efx.alEffecti?(effect, efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_VOCAL_MORPHER?);
			}
			ctx.get_error()?;
		}
		Ok(VocalMorpherEffect{ctx: ctx, effect: effect})
	}


	#[inline]
	fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl VocalMorpherEffect {
	/// `alGetEffecti(AL_VOCAL_MORPHER_PHONEMEA)`
	pub fn phonemea(&self) -> VocalMorpherPhoneme {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { efx.alGetEffecti.unwrap()(self.effect, efx.AL_VOCAL_MORPHER_PHONEMEA.unwrap(), &mut value); }
		VocalMorpherPhoneme::from_i32(value as i32).expect("ALTO ERROR: Unknown vocal morpher phoneme")
	}
	/// `alEffecti(AL_VOCAL_MORPHER_PHONEMEA)`
	pub fn set_phonemea(&mut self, value: VocalMorpherPhoneme) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffecti.unwrap()(self.effect, efx.AL_VOCAL_MORPHER_PHONEMEA.unwrap(), value as sys::ALint); }
		self.ctx.get_error()
	}


	/// `alGetEffecti(AL_VOCAL_MORPHER_PHONEMEB)`
	pub fn phonemeb(&self) -> VocalMorpherPhoneme {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { efx.alGetEffecti.unwrap()(self.effect, efx.AL_VOCAL_MORPHER_PHONEMEB.unwrap(), &mut value); }
		VocalMorpherPhoneme::from_i32(value as i32).expect("ALTO ERROR: Unknown vocal morpher phoneme")
	}
	/// `alEffecti(AL_VOCAL_MORPHER_PHONEMEB)`
	pub fn set_phonemeb(&mut self, value: VocalMorpherPhoneme) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffecti.unwrap()(self.effect, efx.AL_VOCAL_MORPHER_PHONEMEB.unwrap(), value as sys::ALint); }
		self.ctx.get_error()
	}


	/// `alGetEffecti(AL_VOCAL_MORPHER_PHONEMEA_COARSE_TUNING)`
	pub fn phonemea_coarse_tuning(&self) -> sys::ALint {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { efx.alGetEffecti.unwrap()(self.effect, efx.AL_VOCAL_MORPHER_PHONEMEA_COARSE_TUNING.unwrap(), &mut value); }
		value
	}
	/// `alEffecti(AL_VOCAL_MORPHER_PHONEMEA_COARSE_TUNING)`
	pub fn set_phonemea_coarse_tuning(&mut self, value: sys::ALint) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffecti.unwrap()(self.effect, efx.AL_VOCAL_MORPHER_PHONEMEA_COARSE_TUNING.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffecti(AL_VOCAL_MORPHER_PHONEMEB_COARSE_TUNING)`
	pub fn phonemeb_coarse_tuning(&self) -> sys::ALint {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { efx.alGetEffecti.unwrap()(self.effect, efx.AL_VOCAL_MORPHER_PHONEMEB_COARSE_TUNING.unwrap(), &mut value); }
		value
	}
	/// `alEffecti(AL_VOCAL_MORPHER_PHONEMEB_COARSE_TUNING)`
	pub fn set_phonemeb_coarse_tuning(&mut self, value: sys::ALint) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffecti.unwrap()(self.effect, efx.AL_VOCAL_MORPHER_PHONEMEB_COARSE_TUNING.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffecti(AL_VOCAL_MORPHER_WAVEFORM)`
	pub fn waveform(&self) -> VocalMorpherWaveform {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { efx.alGetEffecti.unwrap()(self.effect, efx.AL_VOCAL_MORPHER_WAVEFORM.unwrap(), &mut value); }
		VocalMorpherWaveform::from_i32(value as i32).expect("ALTO ERROR: Unknown vocal morpher waveform")
	}
	/// `alEffecti(AL_VOCAL_MORPHER_WAVEFORM)`
	pub fn set_waveform(&mut self, value: VocalMorpherWaveform) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffecti.unwrap()(self.effect, efx.AL_VOCAL_MORPHER_WAVEFORM.unwrap(), value as sys::ALint); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_VOCAL_MORPHER_RATE)`
	pub fn rate(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_VOCAL_MORPHER_RATE.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_VOCAL_MORPHER_RATE)`
	pub fn set_rate(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_VOCAL_MORPHER_RATE.unwrap(), value); }
		self.ctx.get_error()
	}
}


impl Drop for VocalMorpherEffect {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let ade = efx.alDeleteEffects.unwrap();
		unsafe { ade(1, &mut self.effect as *mut sys::ALuint); }
	}
}


unsafe impl EffectTrait for PitchShifterEffect {
	fn new(ctx: al::Context) -> AltoResult<PitchShifterEffect> {
		let mut effect = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			check_effect_symbols(&efx)?;
			efx.AL_PITCH_SHIFTER_COARSE_TUNE?;
			efx.AL_PITCH_SHIFTER_FINE_TUNE?;
			let _lock = ctx.make_current(true);
			unsafe {
				efx.alGenEffects?(1, &mut effect);
				efx.alEffecti?(effect, efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_PITCH_SHIFTER?);
			}
			ctx.get_error()?;
		}
		Ok(PitchShifterEffect{ctx: ctx, effect: effect})
	}


	#[inline]
	fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl PitchShifterEffect {
	/// `alGetEffecti(AL_PITCH_SHIFTER_COARSE_TUNE)`
	pub fn coarse_tune(&self) -> sys::ALint {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { efx.alGetEffecti.unwrap()(self.effect, efx.AL_PITCH_SHIFTER_COARSE_TUNE.unwrap(), &mut value); }
		value
	}
	/// `alEffecti(AL_PITCH_SHIFTER_COARSE_TUNE)`
	pub fn set_coarse_tune(&mut self, value: sys::ALint) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffecti.unwrap()(self.effect, efx.AL_PITCH_SHIFTER_COARSE_TUNE.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffecti(AL_PITCH_SHIFTER_FINE_TUNE)`
	pub fn fine_tune(&self) -> sys::ALint {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { efx.alGetEffecti.unwrap()(self.effect, efx.AL_PITCH_SHIFTER_FINE_TUNE.unwrap(), &mut value); }
		value
	}
	/// `alEffecti(AL_PITCH_SHIFTER_FINE_TUNE)`
	pub fn set_fine_tune(&mut self, value: sys::ALint) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffecti.unwrap()(self.effect, efx.AL_PITCH_SHIFTER_FINE_TUNE.unwrap(), value); }
		self.ctx.get_error()
	}
}


impl Drop for PitchShifterEffect {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let ade = efx.alDeleteEffects.unwrap();
		unsafe { ade(1, &mut self.effect as *mut sys::ALuint); }
	}
}


unsafe impl EffectTrait for RingModulatorEffect {
	fn new(ctx: al::Context) -> AltoResult<RingModulatorEffect> {
		let mut effect = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			check_effect_symbols(&efx)?;
			efx.AL_RING_MODULATOR_FREQUENCY?;
			efx.AL_RING_MODULATOR_HIGHPASS_CUTOFF?;
			efx.AL_RING_MODULATOR_WAVEFORM?;
			let _lock = ctx.make_current(true);
			unsafe {
				efx.alGenEffects?(1, &mut effect);
				efx.alEffecti?(effect, efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_RING_MODULATOR?);
			}
			ctx.get_error()?;
		}
		Ok(RingModulatorEffect{ctx: ctx, effect: effect})
	}


	#[inline]
	fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl RingModulatorEffect {
	/// `alGetEffectf(AL_RING_MODULATOR_FREQUENCY)`
	pub fn frequency(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_RING_MODULATOR_FREQUENCY.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_RING_MODULATOR_FREQUENCY)`
	pub fn set_frequency(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_RING_MODULATOR_FREQUENCY.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_RING_MODULATOR_HIGHPASS_CUTOFF)`
	pub fn highpass_cutoff(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_RING_MODULATOR_HIGHPASS_CUTOFF.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_RING_MODULATOR_HIGHPASS_CUTOFF)`
	pub fn set_highpass_cutoff(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_RING_MODULATOR_HIGHPASS_CUTOFF.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffecti(AL_RING_MODULATOR_WAVEFORM)`
	pub fn waveform(&self) -> RingModulatorWaveform {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { efx.alGetEffecti.unwrap()(self.effect, efx.AL_RING_MODULATOR_WAVEFORM.unwrap(), &mut value); }
		RingModulatorWaveform::from_i32(value as i32).expect("ALTO ERROR: Unknown ring modulator waveform")
	}
	/// `alEffecti(AL_RING_MODULATOR_WAVEFORM)`
	pub fn set_waveform(&mut self, value: ChorusWaveform) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffecti.unwrap()(self.effect, efx.AL_RING_MODULATOR_WAVEFORM.unwrap(), value as sys::ALint) };
		self.ctx.get_error()
	}
}


impl Drop for RingModulatorEffect {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let ade = efx.alDeleteEffects.unwrap();
		unsafe { ade(1, &mut self.effect as *mut sys::ALuint); }
	}
}


unsafe impl EffectTrait for AutowahEffect {
	fn new(ctx: al::Context) -> AltoResult<AutowahEffect> {
		let mut effect = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			check_effect_symbols(&efx)?;
			efx.AL_AUTOWAH_ATTACK_TIME?;
			efx.AL_AUTOWAH_RELEASE_TIME?;
			efx.AL_AUTOWAH_RESONANCE?;
			efx.AL_AUTOWAH_PEAK_GAIN?;
			let _lock = ctx.make_current(true);
			unsafe {
				efx.alGenEffects?(1, &mut effect);
				efx.alEffecti?(effect, efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_AUTOWAH?);
			}
			ctx.get_error()?;
		}
		Ok(AutowahEffect{ctx: ctx, effect: effect})
	}


	#[inline]
	fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl AutowahEffect {
	/// `alGetEffectf(AL_AUTOWAH_ATTACK_TIME)`
	pub fn attack_time(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_AUTOWAH_ATTACK_TIME.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_AUTOWAH_ATTACK_TIME)`
	pub fn set_attack_time(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_AUTOWAH_ATTACK_TIME.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_AUTOWAH_RELEASE_TIME)`
	pub fn release_time(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_AUTOWAH_RELEASE_TIME.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_AUTOWAH_RELEASE_TIME)`
	pub fn set_release_time(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_AUTOWAH_RELEASE_TIME.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_AUTOWAH_RESONANCE)`
	pub fn resonance(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_AUTOWAH_RESONANCE.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_AUTOWAH_RESONANCE)`
	pub fn set_resonance(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_AUTOWAH_RESONANCE.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_AUTOWAH_PEAK_GAIN)`
	pub fn peak_gain(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_AUTOWAH_PEAK_GAIN.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_AUTOWAH_PEAK_GAIN)`
	pub fn set_peak_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_AUTOWAH_PEAK_GAIN.unwrap(), value); }
		self.ctx.get_error()
	}
}


impl Drop for AutowahEffect {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let ade = efx.alDeleteEffects.unwrap();
		unsafe { ade(1, &mut self.effect as *mut sys::ALuint); }
	}
}


unsafe impl EffectTrait for CompressorEffect {
	fn new(ctx: al::Context) -> AltoResult<CompressorEffect> {
		let mut effect = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			check_effect_symbols(&efx)?;
			efx.AL_COMPRESSOR_ONOFF?;
			let _lock = ctx.make_current(true);
			unsafe {
				efx.alGenEffects?(1, &mut effect);
				efx.alEffecti?(effect, efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_COMPRESSOR?);
			}
			ctx.get_error()?;
		}
		Ok(CompressorEffect{ctx: ctx, effect: effect})
	}


	#[inline]
	fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl CompressorEffect {
	/// `alGetEffecti(AL_COMPRESSOR_ONOFF)`
	pub fn onoff(&self) -> bool {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0;
		unsafe { efx.alGetEffecti.unwrap()(self.effect, efx.AL_COMPRESSOR_ONOFF.unwrap(), &mut value); }
		value == 1 as sys::ALint
	}
	/// `alEffecti(AL_COMPRESSOR_ONOFF)`
	pub fn set_onoff(&mut self, value: bool) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffecti.unwrap()(self.effect, efx.AL_COMPRESSOR_ONOFF.unwrap(), if value { 1 } else { 0 } as sys::ALint); }
		self.ctx.get_error()
	}
}


impl Drop for CompressorEffect {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let ade = efx.alDeleteEffects.unwrap();
		unsafe { ade(1, &mut self.effect as *mut sys::ALuint); }
	}
}


unsafe impl EffectTrait for EqualizerEffect {
	fn new(ctx: al::Context) -> AltoResult<EqualizerEffect> {
		let mut effect = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			check_effect_symbols(&efx)?;
			efx.AL_EQUALIZER_LOW_GAIN?;
			efx.AL_EQUALIZER_LOW_CUTOFF?;
			efx.AL_EQUALIZER_MID1_GAIN?;
			efx.AL_EQUALIZER_MID1_CENTER?;
			efx.AL_EQUALIZER_MID1_WIDTH?;
			efx.AL_EQUALIZER_MID2_GAIN?;
			efx.AL_EQUALIZER_MID2_CENTER?;
			efx.AL_EQUALIZER_MID2_WIDTH?;
			efx.AL_EQUALIZER_HIGH_GAIN?;
			efx.AL_EQUALIZER_HIGH_CUTOFF?;
			let _lock = ctx.make_current(true);
			unsafe {
				efx.alGenEffects?(1, &mut effect);
				efx.alEffecti?(effect, efx.AL_EFFECT_TYPE?, efx.AL_EFFECT_EQUALIZER?);
			}
			ctx.get_error()?;
		}
		Ok(EqualizerEffect{ctx: ctx, effect: effect})
	}


	#[inline]
	fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl EqualizerEffect {
	/// `alGetEffectf(AL_EQUALIZER_LOW_GAIN)`
	pub fn low_gain(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_LOW_GAIN.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EQUALIZER_LOW_GAIN)`
	pub fn set_low_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_LOW_GAIN.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EQUALIZER_LOW_CUTOFF)`
	pub fn low_cutoff(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_LOW_CUTOFF.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EQUALIZER_LOW_CUTOFF)`
	pub fn set_low_cutoff(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_LOW_CUTOFF.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EQUALIZER_MID1_GAIN)`
	pub fn mid1_gain(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_MID1_GAIN.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EQUALIZER_MID1_GAIN)`
	pub fn set_mid1_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_MID1_GAIN.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EQUALIZER_MID1_CENTER)`
	pub fn mid1_center(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_MID1_CENTER.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EQUALIZER_MID1_CENTER)`
	pub fn set_mid1_center(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_MID1_CENTER.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EQUALIZER_MID1_WIDTH)`
	pub fn mid1_width(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_MID1_WIDTH.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EQUALIZER_MID1_WIDTH)`
	pub fn set_mid1_width(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_MID1_WIDTH.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EQUALIZER_MID2_GAIN)`
	pub fn mid2_gain(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_MID2_GAIN.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EQUALIZER_MID2_GAIN)`
	pub fn set_mid2_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_MID2_GAIN.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EQUALIZER_MID2_CENTER)`
	pub fn mid2_center(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_MID2_CENTER.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EQUALIZER_MID2_CENTER)`
	pub fn set_mid2_center(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_MID2_CENTER.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EQUALIZER_MID2_WIDTH)`
	pub fn mid2_width(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_MID2_WIDTH.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EQUALIZER_MID2_WIDTH)`
	pub fn set_mid2_width(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_MID2_WIDTH.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EQUALIZER_HIGH_GAIN)`
	pub fn high_gain(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_HIGH_GAIN.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EQUALIZER_HIGH_GAIN)`
	pub fn set_high_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_HIGH_GAIN.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetEffectf(AL_EQUALIZER_HIGH_CUTOFF)`
	pub fn high_cutoff(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_HIGH_CUTOFF.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EQUALIZER_HIGH_CUTOFF)`
	pub fn set_high_cutoff(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, efx.AL_EQUALIZER_HIGH_CUTOFF.unwrap(), value); }
		self.ctx.get_error()
	}
}


impl Drop for EqualizerEffect {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let ade = efx.alDeleteEffects.unwrap();
		unsafe { ade(1, &mut self.effect as *mut sys::ALuint); }
	}
}


unsafe impl EffectTrait for DedicatedLowFrequencyEffect {
	fn new(ctx: al::Context) -> AltoResult<DedicatedLowFrequencyEffect> {
		let mut effect = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			check_effect_symbols(&efx)?;
			let d = ctx.0.dev.0.exts.ALC_EXT_DEDICATED()?;
			d.AL_EFFECT_DEDICATED_GAIN?;
			let _lock = ctx.make_current(true);
			unsafe {
				efx.alGenEffects?(1, &mut effect);
				efx.alEffecti?(effect, efx.AL_EFFECT_TYPE?, d.AL_EFFECT_DEDICATED_LOW_FREQUENCY_EFFECT?);
			}
			ctx.get_error()?;
		}
		Ok(DedicatedLowFrequencyEffect{ctx: ctx, effect: effect})
	}


	#[inline]
	fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl DedicatedLowFrequencyEffect {
	/// `alGetEffectf(AL_EFFECT_DEDICATED_GAIN)`
	pub fn gain(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let d = self.ctx.0.dev.0.exts.ALC_EXT_DEDICATED().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, d.AL_EFFECT_DEDICATED_GAIN.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EFFECT_DEDICATED_GAIN)`
	pub fn set_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let d = self.ctx.0.dev.0.exts.ALC_EXT_DEDICATED().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, d.AL_EFFECT_DEDICATED_GAIN.unwrap(), value); }
		self.ctx.get_error()
	}
}


impl Drop for DedicatedLowFrequencyEffect {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let ade = efx.alDeleteEffects.unwrap();
		unsafe { ade(1, &mut self.effect as *mut sys::ALuint); }
	}
}


unsafe impl EffectTrait for DedicatedDialogueEffect {
	fn new(ctx: al::Context) -> AltoResult<DedicatedDialogueEffect> {
		let mut effect = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			check_effect_symbols(&efx)?;
			let d = ctx.0.dev.0.exts.ALC_EXT_DEDICATED()?;
			d.AL_EFFECT_DEDICATED_GAIN?;
			let _lock = ctx.make_current(true);
			unsafe {
				efx.alGenEffects?(1, &mut effect);
				efx.alEffecti?(effect, efx.AL_EFFECT_TYPE?, d.AL_EFFECT_DEDICATED_DIALOGUE?);
			}
			ctx.get_error()?;
		}
		Ok(DedicatedDialogueEffect{ctx: ctx, effect: effect})
	}


	#[inline]
	fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.effect }
}


impl DedicatedDialogueEffect {
	/// `alGetEffectf(AL_EFFECT_DEDICATED_GAIN)`
	pub fn gain(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let d = self.ctx.0.dev.0.exts.ALC_EXT_DEDICATED().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetEffectf.unwrap()(self.effect, d.AL_EFFECT_DEDICATED_GAIN.unwrap(), &mut value); }
		value
	}
	/// `alEffectf(AL_EFFECT_DEDICATED_GAIN)`
	pub fn set_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let d = self.ctx.0.dev.0.exts.ALC_EXT_DEDICATED().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alEffectf.unwrap()(self.effect, d.AL_EFFECT_DEDICATED_GAIN.unwrap(), value); }
		self.ctx.get_error()
	}
}


impl Drop for DedicatedDialogueEffect {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let ade = efx.alDeleteEffects.unwrap();
		unsafe { ade(1, &mut self.effect as *mut sys::ALuint); }
	}
}


fn check_filter_symbols(efx: &ext::ALC_EXT_EFX) -> AltoResult<()> {
	efx.alGetFilteri?;
	efx.alGetFilterf?;
	efx.alGetFilteriv?;
	efx.alGetFilterfv?;
	efx.alFilterf?;
	efx.alFilteriv?;
	efx.alFilterfv?;
	efx.alDeleteFilters?;

	Ok(())
}


unsafe impl FilterTrait for LowpassFilter {
	fn new(ctx: al::Context) -> AltoResult<LowpassFilter> {
		let mut filter = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			check_filter_symbols(&efx)?;
			efx.AL_LOWPASS_GAIN?;
			efx.AL_LOWPASS_GAINHF?;
			let _lock = ctx.make_current(true);
			unsafe {
				efx.alGenFilters?(1, &mut filter);
				efx.alFilteri?(filter, efx.AL_FILTER_TYPE?, efx.AL_FILTER_LOWPASS?);
			}
			ctx.get_error()?;
		}
		Ok(LowpassFilter{ctx: ctx, filter: filter})
	}


	#[inline]
	fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.filter }
}


impl LowpassFilter {
	/// `alGetFilterf(AL_LOWPASS_GAIN)`
	pub fn gain(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetFilterf.unwrap()(self.filter, efx.AL_LOWPASS_GAIN.unwrap(), &mut value); }
		value
	}
	/// `alFilterf(AL_LOWPASS_GAIN)`
	pub fn set_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alFilterf.unwrap()(self.filter, efx.AL_LOWPASS_GAIN.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetFilterf(AL_LOWPASS_GAINHF)`
	pub fn gainhf(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetFilterf.unwrap()(self.filter, efx.AL_LOWPASS_GAINHF.unwrap(), &mut value); }
		value
	}
	/// `alFilterf(AL_LOWPASS_GAINHF)`
	pub fn set_gainhf(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alFilterf.unwrap()(self.filter, efx.AL_LOWPASS_GAINHF.unwrap(), value); }
		self.ctx.get_error()
	}
}


impl Drop for LowpassFilter {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let adf = efx.alDeleteFilters.unwrap();
		unsafe { adf(1, &mut self.filter as *mut sys::ALuint); }
	}
}


unsafe impl FilterTrait for HighpassFilter {
	fn new(ctx: al::Context) -> AltoResult<HighpassFilter> {
		let mut filter = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			check_filter_symbols(&efx)?;
			efx.AL_HIGHPASS_GAIN?;
			efx.AL_HIGHPASS_GAINLF?;
			let _lock = ctx.make_current(true);
			unsafe {
				efx.alGenFilters?(1, &mut filter);
				efx.alFilteri?(filter, efx.AL_FILTER_TYPE?, efx.AL_FILTER_HIGHPASS?);
			}
			ctx.get_error()?;
		}
		Ok(HighpassFilter{ctx: ctx, filter: filter})
	}


	#[inline]
	fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.filter }
}


impl HighpassFilter {
	/// `alGetFilterf(AL_HIGHPASS_GAIN)`
	pub fn gain(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetFilterf.unwrap()(self.filter, efx.AL_HIGHPASS_GAIN.unwrap(), &mut value); }
		value
	}
	/// `alFilterf(AL_HIGHPASS_GAIN)`
	pub fn set_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alFilterf.unwrap()(self.filter, efx.AL_HIGHPASS_GAIN.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetFilterf(AL_HIGHPASS_GAINLF)`
	pub fn gainlf(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetFilterf.unwrap()(self.filter, efx.AL_HIGHPASS_GAINLF.unwrap(), &mut value); }
		value
	}
	/// `alFilterf(AL_HIGHPASS_GAINLF)`
	pub fn set_gainlf(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alFilterf.unwrap()(self.filter, efx.AL_HIGHPASS_GAINLF.unwrap(), value); }
		self.ctx.get_error()
	}
}


impl Drop for HighpassFilter {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let adf = efx.alDeleteFilters.unwrap();
		unsafe { adf(1, &mut self.filter as *mut sys::ALuint); }
	}
}


unsafe impl FilterTrait for BandpassFilter {
	fn new(ctx: al::Context) -> AltoResult<BandpassFilter> {
		let mut filter = 0;
		{
			let efx = ctx.0.dev.0.exts.ALC_EXT_EFX()?;
			check_filter_symbols(&efx)?;
			efx.AL_BANDPASS_GAIN?;
			efx.AL_BANDPASS_GAINLF?;
			efx.AL_BANDPASS_GAINHF?;
			let _lock = ctx.make_current(true);
			unsafe {
				efx.alGenFilters?(1, &mut filter);
				efx.alFilteri?(filter, efx.AL_FILTER_TYPE?, efx.AL_FILTER_BANDPASS?);
			}
			ctx.get_error()?;
		}
		Ok(BandpassFilter{ctx: ctx, filter: filter})
	}


	#[inline]
	fn context(&self) -> &al::Context { &self.ctx }
	#[inline]
	fn as_raw(&self) -> sys::ALuint { self.filter }
}


impl BandpassFilter {
	/// `alGetFilterf(AL_BANDPASS_GAIN)`
	pub fn gain(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetFilterf.unwrap()(self.filter, efx.AL_BANDPASS_GAIN.unwrap(), &mut value); }
		value
	}
	/// `alFilterf(AL_BANDPASS_GAIN)`
	pub fn set_gain(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alFilterf.unwrap()(self.filter, efx.AL_BANDPASS_GAIN.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetFilterf(AL_BANDPASS_GAINLF)`
	pub fn gainlf(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetFilterf.unwrap()(self.filter, efx.AL_BANDPASS_GAINLF.unwrap(), &mut value); }
		value
	}
	/// `alFilterf(AL_BANDPASS_GAINLF)`
	pub fn set_gainlf(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alFilterf.unwrap()(self.filter, efx.AL_BANDPASS_GAINLF.unwrap(), value); }
		self.ctx.get_error()
	}


	/// `alGetFilterf(AL_BANDPASS_GAINHF)`
	pub fn gainhf(&self) -> f32 {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let mut value = 0.0;
		unsafe { efx.alGetFilterf.unwrap()(self.filter, efx.AL_BANDPASS_GAINHF.unwrap(), &mut value); }
		value
	}
	/// `alFilterf(AL_BANDPASS_GAINHF)`
	pub fn set_gainhf(&mut self, value: f32) -> AltoResult<()> {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		unsafe { efx.alFilterf.unwrap()(self.filter, efx.AL_BANDPASS_GAINHF.unwrap(), value); }
		self.ctx.get_error()
	}
}


impl Drop for BandpassFilter {
	fn drop(&mut self) {
		let efx = self.ctx.0.dev.0.exts.ALC_EXT_EFX().unwrap();
		let _lock = self.ctx.make_current(true);
		let adf = efx.alDeleteFilters.unwrap();
		unsafe { adf(1, &mut self.filter as *mut sys::ALuint); }
	}
}
