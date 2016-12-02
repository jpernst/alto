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
