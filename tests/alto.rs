extern crate alto;

use std::ffi::CStr;

use alto::{Alto, Capture, DeviceObject, Stereo};

type MyCapture = Capture<Stereo<i16>>;

fn load_alto() -> Alto {
    Alto::load_default().unwrap()
}

fn open_cap(a: &Alto, spec: Option<&CStr>) -> MyCapture {
    a.open_capture(spec, 4096, 1024).unwrap()
}

#[test]
fn load_default() {
    load_alto();
}

#[test]
fn default_output() {
    let a = load_alto();

    let def = a.default_output().unwrap();

    println!("{:?}", def.to_str().unwrap());

    let d_def = a.open(Some(&def)).unwrap();
    let d_none = a.open(None).unwrap();

    assert_eq!(d_def.specifier(), d_none.specifier());
}

#[test]
fn specified_output() {
    let a = load_alto();
    let devices = a.enumerate_outputs();

    for device in devices {
        let dev = a.open(Some(&device)).unwrap();
        assert_eq!(dev.specifier().unwrap(), device.as_ref());
    }
}

#[test]
fn default_input() {
    let a = load_alto();

    let def = a.default_capture().unwrap();
    let d_def = open_cap(&a, Some(&def));
    let d_none = open_cap(&a, Some(&def));

    assert_eq!(d_def.specifier(), d_none.specifier());
}

#[test]
fn specified_input() {
    let a = load_alto();
    let devices = a.enumerate_captures();

    for device in devices {
        let dev = open_cap(&a, Some(&device));
        assert_eq!(dev.specifier().unwrap(), device.as_ref());
    }
}
