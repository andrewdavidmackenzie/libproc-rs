extern crate libc;
use self::libc::{uint32_t, c_void, c_int};

extern crate errno;
use self::errno::errno;

use std::str;

// this extern block links to the libproc library
#[link(name="proc", kind="dylib")]
extern {
    // Original signature from http://opensource.apple.com/source/Libc/Libc-594.9.4/darwin/libproc.c
    // int proc_pidpath(int pid, void * buffer, uint32_t  buffersize)
    fn proc_pidpath(pid: c_int, buffer: *mut c_void, buffersize : uint32_t) -> c_int;
}

// Since we cannot access C macros for constants from Rust - I have had to redefine this, based on Apple's source code
// See http://opensource.apple.com/source/Libc/Libc-594.9.4/darwin/libproc.c
// buffersize must be more than PROC_PIDPATHINFO_SIZE
// buffersize must be less than PROC_PIDPATHINFO_MAXSIZE
//
// See http://opensource.apple.com//source/xnu/xnu-1456.1.26/bsd/sys/proc_info.h
// #define PROC_PIDPATHINFO_SIZE		(MAXPATHLEN)
// #define PROC_PIDPATHINFO_MAXSIZE	(4*MAXPATHLEN)
// in http://opensource.apple.com//source/xnu/xnu-1504.7.4/bsd/sys/param.h
// #define	MAXPATHLEN	PATH_MAX
// in https://opensource.apple.com/source/xnu/xnu-792.25.20/bsd/sys/syslimits.h
// #define	PATH_MAX		 1024
const MAXPATHLEN : usize = 1024;
const PROC_PIDPATHINFO_MAXSIZE : usize = 4 * MAXPATHLEN;

pub fn pidpath(pid : i32) -> Result<String, String> {
    let pathbuf : Vec<u8>= Vec::with_capacity(PROC_PIDPATHINFO_MAXSIZE - 1);

    let buffer_ptr = pathbuf.as_ptr() as *mut c_void;
    let buffer_size = pathbuf.capacity() as u32;

    let ret;
    let rebuilt : Vec<u8>;

    unsafe {
        ret = proc_pidpath(pid, buffer_ptr, buffer_size);
        rebuilt = Vec::from_raw_parts(buffer_ptr as *mut u8, ret as usize, buffer_size as usize);
    };

    if ret <= 0 {
        let e = errno();
        let code = e.0 as i32;
        Err(format!("proc_pidpath() returned {}, errno = {}, '{}'", ret, code, e))
    } else {
        match String::from_utf8(rebuilt) {
            Ok(path) => Ok(path),
            Err(e) => Err(format!("Invalid UTF-8 sequence: {}", e))
        }
    }
}