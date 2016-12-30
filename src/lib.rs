//! #Overview
//! Alto is an idiomatic wrapper for the OpenAL 3D audio API and associated extensions
//! (EFX support is still WIP). This documentation will describe how the API was adapted
//! for rust, but for more general information about OpenAL, the official documentation
//! should be consulted.
//!
//! The core of the API is the [`Alto`](struct.Alto.html) struct. From this struct audio
//! devices can be enumerated and opened. Once a [`Device`](struct.Device.html) or
//! [`LoopbackDevice`](struct.LoopbackDevice.html) is opened, a [`Context`](struct.Context.html)
//! can be created from it. The context governs properties of the listener and allows you to
//! allocate audio [`Buffer`](struct.Buffer.html)s. These buffers can then be played with either
//! a [`StaticSource`](struct.StaticSource.html) or [`StreamingSource`](struct.StreamingSource.html),
//! which are also allocated from the context.


#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rental;
extern crate al_sys;


mod alc;
pub use alc::*;


mod al;
pub use al::*;


pub mod ext;


pub mod sys {
	pub use al_sys::*;
}
