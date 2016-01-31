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

use std::borrow::ToOwned;
use std::fmt;
use std::mem;
use std::str;
use std::ffi::CStr;
use libc;

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

pub fn get_vendor() -> String {
    unsafe { str::from_utf8(CStr::from_ptr(ffi::alGetString(ffi::VENDOR) as *const libc::c_char).to_bytes()).unwrap().to_owned() }
}

pub fn get_version() -> String {
    unsafe { str::from_utf8(CStr::from_ptr(ffi::alGetString(ffi::VERSION) as *const libc::c_char).to_bytes()).unwrap().to_owned() }
}

pub fn get_renderer() -> String {
    unsafe { str::from_utf8(CStr::from_ptr(ffi::alGetString(ffi::RENDERER) as *const libc::c_char).to_bytes()).unwrap().to_owned() }
}

pub fn get_extensions() -> String {
    unsafe { str::from_utf8(CStr::from_ptr(ffi::alGetString(ffi::EXTENSIONS) as *const libc::c_char).to_bytes()).unwrap().to_owned() }
}

pub fn get_doppler_factor() -> ALfloat {
    unsafe { ffi::alGetFloat(ffi::DOPPLER_FACTOR) }
}

pub fn set_doppler_factor(value: ALfloat) {
    unsafe { ffi::alDopplerFactor(value); }
}

pub fn get_doppler_velocity() -> ALfloat {
    unsafe { ffi::alGetFloat(ffi::DOPPLER_VELOCITY) }
}

pub fn set_doppler_velocity(value: ALfloat) {
    unsafe { ffi::alDopplerVelocity(value); }
}

pub fn get_speed_of_sound() -> ALfloat {
    unsafe { ffi::alGetFloat(ffi::SPEED_OF_SOUND) }
}

pub fn set_speed_of_sound(value: ALfloat) {
    unsafe { ffi::alSpeedOfSound(value); }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum DistanceModel {
    Inverse             = ffi::INVERSE_DISTANCE,
    InverseClamped      = ffi::INVERSE_DISTANCE_CLAMPED,
    Linear              = ffi::LINEAR_DISTANCE,
    LinearClamped       = ffi::LINEAR_DISTANCE_CLAMPED,
    Exponent            = ffi::EXPONENT_DISTANCE,
    ExponentClamped     = ffi::EXPONENT_DISTANCE_CLAMPED
}

pub fn get_distance_model() -> Option<DistanceModel> {
    unsafe {
        match ffi::alGetInteger(ffi::DISTANCE_MODEL) {
            ffi::NONE => None,
            model => Some(mem::transmute(model)),
        }
    }
}

pub fn set_distance_model(value: Option<DistanceModel>) {
    unsafe {
        match value {
            Some(model) => ffi::alDistanceModel(model as ALenum),
            None => ffi::alDistanceModel(ffi::NONE),
        }
    }
}

/// An OpenAL error code
#[derive(Copy, Clone, PartialEq, Eq)]
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
pub fn get_error() -> Option<Error> {
    match unsafe { ffi::alGetError() } {
        ffi::INVALID_NAME       => Some(Error::InvalidName),
        ffi::INVALID_ENUM       => Some(Error::InvalidEnum),
        ffi::INVALID_VALUE      => Some(Error::InvalidValue),
        ffi::INVALID_OPERATION  => Some(Error::InvalidOperation),
        ffi::OUT_OF_MEMORY      => Some(Error::OutOfMemory),
        _                       => None,
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidName         => write!(f, "InvalidName: A bad name (ID) was passed to an OpenAL function."),
            Error::InvalidEnum         => write!(f, "InvalidEnum: An invalid enum value was passed to an OpenAL function."),
            Error::InvalidValue        => write!(f, "InvalidValue: An invalid value was passed to an OpenAL function."),
            Error::InvalidOperation    => write!(f, "InvalidOperation: The requested operation is not valid."),
            Error::OutOfMemory         => write!(f, "OutOfMemory: The requested operation resulted in OpenAL running out of memory."),
        }
    }
}

/// Functions for setting and retrieving the properties of the listener. One
/// listener is implied for each context.
pub mod listener {
    use std::mem;
    use super::ffi;
    use super::types::*;

    // The master gain.
    pub fn get_gain() -> ALfloat {
        unsafe {
            let mut value = 0.0;
            ffi::alGetListenerf(ffi::GAIN, &mut value);
            value
        }
    }

    /// Set the master gain (should be positive).
    pub fn set_gain(value: ALfloat) {
        unsafe { ffi::alListenerf(ffi::GAIN, value); }
    }

    // The position of the listener.
    pub fn get_position() -> [ALfloat; 3] {
        unsafe {
            let mut values = [0.0; 3];
            ffi::alGetListenerfv(ffi::POSITION, &mut values[0]);
            values
        }
    }

    /// Set the position of the listener.
    pub fn set_position(values: [ALfloat; 3]) {
        unsafe { ffi::alListenerfv(ffi::POSITION, &values[0]); }
    }

    // The velocity vector.
    pub fn get_velocity() -> [ALfloat; 3] {
        unsafe {
            let mut values = [0.0; 3];
            ffi::alGetListenerfv(ffi::VELOCITY, &mut values[0]);
            values
        }
    }

    /// Set the velocity vector.
    pub fn set_velocity(values: [ALfloat; 3]) {
        unsafe { ffi::alListenerfv(ffi::VELOCITY, &values[0]); }
    }

    // The orientation of the listener, expressed as 'at' and 'up' vectors.
    pub fn get_orientation() -> ([ALfloat; 3], [ALfloat; 3]) {
        unsafe {
            let mut values = ([0.0; 3], [0.0; 3]);
            ffi::alGetListenerfv(ffi::ORIENTATION, mem::transmute(&mut values));
            values
        }
    }

    /// Set the orientation of the listener.
    pub fn set_orientation(at: [ALfloat; 3], up: [ALfloat; 3]) {
        unsafe {
            let values = (at, up);
            ffi::alListenerfv(ffi::ORIENTATION, mem::transmute(&values));
        }
    }
}

/// A reference to a source object
pub struct Source {
    id: ALuint,
}

/// Generate a one or more source objects.
pub fn gen_sources(n: usize) -> Vec<Source> {
    unsafe {
        let mut sources = Vec::new();
        sources.reserve(n);
        ffi::alGenSources(n as ALsizei, mem::transmute(sources.as_mut_ptr()));
        sources.set_len(n);
        sources
    }
}

/// Delete the sources.
///
/// If an error occurs, the sources will not be deleted. The error can be
/// detected using `get_error`. An individual source may be deleted if it is
/// currently playing. In this case it will be stopped, then destroyed.
pub fn delete_sources(sources: Vec<Source>) {
    let _ = sources;
    // unsafe { ffi::alDeleteSources(sources.len() as ALsizei, &sources[0].id); }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum SourceType {
    Static          = ffi::STATIC,
    Streaming       = ffi::STREAMING,
    Undetermined    = ffi::UNDETERMINED,
}

macro_rules! get_source{
    ($self_:ident, fv, $param:expr, $n:expr) => ({
        let mut values = [0.0; $n];
        ffi::alGetSourcef($self_.id, $param, &mut values[0]);
        values
    });
    ($self_:ident, f, $param:expr) => ({
        let mut value = 0.0;
        ffi::alGetSourcef($self_.id, $param, &mut value);
        value
    });
    ($self_:ident, i, $param:expr) => ({
        let mut value = 0;
        ffi::alGetSourcei($self_.id, $param, &mut value);
        value
    });
}

impl Source {
    /// Generate a single source object.
    pub fn gen() -> Source {
        unsafe {
            let mut id = 0;
            ffi::alGenSources(1, &mut id);
            Source { id: id }
        }
    }

    /// Delete the source.
    ///
    /// If an error occurs, the source will not be deleted. The error can be
    /// detected using `get_error`. The source may be deleted if it is
    /// currently playing. In this case it will be stopped, then destroyed.
    pub fn delete(self) {}

    /// Play the buffers attached to the source.
    pub fn play(&self) {
        unsafe { ffi::alSourcePlay(self.id); }
    }

    // Returns `true` if the source is playing.
    pub fn is_playing(&self) -> bool {
        unsafe { (get_source!(self, i, ffi::SOURCE_STATE)) as ALenum == ffi::PLAYING }
    }

    /// Pause the source.
    pub fn pause(&self) {
        unsafe { ffi::alSourcePause(self.id); }
    }

    // Returns `true` if the source is paused.
    pub fn is_paused(&self) -> bool {
        unsafe { (get_source!(self, i, ffi::SOURCE_STATE)) as ALenum == ffi::PAUSED }
    }

    /// Stop the source
    pub fn stop(&self) {
        unsafe { ffi::alSourceStop(self.id); }
    }

    // Returns `true` if the source is stopped.
    pub fn is_stopped(&self) -> bool {
        unsafe { (get_source!(self, i, ffi::SOURCE_STATE)) as ALenum == ffi::STOPPED }
    }

    /// Rewind the source to the initial state.
    pub fn rewind(&self) {
        unsafe { ffi::alSourceRewind(self.id); }
    }

    // Returns `true` if the source is at the initial state.
    pub fn is_initial(&self) -> bool {
        unsafe { (get_source!(self, i, ffi::SOURCE_STATE)) as ALenum == ffi::INITIAL }
    }

    /// Queue a single buffer on the source.
    pub fn queue_buffer(&self, buffer: &Buffer) {
        unsafe { ffi::alSourceQueueBuffers(self.id, 1, &buffer.id); }
    }

    /// Queue the buffers on the source to be played in sequence.
    pub fn queue_buffers(&self, buffers: &[Buffer]) {
        unsafe { ffi::alSourceQueueBuffers(self.id, buffers.len() as ALsizei, &buffers[0].id); }
    }

    /// Remove a single buffer from the queue.
    pub fn unqueue_buffer(&self, buffer: &mut Buffer) {
        unsafe { ffi::alSourceUnqueueBuffers(self.id, 1, &mut buffer.id); }
    }

    /// Remove a set of buffers from the queue.
    pub fn unqueue_buffers(&self, buffers: &mut [Buffer]) {
        unsafe { ffi::alSourceUnqueueBuffers(self.id, buffers.len() as ALsizei, &mut buffers[0].id); }
    }

    // The number of buffers queued on this source.
    pub fn get_buffers_queued(&self) -> usize {
        unsafe { (get_source!(self, i, ffi::BUFFERS_QUEUED)) as usize }
    }

    // the number of buffers in the queue that have been processed.
    pub fn get_buffers_processed(&self) -> usize {
        unsafe { (get_source!(self, i, ffi::BUFFERS_PROCESSED)) as usize }
    }

    // The pitch multiplier.
    pub fn get_pitch(&self) -> ALfloat {
        unsafe { get_source!(self, f, ffi::PITCH) }
    }

    /// Set the pitch multiplier (should be positive).
    pub fn set_pitch(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::PITCH, value); }
    }

    // The source gain.
    pub fn get_gain(&self) -> ALfloat {
        unsafe { get_source!(self, f, ffi::GAIN) }
    }

    /// Set the source gain (should be positive).
    pub fn set_gain(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::GAIN, value); }
    }

    /// Used with the Inverse Clamped Distance Model to set the distance where
    /// there will no longer be any attenuation of the source.
    pub fn get_max_distance(&self) -> ALfloat {
        unsafe { get_source!(self, f, ffi::MAX_DISTANCE) }
    }

    /// Set the max distance for the source.
    pub fn set_max_distance(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::MAX_DISTANCE, value); }
    }

    // The rolloff factor for the source.
    pub fn get_rolloff_factor(&self) -> ALfloat {
        unsafe { get_source!(self, f, ffi::ROLLOFF_FACTOR) }
    }

    /// Set the rolloff factor for the source (default is `1.0`).
    pub fn set_rolloff_factor(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::ROLLOFF_FACTOR, value); }
    }

    /// The distance under which the volume for the source would normally drop
    /// by half (before being influenced by rolloff factor or max distance).
    pub fn get_reference_distance(&self) -> ALfloat {
        unsafe { get_source!(self, f, ffi::REFERENCE_DISTANCE) }
    }

    /// Set the reference distance.
    pub fn set_reference_distance(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::REFERENCE_DISTANCE, value); }
    }

    // The minimum gain for the source.
    pub fn get_min_gain(&self) -> ALfloat {
        unsafe { get_source!(self, f, ffi::MIN_GAIN) }
    }

    /// Set the minimum gain for the source.
    pub fn set_min_gain(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::MIN_GAIN, value); }
    }

    // The maximum gain for the source.
    pub fn get_max_gain(&self) -> ALfloat {
        unsafe { get_source!(self, f, ffi::MAX_GAIN) }
    }

    /// Set the maximum gain for the source.
    pub fn set_max_gain(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::MAX_GAIN, value); }
    }

    // The gain when outside the oriented cone.
    pub fn get_cone_outer_gain(&self) -> ALfloat {
        unsafe { get_source!(self, f, ffi::CONE_OUTER_GAIN) }
    }

    /// Set the gain when outside the oriented cone.
    pub fn set_cone_outer_gain(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::CONE_OUTER_GAIN, value); }
    }

    // The gain when inside the oriented cone.
    pub fn get_cone_inner_angle(&self) -> ALfloat {
        unsafe { get_source!(self, f, ffi::CONE_INNER_ANGLE) }
    }

    /// Set the gain when inside the oriented cone.
    pub fn set_cone_inner_angle(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::CONE_INNER_ANGLE, value); }
    }

    // The outer angle of the sound cone, in degrees.
    pub fn get_cone_outer_angle(&self) -> ALfloat {
        unsafe { get_source!(self, f, ffi::CONE_OUTER_ANGLE) }
    }

    /// Set the outer angle of the sound cone, in degrees. (default is `360.0`)
    pub fn set_cone_outer_angle(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::CONE_OUTER_ANGLE, value); }
    }

    // The position of the source.
    pub fn get_position(&self) -> [ALfloat; 3] {
        unsafe { get_source!(self, fv, ffi::POSITION, 3) }
    }

    /// Set the position of the source.
    pub fn set_position(&self, values: [ALfloat; 3]) {
        unsafe { ffi::alSourcefv(self.id, ffi::POSITION, &values[0]); }
    }

    // The velocity vector of the source.
    pub fn get_velocity(&self) -> [ALfloat; 3] {
        unsafe { get_source!(self, fv, ffi::VELOCITY, 3) }
    }

    /// Set the velocity vector of the source.
    pub fn set_velocity(&self, values: [ALfloat; 3]) {
        unsafe { ffi::alSourcefv(self.id, ffi::VELOCITY, &values[0]); }
    }

    // The direction vector of the source.
    pub fn get_direction(&self) -> [ALfloat; 3] {
        unsafe { get_source!(self, fv, ffi::DIRECTION, 3) }
    }

    /// Set the direction vector of the source.
    pub fn set_direction(&self, values: [ALfloat; 3]) {
        unsafe { ffi::alSourcefv(self.id, ffi::DIRECTION, &values[0]); }
    }

    // Returns `true` if the positions are relative to the listener.
    pub fn is_relative(&self) -> bool {
        unsafe { (get_source!(self, i, ffi::SOURCE_RELATIVE)) as ALboolean == ffi::TRUE }
    }

    /// Set whether the positions are relative to the listener. (default is `false`)
    pub fn set_relative(&self, value: bool) {
        unsafe { ffi::alSourcei(self.id, ffi::SOURCE_RELATIVE, value as ALint); }
    }

    // The source type.
    pub fn get_type(&self) -> SourceType {
        unsafe { mem::transmute(get_source!(self, i, ffi::SOURCE_TYPE) as i32) }
    }

    /// Set the source type.
    pub fn set_type(&self, value: SourceType) {
        unsafe { ffi::alSourcei(self.id, ffi::SOURCE_TYPE, value as ALint); }
    }

    // Returns `true` looping is turned on for this source.
    pub fn is_looping(&self) -> bool {
        unsafe { (get_source!(self, i, ffi::LOOPING)) as ALboolean == ffi::TRUE }
    }

    /// Set looping on/off for this source.
    pub fn set_looping(&self, value: bool) {
        unsafe { ffi::alSourcei(self.id, ffi::LOOPING, value as ALint); }
    }

    // The attached buffer.
    pub fn get_buffer(&self) -> Buffer {
        unsafe { Buffer { id: get_source!(self, i, ffi::BUFFER) as ALuint } }
    }

    /// Set the attached buffer.
    pub fn set_buffer(&self, buffer: Buffer) {
        unsafe { ffi::alSourcei(self.id, ffi::BUFFER, buffer.id as ALint); }
    }

    // The playback position, expressed in seconds.
    pub fn get_sec_offset(&self) -> ALfloat {
        unsafe { get_source!(self, f, ffi::SEC_OFFSET) }
    }

    /// Set the playback position, expressed in seconds.
    pub fn set_sec_offset(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::SEC_OFFSET, value); }
    }

    // The playback position, expressed in samples.
    pub fn get_sample_offset(&self) -> ALfloat {
        unsafe { get_source!(self, f, ffi::SAMPLE_OFFSET) }
    }

    /// Set the playback position, expressed in samples.
    pub fn set_sample_offset(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::SAMPLE_OFFSET, value); }
    }

    // The playback position, expressed in bytes.
    pub fn get_byte_offset(&self) -> ALfloat {
        unsafe { get_source!(self, f, ffi::BYTE_OFFSET) }
    }

    /// Set the playback position, expressed in bytes.
    pub fn set_byte_offset(&self, value: ALfloat) {
        unsafe { ffi::alSourcef(self.id, ffi::BYTE_OFFSET, value); }
    }
}

impl Drop for Source {
    /// Delete the source.
    ///
    /// If an error occurs, the source will not be deleted. The error can be
    /// detected using `get_error`. The source may be deleted if it is
    /// currently playing. In this case it will be stopped, then destroyed.
    fn drop(&mut self) {
        unsafe { ffi::alDeleteSources(1, &self.id); }
    }
}

pub fn play_sources(sources: &[Source]) {
    unsafe { ffi::alSourcePlayv(sources.len() as ALsizei, sources.as_ptr() as *const ALuint); }
}

pub fn stop_sources(sources: &[Source]) {
    unsafe { ffi::alSourceStopv(sources.len() as ALsizei, sources.as_ptr() as *const ALuint); }
}

pub fn rewind_sources(sources: &[Source]) {
    unsafe { ffi::alSourceRewindv(sources.len() as ALsizei, sources.as_ptr() as *const ALuint); }
}

pub fn pause_sources(sources: &[Source]) {
    unsafe { ffi::alSourcePausev(sources.len() as ALsizei, sources.as_ptr() as *const ALuint); }
}

/// A reference to a buffer object
pub struct Buffer {
    id: ALuint,
}

/// Generate a one or more buffer objects.
pub fn gen_buffers(n: usize) -> Vec<Buffer> {
    unsafe {
        let mut buffers = Vec::new();
        buffers.reserve(n);
        ffi::alGenBuffers(n as ALsizei, mem::transmute(buffers.as_mut_ptr()));
        buffers.set_len(n);
        buffers
    }
}

/// Delete the buffers and free the resources they use. Buffers that are
/// currently in use by a source cannot be deleted.
pub fn delete_buffers(buffers: Vec<Buffer>) {
    let _ = buffers;
    // unsafe { ffi::alDeleteBuffers(buffers.len() as ALsizei, &buffers[0].id); }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Format {
    Mono8         = ffi::FORMAT_MONO8     as isize,
    Mono16        = ffi::FORMAT_MONO16    as isize,
    Stereo8       = ffi::FORMAT_STEREO8   as isize,
    Stereo16      = ffi::FORMAT_STEREO16  as isize,
}

impl Buffer {
    /// Generate a single buffer object.
    pub fn gen() -> Buffer {
        unsafe {
            let mut id = 0;
            ffi::alGenBuffers(1, &mut id);
            Buffer { id: id }
        }
    }

    /// Delete the buffer and free the resources it uses. Buffers that are
    /// currently in use by a source cannot be deleted.
    pub fn delete(self) {}

    /// Fill the buffer with PCM audio data.
    pub unsafe fn buffer_data<T>(&self, format: Format, data: &[T], freq: ALsizei) {
        ffi::alBufferData(
            self.id, format as ALenum, data.as_ptr() as *const ALvoid,
            (mem::size_of::<T>() * data.len()) as ALsizei,
            freq
        );
    }

    /// The frequency of the buffer in Hz.
    pub fn get_frequency(&self) -> ALint {
        unsafe {
            let mut value = 0;
            ffi::alGetBufferi(self.id, ffi::FREQUENCY, &mut value);
            value
        }
    }

    /// The bit depth of the buffer.
    pub fn get_bits(&self) -> ALint {
        unsafe {
            let mut value = 0;
            ffi::alGetBufferi(self.id, ffi::BITS, &mut value);
            value
        }
    }

    /// The number of channels in the buffer.
    pub fn get_channels(&self) -> ALint {
        unsafe {
            let mut value = 0;
            ffi::alGetBufferi(self.id, ffi::CHANNELS, &mut value);
            value
        }
    }

    /// The size of the buffer in bytes.
    pub fn get_size(&self) -> ALint {
        unsafe {
            let mut value = 0;
            ffi::alGetBufferi(self.id, ffi::SIZE, &mut value);
            value
        }
    }
}

impl Drop for Buffer {
    /// Delete the buffer and free the resources it uses. Buffers that are
    /// currently in use by a source cannot be deleted.
    fn drop(&mut self) {
        unsafe { ffi::alDeleteBuffers(1, &self.id); }
    }
}
