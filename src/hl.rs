/**
 * High-level wrapper for the OpenAL API
 */

pub mod al {
    
}

pub mod alc {
    use ll::*;
    use types::*;
    use hl::util;
    use consts::alc;
    
    pub struct Device(*ALCdevice);

    pub impl Device {
        fn open_default() -> Result<Device,()> {
            util::err_if_null(
                unsafe { alcOpenDevice(ptr::null()) },
                (), |d| Device(d)
            )
        }
        
        fn open(name: &str) -> Result<Device,()> {
            util::err_if_null(
                unsafe { alcOpenDevice(str::as_c_str(name, |a| a)) },
                (), |d| Device(d)
            )
        }
        
        fn close(&self) -> Result<(),()> {
            match unsafe {
                alcCloseDevice(**self)
            } {
                alc::TRUE => Ok(()),
                _ => Err(())
            }
        }
        
        fn get_name(&self) -> ~str {
            unsafe { str::raw::from_c_str(
                alcGetString(**self, alc::DEVICE_SPECIFIER)
            )}
        }
        
        fn default_name() -> ~str {
            unsafe { str::raw::from_c_str(
                alcGetString(ptr::null(), alc::DEFAULT_DEVICE_SPECIFIER)
            )}
        }
        
        fn get_available() -> ~[~str] {
            unsafe { util::from_c_strs(
                alcGetString(ptr::null(), alc::DEVICE_SPECIFIER)
            )}
        }
    }

    pub struct Context(*ALCcontext);

    pub impl Context {
        fn create(device: &Device) -> Result<Context,()> {
            util::err_if_null(
                unsafe { alcCreateContext(**device, ptr::null()) },
                (), |c| Context(c)
            )
        }
        
        fn make_current(&self) -> Result<(),()> {
            match unsafe {
                alcMakeContextCurrent(**self)
            } {
                alc::TRUE => Ok(()),
                _ => Err(())
            }
        }
        
        fn suspend(&self) {
            unsafe { alcSuspendContext(**self); }
        }
        
        fn destroy(&self) {
            unsafe { alcDestroyContext(**self) };
        }
        
        fn get_current() -> Context {
            Context(unsafe { alcGetCurrentContext() })
        }
        
        fn get_device(&self) -> Device {
            Device(unsafe { alcGetContextsDevice(**self) })
        }
    }

    pub struct CaptureDevice(*ALCdevice);

    pub impl CaptureDevice {
        fn open_default(
            frequency: ALCuint,
            format: ALCenum,
            buffersize: ALCsizei
        ) -> Result<CaptureDevice,()> {
            util::err_if_null(
                unsafe { alcCaptureOpenDevice(
                    ptr::null(),
                    frequency,
                    format,
                    buffersize
                ) }, (), |d| CaptureDevice(d)
            )
        }
        
        fn open(
            name: &str,
            frequency: ALCuint,
            format: ALCenum,
            buffersize: ALCsizei
        ) -> Result<CaptureDevice,()> {
            util::err_if_null(
                unsafe { alcCaptureOpenDevice(
                    str::as_c_str(name, |a| a),
                    frequency,
                    format,
                    buffersize
                ) }, (), |d| CaptureDevice(d)
            )
        }
        
        fn close(&self) -> Result<(),()> {
            match unsafe {
                alcCaptureCloseDevice(**self)
            } {
                alc::TRUE => Ok(()),
                _ => Err(())
            }
        }
        
        fn start(&self) {
            unsafe { alcCaptureStart(**self) };
        }
        
        fn stop(&self) {
            unsafe { alcCaptureStop(**self) };
        }
        
        // fn get_samples(&self, samples: ALCsizei) -> *ALCvoid {}
        
        fn get_name(&self) -> ~str {
            unsafe { str::raw::from_c_str(
                alcGetString(**self, alc::CAPTURE_DEVICE_SPECIFIER)
            )}
        }
        
        fn default_name() -> ~str {
            unsafe { str::raw::from_c_str(
                alcGetString(ptr::null(), alc::CAPTURE_DEFAULT_DEVICE_SPECIFIER)
            )}
        }
        
        fn get_available() -> ~[~str] {
            unsafe { util::from_c_strs(
                alcGetString(ptr::null(), alc::CAPTURE_DEVICE_SPECIFIER)
            )}
        }
    }
}

pub mod util {
    #[inline(always)]
    pub fn err_if_null<T,U,V>(ptr: *T, err: U, f: &fn(*T) -> V) -> Result<V,U> {
        if !ptr.is_null() {
            Ok(f(ptr))
        } else {
            Err(err)
        }
    }
    
    pub unsafe fn from_c_strs(c_strs: *libc::c_char) -> ~[~str] {
        let mut i   = 0u,       // current index
                p   = c_strs,   // lookahead pointer
                pf  = c_strs,   // pointer to the front of the current string
                vec = ~[];      // the vector of strings that will be returned
        
        loop {
            // find the length of the next string
            let mut len = 0u;
            while *p != 0 {
                i += 1;
                len += 1;
                p = c_strs.offset(i);
            }
            
            if len > 0 {
                vec.push(unsafe {
                    str::raw::from_c_str_len(pf, len)
                });
            }
            
            i += 1;                 // step over the null character  
            p = c_strs.offset(i);
            match *p {
                0 => { break }      // break if at the end of the array
                _ => { pf = p }     // shift `pf` to the front of the next string
            }
        }
        return vec;
    }
    
    #[test]
    fn test_from_c_strs() {
        use core::libc::c_char;
        
        let c_strs = ~[
            't' as c_char,
            'h' as c_char,
            'i' as c_char,
            's' as c_char,
            ' ' as c_char,
            'i' as c_char,
            's' as c_char,
             0  as c_char,
            'a' as c_char,
             0  as c_char,
            't' as c_char,
            'e' as c_char,
            's' as c_char,
            't' as c_char,
             0  as c_char,
        ];
        
        assert!(unsafe {
            from_c_strs(&c_strs[0]) == ~[~"this is", ~"a", ~"test"]
        })
    }
}