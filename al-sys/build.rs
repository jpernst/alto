extern crate cmake;

use cmake::Config;
use std::{env, path::PathBuf, process::Command};

// can be overridden by setting the env var ANDROID_NATIVE_API_LEVEL
const DEFAULT_ANDROID_API_LEVEL: &'static str = "16";

const OPENAL_SOFT_TAG: &'static str = "openal-soft-1.19.1";

const OPENAL_REPO: &'static str = "https://github.com/kcat/openal-soft.git";

fn clone_openalsoft() -> PathBuf {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("openal-soft");
    let status = Command::new("git")
        .arg("clone")
        .args(&["--branch", OPENAL_SOFT_TAG])
        .args(&["--depth", "1"])
        .arg(OPENAL_REPO)
        .arg(&out)
        .status()
        .unwrap();
    if !status.success() {
        let status = Command::new("git")
                .arg("clean")
                .arg("-fdx")
                .current_dir(&out)
                .status()
                .unwrap();
        assert!(status.success(), "failed to clone openal-soft");
        let status = Command::new("git")
                .arg("checkout")
                .arg(format!("tags/{}", OPENAL_SOFT_TAG))
                .current_dir(&out)
                .status()
                .unwrap();
        assert!(status.success(), "failed to clone openal-soft");
    }
    out
}

fn build_openalsoft(openal_dir: PathBuf) {
    let target = &*env::var("TARGET").unwrap();
    let ndk_dir = &*env::var("NDK_HOME").expect("set the environment variable `NDK_HOME` to the ndk directory to build `al-sys` for android");

    let toolchain_file = PathBuf::from(ndk_dir).join("build/cmake/android.toolchain.cmake");
    let abi = match target {
        "aarch64-linux-android" => "arm64-v8a",
        "armv7-linux-androideabi" => "armeabi-v7a",
        "arm-linux-androideabi" => "armeabi",
        "thumbv7neon-linux-androideabi" => "armeabi", // TODO: is this correct?
        "i686-linux-android" => "x86",
        "x86_64-linux-android" => "x86_64",
        _ => unreachable!(),
    };
    let libtype = match env::var("CARGO_FEATURE_DYNAMIC") {
        Ok(_) => "SHARED",
        _ => "STATIC"
    };
    let api_level = env::var("ANDROID_NATIVE_API_LEVEL").unwrap_or(DEFAULT_ANDROID_API_LEVEL.to_owned());
    let platform = &*format!("android-{}", api_level);

    let dst = Config::new(openal_dir)
        .define("CMAKE_TOOLCHAIN_FILE", toolchain_file)
        .define("ANDROID_ABI", abi)
        .define("ALSOFT_UTILS", "OFF")
        .define("ALSOFT_EXAMPLES", "OFF")
        .define("ALSOFT_TESTS", "OFF")
        .define("ANDROID_NDK", ndk_dir)
        .define("LIBTYPE", libtype)
        .define("ANDROID_NATIVE_API_LEVEL", api_level)
        .define("ANDROID_PLATFORM", platform)
        .no_build_target(true)
        .build();
    println!("cargo:rerun-if-env-changed=ANDROID_NATIVE_API_LEVEL");
    println!("cargo:rerun-if-env-changed=NDK_HOME");
    println!("cargo:rustc-link-search=native={}/build", dst.display());

    let link_type = match env::var("CARGO_FEATURE_DYNAMIC") {
        Ok(_) => "dylib",
        _ => "static"
    };
    println!("cargo:rustc-link-lib={}=common", link_type);
    println!("cargo:rustc-link-lib={}=openal", link_type);
}

fn main() {
    let target = &*env::var("TARGET").unwrap();
    match target {
        "aarch64-linux-android"
            | "armv7-linux-androideabi"
            | "arm-linux-androideabi"
            | "thumbv7neon-linux-androideabi"
            | "i686-linux-android"
            | "x86_64-linux-android" => {
                let repo_path = clone_openalsoft();
                build_openalsoft(repo_path)
            },
        _ => {}
    }
}
