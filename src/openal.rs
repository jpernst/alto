#![link(name = "openal",
       vers = "0.1",
       uuid = "9450DF9F-7A40-4087-BF6C-0848693D15DC",
       author = "Brendan Zabarauskas",
       url = "https://github.com/bjz/openal-rs")]

#![crate_type = "lib"]

extern crate libc;

pub mod al;
pub mod alc;

#[link(name = "OpenAL", kind = "framework")]
#[cfg(target_os = "macos")]
extern {}

#[link(name = "openal")]
#[cfg(target_os = "linux")]
extern {}
