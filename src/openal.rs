#![link(name = "openal",
       vers = "0.1",
       uuid = "9450DF9F-7A40-4087-BF6C-0848693D15DC",
       author = "Brendan Zabarauskas",
       url = "https://github.com/bjz/openal-rs")]

#![crate_type = "lib"]

extern crate libc;

pub mod al;
pub mod alc;

#[cfg(target_os = "macos")]
#[link(name = "OpenAL", kind = "framework")]
extern {}

#[cfg(target_os = "linux")]
#[link(name = "openal")]
extern {}

#[cfg(all(target_os = "windows", target_env = "msvc"))]
#[link(name = "OpenAL32")]
extern {}

#[cfg(all(target_os = "windows", target_env = "gnu"))]
#[link(name = "openal")]
extern {}
