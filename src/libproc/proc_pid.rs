extern crate libc;
use self::libc::{uint64_t, uint32_t, c_void, c_int};

extern crate errno;
use self::errno::errno;

use std::str;

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

// TODO set the correct values for each
pub enum ProcType { ProcAllPIDS, ProcPGRPOnly, ProcTtyOnly, ProcUIDOnly, ProcUidOnly }

// this extern block links to the libproc library
// Original signatures of functions can be found at http://opensource.apple.com/source/Libc/Libc-594.9.4/darwin/libproc.c
#[link(name="proc", kind="dylib")]
extern {
    // int proc_listpids(uint32_t type, uint32_t typeinfo, void *buffer, int buffersize)

    // int proc_pidinfo(int pid, int flavor, uint64_t arg,  void *buffer, int buffersize)

    // int proc_pidfdinfo(int pid, int fd, int flavor, void * buffer, int buffersize)

    fn proc_name(pid : c_int, buffer : *mut c_void, buffersize : uint32_t) -> c_int;

    fn proc_regionfilename(pid : c_int, address : uint64_t, buffer : *mut c_void, buffersize : uint32_t) -> c_int;

    // int proc_kmsgbuf(void * buffer, uint32_t  buffersize)

    // int proc_pidpath(int pid, void * buffer, uint32_t  buffersize)
    fn proc_pidpath(pid : c_int, buffer : *mut c_void, buffersize : uint32_t) -> c_int;

    // int proc_libversion(int *major, int * minor)
    // return value of 0 indicates success (inconsistent :-( )
    fn proc_libversion(major : *mut c_int, minor : * mut c_int) -> c_int;
}

fn get_errno_with_message(ret : i32) -> String {
    let e = errno();
    let code = e.0 as i32;
    format!("return code = {}, errno = {}, message = '{}'", ret, code, e)
}

pub fn regionfilename(pid : i32, address : u64) -> Result<String, String> {
    let regionfilenamebuf: Vec<u8>= Vec::with_capacity(PROC_PIDPATHINFO_MAXSIZE - 1);
    let buffer_ptr = regionfilenamebuf.as_ptr() as *mut c_void;
    let buffer_size = regionfilenamebuf.capacity() as u32;
    let ret : i32;
    let rebuilt : Vec<u8>;

    unsafe {
        ret = proc_regionfilename(pid, address, buffer_ptr, buffer_size);
        rebuilt = Vec::from_raw_parts(buffer_ptr as *mut u8, ret as usize, buffer_size as usize);
    };

    if ret <= 0 {
        Err(get_errno_with_message(ret))
    } else {
        match String::from_utf8(rebuilt) {
            Ok(regionfilename) => Ok(regionfilename),
            Err(e) => Err(format!("Invalid UTF-8 sequence: {}", e))
        }
    }
}

#[test]
// This checks that it can find the regionfilename of the region at address 0, of the init process with PID 1
fn regionfilename_test() {
    match regionfilename(1, 0) {
        // run tests with 'cargo test -- --nocapture' to see the test output
        Ok(regionfilename) => println!("Region Filename (at address = 0) of init process PID = 1 is '{}'", regionfilename),
        Err(message) => assert!(true, message)
    }
}

pub fn pidpath(pid : i32) -> Result<String, String> {
    let pathbuf : Vec<u8>= Vec::with_capacity(PROC_PIDPATHINFO_MAXSIZE - 1);
    let buffer_ptr = pathbuf.as_ptr() as *mut c_void;
    let buffer_size = pathbuf.capacity() as u32;
    let ret : i32;
    let rebuilt : Vec<u8>;

    unsafe {
        ret = proc_pidpath(pid, buffer_ptr, buffer_size);
        rebuilt = Vec::from_raw_parts(buffer_ptr as *mut u8, ret as usize, buffer_size as usize);
    };

    if ret <= 0 {
        Err(get_errno_with_message(ret))
    } else {
        match String::from_utf8(rebuilt) {
            Ok(path) => Ok(path),
            Err(e) => Err(format!("Invalid UTF-8 sequence: {}", e))
        }
    }
}

#[test]
// This checks that it can find the path of the init process with PID 1
fn pidpath_test_init_pid() {
    match pidpath(1) {
        // run tests with 'cargo test -- --nocapture' to see the test output
        Ok(path) => println!("Path of init process PID = 1 is '{}'", path),
        Err(message) => assert!(true, message)
    }
}

#[test]
#[should_panic]
// This checks that it cannot find the path of the process with pid -1
fn pidpath_test_unknown_pid() {
    match pidpath(-1) {
        // run tests with 'cargo test -- --nocapture' to see the test output
        Ok(path) => assert!(false, "It found the path of process Pwith ID = -1 (path = {}), that's not possible\n", path),
        Err(message) => assert!(false, message)
    }
}

pub fn libversion() -> Result<(i32, i32), String> {
    let mut major = 0;
    let mut minor = 0;
    let ret : i32;

    unsafe {
        ret = proc_libversion(&mut major, &mut minor);
    };

    if ret == 0 {
        Ok((major, minor))
    } else {
        Err(get_errno_with_message(ret))
    }
}

#[test]
fn libversion_test() {
    match libversion() {
        Ok((major, minor)) => {
            // run tests with 'cargo test -- --nocapture' to see the test output
            println!("Major = {}, Minor = {}", major, minor);
        },
        Err(message) => assert!(false, message)
    }
}

pub fn name(pid : i32) -> Result<String, String> {
    let namebuf: Vec<u8>= Vec::with_capacity(PROC_PIDPATHINFO_MAXSIZE - 1);
    let buffer_ptr = namebuf.as_ptr() as *mut c_void;
    let buffer_size = namebuf.capacity() as u32;
    let ret : i32;
    let rebuilt : Vec<u8>;

    unsafe {
        ret = proc_name(pid, buffer_ptr, buffer_size);
        rebuilt = Vec::from_raw_parts(buffer_ptr as *mut u8, ret as usize, buffer_size as usize);
    };

    if ret <= 0 {
        Err(get_errno_with_message(ret))
    } else {
        match String::from_utf8(rebuilt) {
            Ok(name) => Ok(name),
            Err(e) => Err(format!("Invalid UTF-8 sequence: {}", e))
        }
    }
}

#[test]
// This checks that it can find the name of the init process with PID 1
fn name_test_init_pid() {
    match pidpath(1) {
        // run tests with 'cargo test -- --nocapture' to see the test output
        Ok(path) => println!("Name of init process PID = 1 is '{}'", path),
        Err(message) => assert!(true, message)
    }
}