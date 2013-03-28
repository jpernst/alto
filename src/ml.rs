/**
 * Mid-level wrapper functions
 */

pub mod al {
    use ll::*;
    use types::*;
    pub use consts::al::*;

    pub fn enable(capability: ALenum) {
        unsafe { alEnable(capability); }
    }

    pub fn disable(capability: ALenum) {
        unsafe { alDisable(capability); }
    }

    pub fn is_enabled(capability: ALenum) -> ALboolean {
        unsafe { alIsEnabled(capability) }
    }

    pub fn get_string(param: ALenum) -> ~str {
        unsafe { str::raw::from_c_str(alGetString(param)) }
    }

    pub fn get_booleanv(param: ALenum, data: &mut bool) {
        unsafe { alGetBooleanv(param, cast::transmute(data)); }
    }

    pub fn get_integerv(param: ALenum, data: &mut ALint) {
        unsafe { alGetIntegerv(param, cast::transmute(data)); }
    }

    pub fn get_floatv(param: ALenum, data: &mut ALfloat) {
        unsafe { alGetFloatv(param, cast::transmute(data)); }
    }

    pub fn get_doublev(param: ALenum, data: &mut ALdouble) {
        unsafe { alGetDoublev(param, cast::transmute(data)); }
    }

    pub fn get_boolean(param: ALenum) -> bool {
        unsafe { alGetBoolean(param) as bool }
    }

    pub fn get_integer(param: ALenum) -> ALint {
        unsafe { alGetInteger(param) }
    }

    pub fn get_float(param: ALenum) -> ALfloat {
        unsafe { alGetFloat(param) }
    }

    pub fn get_double(param: ALenum) -> ALdouble {
        unsafe { alGetDouble(param) }
    }

    pub fn get_error() -> ALenum {
        unsafe { alGetError() }
    }

    pub fn is_extension_present(extname: &str) -> bool {
        unsafe { alIsExtensionPresent(str::as_c_str(extname, |s| s)) as bool }
    }

    pub fn get_proc_address(fname: &str) -> extern fn() {
        unsafe { cast::transmute(
            alGetProcAddress(str::as_c_str(fname, |s| s))
        ) }
    }

    pub fn get_enum_value(ename: &str) -> ALenum {
        unsafe { alGetEnumValue(str::as_c_str(ename, |s| s)) }
    }


    pub fn listenerf(param: ALenum, value: ALfloat) {
        unsafe { alListenerf(param, value); }
    }

    pub fn listener3f(param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat) {
        unsafe { alListener3f(param, value1, value2, value3); }
    }

    // pub fn Listenerfv(param: ALenum, values: *ALfloat) {
    //     unsafe { alListenerfv(param, values); }
    // }

    pub fn listeneri(param: ALenum, value: ALint) {
        unsafe { alListeneri(param, value); }
    }

    pub fn listener3i(param: ALenum, value1: ALint, value2: ALint, value3: ALint) {
        unsafe { alListener3i(param, value1, value2, value3); }
    }

    // pub fn Listeneriv(param: ALenum, values: *ALint) {
    //     unsafe { alListeneriv(param, values); }
    // }

    pub fn get_listenerf(param: ALenum) -> ALfloat {
        let mut value = 0.0;
        unsafe { alGetListenerf(param, cast::transmute(&value)) }
        value
    }

    pub fn get_listener3f(param: ALenum) -> (ALfloat, ALfloat, ALfloat) {
        let mut value1 = 0.0,
                value2 = 0.0,
                value3 = 0.0;
        unsafe { alGetListener3f(param, cast::transmute(&value1),
                                        cast::transmute(&value2),
                                        cast::transmute(&value3)) }
        (value1, value2, value3)
    }

    // pub fn GetListenerfv(param: ALenum, values: *ALfloat) {
    //     unsafe { alGetListenerfv(param, values) }
    // }

    pub fn get_listeneri(param: ALenum) -> ALint {
        let mut value = 0;
        unsafe { alGetListeneri(param, cast::transmute(&value)) }
        value
    }

    pub fn get_listener3i(param: ALenum) -> (ALint, ALint, ALint) {
        let mut value1 = 0,
                value2 = 0,
                value3 = 0;
        unsafe { alGetListener3i(param, cast::transmute(&value1),
                                        cast::transmute(&value2),
                                        cast::transmute(&value3)) }
        (value1, value2, value3)
    }

    // pub fn GetListeneriv(param: ALenum, values: *ALint) {
    //     unsafe { alGetListeneriv(param, values) }
    // }

    pub fn gen_sources(sources: &[ALuint]) {
        unsafe { alGenSources(sources.len() as ALsizei, cast::transmute(&sources[0])); }
    }

    pub fn delete_sources(sources: &[ALuint]) {
        unsafe { alDeleteSources(sources.len() as ALsizei, cast::transmute(&sources[0])); }
    }

    pub fn is_source(sid: ALuint) -> bool {
        unsafe { alIsSource(sid) as bool }
    }

    pub fn sourcef(sid: ALuint, param: ALenum, value: ALfloat) {
        unsafe { alSourcef(sid, param, value); }
    }

    pub fn source3f(sid: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat) {
        unsafe { alSource3f(sid, param, value1, value2, value3); }
    }

    // pub fn Sourcefv(sid: ALuint, param: ALenum, values: *ALfloat) {
    //     unsafe { alSourcefv(); }
    // }

    pub fn sourcei(sid: ALuint, param: ALenum, value: ALint) {
        unsafe { alSourcei(sid, param, value); }
    }

    pub fn source3i(sid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint) {
        unsafe { alSource3i(sid, param, value1, value2, value3); }
    }

    // pub fn Sourceiv(sid: ALuint, param: ALenum, values: *ALint) {
    //     unsafe { alSourceiv(); }
    // }

    pub fn get_sourcef(sid: ALuint, param: ALenum) -> ALfloat {
        let mut value = 0.0;
        unsafe { alGetSourcef(sid, param, cast::transmute(&value)); }
        value
    }

    pub fn get_source3f(sid: ALuint, param: ALenum) -> (ALfloat, ALfloat, ALfloat) {
        let mut value1 = 0.0,
                value2 = 0.0,
                value3 = 0.0;
        unsafe { alGetSource3f(sid, param, cast::transmute(&value1),
                                           cast::transmute(&value2),
                                           cast::transmute(&value3)); }
        (value1, value2, value3)
    }

    // pub fn GetSourcefv(sid: ALuint, param: ALenum, values: *ALfloat) {
    //     unsafe { alGetSourcefv(); }
    // }

    pub fn get_sourcei(sid: ALuint,  param: ALenum) -> ALint {
        let mut value = 0;
        unsafe { alGetSourcei(sid, param, cast::transmute(&value)); }
        value
    }

    pub fn get_source3i(sid: ALuint, param: ALenum) -> (ALint, ALint, ALint) {
        let mut value1 = 0,
                value2 = 0,
                value3 = 0;
        unsafe { alGetSource3i(sid, param, cast::transmute(&value1),
                                           cast::transmute(&value2),
                                           cast::transmute(&value3)); }
        (value1, value2, value3)
    }

    // pub fn GetSourceiv(sid: ALuint,  param: ALenum, values: *ALint) {
    //     unsafe { alGetSourceiv(); }
    // }

    pub fn source_playv(sids: &[ALuint]) {
        unsafe { alSourcePlayv(sids.len() as ALsizei, cast::transmute(&sids[0])); }
    }

    pub fn source_stopv(sids: &[ALuint]) {
        unsafe { alSourceStopv(sids.len() as ALsizei, cast::transmute(&sids[0])); }
    }

    pub fn source_rewindv(sids: &[ALuint]) {
        unsafe { alSourceRewindv(sids.len() as ALsizei, cast::transmute(&sids[0])); }
    }

    pub fn source_pausev(sids: &[ALuint]) {
        unsafe { alSourcePausev(sids.len() as ALsizei, cast::transmute(&sids[0])); }
    }

    pub fn source_play(sid: ALuint) {
        unsafe { alSourcePlay(sid); }
    }

    pub fn source_stop(sid: ALuint) {
        unsafe { alSourceStop(sid); }
    }

    pub fn source_rewind(sid: ALuint) {
        unsafe { alSourceRewind(sid); }
    }

    pub fn source_pause(sid: ALuint) {
        unsafe { alSourcePause(sid); }
    }

    pub fn source_queue_buffers(sid: ALuint, bids: &[ALuint]) {
        unsafe { alSourceQueueBuffers(sid, bids.len() as ALsizei, cast::transmute(&bids[0])); }
    }

    pub fn source_unqueue_buffers(sid: ALuint, bids: &[ALuint]) {
        unsafe { alSourceUnqueueBuffers(sid, bids.len() as ALsizei, cast::transmute(&bids[0])); }
    }

    pub fn gen_buffers(buffers: &[ALuint]) {
        unsafe { alGenBuffers(buffers.len() as ALsizei, cast::transmute(&buffers[0])); }
    }

    pub fn delete_buffers(buffers: &[ALuint]) {
        unsafe { alDeleteBuffers(buffers.len() as ALsizei, cast::transmute(&buffers[0])); }
    }

    pub fn is_buffer(bid: ALuint) -> bool {
        unsafe { alIsBuffer(bid) as bool }
    }

    pub fn buffer_data<T>(bid: ALuint, format: ALenum, data: &[T], freq: ALsizei) {
        unsafe {
            alBufferData(
                bid, format,
                cast::transmute(&data[0]),
                sys::size_of::<T>() * data.len() as ALsizei,
                freq
            );
        }
    }

    pub fn bufferf(bid: ALuint, param: ALenum, value: ALfloat) {
        unsafe { alBufferf(bid, param, value); }
    }

    pub fn buffer3f(bid: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat) {
        unsafe { alBuffer3f(bid, param, value1, value2, value3); }
    }

    // pub fn Bufferfv(bid: ALuint, param: ALenum, values: *ALfloat) {
    //     unsafe { alBufferfv(); }
    // }

    pub fn bufferi(bid: ALuint, param: ALenum, value: ALint) {
        unsafe { alBufferi(bid, param, value); }
    }

    pub fn buffer3i(bid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint) {
        unsafe { alBuffer3i(bid, param, value1, value2, value3); }
    }

    // pub fn Bufferiv(bid: ALuint, param: ALenum, values: *ALint) {
    //     unsafe { alBufferiv(); }
    // }

    pub fn get_bufferf(sid: ALuint, param: ALenum) -> ALfloat {
        let mut value = 0.0;
        unsafe { alGetBufferf(sid, param, cast::transmute(&value)); }
        value
    }

    pub fn get_buffer3f(sid: ALuint, param: ALenum) -> (ALfloat, ALfloat, ALfloat) {
        let mut value1 = 0.0,
                value2 = 0.0,
                value3 = 0.0;
        unsafe { alGetBuffer3f(sid, param, cast::transmute(&value1),
                                           cast::transmute(&value2),
                                           cast::transmute(&value3)); }
        (value1, value2, value3)
    }

    // pub fn GetBufferfv(bid: ALuint, param: ALenum, values: *ALfloat) {
    //     unsafe { alGetBufferfv(); }
    // }

    pub fn get_bufferi(sid: ALuint,  param: ALenum) -> ALint {
        let mut value = 0;
        unsafe { alGetBufferi(sid, param, cast::transmute(&value)); }
        value
    }

    pub fn get_buffer3i(sid: ALuint, param: ALenum) -> (ALint, ALint, ALint) {
        let mut value1 = 0,
                value2 = 0,
                value3 = 0;
        unsafe { alGetBuffer3i(sid, param, cast::transmute(&value1),
                                           cast::transmute(&value2),
                                           cast::transmute(&value3)); }
        (value1, value2, value3)
    }

    // pub fn GetBufferiv(bid: ALuint, param: ALenum, values: *ALint) {
    //     unsafe { alGetBufferiv(); }
    // }

    pub fn doppler_factor(value: ALfloat) {
        unsafe { alDopplerFactor(value); }
    }

    pub fn doppler_velocity(value: ALfloat) {
        unsafe { alDopplerVelocity(value); }
    }

    pub fn speed_of_sound(value: ALfloat) {
        unsafe { alSpeedOfSound(value); }
    }
}

pub mod alc {
    use ll::*;
    use types::*;
    pub use consts::alc::*;

    // pub fn create_context(device: *ALCdevice, attrlist: *ALCint) -> *ALCcontext {
    //     unsafe { alcCreateContext(device, attrlist) }
    // }

    pub fn make_context_current(context: *ALCcontext) -> bool {
        unsafe { alcMakeContextCurrent(context) as bool }
    }

    pub fn process_context(context: *ALCcontext) {
        unsafe { alcProcessContext(context); }
    }

    pub fn suspend_context(context: *ALCcontext) {
        unsafe { alcSuspendContext(context); }
    }

    pub fn destroy_context(context: *ALCcontext) {
        unsafe { alcDestroyContext(context); }
    }

    pub fn get_current_context() -> *ALCcontext {
        unsafe { alcGetCurrentContext() }
    }

    pub fn get_contexts_device(context: *ALCcontext) -> *ALCdevice {
        unsafe { alcGetContextsDevice(context) }
    }

    pub fn open_device(devicename: &str) -> *ALCdevice {
        unsafe { alcOpenDevice(str::as_c_str(devicename, |s| s)) }
    }

    pub fn close_device(device: *ALCdevice) -> bool {
        unsafe { alcCloseDevice(device) as bool }
    }

    pub fn get_error(device: *ALCdevice) -> ALCenum {
        unsafe { alcGetError(device) }
    }

    pub fn is_extension_present(device: *ALCdevice, extname: &str) -> bool {
        unsafe { alcIsExtensionPresent(device, str::as_c_str(extname, |s| s)) as bool }
    }

    pub fn get_proc_address(device: *ALCdevice, funcname: ~str) -> extern fn() {
        unsafe { cast::transmute(
            alcGetProcAddress(device, str::as_c_str(funcname, |s| s))
        ) }
    }

    pub fn get_enum_value(device: *ALCdevice, enumname: &str) -> ALCenum {
        unsafe { alcGetEnumValue(device, str::as_c_str(enumname, |s| s)) }
    }

    // pub fn get_string(device: *ALCdevice, param: ALCenum) -> *ALCchar {
    //     unsafe { alcGetString(device, param) }
    // }

    // pub fn GetIntegerv(device: *ALCdevice, param: ALCenum, size: ALCsizei, data: *ALCint) {
    //     unsafe { alcGetIntegerv(); }
    // }

    pub fn capture_open_device(devicename: *ALCchar, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> *ALCdevice {
        unsafe { alcCaptureOpenDevice(devicename, frequency, format, buffersize) }
    }

    pub fn capture_close_device(device: *ALCdevice) -> bool {
        unsafe { alcCaptureCloseDevice(device) as bool }
    }

    pub fn capture_start(device: *ALCdevice) {
        unsafe { alcCaptureStart(device); }
    }

    pub fn capture_stop(device: *ALCdevice) {
        unsafe { alcCaptureStop(device); }
    }

    // pub fn CaptureSamples(device: *ALCdevice, buffer: *ALCvoid, samples: ALCsizei) {
    //     unsafe { alcCaptureSamples(); }
    // }
}