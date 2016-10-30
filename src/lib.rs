use std::marker::PhantomData;

mod al;
mod alc;
mod alext;
mod efx_presets;



pub mod ffi {
	pub use al::*;
	pub use alc::*;
	pub use alext::*;
	pub use efx_presets::*;
}



pub struct Device(*mut alc::ALCdevice);
pub struct Context<'d>(*mut alc::ALCcontext, PhantomData<&'d alc::ALCdevice>);



impl Device {
}



impl<'d> Context<'d> {
}
