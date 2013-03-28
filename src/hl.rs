/**
 * High-level wrapper for the OpenAL API
 */

pub mod al {
    
}

pub mod alc {
    use types::*;
    use hl::util;
    use ml::alc;
    
    pub struct Device(*ALCdevice);

    pub impl Device {
        fn open_default() -> Result<Device,()> {
            util::err_if_null(alc::open_device(""), (), |d| Device(d))
        }
        
        fn open(name: &str) -> Result<Device,()> {
            util::err_if_null(alc::open_device(name), (), |d| Device(d))
        }
        
        fn close(&self) -> Result<(),()> {
            match alc::close_device(**self) {
                true  => Ok(()),
                false => Err(())
            }
        }
        
        fn get_name(&self) -> ~str {
            unsafe { str::raw::from_c_str(
                ::ll::alcGetString(**self, alc::DEVICE_SPECIFIER)
            )}
        }
        
        fn default_name() -> ~str {
            unsafe { str::raw::from_c_str(
                ::ll::alcGetString(ptr::null(), alc::DEFAULT_DEVICE_SPECIFIER)
            )}
        }
        
        fn get_available() -> ~[~str] {
            unsafe { util::from_c_strs(
                ::ll::alcGetString(ptr::null(), alc::DEVICE_SPECIFIER)
            )}
        }
    }

    pub struct Context(*ALCcontext);

    pub impl Context {
        fn create(device: &Device, attrlist: &[ALCint]) -> Result<Context,()> {
            util::err_if_null(alc::create_context(**device, attrlist), (), |c| Context(c))
        }
        
        fn make_current(&self) -> Result<(),()> {
            match alc::make_context_current(**self) {
                true  => Ok(()),
                false => Err(())
            }
        }
        
        fn suspend(&self) {
            alc::suspend_context(**self);
        }
        
        fn destroy(&self) {
            alc::destroy_context(**self);
        }
        
        fn get_current() -> Context {
            Context(alc::get_current_context())
        }
        
        fn get_device(&self) -> Device {
            Device(alc::get_contexts_device(**self))
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
                alc::capture_open_device(
                    ptr::null(),
                    frequency,
                    format,
                    buffersize
                ), (), |d| CaptureDevice(d)
            )
        }
        
        fn open(
            name: &str,
            frequency: ALCuint,
            format: ALCenum,
            buffersize: ALCsizei
        ) -> Result<CaptureDevice,()> {
            util::err_if_null(
                alc::capture_open_device(
                    str::as_c_str(name, |a| a),
                    frequency,
                    format,
                    buffersize
                ), (), |d| CaptureDevice(d)
            )
        }
        
        fn close(&self) -> Result<(),()> {
            match alc::capture_close_device(**self) {
                true  => Ok(()),
                false => Err(())
            }
        }
        
        fn start(&self) {
            alc::capture_start(**self);
        }
        
        fn stop(&self) {
            alc::capture_stop(**self);
        }
        
        // fn get_samples(&self, samples: ALCsizei) -> *ALCvoid {}
        
        fn get_name(&self) -> ~str {
            unsafe { str::raw::from_c_str(
                ::ll::alcGetString(**self, alc::CAPTURE_DEVICE_SPECIFIER)
            )}
        }
        
        fn default_name() -> ~str {
            unsafe { str::raw::from_c_str(
                ::ll::alcGetString(ptr::null(), alc::CAPTURE_DEFAULT_DEVICE_SPECIFIER)
            )}
        }
        
        fn get_available() -> ~[~str] {
            unsafe { util::from_c_strs(
                ::ll::alcGetString(ptr::null(), alc::CAPTURE_DEVICE_SPECIFIER)
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