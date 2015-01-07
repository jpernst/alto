// Copyright 2013 The openal-rs Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::ptr;
use std::str;
use std::ffi::{c_str_to_bytes, CString};
use libc;

use self::types::*;

pub mod types {
    use libc::*;
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

    // Boolean values
    pub const FALSE                                : ALCboolean = 0;
    pub const TRUE                                 : ALCboolean = 1;

    // Context management
    pub const FREQUENCY                            : ALCint = 0x1007;
    pub const REFRESH                              : ALCint = 0x1008;
    pub const SYNC                                 : ALCint = 0x1009;
    pub const MONO_SOURCES                         : ALCint = 0x1010;
    pub const STEREO_SOURCES                       : ALCint = 0x1011;

    // Errors
    pub const NO_ERROR                             : ALCenum = FALSE as ALCenum;
    pub const INVALID_DEVICE                       : ALCenum = 0xA001;
    pub const INVALID_CONTEXT                      : ALCenum = 0xA002;
    pub const INVALID_ENUM                         : ALCenum = 0xA003;
    pub const INVALID_VALUE                        : ALCenum = 0xA004;
    pub const OUT_OF_MEMORY                        : ALCenum = 0xA005;

    pub const DEFAULT_DEVICE_SPECIFIER             : ALCenum = 0x1004;
    pub const DEVICE_SPECIFIER                     : ALCenum = 0x1005;
    pub const EXTENSIONS                           : ALCenum = 0x1006;

    pub const MAJOR_VERSION                        : ALCenum = 0x1000;
    pub const MINOR_VERSION                        : ALCenum = 0x1001;

    pub const ATTRIBUTES_SIZE                      : ALCenum = 0x1002;
    pub const ALL_ATTRIBUTES                       : ALCenum = 0x1003;

    // ALC_ENUMERATE_ALL_EXT enums
    pub const DEFAULT_ALL_DEVICES_SPECIFIER        : ALCenum = 0x1012;
    pub const ALL_DEVICES_SPECIFIER                : ALCenum = 0x1013;

    // Capture extension
    pub const CAPTURE_DEVICE_SPECIFIER             : ALCenum = 0x310;
    pub const CAPTURE_DEFAULT_DEVICE_SPECIFIER     : ALCenum = 0x311;
    pub const CAPTURE_SAMPLES                      : ALCenum = 0x312;

    #[allow(missing_copy_implementations)]
    #[repr(C)]
    pub struct ALCdevice;
    #[allow(missing_copy_implementations)]
    #[repr(C)]
    pub struct ALCcontext;

    extern "C" {
        pub fn alcCreateContext(device: *const ALCdevice, attrlist: *const ALCint) -> *const ALCcontext;
        pub fn alcMakeContextCurrent(context: *const ALCcontext) -> ALCboolean;
        pub fn alcProcessContext(context: *const ALCcontext);
        pub fn alcSuspendContext(context: *const ALCcontext);
        pub fn alcDestroyContext(context: *const ALCcontext);
        pub fn alcGetCurrentContext() -> *const ALCcontext;
        pub fn alcGetContextsDevice(context: *const ALCcontext) -> *const ALCdevice;

        pub fn alcOpenDevice(devicename: *const ALCchar) -> *const ALCdevice;
        pub fn alcCloseDevice(device: *const ALCdevice) -> ALCboolean;
        pub fn alcGetError(device: *const ALCdevice) -> ALCenum;
        pub fn alcIsExtensionPresent(device: *const ALCdevice, extname: *const ALCchar) -> ALCboolean;
        pub fn alcGetProcAddress(device: *const ALCdevice, funcname: *const ALCchar) -> Option<extern "C" fn()>;
        pub fn alcGetEnumValue(device: *const ALCdevice, enumname: *const ALCchar) -> ALCenum;
        pub fn alcGetString(device: *const ALCdevice, param: ALCenum) -> *const ALCchar;
        pub fn alcGetIntegerv(device: *const ALCdevice, param: ALCenum, size: ALCsizei, data: *mut ALCint);
        pub fn alcCaptureOpenDevice(devicename: *const ALCchar, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> *const ALCdevice;
        pub fn alcCaptureCloseDevice(device: *const ALCdevice) -> ALCboolean;
        pub fn alcCaptureStart(device: *const ALCdevice);
        pub fn alcCaptureStop(device: *const ALCdevice);
        pub fn alcCaptureSamples(device: *const ALCdevice, buffer: *const ALCvoid, samples: ALCsizei);
    }
}

pub struct Context {
    ptr: *const ffi::ALCcontext,
}

// pub fn get_current_context() -> Context {
//     Context { ptr: unsafe { ffi::alcGetCurrentContext() } }
// }

impl Context {
    pub fn make_current(&self) -> bool {
        unsafe { ffi::alcMakeContextCurrent(self.ptr) == ffi::TRUE }
    }

    pub fn process(&self) {
        unsafe { ffi::alcProcessContext(self.ptr); }
    }

    pub fn suspend(&self) {
        unsafe { ffi::alcSuspendContext(self.ptr); }
    }

    pub fn destroy(self) {}

    // pub fn get_device(&self) -> Device {
    //     Device { ptr: unsafe { ffi::alcGetContextsDevice(self.ptr) } }
    // }

    pub fn is_current(&self) -> bool {
        unsafe { ffi::alcGetCurrentContext() == self.ptr }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ffi::alcDestroyContext(self.ptr); }
    }
}

#[allow(missing_copy_implementations)]
pub struct Device {
    ptr: *const ffi::ALCdevice,
}

impl Device {
    pub fn open(devicename: Option<&str>) -> Option<Device> {
        let ptr = unsafe {
          match devicename {
            Some(devicename) => ffi::alcOpenDevice(CString::from_slice(devicename.as_bytes()).as_ptr()),
            None => ffi::alcOpenDevice(ptr::null())
          }
        };
        if ptr.is_null() { None }
        else { Some(Device { ptr: ptr  }) }
    }

    /// Closes the device.
    ///
    /// The device will not be closed if it contains any contexts or buffers.
    /// If this is the case, the device will be returned again, wrapped in `Err`.
    pub fn close(self) -> Result<(), Device> {
        if unsafe { ffi::alcCloseDevice(self.ptr) == ffi::TRUE } { Ok(()) }
        else { Err(self) }
    }

    pub fn get_error(&self) -> ALCenum {
        unsafe { ffi::alcGetError(self.ptr) }
    }

    pub fn get_string(&self, param: ALCenum) -> String {
        unsafe { String::from_str(str::from_utf8(c_str_to_bytes(&(ffi::alcGetString(self.ptr, param) as *const libc::c_char))).unwrap()) }
    }

    // pub fn GetIntegerv(&self, param: ALCenum, size: ALCsizei, data: *const ALCint) {
    //     unsafe { ffi::alcGetIntegerv(); }
    // }

    pub fn create_context(&self, attr_list: &[ALCint]) -> Option<Context> {
        let attrs_terminated = { let mut v = attr_list.to_vec(); v.push(0); v }; // teminate attributes with a 0
        let ptr = unsafe { ffi::alcCreateContext(self.ptr, attrs_terminated.as_ptr()) };
        if ptr.is_null() { None }
        else { Some(Context { ptr: ptr  }) }
    }
}

#[allow(missing_copy_implementations)]
pub struct CaptureDevice {
    ptr: *const ffi::ALCdevice,
}

impl CaptureDevice {
    pub fn open(devicename: &str, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> Option<CaptureDevice> {
        let ptr = unsafe { ffi::alcCaptureOpenDevice(CString::from_slice(devicename.as_bytes()).as_ptr(), frequency, format, buffersize) };
        if ptr.is_null() { None }
        else { Some(CaptureDevice { ptr: ptr  }) }
    }

    /// Closes the capture device.
    ///
    /// If an error occurs, the device will be returned again, wrapped in `Err`.
    pub fn close(self) -> Result<(), CaptureDevice> {
        if unsafe { ffi::alcCaptureCloseDevice(self.ptr) == ffi::TRUE } { Ok(()) }
        else { Err(self) }
    }

    pub fn start(&self) {
        unsafe { ffi::alcCaptureStart(self.ptr); }
    }

    pub fn stop(&self) {
        unsafe { ffi::alcCaptureStop(self.ptr); }
    }

    // pub fn CaptureSamples(&self, buffer: *const ALCvoid, samples: ALCsizei) {
    //     unsafe { ffi::alcCaptureSamples(); }
    // }
}
