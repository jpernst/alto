/**
 * Mid-level wrapper functions
 */

pub mod al {
    use types::*;
    pub use consts::al::*;

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

    pub fn get_booleanv(param: ALenum, data: &mut bool) {
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

    pub fn get_boolean(param: ALenum) -> bool {
        unsafe { ::ll::alGetBoolean(param) as bool }
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

    pub fn is_extension_present(extname: &str) -> bool {
        unsafe { ::ll::alIsExtensionPresent(str::as_c_str(extname, |s| s)) as bool }
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

    pub fn is_source(sid: ALuint) -> bool {
        unsafe { ::ll::alIsSource(sid) as bool }
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

    pub fn is_buffer(bid: ALuint) -> bool {
        unsafe { ::ll::alIsBuffer(bid) as bool }
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
}

pub mod alc {
    use types::*;
    pub use consts::alc::*;

    pub fn create_context(device: *ALCdevice, attrlist: &[ALCint]) -> *ALCcontext {
        let attrs_terminated = vec::append_one(attrlist.to_owned(), 0);  // teminate attributes with a 0
        unsafe { ::ll::alcCreateContext(device, cast::transmute(&attrs_terminated[0])) }
    }

    pub fn make_context_current(context: *ALCcontext) -> bool {
        unsafe { ::ll::alcMakeContextCurrent(context) as bool }
    }

    pub fn process_context(context: *ALCcontext) {
        unsafe { ::ll::alcProcessContext(context); }
    }

    pub fn suspend_context(context: *ALCcontext) {
        unsafe { ::ll::alcSuspendContext(context); }
    }

    pub fn destroy_context(context: *ALCcontext) {
        unsafe { ::ll::alcDestroyContext(context); }
    }

    pub fn get_current_context() -> *ALCcontext {
        unsafe { ::ll::alcGetCurrentContext() }
    }

    pub fn get_contexts_device(context: *ALCcontext) -> *ALCdevice {
        unsafe { ::ll::alcGetContextsDevice(context) }
    }

    pub fn open_device(devicename: &str) -> *ALCdevice {
        unsafe { ::ll::alcOpenDevice(str::as_c_str(devicename, |s| s)) }
    }

    pub fn close_device(device: *ALCdevice) -> bool {
        unsafe { ::ll::alcCloseDevice(device) as bool }
    }

    pub fn get_error(device: *ALCdevice) -> ALCenum {
        unsafe { ::ll::alcGetError(device) }
    }

    pub fn is_extension_present(device: *ALCdevice, extname: &str) -> bool {
        unsafe { ::ll::alcIsExtensionPresent(device, str::as_c_str(extname, |s| s)) as bool }
    }

    pub fn get_proc_address(device: *ALCdevice, funcname: ~str) -> extern fn() {
        unsafe { cast::transmute(
            ::ll::alcGetProcAddress(device, str::as_c_str(funcname, |s| s))
        ) }
    }

    pub fn get_enum_value(device: *ALCdevice, enumname: &str) -> ALCenum {
        unsafe { ::ll::alcGetEnumValue(device, str::as_c_str(enumname, |s| s)) }
    }

    // pub fn get_string(device: *ALCdevice, param: ALCenum) -> *ALCchar {
    //     unsafe { ::ll::alcGetString(device, param) }
    // }

    // pub fn GetIntegerv(device: *ALCdevice, param: ALCenum, size: ALCsizei, data: *ALCint) {
    //     unsafe { ::ll::alcGetIntegerv(); }
    // }

    pub fn capture_open_device(devicename: *ALCchar, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> *ALCdevice {
        unsafe { ::ll::alcCaptureOpenDevice(devicename, frequency, format, buffersize) }
    }

    pub fn capture_close_device(device: *ALCdevice) -> bool {
        unsafe { ::ll::alcCaptureCloseDevice(device) as bool }
    }

    pub fn capture_start(device: *ALCdevice) {
        unsafe { ::ll::alcCaptureStart(device); }
    }

    pub fn capture_stop(device: *ALCdevice) {
        unsafe { ::ll::alcCaptureStop(device); }
    }

    // pub fn CaptureSamples(device: *ALCdevice, buffer: *ALCvoid, samples: ALCsizei) {
    //     unsafe { ::ll::alcCaptureSamples(); }
    // }
}