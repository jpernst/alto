#[link(name = "openal", 
       vers = "0.1",
       uuid = "9450DF9F-7A40-4087-BF6C-0848693D15DC",
       author = "Brendan Zabarauskas",
       url = "https://github.com/bjz/openal-rs")];

#[comment = "OpenAL 1.1 bindings for Rust."];
#[crate_type = "lib"];

pub mod ll;
pub mod ml;
pub mod hl;

/// Core OpenAL typedefs
pub mod types {
    use core::libc::*;
    
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

    pub struct ALCdevice;
    pub struct ALCcontext;

    pub type ALCboolean             = c_char;
    pub type ALCchar                = c_char;
    pub type ALCbyte                = c_char;
    pub type ALCubyte               = c_uchar;
    pub type ALCshort               = c_short;
    pub type ALCushort              = c_ushort;
    pub type ALCint                 = c_int;
    pub type ALCuint                = c_uint;
    pub type ALCsizei               = c_int;
    pub type ALCenum                = c_int;
    pub type ALCfloat               = c_float;
    pub type ALCdouble              = c_double;
    pub type ALCvoid                = c_void;
}

pub mod consts {
    pub mod al {
        use types::*;
        
        pub static NONE                           : ALenum = 0;
        pub static FALSE                          : ALCboolean = 0;
        pub static TRUE                           : ALCboolean = 1;
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
    }

    pub mod alc {
        use types::*;
        
        // TODO: not sure what types these are meant to be...
        pub static INVALID                              : libc::c_int = 0;
        pub static VERSION_0_1                          : libc::c_int = 1;

        pub static FALSE                                : ALCboolean = 0;
        pub static TRUE                                 : ALCboolean = 1;
        
        pub static FREQUENCY                            : ALCint = 0x1007;
        pub static REFRESH                              : ALCint = 0x1008;
        pub static SYNC                                 : ALCint = 0x1009;
        pub static MONO_SOURCES                         : ALCint = 0x1010;
        pub static STEREO_SOURCES                       : ALCint = 0x1011;

        pub static NO_ERROR                             : ALCenum = FALSE as ALCenum;
        pub static INVALID_DEVICE                       : ALCenum = 0xA001;
        pub static INVALID_CONTEXT                      : ALCenum = 0xA002;
        pub static INVALID_ENUM                         : ALCenum = 0xA003;
        pub static INVALID_VALUE                        : ALCenum = 0xA004;
        pub static OUT_OF_MEMORY                        : ALCenum = 0xA005;

        pub static DEFAULT_DEVICE_SPECIFIER             : ALCenum = 0x1004;
        pub static DEVICE_SPECIFIER                     : ALCenum = 0x1005;
        pub static EXTENSIONS                           : ALCenum = 0x1006;
        pub static MAJOR_VERSION                        : ALCenum = 0x1000;
        pub static MINOR_VERSION                        : ALCenum = 0x1001;
        pub static ATTRIBUTES_SIZE                      : ALCenum = 0x1002;
        pub static ALL_ATTRIBUTES                       : ALCenum = 0x1003;
        pub static DEFAULT_ALL_DEVICES_SPECIFIER        : ALCenum = 0x1012;
        pub static ALL_DEVICES_SPECIFIER                : ALCenum = 0x1013;
        pub static CAPTURE_DEVICE_SPECIFIER             : ALCenum = 0x310;
        pub static CAPTURE_DEFAULT_DEVICE_SPECIFIER     : ALCenum = 0x311;
        pub static CAPTURE_SAMPLES                      : ALCenum = 0x312;
    }
}