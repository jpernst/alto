use types::*;
use ffi;
use std::cast;
use std::ptr::to_unsafe_ptr;
use std::str;
use std::sys;

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
pub fn is_enabled(capability: ALenum) -> ALboolean {
    unsafe { ffi::alIsEnabled(capability) }
}

#[fixed_stack_segment]
pub fn get_string(param: ALenum) -> ~str {
    unsafe { str::raw::from_c_str(ffi::alGetString(param)) }
}

#[fixed_stack_segment]
pub fn get_booleanv(param: ALenum, data: &mut ALboolean) {
    unsafe { ffi::alGetBooleanv(param, cast::transmute(data)); }
}

#[fixed_stack_segment]
pub fn get_integerv(param: ALenum, data: &mut ALint) {
    unsafe { ffi::alGetIntegerv(param, cast::transmute(data)); }
}

#[fixed_stack_segment]
pub fn get_floatv(param: ALenum, data: &mut ALfloat) {
    unsafe { ffi::alGetFloatv(param, cast::transmute(data)); }
}

#[fixed_stack_segment]
pub fn get_doublev(param: ALenum, data: &mut ALdouble) {
    unsafe { ffi::alGetDoublev(param, cast::transmute(data)); }
}

#[fixed_stack_segment]
pub fn get_boolean(param: ALenum) -> ALboolean {
    unsafe { ffi::alGetBoolean(param) }
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
pub fn is_extension_present(extname: &str) -> ALboolean {
    unsafe { extname.with_c_str(|c_str| ffi::alIsExtensionPresent(c_str)) }
}

#[fixed_stack_segment]
pub fn get_proc_address(fname: &str) -> extern "C" fn() {
    unsafe { cast::transmute(
        fname.with_c_str(|c_str| ffi::alGetProcAddress(c_str))
    ) }
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
pub fn listener3f(param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat) {
    unsafe { ffi::alListener3f(param, value1, value2, value3); }
}

// #[fixed_stack_segment]
// pub fn Listenerfv(param: ALenum, values: *ALfloat) {
//     unsafe { ffi::alListenerfv(param, values); }
// }

#[fixed_stack_segment]
pub fn listeneri(param: ALenum, value: ALint) {
    unsafe { ffi::alListeneri(param, value); }
}

#[fixed_stack_segment]
pub fn listener3i(param: ALenum, value1: ALint, value2: ALint, value3: ALint) {
    unsafe { ffi::alListener3i(param, value1, value2, value3); }
}

// pub fn Listeneriv(param: ALenum, values: *ALint) {
//     unsafe { ffi::alListeneriv(param, values); }
// }

#[fixed_stack_segment]
pub fn get_listenerf(param: ALenum) -> ALfloat {
    unsafe {
        let value = 0.0;
        ffi::alGetListenerf(param, cast::transmute(&value));
        value
    }
}

#[fixed_stack_segment]
pub fn get_listener3f(param: ALenum) -> (ALfloat, ALfloat, ALfloat) {
    unsafe {
        let (value0, value1, value2) = (0.0, 0.0, 0.0);
        ffi::alGetListener3f(param, to_unsafe_ptr(&value0),
                                    to_unsafe_ptr(&value1),
                                    to_unsafe_ptr(&value2));
        (value0, value1, value2)
    }
}

// #[fixed_stack_segment]
// pub fn GetListenerfv(param: ALenum, values: *ALfloat) {
//     unsafe { ffi::alGetListenerfv(param, values) }
// }

#[fixed_stack_segment]
pub fn get_listeneri(param: ALenum) -> ALint {
    unsafe {
        let value = 0;
        ffi::alGetListeneri(param, cast::transmute(&value));
        value
    }
}

#[fixed_stack_segment]
pub fn get_listener3i(param: ALenum) -> (ALint, ALint, ALint) {
    unsafe {
        let (value0, value1, value2) = (0, 0, 0);
        ffi::alGetListener3i(param, to_unsafe_ptr(&value0),
                                    to_unsafe_ptr(&value1),
                                    to_unsafe_ptr(&value2));
        (value0, value1, value2)
    }
}

// #[fixed_stack_segment]
// pub fn GetListeneriv(param: ALenum, values: *ALint) {
//     unsafe { ffi::alGetListeneriv(param, values) }
// }

#[fixed_stack_segment]
pub fn gen_sources(sources: &[ALuint]) {
    unsafe { ffi::alGenSources(sources.len() as ALsizei, cast::transmute(&sources[0])); }
}

#[fixed_stack_segment]
pub fn delete_sources(sources: &[ALuint]) {
    unsafe { ffi::alDeleteSources(sources.len() as ALsizei, cast::transmute(&sources[0])); }
}

#[fixed_stack_segment]
pub fn is_source(sid: ALuint) -> ALboolean {
    unsafe { ffi::alIsSource(sid) }
}

#[fixed_stack_segment]
pub fn sourcef(sid: ALuint, param: ALenum, value: ALfloat) {
    unsafe { ffi::alSourcef(sid, param, value); }
}

#[fixed_stack_segment]
pub fn source3f(sid: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat) {
    unsafe { ffi::alSource3f(sid, param, value1, value2, value3); }
}

// #[fixed_stack_segment]
// pub fn Sourcefv(sid: ALuint, param: ALenum, values: *ALfloat) {
//     unsafe { ffi::alSourcefv(); }
// }

#[fixed_stack_segment]
pub fn sourcei(sid: ALuint, param: ALenum, value: ALint) {
    unsafe { ffi::alSourcei(sid, param, value); }
}

#[fixed_stack_segment]
pub fn source3i(sid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint) {
    unsafe { ffi::alSource3i(sid, param, value1, value2, value3); }
}

// #[fixed_stack_segment]
// pub fn Sourceiv(sid: ALuint, param: ALenum, values: *ALint) {
//     unsafe { ffi::alSourceiv(); }
// }

#[fixed_stack_segment]
pub fn get_sourcef(sid: ALuint, param: ALenum) -> ALfloat {
    unsafe {
        let value = 0.0;
        ffi::alGetSourcef(sid, param, to_unsafe_ptr(&value));
        value
    }
}

#[fixed_stack_segment]
pub fn get_source3f(sid: ALuint, param: ALenum) -> (ALfloat, ALfloat, ALfloat) {
    unsafe {
        let (value0, value1, value2) = (0.0, 0.0, 0.0);
        ffi::alGetSource3f(sid, param, to_unsafe_ptr(&value0),
                                       to_unsafe_ptr(&value1),
                                       to_unsafe_ptr(&value2));
        (value0, value1, value2)
    }
}

// #[fixed_stack_segment]
// pub fn GetSourcefv(sid: ALuint, param: ALenum, values: *ALfloat) {
//     unsafe { ffi::alGetSourcefv(); }
// }

#[fixed_stack_segment]
pub fn get_sourcei(sid: ALuint,  param: ALenum) -> ALint {
    unsafe {
        let value = 0;
        ffi::alGetSourcei(sid, param, to_unsafe_ptr(&value));
        value
    }
}

#[fixed_stack_segment]
pub fn get_source3i(sid: ALuint, param: ALenum) -> (ALint, ALint, ALint) {
    unsafe {
        let (value0, value1, value2) = (0, 0, 0);
        ffi::alGetSource3i(sid, param, to_unsafe_ptr(&value0),
                                       to_unsafe_ptr(&value1),
                                       to_unsafe_ptr(&value2));
        (value0, value1, value2)
    }
}

// #[fixed_stack_segment]
// pub fn GetSourceiv(sid: ALuint,  param: ALenum, values: *ALint) {
//     unsafe { ffi::alGetSourceiv(); }
// }

#[fixed_stack_segment]
pub fn source_playv(sids: &[ALuint]) {
    unsafe { ffi::alSourcePlayv(sids.len() as ALsizei, cast::transmute(&sids[0])); }
}

#[fixed_stack_segment]
pub fn source_stopv(sids: &[ALuint]) {
    unsafe { ffi::alSourceStopv(sids.len() as ALsizei, cast::transmute(&sids[0])); }
}

#[fixed_stack_segment]
pub fn source_rewindv(sids: &[ALuint]) {
    unsafe { ffi::alSourceRewindv(sids.len() as ALsizei, cast::transmute(&sids[0])); }
}

#[fixed_stack_segment]
pub fn source_pausev(sids: &[ALuint]) {
    unsafe { ffi::alSourcePausev(sids.len() as ALsizei, cast::transmute(&sids[0])); }
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
    unsafe { ffi::alSourceQueueBuffers(sid, bids.len() as ALsizei, cast::transmute(&bids[0])); }
}

#[fixed_stack_segment]
pub fn source_unqueue_buffers(sid: ALuint, bids: &[ALuint]) {
    unsafe { ffi::alSourceUnqueueBuffers(sid, bids.len() as ALsizei, cast::transmute(&bids[0])); }
}

#[fixed_stack_segment]
pub fn gen_buffers(buffers: &[ALuint]) {
    unsafe { ffi::alGenBuffers(buffers.len() as ALsizei, cast::transmute(&buffers[0])); }
}

#[fixed_stack_segment]
pub fn delete_buffers(buffers: &[ALuint]) {
    unsafe { ffi::alDeleteBuffers(buffers.len() as ALsizei, cast::transmute(&buffers[0])); }
}

#[fixed_stack_segment]
pub fn is_buffer(bid: ALuint) -> ALboolean {
    unsafe { ffi::alIsBuffer(bid) }
}

#[fixed_stack_segment]
pub fn buffer_data<T>(bid: ALuint, format: ALenum, data: &[T], freq: ALsizei) {
    unsafe {
        ffi::alBufferData(
            bid, format,
            cast::transmute(&data[0]),
            sys::size_of::<T>() as ALsizei * data.len() as ALsizei,
            freq
        );
    }
}

#[fixed_stack_segment]
pub fn bufferf(bid: ALuint, param: ALenum, value: ALfloat) {
    unsafe { ffi::alBufferf(bid, param, value); }
}

#[fixed_stack_segment]
pub fn buffer3f(bid: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat) {
    unsafe { ffi::alBuffer3f(bid, param, value1, value2, value3); }
}

// #[fixed_stack_segment]
// pub fn Bufferfv(bid: ALuint, param: ALenum, values: *ALfloat) {
//     unsafe { ffi::alBufferfv(); }
// }

#[fixed_stack_segment]
pub fn bufferi(bid: ALuint, param: ALenum, value: ALint) {
    unsafe { ffi::alBufferi(bid, param, value); }
}

#[fixed_stack_segment]
pub fn buffer3i(bid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint) {
    unsafe { ffi::alBuffer3i(bid, param, value1, value2, value3); }
}

// #[fixed_stack_segment]
// pub fn Bufferiv(bid: ALuint, param: ALenum, values: *ALint) {
//     unsafe { ffi::alBufferiv(); }
// }

#[fixed_stack_segment]
pub fn get_bufferf(sid: ALuint, param: ALenum) -> ALfloat {
    unsafe {
        let value = 0.0;
        ffi::alGetBufferf(sid, param, to_unsafe_ptr(&value));
        value
    }
}

#[fixed_stack_segment]
pub fn get_buffer3f(sid: ALuint, param: ALenum) -> (ALfloat, ALfloat, ALfloat) {
    unsafe {
        let (value0, value1, value2) = (0.0, 0.0, 0.0);
        ffi::alGetBuffer3f(sid, param, to_unsafe_ptr(&value0),
                                       to_unsafe_ptr(&value1),
                                       to_unsafe_ptr(&value2));
        (value0, value1, value2)
    }
}

// #[fixed_stack_segment]
// pub fn GetBufferfv(bid: ALuint, param: ALenum, values: *ALfloat) {
//     unsafe { ffi::alGetBufferfv(); }
// }

#[fixed_stack_segment]
pub fn get_bufferi(sid: ALuint,  param: ALenum) -> ALint {
    unsafe {
        let value = 0;
        ffi::alGetBufferi(sid, param, to_unsafe_ptr(&value));
        value
    }
}

#[fixed_stack_segment]
pub fn get_buffer3i(sid: ALuint, param: ALenum) -> (ALint, ALint, ALint) {
    unsafe {
        let (value0, value1, value2) = (0, 0, 0);
        ffi::alGetBuffer3i(sid, param, to_unsafe_ptr(&value0),
                                       to_unsafe_ptr(&value1),
                                       to_unsafe_ptr(&value2));
        (value0, value1, value2)
    }
}

// #[fixed_stack_segment]
// pub fn GetBufferiv(bid: ALuint, param: ALenum, values: *ALint) {
//     unsafe { ffi::alGetBufferiv(); }
// }

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
