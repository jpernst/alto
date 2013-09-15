use types::*;
use std::{cast, str, sys, uint};

// pub static NONE                           : ALenum = 0;
// pub static FALSE                          : ALboolean = 0;
// pub static TRUE                           : ALboolean = 1;
// pub static SOURCE_RELATIVE                : ALenum = 0x202;
// pub static CONE_INNER_ANGLE               : ALenum = 0x1001;
// pub static CONE_OUTER_ANGLE               : ALenum = 0x1002;
// pub static PITCH                          : ALenum = 0x1003;
// pub static POSITION                       : ALenum = 0x1004;
// pub static DIRECTION                      : ALenum = 0x1005;
// pub static VELOCITY                       : ALenum = 0x1006;
// pub static LOOPING                        : ALenum = 0x1007;
// pub static BUFFER                         : ALenum = 0x1009;
// pub static GAIN                           : ALenum = 0x100A;
// pub static MIN_GAIN                       : ALenum = 0x100D;
// pub static MAX_GAIN                       : ALenum = 0x100E;
// pub static ORIENTATION                    : ALenum = 0x100F;
// pub static SOURCE_STATE                   : ALenum = 0x1010;
// pub static INITIAL                        : ALenum = 0x1011;
// pub static PLAYING                        : ALenum = 0x1012;
// pub static PAUSED                         : ALenum = 0x1013;
// pub static STOPPED                        : ALenum = 0x1014;
// pub static BUFFERS_QUEUED                 : ALenum = 0x1015;
// pub static BUFFERS_PROCESSED              : ALenum = 0x1016;
// pub static SEC_OFFSET                     : ALenum = 0x1024;
// pub static SAMPLE_OFFSET                  : ALenum = 0x1025;
// pub static BYTE_OFFSET                    : ALenum = 0x1026;
// pub static SOURCE_TYPE                    : ALenum = 0x1027;
// pub static STATIC                         : ALenum = 0x1028;
// pub static STREAMING                      : ALenum = 0x1029;
// pub static UNDETERMINED                   : ALenum = 0x1030;
// pub static FORMAT_MONO8                   : ALenum = 0x1100;
// pub static FORMAT_MONO16                  : ALenum = 0x1101;
// pub static FORMAT_STEREO8                 : ALenum = 0x1102;
// pub static FORMAT_STEREO16                : ALenum = 0x1103;
// pub static REFERENCE_DISTANCE             : ALenum = 0x1020;
// pub static ROLLOFF_FACTOR                 : ALenum = 0x1021;
// pub static CONE_OUTER_GAIN                : ALenum = 0x1022;
// pub static MAX_DISTANCE                   : ALenum = 0x1023;
// pub static FREQUENCY                      : ALenum = 0x2001;
// pub static BITS                           : ALenum = 0x2002;
// pub static CHANNELS                       : ALenum = 0x2003;
// pub static SIZE                           : ALenum = 0x2004;
// pub static UNUSED                         : ALenum = 0x2010;
// pub static PENDING                        : ALenum = 0x2011;
// pub static PROCESSED                      : ALenum = 0x2012;
// pub static NO_ERROR                       : ALenum = FALSE as ALenum;
// pub static INVALID_NAME                   : ALenum = 0xA001;
// pub static INVALID_ENUM                   : ALenum = 0xA002;
// pub static INVALID_VALUE                  : ALenum = 0xA003;
// pub static INVALID_OPERATION              : ALenum = 0xA004;
// pub static OUT_OF_MEMORY                  : ALenum = 0xA005;
// pub static VENDOR                         : ALenum = 0xB001;
// pub static VERSION                        : ALenum = 0xB002;
// pub static RENDERER                       : ALenum = 0xB003;
// pub static EXTENSIONS                     : ALenum = 0xB004;
// pub static DOPPLER_FACTOR                 : ALenum = 0xC000;
// pub static DOPPLER_VELOCITY               : ALenum = 0xC001;
// pub static SPEED_OF_SOUND                 : ALenum = 0xC003;
// pub static DISTANCE_MODEL                 : ALenum = 0xD000;
// pub static INVERSE_DISTANCE               : ALenum = 0xD001;
// pub static INVERSE_DISTANCE_CLAMPED       : ALenum = 0xD002;
// pub static LINEAR_DISTANCE                : ALenum = 0xD003;
// pub static LINEAR_DISTANCE_CLAMPED        : ALenum = 0xD004;
// pub static EXPONENT_DISTANCE              : ALenum = 0xD005;
// pub static EXPONENT_DISTANCE_CLAMPED      : ALenum = 0xD006;

pub enum ALenum {
    None_NoError = 0,
    SourceRelative = 0x202,
    ConeInnerAngle = 0x1001,
    ConeOuterAngle = 0x1002,
    Pitch = 0x1003,
    Position = 0x1004,
    Direction = 0x1005,
    Velocity = 0x1006,
    Looping = 0x1007,
    Buffer = 0x1009,
    Gain = 0x100A,
    MinGain = 0x100d,
    MaxGain = 0x100e,
    Orientation = 0x100f,
    SourceState = 0x1010,
    Initial = 0x1011,
    Playing = 0x1012,
    Paused = 0x1013,
    Stopped = 0x1014,
    BuffersQueued = 0x1015,
    BuffersProcessed = 0x1016,
    SecOffset = 0x1024,
    SampleOffset = 0x1025,
    ByteOffset = 0x1026,
    SourceType = 0x1027,
    Static = 0x1028,
    Streaming = 0x1029,
    Undetermined = 0x1030,
    FormatMono8 = 0x1100,
    FormatMono16 = 0x1101,
    FormatStereo8 = 0x1102,
    FormatStereo16 = 0x1103,
    ReferenceDistance = 0x1020,
    RolloffFactor = 0x1021,
    ConeOuterGain = 0x1022,
    MaxDistance = 0x1023,
    Frequency = 0x2001,
    Bits = 0x2002,
    Channels = 0x2003,
    Size = 0x2004,
    Unused = 0x2010,
    Pending = 0x2011,
    Processed = 0x2012,
    InvalidName = 0xA001,
    InvalidEnum = 0xA002,
    InvalidValue = 0xA003,
    InvalidOperation = 0xA004,
    OutOfMemory = 0xA005,
    Vendor = 0xb001,
    Version = 0xb002,
    Renderer = 0xb003,
    Extensions = 0xb004,
    DopplerFactor = 0xc000,
    DopplerVelocity = 0xc001,
    SpeedofSound = 0xc003,
    DistanceModel = 0xd000,
    InverseDistance = 0xd001,
    InverseDistanceClamped = 0xd002,
    LinearDistance = 0xd003,
    LinearDistanceClamped = 0xd004,
    ExponentDistance = 0xd005,
    ExponentDistanceClamped = 0xd006
}

fn uint_to_ALenum(n: uint) -> Option<ALenum> {
    unsafe {
        match n {
            1u .. 513u => None,
            515u .. 4096u => None,
            4104u => None,
            4107u .. 4108u => None,
            4119u .. 4127u => None,
            4138u .. 4143u => None,
            4145u .. 4351u => None,
            4356u .. 8192u => None,
            8197u .. 8207u => None,
            8211u .. 40960u => None,
            40966u .. 45056u => None,
            45061u .. 49151u => None,
            49154u => None,
            49156u .. 53247u => None,
            53255u .. uint::max_value => None,
            _ => Some(cast::transmute(n as uint)),
        }
    }
}

#[fixed_stack_segment]
pub fn enable(capability: ALenum) {
    unsafe { ::ll::alEnable(capability as i32); }
}

#[fixed_stack_segment]
pub fn disable(capability: ALenum) {
    unsafe { ::ll::alDisable(capability as i32); }
}

#[fixed_stack_segment]
pub fn is_enabled(capability: ALenum) -> ALboolean {
    unsafe { ::ll::alIsEnabled(capability as i32) }
}

#[fixed_stack_segment]
pub fn get_string(param: ALenum) -> ~str {
    unsafe { str::raw::from_c_str(::ll::alGetString(param as i32)) }
}

#[fixed_stack_segment]
pub fn get_booleanv(param: ALenum, data: &mut ALboolean) {
    unsafe { ::ll::alGetBooleanv(param as i32, cast::transmute(data)); }
}

#[fixed_stack_segment]
pub fn get_integerv(param: ALenum, data: &mut ALint) {
    unsafe { ::ll::alGetIntegerv(param as i32, cast::transmute(data)); }
}

#[fixed_stack_segment]
pub fn get_floatv(param: ALenum, data: &mut ALfloat) {
    unsafe { ::ll::alGetFloatv(param as i32, cast::transmute(data)); }
}

#[fixed_stack_segment]
pub fn get_doublev(param: ALenum, data: &mut ALdouble) {
    unsafe { ::ll::alGetDoublev(param as i32, cast::transmute(data)); }
}

#[fixed_stack_segment]
pub fn get_boolean(param: ALenum) -> ALboolean {
    unsafe { ::ll::alGetBoolean(param as i32) }
}

#[fixed_stack_segment]
pub fn get_integer(param: ALenum) -> ALint {
    unsafe { ::ll::alGetInteger(param as i32) }
}

#[fixed_stack_segment]
pub fn get_float(param: ALenum) -> ALfloat {
    unsafe { ::ll::alGetFloat(param as i32) }
}

#[fixed_stack_segment]
pub fn get_double(param: ALenum) -> ALdouble {
    unsafe { ::ll::alGetDouble(param as i32) }
}

#[fixed_stack_segment]
pub fn get_error() -> ALenum {
    unsafe { uint_to_ALenum(::ll::alGetError() as uint).unwrap() }
}

#[fixed_stack_segment]
pub fn is_extension_present(extname: &str) -> ALboolean {
    unsafe { ::ll::alIsExtensionPresent(extname.with_c_str( |s| s)) }
}

#[fixed_stack_segment]
pub fn get_proc_address(fname: &str) -> extern fn() {
    unsafe { cast::transmute(
        ::ll::alGetProcAddress( fname.with_c_str( |s| s))
    ) }
}

#[fixed_stack_segment]
pub fn get_enum_value(ename: &str) -> ALenum {
    unsafe { uint_to_ALenum(::ll::alGetEnumValue( ename.with_c_str( |s| s)) as uint).unwrap() }
}


#[fixed_stack_segment]
pub fn listenerf(param: ALenum, value: ALfloat) {
    unsafe { ::ll::alListenerf(param as i32, value); }
}

#[fixed_stack_segment]
pub fn listener3f(param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat) {
    unsafe { ::ll::alListener3f(param as i32, value1, value2, value3); }
}

// #[fixed_stack_segment]
// pub fn Listenerfv(param: ALenum, values: *ALfloat) {
//     unsafe { ::ll::alListenerfv(param, values); }
// }

#[fixed_stack_segment]
pub fn listeneri(param: ALenum, value: ALint) {
    unsafe { ::ll::alListeneri(param as i32, value); }
}

#[fixed_stack_segment]
pub fn listener3i(param: ALenum, value1: ALint, value2: ALint, value3: ALint) {
    unsafe { ::ll::alListener3i(param as i32, value1, value2, value3); }
}

// pub fn Listeneriv(param: ALenum, values: *ALint) {
//     unsafe { ::ll::alListeneriv(param, values); }
// }

#[fixed_stack_segment]
pub fn get_listenerf(param: ALenum) -> ALfloat {
    let mut value = 0.0;
    unsafe { ::ll::alGetListenerf(param as i32, cast::transmute(&value)) }
    value
}

#[fixed_stack_segment]
pub fn get_listener3f(param: ALenum) -> (ALfloat, ALfloat, ALfloat) {
    let mut value1 = 0.0;
    let mut value2 = 0.0;
    let mut value3 = 0.0;
    unsafe { ::ll::alGetListener3f(param as i32, cast::transmute(&value1),
                                        cast::transmute(&value2),
                                        cast::transmute(&value3)) }
    (value1, value2, value3)
}

// #[fixed_stack_segment]
// pub fn GetListenerfv(param: ALenum, values: *ALfloat) {
//     unsafe { ::ll::alGetListenerfv(param, values) }
// }

#[fixed_stack_segment]
pub fn get_listeneri(param: ALenum) -> ALint {
    let mut value = 0;
    unsafe { ::ll::alGetListeneri(param as i32, cast::transmute(&value)) }
    value
}

#[fixed_stack_segment]
pub fn get_listener3i(param: ALenum) -> (ALint, ALint, ALint) {
    let mut value1 = 0;
    let mut value2 = 0;
    let mut value3 = 0;
    unsafe { ::ll::alGetListener3i(param as i32, cast::transmute(&value1),
                                        cast::transmute(&value2),
                                        cast::transmute(&value3)) }
    (value1, value2, value3)
}

// #[fixed_stack_segment]
// pub fn GetListeneriv(param: ALenum, values: *ALint) {
//     unsafe { ::ll::alGetListeneriv(param, values) }
// }

#[fixed_stack_segment]
pub fn gen_sources(sources: &[ALuint]) {
    unsafe { ::ll::alGenSources(sources.len() as ALsizei, cast::transmute(&sources[0])); }
}

#[fixed_stack_segment]
pub fn delete_sources(sources: &[ALuint]) {
    unsafe { ::ll::alDeleteSources(sources.len() as ALsizei, cast::transmute(&sources[0])); }
}

#[fixed_stack_segment]
pub fn is_source(sid: ALuint) -> ALboolean {
    unsafe { ::ll::alIsSource(sid) }
}

#[fixed_stack_segment]
pub fn sourcef(sid: ALuint, param: ALenum, value: ALfloat) {
    unsafe { ::ll::alSourcef(sid, param as i32, value); }
}

#[fixed_stack_segment]
pub fn source3f(sid: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat) {
    unsafe { ::ll::alSource3f(sid, param as i32, value1, value2, value3); }
}

// #[fixed_stack_segment]
// pub fn Sourcefv(sid: ALuint, param: ALenum, values: *ALfloat) {
//     unsafe { ::ll::alSourcefv(); }
// }

#[fixed_stack_segment]
pub fn sourcei(sid: ALuint, param: ALenum, value: ALint) {
    unsafe { ::ll::alSourcei(sid, param as i32, value); }
}

#[fixed_stack_segment]
pub fn source3i(sid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint) {
    unsafe { ::ll::alSource3i(sid, param as i32, value1, value2, value3); }
}

// #[fixed_stack_segment]
// pub fn Sourceiv(sid: ALuint, param: ALenum, values: *ALint) {
//     unsafe { ::ll::alSourceiv(); }
// }

#[fixed_stack_segment]
pub fn get_sourcef(sid: ALuint, param: ALenum) -> ALfloat {
    let mut value = 0.0;
    unsafe { ::ll::alGetSourcef(sid, param as i32, cast::transmute(&value)); }
    value
}

#[fixed_stack_segment]
pub fn get_source3f(sid: ALuint, param: ALenum) -> (ALfloat, ALfloat, ALfloat) {
    let mut value1 = 0.0;
    let mut value2 = 0.0;
    let mut value3 = 0.0;
    unsafe { ::ll::alGetSource3f(sid, param as i32, cast::transmute(&value1),
                                           cast::transmute(&value2),
                                           cast::transmute(&value3)); }
    (value1, value2, value3)
}

// #[fixed_stack_segment]
// pub fn GetSourcefv(sid: ALuint, param: ALenum, values: *ALfloat) {
//     unsafe { ::ll::alGetSourcefv(); }
// }

#[fixed_stack_segment]
pub fn get_sourcei(sid: ALuint,  param: ALenum) -> ALint {
    let mut value = 0;
    unsafe { ::ll::alGetSourcei(sid, param as i32, cast::transmute(&value)); }
    value
}

#[fixed_stack_segment]
pub fn get_source3i(sid: ALuint, param: ALenum) -> (ALint, ALint, ALint) {
    let mut value1 = 0;
    let mut value2 = 0;
    let mut value3 = 0;
    unsafe { ::ll::alGetSource3i(sid, param as i32, cast::transmute(&value1),
                                           cast::transmute(&value2),
                                           cast::transmute(&value3)); }
    (value1, value2, value3)
}

// #[fixed_stack_segment]
// pub fn GetSourceiv(sid: ALuint,  param: ALenum, values: *ALint) {
//     unsafe { ::ll::alGetSourceiv(); }
// }

#[fixed_stack_segment]
pub fn source_playv(sids: &[ALuint]) {
    unsafe { ::ll::alSourcePlayv(sids.len() as ALsizei, cast::transmute(&sids[0])); }
}

#[fixed_stack_segment]
pub fn source_stopv(sids: &[ALuint]) {
    unsafe { ::ll::alSourceStopv(sids.len() as ALsizei, cast::transmute(&sids[0])); }
}

#[fixed_stack_segment]
pub fn source_rewindv(sids: &[ALuint]) {
    unsafe { ::ll::alSourceRewindv(sids.len() as ALsizei, cast::transmute(&sids[0])); }
}

#[fixed_stack_segment]
pub fn source_pausev(sids: &[ALuint]) {
    unsafe { ::ll::alSourcePausev(sids.len() as ALsizei, cast::transmute(&sids[0])); }
}

#[fixed_stack_segment]
pub fn source_play(sid: ALuint) {
    unsafe { ::ll::alSourcePlay(sid); }
}

#[fixed_stack_segment]
pub fn source_stop(sid: ALuint) {
    unsafe { ::ll::alSourceStop(sid); }
}

#[fixed_stack_segment]
pub fn source_rewind(sid: ALuint) {
    unsafe { ::ll::alSourceRewind(sid); }
}

#[fixed_stack_segment]
pub fn source_pause(sid: ALuint) {
    unsafe { ::ll::alSourcePause(sid); }
}

#[fixed_stack_segment]
pub fn source_queue_buffers(sid: ALuint, bids: &[ALuint]) {
    unsafe { ::ll::alSourceQueueBuffers(sid, bids.len() as ALsizei, cast::transmute(&bids[0])); }
}

#[fixed_stack_segment]
pub fn source_unqueue_buffers(sid: ALuint, bids: &[ALuint]) {
    unsafe { ::ll::alSourceUnqueueBuffers(sid, bids.len() as ALsizei, cast::transmute(&bids[0])); }
}

#[fixed_stack_segment]
pub fn gen_buffers(buffers: &[ALuint]) {
    unsafe { ::ll::alGenBuffers(buffers.len() as ALsizei, cast::transmute(&buffers[0])); }
}

#[fixed_stack_segment]
pub fn delete_buffers(buffers: &[ALuint]) {
    unsafe { ::ll::alDeleteBuffers(buffers.len() as ALsizei, cast::transmute(&buffers[0])); }
}

#[fixed_stack_segment]
pub fn is_buffer(bid: ALuint) -> ALboolean {
    unsafe { ::ll::alIsBuffer(bid) }
}

#[fixed_stack_segment]
pub fn buffer_data<T>(bid: ALuint, format: ALenum, data: &[T], freq: ALsizei) {
    unsafe {
        ::ll::alBufferData(
            bid, format as i32,
            cast::transmute(&data[0]),
            sys::size_of::<T>() as ALsizei * data.len() as ALsizei,
            freq
        );
    }
}

#[fixed_stack_segment]
pub fn bufferf(bid: ALuint, param: ALenum, value: ALfloat) {
    unsafe { ::ll::alBufferf(bid, param as i32, value); }
}

#[fixed_stack_segment]
pub fn buffer3f(bid: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat) {
    unsafe { ::ll::alBuffer3f(bid, param as i32, value1, value2, value3); }
}

// #[fixed_stack_segment]
// pub fn Bufferfv(bid: ALuint, param: ALenum, values: *ALfloat) {
//     unsafe { ::ll::alBufferfv(); }
// }

#[fixed_stack_segment]
pub fn bufferi(bid: ALuint, param: ALenum, value: ALint) {
    unsafe { ::ll::alBufferi(bid, param as i32, value); }
}

#[fixed_stack_segment]
pub fn buffer3i(bid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint) {
    unsafe { ::ll::alBuffer3i(bid, param as i32, value1, value2, value3); }
}

// #[fixed_stack_segment]
// pub fn Bufferiv(bid: ALuint, param: ALenum, values: *ALint) {
//     unsafe { ::ll::alBufferiv(); }
// }

#[fixed_stack_segment]
pub fn get_bufferf(sid: ALuint, param: ALenum) -> ALfloat {
    let mut value = 0.0;
    unsafe { ::ll::alGetBufferf(sid, param as i32, cast::transmute(&value)); }
    value
}

#[fixed_stack_segment]
pub fn get_buffer3f(sid: ALuint, param: ALenum) -> (ALfloat, ALfloat, ALfloat) {
    let mut value1 = 0.0;
    let mut value2 = 0.0;
    let mut value3 = 0.0;
    unsafe { ::ll::alGetBuffer3f(sid, param as i32, cast::transmute(&value1),
                                           cast::transmute(&value2),
                                           cast::transmute(&value3)); }
    (value1, value2, value3)
}

// #[fixed_stack_segment]
// pub fn GetBufferfv(bid: ALuint, param: ALenum, values: *ALfloat) {
//     unsafe { ::ll::alGetBufferfv(); }
// }

#[fixed_stack_segment]
pub fn get_bufferi(sid: ALuint,  param: ALenum) -> ALint {
    let mut value = 0;
    unsafe { ::ll::alGetBufferi(sid, param as i32, cast::transmute(&value)); }
    value
}

#[fixed_stack_segment]
pub fn get_buffer3i(sid: ALuint, param: ALenum) -> (ALint, ALint, ALint) {
    let mut value1 = 0;
    let mut value2 = 0;
    let mut value3 = 0;
    unsafe { ::ll::alGetBuffer3i(sid, param as i32, cast::transmute(&value1),
                                           cast::transmute(&value2),
                                           cast::transmute(&value3)); }
    (value1, value2, value3)
}

// #[fixed_stack_segment]
// pub fn GetBufferiv(bid: ALuint, param: ALenum, values: *ALint) {
//     unsafe { ::ll::alGetBufferiv(); }
// }

#[fixed_stack_segment]
pub fn doppler_factor(value: ALfloat) {
    unsafe { ::ll::alDopplerFactor(value); }
}

#[fixed_stack_segment]
pub fn doppler_velocity(value: ALfloat) {
    unsafe { ::ll::alDopplerVelocity(value); }
}

#[fixed_stack_segment]
pub fn speed_of_sound(value: ALfloat) {
    unsafe { ::ll::alSpeedOfSound(value); }
}