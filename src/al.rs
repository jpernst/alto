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

pub use self::types::*;

pub mod types {
    use libc::*;
    pub type ALboolean              = c_char;
    pub type ALchar                 = c_char;
    pub type ALbyte                 = c_char;
    pub type ALubyte                = c_uchar;
    pub type ALshort                = c_short;
    pub type ALushort               = c_ushort;
    pub type ALint                  = c_int;
    pub type ALuint                 = c_uint;
    pub type ALsizei                = c_int;
    pub type ALenum                 = c_int;
    pub type ALfloat                = c_float;
    pub type ALdouble               = c_double;
    pub type ALvoid                 = c_void;
}

pub mod ffi {
    use super::types::*;

    // "no distance model" or "no buffer"
    pub const NONE                           : ALenum = 0;

    // Boolean values
    pub const FALSE                          : ALboolean = 0;
    pub const TRUE                           : ALboolean = 1;

    // General properties
    pub const SOURCE_RELATIVE                : ALenum = 0x202;
    pub const CONE_INNER_ANGLE               : ALenum = 0x1001;
    pub const CONE_OUTER_ANGLE               : ALenum = 0x1002;
    pub const PITCH                          : ALenum = 0x1003;
    pub const POSITION                       : ALenum = 0x1004;
    pub const DIRECTION                      : ALenum = 0x1005;
    pub const VELOCITY                       : ALenum = 0x1006;
    pub const LOOPING                        : ALenum = 0x1007;
    pub const BUFFER                         : ALenum = 0x1009;
    pub const GAIN                           : ALenum = 0x100A;
    pub const MIN_GAIN                       : ALenum = 0x100D;
    pub const MAX_GAIN                       : ALenum = 0x100E;
    pub const ORIENTATION                    : ALenum = 0x100F;
    pub const REFERENCE_DISTANCE             : ALenum = 0x1020;
    pub const ROLLOFF_FACTOR                 : ALenum = 0x1021;
    pub const CONE_OUTER_GAIN                : ALenum = 0x1022;
    pub const MAX_DISTANCE                   : ALenum = 0x1023;

    // Source state
    pub const SOURCE_STATE                   : ALenum = 0x1010;
    pub const INITIAL                        : ALenum = 0x1011;
    pub const PLAYING                        : ALenum = 0x1012;
    pub const PAUSED                         : ALenum = 0x1013;
    pub const STOPPED                        : ALenum = 0x1014;

    // Buffer Queue params
    pub const BUFFERS_QUEUED                 : ALenum = 0x1015;
    pub const BUFFERS_PROCESSED              : ALenum = 0x1016;

    // Source buffer position information
    pub const SEC_OFFSET                     : ALenum = 0x1024;
    pub const SAMPLE_OFFSET                  : ALenum = 0x1025;
    pub const BYTE_OFFSET                    : ALenum = 0x1026;

    // Source type
    pub const SOURCE_TYPE                    : ALenum = 0x1027;
    pub const STATIC                         : ALenum = 0x1028;
    pub const STREAMING                      : ALenum = 0x1029;
    pub const UNDETERMINED                   : ALenum = 0x1030;

    // Buffer format
    pub const FORMAT_MONO8                   : ALenum = 0x1100;
    pub const FORMAT_MONO16                  : ALenum = 0x1101;
    pub const FORMAT_STEREO8                 : ALenum = 0x1102;
    pub const FORMAT_STEREO16                : ALenum = 0x1103;

    // Buffer properties
    pub const FREQUENCY                      : ALenum = 0x2001;
    pub const BITS                           : ALenum = 0x2002;
    pub const CHANNELS                       : ALenum = 0x2003;
    pub const SIZE                           : ALenum = 0x2004;

    // Buffer state
    pub const UNUSED                         : ALenum = 0x2010;
    pub const PENDING                        : ALenum = 0x2011;
    pub const PROCESSED                      : ALenum = 0x2012;

    // Error codes
    pub const NO_ERROR                       : ALenum = FALSE as ALenum;
    pub const INVALID_NAME                   : ALenum = 0xA001;
    pub const INVALID_ENUM                   : ALenum = 0xA002;
    pub const INVALID_VALUE                  : ALenum = 0xA003;
    pub const INVALID_OPERATION              : ALenum = 0xA004;
    pub const OUT_OF_MEMORY                  : ALenum = 0xA005;

    // String tokens
    pub const VENDOR                         : ALenum = 0xB001;
    pub const VERSION                        : ALenum = 0xB002;
    pub const RENDERER                       : ALenum = 0xB003;
    pub const EXTENSIONS                     : ALenum = 0xB004;

    // Global tweakage
    pub const DOPPLER_FACTOR                 : ALenum = 0xC000;
    pub const DOPPLER_VELOCITY               : ALenum = 0xC001;
    pub const SPEED_OF_SOUND                 : ALenum = 0xC003;

    // Distance models
    pub const DISTANCE_MODEL                 : ALenum = 0xD000;
    pub const INVERSE_DISTANCE               : ALenum = 0xD001;
    pub const INVERSE_DISTANCE_CLAMPED       : ALenum = 0xD002;
    pub const LINEAR_DISTANCE                : ALenum = 0xD003;
    pub const LINEAR_DISTANCE_CLAMPED        : ALenum = 0xD004;
    pub const EXPONENT_DISTANCE              : ALenum = 0xD005;
    pub const EXPONENT_DISTANCE_CLAMPED      : ALenum = 0xD006;

    extern "C" {
        pub fn alEnable(capability: ALenum);
        pub fn alDisable(capability: ALenum);
        pub fn alIsEnabled(capability: ALenum) -> ALboolean;
        pub fn alGetString(param: ALenum) -> *const ALchar;
        pub fn alGetBooleanv(param: ALenum, data: *mut ALboolean);
        pub fn alGetIntegerv(param: ALenum, data: *mut ALint);
        pub fn alGetFloatv(param: ALenum, data: *mut ALfloat);
        pub fn alGetDoublev(param: ALenum, data: *mut ALdouble);
        pub fn alGetBoolean(param: ALenum) -> ALboolean;
        pub fn alGetInteger(param: ALenum) -> ALint;
        pub fn alGetFloat(param: ALenum) -> ALfloat;
        pub fn alGetDouble(param: ALenum) -> ALdouble;
        pub fn alGetError() -> ALenum;
        pub fn alIsExtensionPresent(extname: *const ALchar) -> ALboolean;
        pub fn alGetProcAddress(fname: *const ALchar) -> Option<extern "C" fn()>;
        pub fn alGetEnumValue(ename: *const ALchar) -> ALenum;

        pub fn alListenerf(param: ALenum, value: ALfloat);
        pub fn alListener3f(param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
        pub fn alListenerfv(param: ALenum, values: *const ALfloat);
        pub fn alListeneri(param: ALenum, value: ALint);
        pub fn alListener3i(param: ALenum, value1: ALint, value2: ALint, value3: ALint);
        pub fn alListeneriv(param: ALenum, values: *const ALint);
        pub fn alGetListenerf(param: ALenum, value: *mut ALfloat);
        pub fn alGetListener3f(param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat);
        pub fn alGetListenerfv(param: ALenum, values: *mut ALfloat);
        pub fn alGetListeneri(param: ALenum, value: *mut ALint);
        pub fn alGetListener3i(param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint);
        pub fn alGetListeneriv(param: ALenum, values: *mut ALint);
        pub fn alGenSources(n: ALsizei, sources: *mut ALuint);
        pub fn alDeleteSources(n: ALsizei, sources: *const ALuint);
        pub fn alIsSource(sid: ALuint) -> ALboolean;
        pub fn alSourcef(sid: ALuint, param: ALenum, value: ALfloat);
        pub fn alSource3f(sid: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
        pub fn alSourcefv(sid: ALuint, param: ALenum, values: *const ALfloat);
        pub fn alSourcei(sid: ALuint, param: ALenum, value: ALint);
        pub fn alSource3i(sid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint);
        pub fn alSourceiv(sid: ALuint, param: ALenum, values: *const ALint);
        pub fn alGetSourcef(sid: ALuint, param: ALenum, value: *mut ALfloat);
        pub fn alGetSource3f(sid: ALuint, param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat);
        pub fn alGetSourcefv(sid: ALuint, param: ALenum, values: *mut ALfloat);
        pub fn alGetSourcei(sid: ALuint,  param: ALenum, value: *mut ALint);
        pub fn alGetSource3i(sid: ALuint, param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint);
        pub fn alGetSourceiv(sid: ALuint,  param: ALenum, values: *mut ALint);
        pub fn alSourcePlayv(ns: ALsizei, sids: *const ALuint);
        pub fn alSourceStopv(ns: ALsizei, sids: *const ALuint);
        pub fn alSourceRewindv(ns: ALsizei, sids: *const ALuint);
        pub fn alSourcePausev(ns: ALsizei, sids: *const ALuint);
        pub fn alSourcePlay(sid: ALuint);
        pub fn alSourceStop(sid: ALuint);
        pub fn alSourceRewind(sid: ALuint);
        pub fn alSourcePause(sid: ALuint);
        pub fn alSourceQueueBuffers(sid: ALuint, numEntries: ALsizei, bids: *const ALuint);
        pub fn alSourceUnqueueBuffers(sid: ALuint, numEntries: ALsizei, bids: *mut ALuint);
        pub fn alGenBuffers(n: ALsizei, buffers: *mut ALuint);
        pub fn alDeleteBuffers(n: ALsizei, buffers: *const ALuint);
        pub fn alIsBuffer(bid: ALuint) -> ALboolean;
        pub fn alBufferData(bid: ALuint, format: ALenum, data: *const ALvoid, size: ALsizei, freq: ALsizei);
        pub fn alBufferf(bid: ALuint, param: ALenum, value: ALfloat);
        pub fn alBuffer3f(bid: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
        pub fn alBufferfv(bid: ALuint, param: ALenum, values: *const ALfloat);
        pub fn alBufferi(bid: ALuint, param: ALenum, value: ALint);
        pub fn alBuffer3i(bid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint);
        pub fn alBufferiv(bid: ALuint, param: ALenum, values: *const ALint);
        pub fn alGetBufferf(bid: ALuint, param: ALenum, value: *mut ALfloat);
        pub fn alGetBuffer3f(bid: ALuint, param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat);
        pub fn alGetBufferfv(bid: ALuint, param: ALenum, values: *mut ALfloat);
        pub fn alGetBufferi(bid: ALuint, param: ALenum, value: *mut ALint);
        pub fn alGetBuffer3i(bid: ALuint, param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint);
        pub fn alGetBufferiv(bid: ALuint, param: ALenum, values: *mut ALint);
        pub fn alDopplerFactor(value: ALfloat);
        pub fn alDopplerVelocity(value: ALfloat);
        pub fn alSpeedOfSound(value: ALfloat);
        pub fn alDistanceModel(value: ALenum);
    }
}

