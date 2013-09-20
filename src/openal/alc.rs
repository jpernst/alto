use std::cast;
use std::vec;

use self::types::*;

pub mod types {
    use std::libc::*;
    pub type ALCboolean             = c_char;
    pub type ALCchar                = c_char;
    pub type ALCbyte                = c_char;
    pub type ALCubyte               = c_uchar;
    pub type ALCshort               = c_short;
    pub type ALCushort              = c_ushort;
    pub type ALCint                 = c_int;
    pub type ALCuint                = c_uint;
    pub type ALCsizei               = c_int;
    pub type ALCenum                = c_int;
    pub type ALCfloat               = c_float;
    pub type ALCdouble              = c_double;
    pub type ALCvoid                = c_void;
}

pub mod ffi {
    use super::types::*;

    pub struct ALCdevice;
    pub struct ALCcontext;

    extern "C" {
        pub fn alcCreateContext(device: *ALCdevice, attrlist: *ALCint) -> *ALCcontext;
        pub fn alcMakeContextCurrent(context: *ALCcontext) -> ALCboolean;
        pub fn alcProcessContext(context: *ALCcontext);
        pub fn alcSuspendContext(context: *ALCcontext);
        pub fn alcDestroyContext(context: *ALCcontext);
        pub fn alcGetCurrentContext() -> *ALCcontext;
        pub fn alcGetContextsDevice(context: *ALCcontext) -> *ALCdevice;
        pub fn alcOpenDevice(devicename: *ALCchar) -> *ALCdevice;
        pub fn alcCloseDevice(device: *ALCdevice) -> ALCboolean;
        pub fn alcGetError(device: *ALCdevice) -> ALCenum;
        pub fn alcIsExtensionPresent(device: *ALCdevice, extname: *ALCchar) -> ALCboolean;
        pub fn alcGetProcAddress(device: *ALCdevice, funcname: *ALCchar) -> Option<extern "C" fn()>;
        pub fn alcGetEnumValue(device: *ALCdevice, enumname: *ALCchar) -> ALCenum;
        pub fn alcGetString(device: *ALCdevice, param: ALCenum) -> *ALCchar;
        pub fn alcGetIntegerv(device: *ALCdevice, param: ALCenum, size: ALCsizei, data: *ALCint);
        pub fn alcCaptureOpenDevice(devicename: *ALCchar, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> *ALCdevice;
        pub fn alcCaptureCloseDevice(device: *ALCdevice) -> ALCboolean;
        pub fn alcCaptureStart(device: *ALCdevice);
        pub fn alcCaptureStop(device: *ALCdevice);
        pub fn alcCaptureSamples(device: *ALCdevice, buffer: *ALCvoid, samples: ALCsizei);
    }
}

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
pub fn create_context(device: *ffi::ALCdevice, attrlist: &[ALCint]) -> *ffi::ALCcontext {
    let attrs_terminated = vec::append_one(attrlist.to_owned(), 0);  // teminate attributes with a 0
    unsafe { ffi::alcCreateContext(device, cast::transmute(&attrs_terminated[0])) }
}

#[fixed_stack_segment]
pub fn make_context_current(context: *ffi::ALCcontext) -> bool {
    unsafe { ffi::alcMakeContextCurrent(context) == TRUE }
}

#[fixed_stack_segment]
pub fn process_context(context: *ffi::ALCcontext) {
    unsafe { ffi::alcProcessContext(context); }
}

#[fixed_stack_segment]
pub fn suspend_context(context: *ffi::ALCcontext) {
    unsafe { ffi::alcSuspendContext(context); }
}

#[fixed_stack_segment]
pub fn destroy_context(context: *ffi::ALCcontext) {
    unsafe { ffi::alcDestroyContext(context); }
}

#[fixed_stack_segment]
pub fn get_current_context() -> *ffi::ALCcontext {
    unsafe { ffi::alcGetCurrentContext() }
}

#[fixed_stack_segment]
pub fn get_contexts_device(context: *ffi::ALCcontext) -> *ffi::ALCdevice {
    unsafe { ffi::alcGetContextsDevice(context) }
}

#[fixed_stack_segment]
pub fn open_device(devicename: &str) -> *ffi::ALCdevice {
    unsafe { devicename.with_c_str(|c_str| ffi::alcOpenDevice(c_str)) }
}

#[fixed_stack_segment]
pub fn close_device(device: *ffi::ALCdevice) -> bool {
    unsafe { ffi::alcCloseDevice(device) == TRUE }
}

#[fixed_stack_segment]
pub fn get_error(device: *ffi::ALCdevice) -> ALCenum {
    unsafe { ffi::alcGetError(device) }
}

#[fixed_stack_segment]
pub fn is_extension_present(device: *ffi::ALCdevice, extname: &str) -> bool {
    unsafe { extname.with_c_str(|c_str| ffi::alcIsExtensionPresent(device, c_str)) == TRUE }
}

#[fixed_stack_segment]
pub fn get_proc_address(device: *ffi::ALCdevice, funcname: ~str) -> Option<extern "C" fn()> {
    unsafe { funcname.with_c_str(|c_str| ffi::alcGetProcAddress(device, c_str)) }
}

#[fixed_stack_segment]
pub fn get_enum_value(device: *ffi::ALCdevice, enumname: &str) -> ALCenum {
    unsafe { enumname.with_c_str(|c_str| ffi::alcGetEnumValue(device, c_str)) }
}

// #[fixed_stack_segment]
// pub fn get_string(device: *ffi::ALCdevice, param: ALCenum) -> *ALCchar {
//     unsafe { ffi::alcGetString(device, param) }
// }

// #[fixed_stack_segment]
// pub fn GetIntegerv(device: *ffi::ALCdevice, param: ALCenum, size: ALCsizei, data: *ALCint) {
//     unsafe { ffi::alcGetIntegerv(); }
// }

#[fixed_stack_segment]
pub fn capture_open_device(devicename: *ALCchar, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> *ffi::ALCdevice {
    unsafe { ffi::alcCaptureOpenDevice(devicename, frequency, format, buffersize) }
}

#[fixed_stack_segment]
pub fn capture_close_device(device: *ffi::ALCdevice) -> bool {
    unsafe { ffi::alcCaptureCloseDevice(device) == TRUE }
}

#[fixed_stack_segment]
pub fn capture_start(device: *ffi::ALCdevice) {
    unsafe { ffi::alcCaptureStart(device); }
}

#[fixed_stack_segment]
pub fn capture_stop(device: *ffi::ALCdevice) {
    unsafe { ffi::alcCaptureStop(device); }
}

// #[fixed_stack_segment]
// pub fn CaptureSamples(device: *ffi::ALCdevice, buffer: *ALCvoid, samples: ALCsizei) {
//     unsafe { ffi::alcCaptureSamples(); }
// }
