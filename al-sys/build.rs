extern crate pkg_config;
//extern crate cmake;


fn main() {
	if cfg!(feature = "use-system") {
		if pkg_config::Config::new().statik(false).probe("openal").is_ok() {
			return;
		}
	}

//	let mut config = cmake::Config::new("openal-soft-1.17.2");
//	config
//		.define("ALSOFT_TESTS", "0")
//		.define("ALSOFT_UTILS", "0")
//		.define("ALSOFT_NO_CONFIG_UTIL", "1")
//		.define("ALSOFT_EXAMPLES", "0")
//		.define("ALSOFT_CONFIG", "0")
//		.define("ALSOFT_INSTALL", "0")
//		.define("ALSOFT_HRTF_DEFS", "0");
//
//	if !cfg!(feature = "native-backends") {
//		config
//			.define("ALSOFT_BACKEND_ALSA", "0")
//			.define("ALSOFT_BACKEND_OSS", "0")
//			.define("ALSOFT_BACKEND_SOLARIS", "0")
//			.define("ALSOFT_BACKEND_SNDIO", "0")
//			.define("ALSOFT_BACKEND_QSA", "0")
//			.define("ALSOFT_BACKEND_WINMM", "0")
//			.define("ALSOFT_BACKEND_DSOUND", "0")
//			.define("ALSOFT_BACKEND_MMDEVAPI", "0")
//			.define("ALSOFT_BACKEND_PORTAUDIO", "0")
//			.define("ALSOFT_BACKEND_PULSEAUDIO", "0")
//			.define("ALSOFT_BACKEND_JACK", "0")
//			.define("ALSOFT_BACKEND_COREAUDIO", "0")
//			.define("ALSOFT_BACKEND_OPENSL", "0")
//			.define("ALSOFT_BACKEND_WAVE", "0");
//	}
//
//	let dst = config.build();
//	println!("cargo:rustc-link-search={}", dst.display());
//	println!("cargo:rustc-link-lib=dylib=openal");
}
