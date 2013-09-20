use types::*;
use ffi;
use std::{cast, vec};

// TODO: not sure what types these are meant to be...
pub static INVALID                              : ALCboolean = 0;
pub static VERSION_0_1                          : ALCboolean = 1;

pub static FALSE                                : ALCboolean = 0;
pub static TRUE                                 : ALCboolean = 1;

pub static FREQUENCY                            : ALCint = 0x1007;
pub static REFRESH                              : ALCint = 0x1008;
pub static SYNC                                 : ALCint = 0x1009;
pub static MONO_SOURCES                         : ALCint = 0x1010;
pub static STEREO_SOURCES                       : ALCint = 0x1011;

pub static NO_ERROR                             : ALCenum = FALSE as ALCenum;
pub static INVALID_DEVICE                       : ALCenum = 0xA001;
pub static INVALID_CONTEXT                      : ALCenum = 0xA002;
pub static INVALID_ENUM                         : ALCenum = 0xA003;
pub static INVALID_VALUE                        : ALCenum = 0xA004;
pub static OUT_OF_MEMORY                        : ALCenum = 0xA005;

pub static DEFAULT_DEVICE_SPECIFIER             : ALCenum = 0x1004;
pub static DEVICE_SPECIFIER                     : ALCenum = 0x1005;
pub static EXTENSIONS                           : ALCenum = 0x1006;
pub static MAJOR_VERSION                        : ALCenum = 0x1000;
pub static MINOR_VERSION                        : ALCenum = 0x1001;
pub static ATTRIBUTES_SIZE                      : ALCenum = 0x1002;
pub static ALL_ATTRIBUTES                       : ALCenum = 0x1003;
pub static DEFAULT_ALL_DEVICES_SPECIFIER        : ALCenum = 0x1012;
pub static ALL_DEVICES_SPECIFIER                : ALCenum = 0x1013;
pub static CAPTURE_DEVICE_SPECIFIER             : ALCenum = 0x310;
pub static CAPTURE_DEFAULT_DEVICE_SPECIFIER     : ALCenum = 0x311;
pub static CAPTURE_SAMPLES                      : ALCenum = 0x312;

#[fixed_stack_segment]
pub fn create_context(device: *ALCdevice, attrlist: &[ALCint]) -> *ALCcontext {
    let attrs_terminated = vec::append_one(attrlist.to_owned(), 0);  // teminate attributes with a 0
    unsafe { ffi::alcCreateContext(device, cast::transmute(&attrs_terminated[0])) }
}

#[fixed_stack_segment]
pub fn make_context_current(context: *ALCcontext) -> ALboolean {
    unsafe { ffi::alcMakeContextCurrent(context) }
}

#[fixed_stack_segment]
pub fn process_context(context: *ALCcontext) {
    unsafe { ffi::alcProcessContext(context); }
}

#[fixed_stack_segment]
pub fn suspend_context(context: *ALCcontext) {
    unsafe { ffi::alcSuspendContext(context); }
}

#[fixed_stack_segment]
pub fn destroy_context(context: *ALCcontext) {
    unsafe { ffi::alcDestroyContext(context); }
}

#[fixed_stack_segment]
pub fn get_current_context() -> *ALCcontext {
    unsafe { ffi::alcGetCurrentContext() }
}

#[fixed_stack_segment]
pub fn get_contexts_device(context: *ALCcontext) -> *ALCdevice {
    unsafe { ffi::alcGetContextsDevice(context) }
}

#[fixed_stack_segment]
pub fn open_device(devicename: &str) -> *ALCdevice {
    unsafe { ffi::alcOpenDevice(devicename.with_c_str( |s| s)) }
}

#[fixed_stack_segment]
pub fn close_device(device: *ALCdevice) -> ALboolean {
    unsafe { ffi::alcCloseDevice(device) }
}

#[fixed_stack_segment]
pub fn get_error(device: *ALCdevice) -> ALCenum {
    unsafe { ffi::alcGetError(device) }
}

#[fixed_stack_segment]
pub fn is_extension_present(device: *ALCdevice, extname: &str) -> ALboolean {
    unsafe { ffi::alcIsExtensionPresent(device, extname.with_c_str( |s| s)) }
}

#[fixed_stack_segment]
pub fn get_proc_address(device: *ALCdevice, funcname: ~str) -> extern fn() {
    unsafe { cast::transmute(
        ffi::alcGetProcAddress(device, funcname.with_c_str( |s| s))
    ) }
}

#[fixed_stack_segment]
pub fn get_enum_value(device: *ALCdevice, enumname: &str) -> ALCenum {
    unsafe { ffi::alcGetEnumValue(device, enumname.with_c_str( |s| s)) }
}

// #[fixed_stack_segment]
// pub fn get_string(device: *ALCdevice, param: ALCenum) -> *ALCchar {
//     unsafe { ffi::alcGetString(device, param) }
// }

// #[fixed_stack_segment]
// pub fn GetIntegerv(device: *ALCdevice, param: ALCenum, size: ALCsizei, data: *ALCint) {
//     unsafe { ffi::alcGetIntegerv(); }
// }

#[fixed_stack_segment]
pub fn capture_open_device(devicename: *ALCchar, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> *ALCdevice {
    unsafe { ffi::alcCaptureOpenDevice(devicename, frequency, format, buffersize) }
}

#[fixed_stack_segment]
pub fn capture_close_device(device: *ALCdevice) -> ALboolean {
    unsafe { ffi::alcCaptureCloseDevice(device) }
}

#[fixed_stack_segment]
pub fn capture_start(device: *ALCdevice) {
    unsafe { ffi::alcCaptureStart(device); }
}

#[fixed_stack_segment]
pub fn capture_stop(device: *ALCdevice) {
    unsafe { ffi::alcCaptureStop(device); }
}

// #[fixed_stack_segment]
// pub fn CaptureSamples(device: *ALCdevice, buffer: *ALCvoid, samples: ALCsizei) {
//     unsafe { ffi::alcCaptureSamples(); }
// }