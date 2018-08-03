extern crate alto;

use std::ffi::CStr;
use alto::*;

type MyCapture = Capture<Stereo<i16>>;


fn main() {
    let a = load_alto();
    let devices = a.enumerate_captures();

    for device in devices {
        let dev = open_cap(&a, Some(&device));
        assert_eq!(dev.specifier().unwrap(), device.as_ref());
    }
}

fn load_alto() -> Alto {
    Alto::load_default().unwrap()
}

fn open_cap(a: &Alto, spec: Option<&CStr>) -> MyCapture {
    a.open_capture(spec, 4096, 1024).unwrap()
}

