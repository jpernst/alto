/**
 * Basic ALC wrapper functions
 */

use ll::*;
use types::*;

// pub fn create_context(device: *ALCdevice, attrlist: *ALCint) -> *ALCcontext {
//     unsafe { alcCreateContext(device, attrlist) }
// }

pub fn make_context_current(context: *ALCcontext) -> bool {
    unsafe { alcMakeContextCurrent(context) as bool }
}

pub fn process_context(context: *ALCcontext) {
    unsafe { alcProcessContext(context); }
}

pub fn suspend_context(context: *ALCcontext) {
    unsafe { alcSuspendContext(context); }
}

pub fn destroy_context(context: *ALCcontext) {
    unsafe { alcDestroyContext(context); }
}

pub fn get_current_context() -> *ALCcontext {
    unsafe { alcGetCurrentContext() }
}

pub fn get_contexts_device(context: *ALCcontext) -> *ALCdevice {
    unsafe { alcGetContextsDevice(context) }
}

pub fn open_device(devicename: &str) -> *ALCdevice {
    unsafe { alcOpenDevice(str::as_c_str(devicename, |s| s)) }
}

pub fn close_device(device: *ALCdevice) -> bool {
    unsafe { alcCloseDevice(device) as bool }
}

pub fn get_error(device: *ALCdevice) -> ALCenum {
    unsafe { alcGetError(device) }
}

pub fn is_extension_present(device: *ALCdevice, extname: &str) -> bool {
    unsafe { alcIsExtensionPresent(device, str::as_c_str(extname, |s| s)) as bool }
}

pub fn get_proc_address(device: *ALCdevice, funcname: ~str) -> extern fn() {
    unsafe { cast::transmute(
        alcGetProcAddress(device, str::as_c_str(funcname, |s| s))
    ) }
}

pub fn get_enum_value(device: *ALCdevice, enumname: &str) -> ALCenum {
    unsafe { alcGetEnumValue(device, str::as_c_str(enumname, |s| s)) }
}

// pub fn get_string(device: *ALCdevice, param: ALCenum) -> *ALCchar {
//     unsafe { alcGetString(device, param) }
// }

// pub fn GetIntegerv(device: *ALCdevice, param: ALCenum, size: ALCsizei, data: *ALCint) {
//     unsafe { alcGetIntegerv(); }
// }

pub fn capture_open_device(devicename: *ALCchar, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> *ALCdevice {
    unsafe { alcCaptureOpenDevice(devicename, frequency, format, buffersize) }
}

pub fn capture_close_device(device: *ALCdevice) -> bool {
    unsafe { alcCaptureCloseDevice(device) as bool }
}

pub fn capture_start(device: *ALCdevice) {
    unsafe { alcCaptureStart(device); }
}

pub fn capture_stop(device: *ALCdevice) {
    unsafe { alcCaptureStop(device); }
}

// pub fn CaptureSamples(device: *ALCdevice, buffer: *ALCvoid, samples: ALCsizei) {
//     unsafe { alcCaptureSamples(); }
// }
