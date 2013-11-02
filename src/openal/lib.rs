#[link(name = "openal",
       vers = "0.1",
       uuid = "9450DF9F-7A40-4087-BF6C-0848693D15DC",
       author = "Brendan Zabarauskas",
       url = "https://github.com/bjz/openal-rs")];

#[comment = "OpenAL 1.1 bindings for Rust."];
#[crate_type = "lib"];

#[feature(macro_rules)];
#[feature(globs)];

pub mod al;
pub mod alc;

#[nolink]
#[link_args="-framework OpenAL"]
#[cfg(target_os = "macos")]
extern {}

#[nolink]
#[link_args="-lopenal"]
#[cfg(target_os = "linux")]
extern {}
