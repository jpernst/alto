/**
 * Low-level function bindings
 */

#[nolink]
#[link_args="-framework OpenAL"]
#[cfg(target_os = "macos")]
pub extern mod linkhack {}

#[nolink]
#[link_args="-lopenal"]
#[cfg(target_os = "linux")]
pub extern mod linkhack {}

use types::*;

pub extern "C" {
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
    pub fn alGetProcAddress(fname: *ALchar) -> *libc::c_void;
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
    
    pub fn alcCreateContext(device: *ALCdevice, attrlist: *ALCint) -> *ALCcontext;
    pub fn alcMakeContextCurrent(context: *ALCcontext) -> ALCboolean;
    pub fn alcProcessContext(context: *ALCcontext);
    pub fn alcSuspendContext(context: *ALCcontext);
    pub fn alcDestroyContext(context: *ALCcontext);
    pub fn alcGetCurrentContext() -> *ALCcontext;
    pub fn alcGetContextsDevice(context: *ALCcontext) -> *ALCdevice;
    pub fn alcOpenDevice(devicename: *ALCchar) -> *ALCdevice;
    pub fn alcCloseDevice(device: *ALCdevice) -> ALCboolean;
    pub fn alcGetError(device: *ALCdevice) -> ALCenum;
    pub fn alcIsExtensionPresent(device: *ALCdevice, extname: *ALCchar) -> ALCboolean;
    pub fn alcGetProcAddress(device: *ALCdevice, funcname: *ALCchar) -> *libc::c_void;
    pub fn alcGetEnumValue(device: *ALCdevice, enumname: *ALCchar) -> ALCenum;
    pub fn alcGetString(device: *ALCdevice, param: ALCenum) -> *ALCchar;
    pub fn alcGetIntegerv(device: *ALCdevice, param: ALCenum, size: ALCsizei, data: *ALCint);
    pub fn alcCaptureOpenDevice(devicename: *ALCchar, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> *ALCdevice;
    pub fn alcCaptureCloseDevice(device: *ALCdevice) -> ALCboolean;
    pub fn alcCaptureStart(device: *ALCdevice);
    pub fn alcCaptureStop(device: *ALCdevice);
    pub fn alcCaptureSamples(device: *ALCdevice, buffer: *ALCvoid, samples: ALCsizei);
}