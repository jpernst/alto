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

    pub enum ALCdevice { }
    pub enum ALCcontext { }

    extern "C" {
        pub fn alcCreateContext(device: *mut ALCdevice, attrlist: *mut ALCint) -> *mut ALCcontext;
        pub fn alcMakeContextCurrent(context: *mut ALCcontext) -> ALCboolean;
        pub fn alcProcessContext(context: *mut ALCcontext);
        pub fn alcSuspendContext(context: *mut ALCcontext);
        pub fn alcDestroyContext(context: *mut ALCcontext);
        pub fn alcGetCurrentContext() -> *mut ALCcontext;
        pub fn alcGetContextsDevice(context: *mut ALCcontext) -> *mut ALCdevice;

        pub fn alcOpenDevice(devicename: *const ALCchar) -> *mut ALCdevice;
        pub fn alcCloseDevice(device: *mut ALCdevice) -> ALCboolean;
        pub fn alcGetError(device: *mut ALCdevice) -> ALCenum;
        pub fn alcIsExtensionPresent(device: *mut ALCdevice, extname: *const ALCchar) -> ALCboolean;
        pub fn alcGetProcAddress(device: *mut ALCdevice, funcname: *const ALCchar) -> Option<extern "C" fn()>;
        pub fn alcGetEnumValue(device: *mut ALCdevice, enumname: *const ALCchar) -> ALCenum;
        pub fn alcGetString(device: *mut ALCdevice, param: ALCenum) -> *const ALCchar;
        pub fn alcGetIntegerv(device: *mut ALCdevice, param: ALCenum, size: ALCsizei, data: *mut ALCint);
        pub fn alcCaptureOpenDevice(devicename: *const ALCchar, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> *mut ALCdevice;
        pub fn alcCaptureCloseDevice(device: *mut ALCdevice) -> ALCboolean;
        pub fn alcCaptureStart(device: *mut ALCdevice);
        pub fn alcCaptureStop(device: *mut ALCdevice);
        pub fn alcCaptureSamples(device: *mut ALCdevice, buffer: *mut ALCvoid, samples: ALCsizei);
    }
}

