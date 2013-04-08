use types::*;

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

pub fn enable(capability: ALenum) {
    unsafe { ::ll::alEnable(capability); }
}

pub fn disable(capability: ALenum) {
    unsafe { ::ll::alDisable(capability); }
}

pub fn is_enabled(capability: ALenum) -> ALboolean {
    unsafe { ::ll::alIsEnabled(capability) }
}

pub fn get_string(param: ALenum) -> ~str {
    unsafe { str::raw::from_c_str(::ll::alGetString(param)) }
}

pub fn get_booleanv(param: ALenum, data: &mut ALboolean) {
    unsafe { ::ll::alGetBooleanv(param, cast::transmute(data)); }
}

pub fn get_integerv(param: ALenum, data: &mut ALint) {
    unsafe { ::ll::alGetIntegerv(param, cast::transmute(data)); }
}

pub fn get_floatv(param: ALenum, data: &mut ALfloat) {
    unsafe { ::ll::alGetFloatv(param, cast::transmute(data)); }
}

pub fn get_doublev(param: ALenum, data: &mut ALdouble) {
    unsafe { ::ll::alGetDoublev(param, cast::transmute(data)); }
}

pub fn get_boolean(param: ALenum) -> ALboolean {
    unsafe { ::ll::alGetBoolean(param) }
}

pub fn get_integer(param: ALenum) -> ALint {
    unsafe { ::ll::alGetInteger(param) }
}

pub fn get_float(param: ALenum) -> ALfloat {
    unsafe { ::ll::alGetFloat(param) }
}

pub fn get_double(param: ALenum) -> ALdouble {
    unsafe { ::ll::alGetDouble(param) }
}

pub fn get_error() -> ALenum {
    unsafe { ::ll::alGetError() }
}

pub fn is_extension_present(extname: &str) -> ALboolean {
    unsafe { ::ll::alIsExtensionPresent(str::as_c_str(extname, |s| s)) }
}

pub fn get_proc_address(fname: &str) -> extern fn() {
    unsafe { cast::transmute(
        ::ll::alGetProcAddress(str::as_c_str(fname, |s| s))
    ) }
}

pub fn get_enum_value(ename: &str) -> ALenum {
    unsafe { ::ll::alGetEnumValue(str::as_c_str(ename, |s| s)) }
}


pub fn listenerf(param: ALenum, value: ALfloat) {
    unsafe { ::ll::alListenerf(param, value); }
}

pub fn listener3f(param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat) {
    unsafe { ::ll::alListener3f(param, value1, value2, value3); }
}

// pub fn Listenerfv(param: ALenum, values: *ALfloat) {
//     unsafe { ::ll::alListenerfv(param, values); }
// }

pub fn listeneri(param: ALenum, value: ALint) {
    unsafe { ::ll::alListeneri(param, value); }
}

pub fn listener3i(param: ALenum, value1: ALint, value2: ALint, value3: ALint) {
    unsafe { ::ll::alListener3i(param, value1, value2, value3); }
}

// pub fn Listeneriv(param: ALenum, values: *ALint) {
//     unsafe { ::ll::alListeneriv(param, values); }
// }

pub fn get_listenerf(param: ALenum) -> ALfloat {
    let mut value = 0.0;
    unsafe { ::ll::alGetListenerf(param, cast::transmute(&value)) }
    value
}

pub fn get_listener3f(param: ALenum) -> (ALfloat, ALfloat, ALfloat) {
    let mut value1 = 0.0,
            value2 = 0.0,
            value3 = 0.0;
    unsafe { ::ll::alGetListener3f(param, cast::transmute(&value1),
                                        cast::transmute(&value2),
                                        cast::transmute(&value3)) }
    (value1, value2, value3)
}

// pub fn GetListenerfv(param: ALenum, values: *ALfloat) {
//     unsafe { ::ll::alGetListenerfv(param, values) }
// }

pub fn get_listeneri(param: ALenum) -> ALint {
    let mut value = 0;
    unsafe { ::ll::alGetListeneri(param, cast::transmute(&value)) }
    value
}

pub fn get_listener3i(param: ALenum) -> (ALint, ALint, ALint) {
    let mut value1 = 0,
            value2 = 0,
            value3 = 0;
    unsafe { ::ll::alGetListener3i(param, cast::transmute(&value1),
                                        cast::transmute(&value2),
                                        cast::transmute(&value3)) }
    (value1, value2, value3)
}

// pub fn GetListeneriv(param: ALenum, values: *ALint) {
//     unsafe { ::ll::alGetListeneriv(param, values) }
// }

pub fn gen_sources(sources: &[ALuint]) {
    unsafe { ::ll::alGenSources(sources.len() as ALsizei, cast::transmute(&sources[0])); }
}

pub fn delete_sources(sources: &[ALuint]) {
    unsafe { ::ll::alDeleteSources(sources.len() as ALsizei, cast::transmute(&sources[0])); }
}

pub fn is_source(sid: ALuint) -> ALboolean {
    unsafe { ::ll::alIsSource(sid) }
}

pub fn sourcef(sid: ALuint, param: ALenum, value: ALfloat) {
    unsafe { ::ll::alSourcef(sid, param, value); }
}

pub fn source3f(sid: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat) {
    unsafe { ::ll::alSource3f(sid, param, value1, value2, value3); }
}

// pub fn Sourcefv(sid: ALuint, param: ALenum, values: *ALfloat) {
//     unsafe { ::ll::alSourcefv(); }
// }

pub fn sourcei(sid: ALuint, param: ALenum, value: ALint) {
    unsafe { ::ll::alSourcei(sid, param, value); }
}

pub fn source3i(sid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint) {
    unsafe { ::ll::alSource3i(sid, param, value1, value2, value3); }
}

// pub fn Sourceiv(sid: ALuint, param: ALenum, values: *ALint) {
//     unsafe { ::ll::alSourceiv(); }
// }

pub fn get_sourcef(sid: ALuint, param: ALenum) -> ALfloat {
    let mut value = 0.0;
    unsafe { ::ll::alGetSourcef(sid, param, cast::transmute(&value)); }
    value
}

pub fn get_source3f(sid: ALuint, param: ALenum) -> (ALfloat, ALfloat, ALfloat) {
    let mut value1 = 0.0,
            value2 = 0.0,
            value3 = 0.0;
    unsafe { ::ll::alGetSource3f(sid, param, cast::transmute(&value1),
                                           cast::transmute(&value2),
                                           cast::transmute(&value3)); }
    (value1, value2, value3)
}

// pub fn GetSourcefv(sid: ALuint, param: ALenum, values: *ALfloat) {
//     unsafe { ::ll::alGetSourcefv(); }
// }

pub fn get_sourcei(sid: ALuint,  param: ALenum) -> ALint {
    let mut value = 0;
    unsafe { ::ll::alGetSourcei(sid, param, cast::transmute(&value)); }
    value
}

pub fn get_source3i(sid: ALuint, param: ALenum) -> (ALint, ALint, ALint) {
    let mut value1 = 0,
            value2 = 0,
            value3 = 0;
    unsafe { ::ll::alGetSource3i(sid, param, cast::transmute(&value1),
                                           cast::transmute(&value2),
                                           cast::transmute(&value3)); }
    (value1, value2, value3)
}

// pub fn GetSourceiv(sid: ALuint,  param: ALenum, values: *ALint) {
//     unsafe { ::ll::alGetSourceiv(); }
// }

pub fn source_playv(sids: &[ALuint]) {
    unsafe { ::ll::alSourcePlayv(sids.len() as ALsizei, cast::transmute(&sids[0])); }
}

pub fn source_stopv(sids: &[ALuint]) {
    unsafe { ::ll::alSourceStopv(sids.len() as ALsizei, cast::transmute(&sids[0])); }
}

pub fn source_rewindv(sids: &[ALuint]) {
    unsafe { ::ll::alSourceRewindv(sids.len() as ALsizei, cast::transmute(&sids[0])); }
}

pub fn source_pausev(sids: &[ALuint]) {
    unsafe { ::ll::alSourcePausev(sids.len() as ALsizei, cast::transmute(&sids[0])); }
}

pub fn source_play(sid: ALuint) {
    unsafe { ::ll::alSourcePlay(sid); }
}

pub fn source_stop(sid: ALuint) {
    unsafe { ::ll::alSourceStop(sid); }
}

pub fn source_rewind(sid: ALuint) {
    unsafe { ::ll::alSourceRewind(sid); }
}

pub fn source_pause(sid: ALuint) {
    unsafe { ::ll::alSourcePause(sid); }
}

pub fn source_queue_buffers(sid: ALuint, bids: &[ALuint]) {
    unsafe { ::ll::alSourceQueueBuffers(sid, bids.len() as ALsizei, cast::transmute(&bids[0])); }
}

pub fn source_unqueue_buffers(sid: ALuint, bids: &[ALuint]) {
    unsafe { ::ll::alSourceUnqueueBuffers(sid, bids.len() as ALsizei, cast::transmute(&bids[0])); }
}

pub fn gen_buffers(buffers: &[ALuint]) {
    unsafe { ::ll::alGenBuffers(buffers.len() as ALsizei, cast::transmute(&buffers[0])); }
}

pub fn delete_buffers(buffers: &[ALuint]) {
    unsafe { ::ll::alDeleteBuffers(buffers.len() as ALsizei, cast::transmute(&buffers[0])); }
}

pub fn is_buffer(bid: ALuint) -> ALboolean {
    unsafe { ::ll::alIsBuffer(bid) }
}

pub fn buffer_data<T>(bid: ALuint, format: ALenum, data: &[T], freq: ALsizei) {
    unsafe {
        ::ll::alBufferData(
            bid, format,
            cast::transmute(&data[0]),
            sys::size_of::<T>() * data.len() as ALsizei,
            freq
        );
    }
}

pub fn bufferf(bid: ALuint, param: ALenum, value: ALfloat) {
    unsafe { ::ll::alBufferf(bid, param, value); }
}

pub fn buffer3f(bid: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat) {
    unsafe { ::ll::alBuffer3f(bid, param, value1, value2, value3); }
}

// pub fn Bufferfv(bid: ALuint, param: ALenum, values: *ALfloat) {
//     unsafe { ::ll::alBufferfv(); }
// }

pub fn bufferi(bid: ALuint, param: ALenum, value: ALint) {
    unsafe { ::ll::alBufferi(bid, param, value); }
}

pub fn buffer3i(bid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint) {
    unsafe { ::ll::alBuffer3i(bid, param, value1, value2, value3); }
}

// pub fn Bufferiv(bid: ALuint, param: ALenum, values: *ALint) {
//     unsafe { ::ll::alBufferiv(); }
// }

pub fn get_bufferf(sid: ALuint, param: ALenum) -> ALfloat {
    let mut value = 0.0;
    unsafe { ::ll::alGetBufferf(sid, param, cast::transmute(&value)); }
    value
}

pub fn get_buffer3f(sid: ALuint, param: ALenum) -> (ALfloat, ALfloat, ALfloat) {
    let mut value1 = 0.0,
            value2 = 0.0,
            value3 = 0.0;
    unsafe { ::ll::alGetBuffer3f(sid, param, cast::transmute(&value1),
                                           cast::transmute(&value2),
                                           cast::transmute(&value3)); }
    (value1, value2, value3)
}

// pub fn GetBufferfv(bid: ALuint, param: ALenum, values: *ALfloat) {
//     unsafe { ::ll::alGetBufferfv(); }
// }

pub fn get_bufferi(sid: ALuint,  param: ALenum) -> ALint {
    let mut value = 0;
    unsafe { ::ll::alGetBufferi(sid, param, cast::transmute(&value)); }
    value
}

pub fn get_buffer3i(sid: ALuint, param: ALenum) -> (ALint, ALint, ALint) {
    let mut value1 = 0,
            value2 = 0,
            value3 = 0;
    unsafe { ::ll::alGetBuffer3i(sid, param, cast::transmute(&value1),
                                           cast::transmute(&value2),
                                           cast::transmute(&value3)); }
    (value1, value2, value3)
}

// pub fn GetBufferiv(bid: ALuint, param: ALenum, values: *ALint) {
//     unsafe { ::ll::alGetBufferiv(); }
// }

pub fn doppler_factor(value: ALfloat) {
    unsafe { ::ll::alDopplerFactor(value); }
}

pub fn doppler_velocity(value: ALfloat) {
    unsafe { ::ll::alDopplerVelocity(value); }
}

pub fn speed_of_sound(value: ALfloat) {
    unsafe { ::ll::alSpeedOfSound(value); }
}