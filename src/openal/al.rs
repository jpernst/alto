use std::cast;
use std::ptr::to_unsafe_ptr;
use std::str;
use std::sys;
use std::vec;

use self::types::*;

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

    extern "C" {
        pub fn alEnable(capability: ALenum);
        pub fn alDisable(capability: ALenum);
        pub fn alIsEnabled(capability: ALenum) -> ALboolean;
        pub fn alGetString(param: ALenum) -> *ALchar;
        pub fn alGetBooleanv(param: ALenum, data: *ALboolean);
        pub fn alGetIntegerv(param: ALenum, data: *ALint);
        pub fn alGetFloatv(param: ALenum, data: *ALfloat);
        pub fn alGetDoublev(param: ALenum, data: *ALdouble);
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
        pub fn alGetListenerf(param: ALenum, value: *ALfloat);
        pub fn alGetListener3f(param: ALenum, value1: *ALfloat, value2: *ALfloat, value3: *ALfloat);
        pub fn alGetListenerfv(param: ALenum, values: *ALfloat);
        pub fn alGetListeneri(param: ALenum, value: *ALint);
        pub fn alGetListener3i(param: ALenum, value1: *ALint, value2: *ALint, value3: *ALint);
        pub fn alGetListeneriv(param: ALenum, values: *ALint);
        pub fn alGenSources(n: ALsizei, sources: *ALuint);
        pub fn alDeleteSources(n: ALsizei, sources: *ALuint);
        pub fn alIsSource(sid: ALuint) -> ALboolean;
        pub fn alSourcef(sid: ALuint, param: ALenum, value: ALfloat);
        pub fn alSource3f(sid: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
        pub fn alSourcefv(sid: ALuint, param: ALenum, values: *ALfloat);
        pub fn alSourcei(sid: ALuint, param: ALenum, value: ALint);
        pub fn alSource3i(sid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint);
        pub fn alSourceiv(sid: ALuint, param: ALenum, values: *ALint);
        pub fn alGetSourcef(sid: ALuint, param: ALenum, value: *ALfloat);
        pub fn alGetSource3f(sid: ALuint, param: ALenum, value1: *ALfloat, value2: *ALfloat, value3: *ALfloat);
        pub fn alGetSourcefv(sid: ALuint, param: ALenum, values: *ALfloat);
        pub fn alGetSourcei(sid: ALuint,  param: ALenum, value: *ALint);
        pub fn alGetSource3i(sid: ALuint, param: ALenum, value1: *ALint, value2: *ALint, value3: *ALint);
        pub fn alGetSourceiv(sid: ALuint,  param: ALenum, values: *ALint);
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
        pub fn alGenBuffers(n: ALsizei, buffers: *ALuint);
        pub fn alDeleteBuffers(n: ALsizei, buffers: *ALuint);
        pub fn alIsBuffer(bid: ALuint) -> ALboolean;
        pub fn alBufferData(bid: ALuint, format: ALenum, data: *ALvoid, size: ALsizei, freq: ALsizei);
        pub fn alBufferf(bid: ALuint, param: ALenum, value: ALfloat);
        pub fn alBuffer3f(bid: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
        pub fn alBufferfv(bid: ALuint, param: ALenum, values: *ALfloat);
        pub fn alBufferi(bid: ALuint, param: ALenum, value: ALint);
        pub fn alBuffer3i(bid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint);
        pub fn alBufferiv(bid: ALuint, param: ALenum, values: *ALint);
        pub fn alGetBufferf(bid: ALuint, param: ALenum, value: *ALfloat);
        pub fn alGetBuffer3f(bid: ALuint, param: ALenum, value1: *ALfloat, value2: *ALfloat, value3: *ALfloat);
        pub fn alGetBufferfv(bid: ALuint, param: ALenum, values: *ALfloat);
        pub fn alGetBufferi(bid: ALuint, param: ALenum, value: *ALint);
        pub fn alGetBuffer3i(bid: ALuint, param: ALenum, value1: *ALint, value2: *ALint, value3: *ALint);
        pub fn alGetBufferiv(bid: ALuint, param: ALenum, values: *ALint);
        pub fn alDopplerFactor(value: ALfloat);
        pub fn alDopplerVelocity(value: ALfloat);
        pub fn alSpeedOfSound(value: ALfloat);
    }
}

pub static NONE                           : ALenum = 0;

pub static FALSE                          : ALboolean = 0;
pub static TRUE                           : ALboolean = 1;

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
pub static SOURCE_STATE                   : ALenum = 0x1010;
pub static INITIAL                        : ALenum = 0x1011;
pub static PLAYING                        : ALenum = 0x1012;
pub static PAUSED                         : ALenum = 0x1013;
pub static STOPPED                        : ALenum = 0x1014;
pub static BUFFERS_QUEUED                 : ALenum = 0x1015;
pub static BUFFERS_PROCESSED              : ALenum = 0x1016;
pub static SEC_OFFSET                     : ALenum = 0x1024;
pub static SAMPLE_OFFSET                  : ALenum = 0x1025;
pub static BYTE_OFFSET                    : ALenum = 0x1026;
pub static SOURCE_TYPE                    : ALenum = 0x1027;
pub static STATIC                         : ALenum = 0x1028;
pub static STREAMING                      : ALenum = 0x1029;
pub static UNDETERMINED                   : ALenum = 0x1030;
pub static FORMAT_MONO8                   : ALenum = 0x1100;
pub static FORMAT_MONO16                  : ALenum = 0x1101;
pub static FORMAT_STEREO8                 : ALenum = 0x1102;
pub static FORMAT_STEREO16                : ALenum = 0x1103;
pub static REFERENCE_DISTANCE             : ALenum = 0x1020;
pub static ROLLOFF_FACTOR                 : ALenum = 0x1021;
pub static CONE_OUTER_GAIN                : ALenum = 0x1022;
pub static MAX_DISTANCE                   : ALenum = 0x1023;
pub static FREQUENCY                      : ALenum = 0x2001;
pub static BITS                           : ALenum = 0x2002;
pub static CHANNELS                       : ALenum = 0x2003;
pub static SIZE                           : ALenum = 0x2004;
pub static UNUSED                         : ALenum = 0x2010;
pub static PENDING                        : ALenum = 0x2011;
pub static PROCESSED                      : ALenum = 0x2012;
pub static NO_ERROR                       : ALenum = FALSE as ALenum;
pub static INVALID_NAME                   : ALenum = 0xA001;
pub static INVALID_ENUM                   : ALenum = 0xA002;
pub static INVALID_VALUE                  : ALenum = 0xA003;
pub static INVALID_OPERATION              : ALenum = 0xA004;
pub static OUT_OF_MEMORY                  : ALenum = 0xA005;
pub static VENDOR                         : ALenum = 0xB001;
pub static VERSION                        : ALenum = 0xB002;
pub static RENDERER                       : ALenum = 0xB003;
pub static EXTENSIONS                     : ALenum = 0xB004;
pub static DOPPLER_FACTOR                 : ALenum = 0xC000;
pub static DOPPLER_VELOCITY               : ALenum = 0xC001;
pub static SPEED_OF_SOUND                 : ALenum = 0xC003;
pub static DISTANCE_MODEL                 : ALenum = 0xD000;
pub static INVERSE_DISTANCE               : ALenum = 0xD001;
pub static INVERSE_DISTANCE_CLAMPED       : ALenum = 0xD002;
pub static LINEAR_DISTANCE                : ALenum = 0xD003;
pub static LINEAR_DISTANCE_CLAMPED        : ALenum = 0xD004;
pub static EXPONENT_DISTANCE              : ALenum = 0xD005;
pub static EXPONENT_DISTANCE_CLAMPED      : ALenum = 0xD006;

#[fixed_stack_segment]
pub fn enable(capability: ALenum) {
    unsafe { ffi::alEnable(capability); }
}

#[fixed_stack_segment]
pub fn disable(capability: ALenum) {
    unsafe { ffi::alDisable(capability); }
}

#[fixed_stack_segment]
pub fn is_enabled(capability: ALenum) -> bool {
    unsafe { ffi::alIsEnabled(capability) == TRUE }
}

#[fixed_stack_segment]
pub fn get_string(param: ALenum) -> ~str {
    unsafe { str::raw::from_c_str(ffi::alGetString(param)) }
}

#[fixed_stack_segment]
pub fn get_booleanv(param: ALenum, data: *ALboolean) {
    unsafe { ffi::alGetBooleanv(param, data); }
}

#[fixed_stack_segment]
pub fn get_integerv(param: ALenum, data: *ALint) {
    unsafe { ffi::alGetIntegerv(param, data); }
}

#[fixed_stack_segment]
pub fn get_floatv(param: ALenum, data: *ALfloat) {
    unsafe { ffi::alGetFloatv(param, data); }
}

#[fixed_stack_segment]
pub fn get_doublev(param: ALenum, data: *ALdouble) {
    unsafe { ffi::alGetDoublev(param, data); }
}

#[fixed_stack_segment]
pub fn get_boolean(param: ALenum) -> bool {
    unsafe { ffi::alGetBoolean(param) == TRUE }
}

#[fixed_stack_segment]
pub fn get_integer(param: ALenum) -> ALint {
    unsafe { ffi::alGetInteger(param) }
}

#[fixed_stack_segment]
pub fn get_float(param: ALenum) -> ALfloat {
    unsafe { ffi::alGetFloat(param) }
}

#[fixed_stack_segment]
pub fn get_double(param: ALenum) -> ALdouble {
    unsafe { ffi::alGetDouble(param) }
}

#[fixed_stack_segment]
pub fn get_error() -> ALenum {
    unsafe { ffi::alGetError() }
}

#[fixed_stack_segment]
pub fn is_extension_present(extname: &str) -> bool {
    unsafe { extname.with_c_str(|c_str| ffi::alIsExtensionPresent(c_str)) == TRUE }
}

#[fixed_stack_segment]
pub fn get_proc_address(fname: &str) -> Option<extern "C" fn()> {
    unsafe { fname.with_c_str(|c_str| ffi::alGetProcAddress(c_str)) }
}

#[fixed_stack_segment]
pub fn get_enum_value(ename: &str) -> ALenum {
    unsafe { ename.with_c_str(|c_str| ffi::alGetEnumValue(c_str)) }
}

#[fixed_stack_segment]
pub fn listenerf(param: ALenum, value: ALfloat) {
    unsafe { ffi::alListenerf(param, value); }
}

#[fixed_stack_segment]
pub fn listener3f(param: ALenum, values: [ALfloat, ..3]) {
    unsafe { ffi::alListener3f(param, values[0], values[1], values[2]); }
}

#[fixed_stack_segment]
pub fn listenerfv(param: ALenum, values: *ALfloat) {
    unsafe { ffi::alListenerfv(param, values); }
}

#[fixed_stack_segment]
pub fn listeneri(param: ALenum, value: ALint) {
    unsafe { ffi::alListeneri(param, value); }
}

#[fixed_stack_segment]
pub fn listener3i(param: ALenum, values: [ALint, ..3]) {
    unsafe { ffi::alListener3i(param, values[0], values[1], values[2]); }
}

#[fixed_stack_segment]
pub fn listeneriv(param: ALenum, values: *ALint) {
    unsafe { ffi::alListeneriv(param, values); }
}

#[fixed_stack_segment]
pub fn get_listenerf(param: ALenum) -> ALfloat {
    unsafe {
        let value = 0.0;
        ffi::alGetListenerf(param, cast::transmute(&value));
        value
    }
}

#[fixed_stack_segment]
pub fn get_listener3f(param: ALenum) -> [ALfloat, ..3] {
    unsafe {
        let values = [0.0, ..3];
        ffi::alGetListener3f(param, to_unsafe_ptr(&values[0]), to_unsafe_ptr(&values[1]), to_unsafe_ptr(&values[2]));
        values
    }
}

#[fixed_stack_segment]
pub unsafe fn get_listenerfv(param: ALenum, values: *ALfloat) {
    ffi::alGetListenerfv(param, values)
}

#[fixed_stack_segment]
pub fn get_listeneri(param: ALenum) -> ALint {
    unsafe {
        let value = 0;
        ffi::alGetListeneri(param, cast::transmute(&value));
        value
    }
}

#[fixed_stack_segment]
pub fn get_listener3i(param: ALenum) -> [ALint, ..3] {
    unsafe {
        let values = [0, ..3];
        ffi::alGetListener3i(param, to_unsafe_ptr(&values[0]), to_unsafe_ptr(&values[1]), to_unsafe_ptr(&values[2]));
        values
    }
}

#[fixed_stack_segment]
pub unsafe fn get_listeneriv(param: ALenum, values: *ALint) {
    ffi::alGetListeneriv(param, values)
}

#[fixed_stack_segment]
pub fn gen_sources(n: uint) -> ~[ALuint] {
    unsafe {
        let mut sources = ~[];
        ffi::alGenSources(n as ALsizei, vec::raw::to_ptr(sources));
        vec::raw::set_len(&mut sources, n);
        sources
    }
}

#[fixed_stack_segment]
pub fn delete_sources(sources: &[ALuint]) {
    unsafe { ffi::alDeleteSources(sources.len() as ALsizei, vec::raw::to_ptr(sources)); }
}

#[fixed_stack_segment]
pub fn is_source(sid: ALuint) -> bool {
    unsafe { ffi::alIsSource(sid) == TRUE }
}

#[fixed_stack_segment]
pub fn sourcef(sid: ALuint, param: ALenum, value: ALfloat) {
    unsafe { ffi::alSourcef(sid, param, value); }
}

#[fixed_stack_segment]
pub fn source3f(sid: ALuint, param: ALenum, values: [ALfloat, ..3]) {
    unsafe { ffi::alSource3f(sid, param, values[0], values[1], values[2]); }
}

#[fixed_stack_segment]
pub fn sourcefv(sid: ALuint, param: ALenum, values: *ALfloat) {
    unsafe { ffi::alSourcefv(sid, param, values); }
}

#[fixed_stack_segment]
pub fn sourcei(sid: ALuint, param: ALenum, value: ALint) {
    unsafe { ffi::alSourcei(sid, param, value); }
}

#[fixed_stack_segment]
pub fn source3i(sid: ALuint, param: ALenum, values: [ALint, ..3]) {
    unsafe { ffi::alSource3i(sid, param, values[0], values[1], values[2]); }
}

#[fixed_stack_segment]
pub unsafe fn sourceiv(sid: ALuint, param: ALenum, values: *ALint) {
    ffi::alSourceiv(sid, param, values);
}

#[fixed_stack_segment]
pub fn get_sourcef(sid: ALuint, param: ALenum) -> ALfloat {
    unsafe {
        let value = 0.0;
        ffi::alGetSourcef(sid, param, to_unsafe_ptr(&value));
        value
    }
}

#[fixed_stack_segment]
pub fn get_source3f(sid: ALuint, param: ALenum) -> [ALfloat, ..3] {
    unsafe {
        let values = [0.0, ..3];
        ffi::alGetSource3f(sid, param, to_unsafe_ptr(&values[0]), to_unsafe_ptr(&values[1]), to_unsafe_ptr(&values[2]));
        values
    }
}

#[fixed_stack_segment]
pub unsafe fn get_sourcefv(sid: ALuint, param: ALenum, values: *ALfloat) {
    ffi::alGetSourcefv(sid, param, values);
}

#[fixed_stack_segment]
pub fn get_sourcei(sid: ALuint,  param: ALenum) -> ALint {
    unsafe {
        let value = 0;
        ffi::alGetSourcei(sid, param, to_unsafe_ptr(&value));
        value
    }
}

#[fixed_stack_segment]
pub fn get_source3i(sid: ALuint, param: ALenum) -> [ALint, ..3] {
    unsafe {
        let values = [0, ..3];
        ffi::alGetSource3i(sid, param, to_unsafe_ptr(&values[0]), to_unsafe_ptr(&values[1]), to_unsafe_ptr(&values[2]));
        values
    }
}

#[fixed_stack_segment]
pub fn get_sourceiv(sid: ALuint,  param: ALenum, values: *ALint) {
    unsafe { ffi::alGetSourceiv(sid, param, values); }
}

#[fixed_stack_segment]
pub fn source_playv(sids: &[ALuint]) {
    unsafe { ffi::alSourcePlayv(sids.len() as ALsizei, vec::raw::to_ptr(sids)); }
}

#[fixed_stack_segment]
pub fn source_stopv(sids: &[ALuint]) {
    unsafe { ffi::alSourceStopv(sids.len() as ALsizei, vec::raw::to_ptr(sids)); }
}

#[fixed_stack_segment]
pub fn source_rewindv(sids: &[ALuint]) {
    unsafe { ffi::alSourceRewindv(sids.len() as ALsizei, vec::raw::to_ptr(sids)); }
}

#[fixed_stack_segment]
pub fn source_pausev(sids: &[ALuint]) {
    unsafe { ffi::alSourcePausev(sids.len() as ALsizei, vec::raw::to_ptr(sids)); }
}

#[fixed_stack_segment]
pub fn source_play(sid: ALuint) {
    unsafe { ffi::alSourcePlay(sid); }
}

#[fixed_stack_segment]
pub fn source_stop(sid: ALuint) {
    unsafe { ffi::alSourceStop(sid); }
}

#[fixed_stack_segment]
pub fn source_rewind(sid: ALuint) {
    unsafe { ffi::alSourceRewind(sid); }
}

#[fixed_stack_segment]
pub fn source_pause(sid: ALuint) {
    unsafe { ffi::alSourcePause(sid); }
}

#[fixed_stack_segment]
pub fn source_queue_buffers(sid: ALuint, bids: &[ALuint]) {
    unsafe { ffi::alSourceQueueBuffers(sid, bids.len() as ALsizei, vec::raw::to_ptr(bids)); }
}

#[fixed_stack_segment]
pub fn source_unqueue_buffers(sid: ALuint, bids: &[ALuint]) {
    unsafe { ffi::alSourceUnqueueBuffers(sid, bids.len() as ALsizei, vec::raw::to_ptr(bids)); }
}

#[fixed_stack_segment]
pub fn gen_buffers(n: uint) -> ~[ALuint] {
    unsafe {
        let mut buffers = ~[];
        ffi::alGenBuffers(n as ALsizei, vec::raw::to_ptr(buffers));
        vec::raw::set_len(&mut buffers, n);
        buffers
    }
}

#[fixed_stack_segment]
pub fn delete_buffers(buffers: &[ALuint]) {
    unsafe { ffi::alDeleteBuffers(buffers.len() as ALsizei, vec::raw::to_ptr(buffers)); }
}

#[fixed_stack_segment]
pub fn is_buffer(bid: ALuint) -> bool {
    unsafe { ffi::alIsBuffer(bid) == TRUE }
}

#[fixed_stack_segment]
pub unsafe fn buffer_data<T>(bid: ALuint, format: ALenum, data: &[T], freq: ALsizei) {
    ffi::alBufferData(
        bid, format,
        cast::transmute(&data[0]),
        sys::size_of::<T>() as ALsizei * data.len() as ALsizei,
        freq
    );
}

#[fixed_stack_segment]
pub fn bufferf(bid: ALuint, param: ALenum, value: ALfloat) {
    unsafe { ffi::alBufferf(bid, param, value); }
}

#[fixed_stack_segment]
pub fn buffer3f(bid: ALuint, param: ALenum, values: [ALfloat, ..3]) {
    unsafe { ffi::alBuffer3f(bid, param, values[0], values[1], values[2]); }
}

#[fixed_stack_segment]
pub fn bufferfv(bid: ALuint, param: ALenum, values: *ALfloat) {
    unsafe { ffi::alBufferfv(bid, param, values); }
}

#[fixed_stack_segment]
pub fn bufferi(bid: ALuint, param: ALenum, value: ALint) {
    unsafe { ffi::alBufferi(bid, param, value); }
}

#[fixed_stack_segment]
pub fn buffer3i(bid: ALuint, param: ALenum, values: [ALint, ..3]) {
    unsafe { ffi::alBuffer3i(bid, param, values[0], values[1], values[2]); }
}

#[fixed_stack_segment]
pub fn bufferiv(bid: ALuint, param: ALenum, values: *ALint) {
    unsafe { ffi::alBufferiv(bid, param, values); }
}

#[fixed_stack_segment]
pub fn get_bufferf(bid: ALuint, param: ALenum) -> ALfloat {
    unsafe {
        let value = 0.0;
        ffi::alGetBufferf(bid, param, to_unsafe_ptr(&value));
        value
    }
}

#[fixed_stack_segment]
pub fn get_buffer3f(bid: ALuint, param: ALenum) -> [ALfloat, ..3] {
    unsafe {
        let values = [0.0, ..3];
        ffi::alGetBuffer3f(bid, param, to_unsafe_ptr(&values[0]), to_unsafe_ptr(&values[1]), to_unsafe_ptr(&values[2]));
        values
    }
}

#[fixed_stack_segment]
pub unsafe fn get_bufferfv(bid: ALuint, param: ALenum, values: *ALfloat) {
    ffi::alGetBufferfv(bid, param, values);
}

#[fixed_stack_segment]
pub fn get_bufferi(bid: ALuint,  param: ALenum) -> ALint {
    unsafe {
        let value = 0;
        ffi::alGetBufferi(bid, param, to_unsafe_ptr(&value));
        value
    }
}

#[fixed_stack_segment]
pub fn get_buffer3i(bid: ALuint, param: ALenum) -> [ALint, ..3] {
    unsafe {
        let values = [0, ..3];
        ffi::alGetBuffer3i(bid, param, to_unsafe_ptr(&values[0]), to_unsafe_ptr(&values[1]), to_unsafe_ptr(&values[2]));
        values
    }
}

#[fixed_stack_segment]
pub unsafe fn get_bufferiv(bid: ALuint, param: ALenum, values: *ALint) {
    ffi::alGetBufferiv(bid, param, values);
}

#[fixed_stack_segment]
pub fn doppler_factor(value: ALfloat) {
    unsafe { ffi::alDopplerFactor(value); }
}

#[fixed_stack_segment]
pub fn doppler_velocity(value: ALfloat) {
    unsafe { ffi::alDopplerVelocity(value); }
}

#[fixed_stack_segment]
pub fn speed_of_sound(value: ALfloat) {
    unsafe { ffi::alSpeedOfSound(value); }
}
