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

use std::cast;
use std::fmt;
use std::str;
use std::sys;
use std::vec;

pub use self::types::*;

pub mod types {
    use std::libc::*;
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
    pub static NONE                           : ALenum = 0;

    // Boolean values
    pub static FALSE                          : ALboolean = 0;
    pub static TRUE                           : ALboolean = 1;

    // General properties
    pub static SOURCE_RELATIVE                : ALenum = 0x202;
    pub static CONE_INNER_ANGLE               : ALenum = 0x1001;
    pub static CONE_OUTER_ANGLE               : ALenum = 0x1002;
    pub static PITCH                          : ALenum = 0x1003;
    pub static POSITION                       : ALenum = 0x1004;
    pub static DIRECTION                      : ALenum = 0x1005;
    pub static VELOCITY                       : ALenum = 0x1006;
    pub static LOOPING                        : ALenum = 0x1007;
    pub static BUFFER                         : ALenum = 0x1009;
    pub static GAIN                           : ALenum = 0x100A;
    pub static MIN_GAIN                       : ALenum = 0x100D;
    pub static MAX_GAIN                       : ALenum = 0x100E;
    pub static ORIENTATION                    : ALenum = 0x100F;
    pub static REFERENCE_DISTANCE             : ALenum = 0x1020;
    pub static ROLLOFF_FACTOR                 : ALenum = 0x1021;
    pub static CONE_OUTER_GAIN                : ALenum = 0x1022;
    pub static MAX_DISTANCE                   : ALenum = 0x1023;

    // Source state
    pub static SOURCE_STATE                   : ALenum = 0x1010;
    pub static INITIAL                        : ALenum = 0x1011;
    pub static PLAYING                        : ALenum = 0x1012;
    pub static PAUSED                         : ALenum = 0x1013;
    pub static STOPPED                        : ALenum = 0x1014;

    // Buffer Queue params
    pub static BUFFERS_QUEUED                 : ALenum = 0x1015;
    pub static BUFFERS_PROCESSED              : ALenum = 0x1016;

    // Source buffer position information
    pub static SEC_OFFSET                     : ALenum = 0x1024;
    pub static SAMPLE_OFFSET                  : ALenum = 0x1025;
    pub static BYTE_OFFSET                    : ALenum = 0x1026;

    // Source type
    pub static SOURCE_TYPE                    : ALenum = 0x1027;
    pub static STATIC                         : ALenum = 0x1028;
    pub static STREAMING                      : ALenum = 0x1029;
    pub static UNDETERMINED                   : ALenum = 0x1030;

    // Buffer format
    pub static FORMAT_MONO8                   : ALenum = 0x1100;
    pub static FORMAT_MONO16                  : ALenum = 0x1101;
    pub static FORMAT_STEREO8                 : ALenum = 0x1102;
    pub static FORMAT_STEREO16                : ALenum = 0x1103;

    // Buffer properties
    pub static FREQUENCY                      : ALenum = 0x2001;
    pub static BITS                           : ALenum = 0x2002;
    pub static CHANNELS                       : ALenum = 0x2003;
    pub static SIZE                           : ALenum = 0x2004;

    // Buffer state
    pub static UNUSED                         : ALenum = 0x2010;
    pub static PENDING                        : ALenum = 0x2011;
    pub static PROCESSED                      : ALenum = 0x2012;

    // Error codes
    pub static NO_ERROR                       : ALenum = FALSE as ALenum;
    pub static INVALID_NAME                   : ALenum = 0xA001;
    pub static INVALID_ENUM                   : ALenum = 0xA002;
    pub static INVALID_VALUE                  : ALenum = 0xA003;
    pub static INVALID_OPERATION              : ALenum = 0xA004;
    pub static OUT_OF_MEMORY                  : ALenum = 0xA005;

    // String tokens
    pub static VENDOR                         : ALenum = 0xB001;
    pub static VERSION                        : ALenum = 0xB002;
    pub static RENDERER                       : ALenum = 0xB003;
    pub static EXTENSIONS                     : ALenum = 0xB004;

    // Global tweakage
    pub static DOPPLER_FACTOR                 : ALenum = 0xC000;
    pub static DOPPLER_VELOCITY               : ALenum = 0xC001;
    pub static SPEED_OF_SOUND                 : ALenum = 0xC003;

    // Distance models
    pub static DISTANCE_MODEL                 : ALenum = 0xD000;
    pub static INVERSE_DISTANCE               : ALenum = 0xD001;
    pub static INVERSE_DISTANCE_CLAMPED       : ALenum = 0xD002;
    pub static LINEAR_DISTANCE                : ALenum = 0xD003;
    pub static LINEAR_DISTANCE_CLAMPED        : ALenum = 0xD004;
    pub static EXPONENT_DISTANCE              : ALenum = 0xD005;
    pub static EXPONENT_DISTANCE_CLAMPED      : ALenum = 0xD006;

    extern "C" {
        pub fn alEnable(capability: ALenum);
        pub fn alDisable(capability: ALenum);
        pub fn alIsEnabled(capability: ALenum) -> ALboolean;
        pub fn alGetString(param: ALenum) -> *ALchar;
        pub fn alGetBooleanv(param: ALenum, data: *mut ALboolean);
        pub fn alGetIntegerv(param: ALenum, data: *mut ALint);
        pub fn alGetFloatv(param: ALenum, data: *mut ALfloat);
        pub fn alGetDoublev(param: ALenum, data: *mut ALdouble);
        pub fn alGetBoolean(param: ALenum) -> ALboolean;
        pub fn alGetInteger(param: ALenum) -> ALint;
        pub fn alGetFloat(param: ALenum) -> ALfloat;
        pub fn alGetDouble(param: ALenum) -> ALdouble;
        pub fn alGetError() -> ALenum;
        pub fn alIsExtensionPresent(extname: *ALchar) -> ALboolean;
        pub fn alGetProcAddress(fname: *ALchar) -> Option<extern "C" fn()>;
        pub fn alGetEnumValue(ename: *ALchar) -> ALenum;

        pub fn alListenerf(param: ALenum, value: ALfloat);
        pub fn alListener3f(param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
        pub fn alListenerfv(param: ALenum, values: *ALfloat);
        pub fn alListeneri(param: ALenum, value: ALint);
        pub fn alListener3i(param: ALenum, value1: ALint, value2: ALint, value3: ALint);
        pub fn alListeneriv(param: ALenum, values: *ALint);
        pub fn alGetListenerf(param: ALenum, value: *mut ALfloat);
        pub fn alGetListener3f(param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat);
        pub fn alGetListenerfv(param: ALenum, values: *mut ALfloat);
        pub fn alGetListeneri(param: ALenum, value: *mut ALint);
        pub fn alGetListener3i(param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint);
        pub fn alGetListeneriv(param: ALenum, values: *mut ALint);
        pub fn alGenSources(n: ALsizei, sources: *mut ALuint);
        pub fn alDeleteSources(n: ALsizei, sources: *ALuint);
        pub fn alIsSource(sid: ALuint) -> ALboolean;
        pub fn alSourcef(sid: ALuint, param: ALenum, value: ALfloat);
        pub fn alSource3f(sid: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
        pub fn alSourcefv(sid: ALuint, param: ALenum, values: *ALfloat);
        pub fn alSourcei(sid: ALuint, param: ALenum, value: ALint);
        pub fn alSource3i(sid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint);
        pub fn alSourceiv(sid: ALuint, param: ALenum, values: *ALint);
        pub fn alGetSourcef(sid: ALuint, param: ALenum, value: *mut ALfloat);
        pub fn alGetSource3f(sid: ALuint, param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat);
        pub fn alGetSourcefv(sid: ALuint, param: ALenum, values: *mut ALfloat);
        pub fn alGetSourcei(sid: ALuint,  param: ALenum, value: *mut ALint);
        pub fn alGetSource3i(sid: ALuint, param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint);
        pub fn alGetSourceiv(sid: ALuint,  param: ALenum, values: *mut ALint);
        pub fn alSourcePlayv(ns: ALsizei, sids: *ALuint);
        pub fn alSourceStopv(ns: ALsizei, sids: *ALuint);
        pub fn alSourceRewindv(ns: ALsizei, sids: *ALuint);
        pub fn alSourcePausev(ns: ALsizei, sids: *ALuint);
        pub fn alSourcePlay(sid: ALuint);
        pub fn alSourceStop(sid: ALuint);
        pub fn alSourceRewind(sid: ALuint);
        pub fn alSourcePause(sid: ALuint);
        pub fn alSourceQueueBuffers(sid: ALuint, numEntries: ALsizei, bids: *ALuint);
        pub fn alSourceUnqueueBuffers(sid: ALuint, numEntries: ALsizei, bids: *ALuint);
        pub fn alGenBuffers(n: ALsizei, buffers: *mut ALuint);
        pub fn alDeleteBuffers(n: ALsizei, buffers: *ALuint);
        pub fn alIsBuffer(bid: ALuint) -> ALboolean;
        pub fn alBufferData(bid: ALuint, format: ALenum, data: *ALvoid, size: ALsizei, freq: ALsizei);
        pub fn alBufferf(bid: ALuint, param: ALenum, value: ALfloat);
        pub fn alBuffer3f(bid: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
        pub fn alBufferfv(bid: ALuint, param: ALenum, values: *ALfloat);
        pub fn alBufferi(bid: ALuint, param: ALenum, value: ALint);
        pub fn alBuffer3i(bid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint);
        pub fn alBufferiv(bid: ALuint, param: ALenum, values: *ALint);
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

#[fixed_stack_segment]
pub fn get_vendor() -> ~str {
    unsafe { str::raw::from_c_str(ffi::alGetString(ffi::VENDOR)) }
}

#[fixed_stack_segment]
pub fn get_version() -> ~str {
    unsafe { str::raw::from_c_str(ffi::alGetString(ffi::VERSION)) }
}

#[fixed_stack_segment]
pub fn get_renderer() -> ~str {
    unsafe { str::raw::from_c_str(ffi::alGetString(ffi::RENDERER)) }
}

#[fixed_stack_segment]
pub fn get_extensions() -> ~str {
    unsafe { str::raw::from_c_str(ffi::alGetString(ffi::EXTENSIONS)) }
}

#[fixed_stack_segment]
pub fn get_doppler_factor() -> ALfloat {
    unsafe { ffi::alGetFloat(ffi::DOPPLER_FACTOR) }
}

#[fixed_stack_segment]
pub fn set_doppler_factor(value: ALfloat) {
    unsafe { ffi::alDopplerFactor(value); }
}

#[fixed_stack_segment]
pub fn get_doppler_velocity() -> ALfloat {
    unsafe { ffi::alGetFloat(ffi::DOPPLER_VELOCITY) }
}

#[fixed_stack_segment]
pub fn set_doppler_velocity(value: ALfloat) {
    unsafe { ffi::alDopplerVelocity(value); }
}

#[fixed_stack_segment]
pub fn get_speed_of_sound() -> ALfloat {
    unsafe { ffi::alGetFloat(ffi::SPEED_OF_SOUND) }
}

#[fixed_stack_segment]
pub fn set_speed_of_sound(value: ALfloat) {
    unsafe { ffi::alSpeedOfSound(value); }
}

pub enum DistanceModel {
    InverseDistance             = ffi::INVERSE_DISTANCE             as int,
    InverseDistanceClamped      = ffi::INVERSE_DISTANCE_CLAMPED     as int,
    LinearDistance              = ffi::LINEAR_DISTANCE              as int,
    LinearDistanceClamped       = ffi::LINEAR_DISTANCE_CLAMPED      as int,
    ExponentDistance            = ffi::EXPONENT_DISTANCE            as int,
    ExponentDistanceClamped     = ffi::EXPONENT_DISTANCE_CLAMPED    as int,
}

#[fixed_stack_segment]
pub fn get_distance_model() -> Option<DistanceModel> {
    unsafe {
        match ffi::alGetInteger(ffi::DISTANCE_MODEL) {
            ffi::NONE => None,
            model => Some(cast::transmute(model as int)),
        }
    }
}

#[fixed_stack_segment]
pub fn set_distance_model(value: Option<DistanceModel>) {
    unsafe {
        match value {
            Some(model) => ffi::alDistanceModel(model as ALenum),
            None => ffi::alDistanceModel(ffi::NONE),
        }
    }
}

/// An OpenAL error code
#[deriving(Eq, ToStr)]
pub enum Error {
    /// A bad name (ID) was passed to an OpenAL function.
    InvalidName,
    /// An invalid enum value was passed to an OpenAL function.
    InvalidEnum,
    /// An invalid value was passed to an OpenAL function.
    InvalidValue,
    /// The requested operation is not valid.
    InvalidOperation,
    /// The requested operation resulted in OpenAL running out of memory.
    OutOfMemory,
}

/// Returns the current error state then clears it.
#[fixed_stack_segment]
pub fn get_error() -> Option<&'static Error> {
    match unsafe { ffi::alGetError() } {
        ffi::INVALID_NAME       => Some(&InvalidName),
        ffi::INVALID_ENUM       => Some(&InvalidEnum),
        ffi::INVALID_VALUE      => Some(&InvalidValue),
        ffi::INVALID_OPERATION  => Some(&InvalidOperation),
        ffi::OUT_OF_MEMORY      => Some(&OutOfMemory),
        _                       => None,
    }
}

impl fmt::Default for Error {
    fn fmt(&err: &Error, f: &mut fmt::Formatter) {
        match err {
            InvalidName         => write!(f.buf, "InvalidName: A bad name (ID) was passed to an OpenAL function."),
            InvalidEnum         => write!(f.buf, "InvalidEnum: An invalid enum value was passed to an OpenAL function."),
            InvalidValue        => write!(f.buf, "InvalidValue: An invalid value was passed to an OpenAL function."),
            InvalidOperation    => write!(f.buf, "InvalidOperation: The requested operation is not valid."),
            OutOfMemory         => write!(f.buf, "OutOfMemory: The requested operation resulted in OpenAL running out of memory."),
        }
    }
}

/// Functions for setting and retrieving the properties of the listener. One
/// listener is implied for each context.
pub mod listener {
    use std::cast;
    use super::ffi;
    use super::types::*;

    // The master gain.
    #[fixed_stack_segment]
    pub fn get_gain() -> ALfloat {
        unsafe {
            let mut value = 0.0;
            ffi::alGetListenerf(ffi::GAIN, &mut value);
            value
        }
    }

    /// Set the master gain (should be positive).
    #[fixed_stack_segment]
    pub fn set_gain(value: ALfloat) {
        unsafe { ffi::alListenerf(ffi::GAIN, value); }
    }

    // The position of the listener.
    #[fixed_stack_segment]
    pub fn get_position() -> [ALfloat, ..3] {
        unsafe {
            let mut values = [0.0, ..3];
            ffi::alGetListenerfv(ffi::GAIN, &mut values[0]);
            values
        }
    }

    /// Set the position of the listener.
    #[fixed_stack_segment]
    pub fn set_position(values: [ALfloat, ..3]) {
        unsafe { ffi::alListenerfv(ffi::POSITION, &values[0]); }
    }

    // The velocity vector.
    #[fixed_stack_segment]
    pub fn get_velocity() -> [ALfloat, ..3] {
        unsafe {
            let mut values = [0.0, ..3];
            ffi::alGetListenerfv(ffi::VELOCITY, &mut values[0]);
            values
        }
    }

    /// Set the velocity vector.
    #[fixed_stack_segment]
    pub fn set_velocity(values: [ALfloat, ..3]) {
        unsafe { ffi::alListenerfv(ffi::VELOCITY, &values[0]); }
    }

    // The orientation of the listener, expressed as 'at' and 'up' vectors.
    #[fixed_stack_segment]
    pub fn get_orientation() -> ([ALfloat, ..3], [ALfloat, ..3]) {
        unsafe {
            let mut values = ([0.0, ..3], [0.0, ..3]);
            ffi::alGetListenerfv(ffi::ORIENTATION, cast::transmute(&mut values));
            values
        }
    }

    /// Set the orientation of the listener.
    #[fixed_stack_segment]
    pub fn set_orientation(at: [ALfloat, ..3], up: [ALfloat, ..3]) {
        unsafe {
            let values = (at, up);
            ffi::alListenerfv(ffi::ORIENTATION, cast::transmute(&values));
        }
    }
}

/// A reference to a source object
#[deriving(Clone, Eq)]
pub struct Source {
    id: ALuint,
}

/// Generate a one or more source objects.
#[fixed_stack_segment]
pub fn gen_sources(n: uint) -> ~[Source] {
    unsafe {
        let mut sources = vec::from_elem(n, Source::null());
        ffi::alGenSources(n as ALsizei, &mut sources[0].id);
        sources
    }
}

#[fixed_stack_segment]
pub fn delete_sources(sources: &[Source]) {
    unsafe { ffi::alDeleteSources(sources.len() as ALsizei, &sources[0].id); }
}

#[deriving(Eq)]
pub enum SourceType {
    Static          = ffi::STATIC       as int,
    Streaming       = ffi::STREAMING    as int,
    Undetermined    = ffi::UNDETERMINED as int,
}

macro_rules! get_source(
    (fv, $param:expr, $n:expr) => ({
        let mut values = [0.0, ..$n];
        ffi::alGetSourcef(self.id, $param, &mut values[0]);
        values
    });
    (f, $param:expr) => ({
        let mut value = 0.0;
        ffi::alGetSourcef(self.id, $param, &mut value);
        value
    });
    (i, $param:expr) => ({
        let mut value = 0;
        ffi::alGetSourcei(self.id, $param, &mut value);
        value
    });
)

impl Source {
    /// An uninitialised source object. Any operation performed on this
    /// object will generate an error.
    #[inline]
    fn null() -> Source {
        Source { id: 0 }
    }

    /// Generate a single source object.
    #[fixed_stack_segment]
    pub fn gen() -> Source {
        unsafe {
            let mut source = Source::null();
            ffi::alGenSources(1, &mut source.id);
            source
        }
    }

    /// Delete the source.
    #[fixed_stack_segment]
    pub fn delete(&self) {
        unsafe { ffi::alDeleteSources(1, &self.id); }
    }

    /// Play the buffers attached to the source.
    #[fixed_stack_segment]
    pub fn play(&self) {
        unsafe { ffi::alSourcePlay(self.id); }
    }

    // Returns `true` if the source is playing.
    #[fixed_stack_segment]
    pub fn is_playing(&self) -> bool {
        unsafe { (get_source!(i, ffi::SOURCE_STATE)) as ALenum == ffi::PLAYING }
    }

    /// Pause the source.
    #[fixed_stack_segment]
    pub fn pause(&self) {
        unsafe { ffi::alSourcePause(self.id); }
    }

    // Returns `true` if the source is paused.
    #[fixed_stack_segment]
    pub fn is_paused(&self) -> bool {
        unsafe { (get_source!(i, ffi::SOURCE_STATE)) as ALenum == ffi::PAUSED }
    }

    /// Stop the source
    #[fixed_stack_segment]
    pub fn stop(&self) {
        unsafe { ffi::alSourceStop(self.id); }
    }

    // Returns `true` if the source is stopped.
    #[fixed_stack_segment]
    pub fn is_stopped(&self) -> bool {
        unsafe { (get_source!(i, ffi::SOURCE_STATE)) as ALenum == ffi::STOPPED }
    }

    /// Rewind the source to the initial state.
    #[fixed_stack_segment]
    pub fn rewind(&self) {
        unsafe { ffi::alSourceRewind(self.id); }
    }

    // Returns `true` if the source is at the initial state.
    #[fixed_stack_segment]
    pub fn is_initial(&self) -> bool {
        unsafe { (get_source!(i, ffi::SOURCE_STATE)) as ALenum == ffi::INITIAL }
    }

    /// Queue a single buffer on the source.
    #[fixed_stack_segment]
    pub fn queue_buffer(&self, buffer: &Buffer) {
        unsafe { ffi::alSourceQueueBuffers(self.id, 1, &buffer.id); }
    }

    /// Queue the buffers on the source to be played in sequence.
    #[fixed_stack_segment]
    pub fn queue_buffers(&self, buffers: &[Buffer]) {
        unsafe { ffi::alSourceQueueBuffers(self.id, buffers.len() as ALsizei, &buffers[0].id); }
    }

    /// Remove a single buffer from the queue.
    #[fixed_stack_segment]
    pub fn unqueue_buffer(&self, buffer: &Buffer) {
        unsafe { ffi::alSourceUnqueueBuffers(self.id, 1, &buffer.id); }
    }

    /// Remove a set of buffers from the queue.
    #[fixed_stack_segment]
    pub fn unqueue_buffers(&self, buffers: &[Buffer]) {
        unsafe { ffi::alSourceUnqueueBuffers(self.id, buffers.len() as ALsizei, &buffers[0].id); }
    }

    // The number of buffers queued on this source.
    #[fixed_stack_segment]
    pub fn get_buffers_queued(&self) -> uint {
        unsafe { (get_source!(i, ffi::BUFFERS_QUEUED)) as uint }
    }

    // the number of buffers in the queue that have been processed.
    #[fixed_stack_segment]
    pub fn get_buffers_processed(&self) -> uint {
        unsafe { (get_source!(i, ffi::BUFFERS_PROCESSED)) as uint }
    }

    // The pitch multiplier.
    #[fixed_stack_segment]
    pub fn get_pitch(&self) -> ALfloat {
        unsafe { get_source!(f, ffi::PITCH) }
    }

    /// Set the pitch multiplier (should be positive).
    #[fixed_stack_segment]
    pub fn set_pitch(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::PITCH, value); }
    }

    // The source gain.
    #[fixed_stack_segment]
    pub fn get_gain(&self) -> ALfloat {
        unsafe { get_source!(f, ffi::GAIN) }
    }

    /// Set the source gain (should be positive).
    #[fixed_stack_segment]
    pub fn set_gain(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::GAIN, value); }
    }

    /// Used with the Inverse Clamped Distance Model to set the distance where
    /// there will no longer be any attenuation of the source.
    #[fixed_stack_segment]
    pub fn get_max_distance(&self) -> ALfloat {
        unsafe { get_source!(f, ffi::MAX_DISTANCE) }
    }

    /// Set the max distance for the source.
    #[fixed_stack_segment]
    pub fn set_max_distance(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::MAX_DISTANCE, value); }
    }

    // The rolloff factor for the source.
    #[fixed_stack_segment]
    pub fn get_rolloff_factor(&self) -> ALfloat {
        unsafe { get_source!(f, ffi::ROLLOFF_FACTOR) }
    }

    /// Set the rolloff factor for the source (default is `1.0`).
    #[fixed_stack_segment]
    pub fn set_rolloff_factor(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::ROLLOFF_FACTOR, value); }
    }

    /// The distance under which the volume for the source would normally drop
    /// by half (before being influenced by rolloff factor or max distance).
    #[fixed_stack_segment]
    pub fn get_reference_distance(&self) -> ALfloat {
        unsafe { get_source!(f, ffi::REFERENCE_DISTANCE) }
    }

    /// Set the reference distance.
    #[fixed_stack_segment]
    pub fn set_reference_distance(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::REFERENCE_DISTANCE, value); }
    }

    // The minimum gain for the source.
    #[fixed_stack_segment]
    pub fn get_min_gain(&self) -> ALfloat {
        unsafe { get_source!(f, ffi::MIN_GAIN) }
    }

    /// Set the minimum gain for the source.
    #[fixed_stack_segment]
    pub fn set_min_gain(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::MIN_GAIN, value); }
    }

    // The maximum gain for the source.
    #[fixed_stack_segment]
    pub fn get_max_gain(&self) -> ALfloat {
        unsafe { get_source!(f, ffi::MAX_GAIN) }
    }

    /// Set the maximum gain for the source.
    #[fixed_stack_segment]
    pub fn set_max_gain(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::MAX_GAIN, value); }
    }

    // The gain when outside the oriented cone.
    #[fixed_stack_segment]
    pub fn get_cone_outer_gain(&self) -> ALfloat {
        unsafe { get_source!(f, ffi::CONE_OUTER_GAIN) }
    }

    /// Set the gain when outside the oriented cone.
    #[fixed_stack_segment]
    pub fn set_cone_outer_gain(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::CONE_OUTER_GAIN, value); }
    }

    // The gain when inside the oriented cone.
    #[fixed_stack_segment]
    pub fn get_cone_inner_angle(&self) -> ALfloat {
        unsafe { get_source!(f, ffi::CONE_INNER_ANGLE) }
    }

    /// Set the gain when inside the oriented cone.
    #[fixed_stack_segment]
    pub fn set_cone_inner_angle(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::CONE_INNER_ANGLE, value); }
    }

    // The outer angle of the sound cone, in degrees.
    #[fixed_stack_segment]
    pub fn get_cone_outer_angle(&self) -> ALfloat {
        unsafe { get_source!(f, ffi::CONE_OUTER_ANGLE) }
    }

    /// Set the outer angle of the sound cone, in degrees. (default is `360.0`)
    #[fixed_stack_segment]
    pub fn set_cone_outer_angle(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::CONE_OUTER_ANGLE, value); }
    }

    // The position of the source.
    #[fixed_stack_segment]
    pub fn get_position(&self) -> [ALfloat, ..3] {
        unsafe { get_source!(fv, ffi::POSITION, 3) }
    }

    /// Set the position of the source.
    #[fixed_stack_segment]
    pub fn set_position(&self, values: [ALfloat, ..3]) {
        unsafe { ffi::alSourcefv(self.id, ffi::POSITION, &values[0]); }
    }

    // The velocity vector of the source.
    #[fixed_stack_segment]
    pub fn get_velocity(&self) -> [ALfloat, ..3] {
        unsafe { get_source!(fv, ffi::VELOCITY, 3) }
    }

    /// Set the velocity vector of the source.
    #[fixed_stack_segment]
    pub fn set_velocity(&self, values: [ALfloat, ..3]) {
        unsafe { ffi::alSourcefv(self.id, ffi::VELOCITY, &values[0]); }
    }

    // The direction vector of the source.
    #[fixed_stack_segment]
    pub fn get_direction(&self) -> [ALfloat, ..3] {
        unsafe { get_source!(fv, ffi::DIRECTION, 3) }
    }

    /// Set the direction vector of the source.
    #[fixed_stack_segment]
    pub fn set_direction(&self, values: [ALfloat, ..3]) {
        unsafe { ffi::alSourcefv(self.id, ffi::DIRECTION, &values[0]); }
    }

    // Returns `true` if the positions are relative to the listener.
    #[fixed_stack_segment]
    pub fn is_relative(&self) -> bool {
        unsafe { (get_source!(i, ffi::SOURCE_RELATIVE)) as ALboolean == ffi::TRUE }
    }

    /// Set whether the positions are relative to the listener. (default is `false`)
    #[fixed_stack_segment]
    pub fn set_relative(&self, value: bool) {
        unsafe { ffi::alSourcei(self.id, ffi::SOURCE_RELATIVE, value as ALint); }
    }

    // The source type.
    #[fixed_stack_segment]
    pub fn get_type(&self) -> SourceType {
        unsafe { cast::transmute(get_source!(i, ffi::SOURCE_TYPE) as int) }
    }

    /// Set the source type.
    #[fixed_stack_segment]
    pub fn set_type(&self, value: SourceType) {
        unsafe { ffi::alSourcei(self.id, ffi::SOURCE_TYPE, value as ALint); }
    }

    // Returns `true` looping is turned on for this source.
    #[fixed_stack_segment]
    pub fn is_looping(&self) -> bool {
        unsafe { (get_source!(i, ffi::LOOPING)) as ALboolean == ffi::TRUE }
    }

    /// Set looping on/off for this source.
    #[fixed_stack_segment]
    pub fn set_looping(&self, value: bool) {
        unsafe { ffi::alSourcei(self.id, ffi::LOOPING, value as ALint); }
    }

    // The attached buffer.
    #[fixed_stack_segment]
    pub fn get_buffer(&self) -> Buffer {
        unsafe { Buffer { id: get_source!(i, ffi::BUFFER) as ALuint } }
    }

    /// Set the attached buffer.
    #[fixed_stack_segment]
    pub fn set_buffer(&self, buffer: Buffer) {
        unsafe { ffi::alSourcei(self.id, ffi::BUFFER, buffer.id as ALint); }
    }

    // The playback position, expressed in seconds.
    #[fixed_stack_segment]
    pub fn get_sec_offset(&self) -> ALfloat {
        unsafe { get_source!(f, ffi::SEC_OFFSET) }
    }

    /// Set the playback position, expressed in seconds.
    #[fixed_stack_segment]
    pub fn set_sec_offset(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::SEC_OFFSET, value); }
    }

    // The playback position, expressed in samples.
    #[fixed_stack_segment]
    pub fn get_sample_offset(&self) -> ALfloat {
        unsafe { get_source!(f, ffi::SAMPLE_OFFSET) }
    }

    /// Set the playback position, expressed in samples.
    #[fixed_stack_segment]
    pub fn set_sample_offset(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::SAMPLE_OFFSET, value); }
    }

    // The playback position, expressed in bytes.
    #[fixed_stack_segment]
    pub fn get_byte_offset(&self) -> ALfloat {
        unsafe { get_source!(f, ffi::BYTE_OFFSET) }
    }

    /// Set the playback position, expressed in bytes.
    #[fixed_stack_segment]
    pub fn set_byte_offset(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::BYTE_OFFSET, value); }
    }
}

#[fixed_stack_segment]
pub fn play_sources(sources: &[Source]) {
    unsafe { ffi::alSourcePlayv(sources.len() as ALsizei, &sources[0].id); }
}

#[fixed_stack_segment]
pub fn stop_sources(sources: &[Source]) {
    unsafe { ffi::alSourceStopv(sources.len() as ALsizei, &sources[0].id); }
}

#[fixed_stack_segment]
pub fn rewind_sources(sources: &[Source]) {
    unsafe { ffi::alSourceRewindv(sources.len() as ALsizei, &sources[0].id); }
}

#[fixed_stack_segment]
pub fn pause_sources(sources: &[Source]) {
    unsafe { ffi::alSourcePausev(sources.len() as ALsizei, &sources[0].id); }
}

/// A reference to a buffer object
#[deriving(Clone, Eq)]
pub struct Buffer {
    id: ALuint,
}

/// Generate a one or more buffer objects.
#[fixed_stack_segment]
pub fn gen_buffers(n: uint) -> ~[Buffer] {
    unsafe {
        let mut buffers = vec::from_elem(n, Buffer::null());
        ffi::alGenBuffers(n as ALsizei, &mut buffers[0].id);
        buffers
    }
}

/// Delete the buffers and free the resources they use. Buffers that are
/// currently in use by a source cannot be deleted.
#[fixed_stack_segment]
pub fn delete_buffers(buffers: &[Buffer]) {
    unsafe { ffi::alDeleteBuffers(buffers.len() as ALsizei, &buffers[0].id); }
}

#[deriving(Eq)]
pub enum Format {
    FormatMono8         = ffi::FORMAT_MONO8     as int,
    FormatMono16        = ffi::FORMAT_MONO16    as int,
    FormatStereo8       = ffi::FORMAT_STEREO8   as int,
    FormatStereo16      = ffi::FORMAT_STEREO16  as int,
}

impl Buffer {
    /// An uninitialised buffer object. Any operation performed on this
    /// object will generate an error.
    #[inline]
    fn null() -> Buffer {
        Buffer { id: 0 }
    }

    /// Generate a single buffer object.
    #[fixed_stack_segment]
    pub fn gen() -> Buffer {
        unsafe {
            let mut buffer = Buffer::null();
            ffi::alGenBuffers(1, &mut buffer.id);
            buffer
        }
    }

    /// Delete the buffer and free the resources it uses. Buffers that are
    /// currently in use by a source cannot be deleted.
    #[fixed_stack_segment]
    pub fn delete(&self) {
        unsafe { ffi::alDeleteBuffers(1, &self.id); }
    }

    /// Fill the buffer with PCM audio data.
    #[fixed_stack_segment]
    pub unsafe fn buffer_data<T>(&self, format: Format, data: &[T], freq: ALsizei) {
        ffi::alBufferData(
            self.id, format as ALenum, cast::transmute(&data[0]),
            (sys::size_of::<T>() * data.len()) as ALsizei,
            freq
        );
    }

    /// The frequency of the buffer in Hz.
    #[fixed_stack_segment]
    pub fn get_frequency(&self) -> ALint {
        unsafe {
            let mut value = 0;
            ffi::alGetBufferi(self.id, ffi::FREQUENCY, &mut value);
            value
        }
    }

    /// The bit depth of the buffer.
    #[fixed_stack_segment]
    pub fn get_bits(&self) -> ALint {
        unsafe {
            let mut value = 0;
            ffi::alGetBufferi(self.id, ffi::BITS, &mut value);
            value
        }
    }

    /// The number of channels in the buffer.
    #[fixed_stack_segment]
    pub fn get_channels(&self) -> ALint {
        unsafe {
            let mut value = 0;
            ffi::alGetBufferi(self.id, ffi::CHANNELS, &mut value);
            value
        }
    }

    /// The size of the buffer in bytes.
    #[fixed_stack_segment]
    pub fn get_size(&self) -> ALint {
        unsafe {
            let mut value = 0;
            ffi::alGetBufferi(self.id, ffi::SIZE, &mut value);
            value
        }
    }
}
