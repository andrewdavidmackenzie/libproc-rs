extern crate libc;

#[cfg(target_os = "macos")]
use std::mem;
#[cfg(target_os = "linux")]
use std::ffi::CString;
#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "linux")]
use std::fs::File;
#[cfg(target_os = "linux")]
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
#[cfg(target_os = "macos")]
use std::ptr;
use std::env;

#[cfg(target_os = "linux")]
use libc::PATH_MAX;
use libc::pid_t;

use crate::libproc::bsd_info::BSDInfo;
use crate::libproc::helpers;
use crate::libproc::task_info::{TaskAllInfo, TaskInfo};
use crate::libproc::thread_info::ThreadInfo;
use crate::libproc::work_queue_info::WorkQueueInfo;

#[cfg(target_os = "linux")]
use self::libc::{c_char, readlink};
#[cfg(target_os = "macos")]
use self::libc::{c_int, c_void};

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
#[cfg(target_os = "macos")]
const MAXPATHLEN: usize = 1024;
#[cfg(target_os = "macos")]
const PROC_PIDPATHINFO_MAXSIZE: usize = 4 * MAXPATHLEN;

// From http://opensource.apple.com//source/xnu/xnu-1456.1.26/bsd/sys/proc_info.h and
// http://fxr.watson.org/fxr/source/bsd/sys/proc_info.h?v=xnu-2050.18.24
/// The `ProcType` type. Used to specify what type of processes you are interested
/// in in other calls, such as `listpids`.
#[derive(Copy, Clone)]
pub enum ProcType {
    // All processes
    ProcAllPIDS = 1,
    ProcPGRPOnly = 2,
    ProcTTYOnly = 3,
    ProcUIDOnly = 4,
    ProcRUIDOnly = 5,
    ProcPPIDOnly = 6,
}

// This trait is needed for polymorphism on pidinfo types, also abstracting flavor in order to provide
// type-guaranteed flavor correctness
pub trait PIDInfo: Default {
    fn flavor() -> PidInfoFlavor;
}

// From http://opensource.apple.com/source/xnu/xnu-1504.7.4/bsd/kern/proc_info.c
/// An enum used to specify what type of information about a process is referenced
pub enum PidInfoFlavor {
    ListFDs = 1,
    // list of ints?
    TaskAllInfo = 2,
    // struct proc_taskallinfo
    TBSDInfo = 3,
    // struct proc_bsdinfo
    TaskInfo = 4,
    // struct proc_taskinfo
    ThreadInfo = 5,
    // struct proc_threadinfo
    ListThreads = 6,
    // list if int thread ids
    RegionInfo = 7,
    RegionPathInfo = 8,
    // string?
    VNodePathInfo = 9,
    // string?
    ThreadPathInfo = 10,
    // String?
    PathInfo = 11,
    // String
    WorkQueueInfo = 12,    // struct proc_workqueueinfo
}

/// The `PidInfo` enum contains a piece of information about a processes
#[allow(clippy::large_enum_variant)]
pub enum PidInfo {
    /// File Descriptors used by Process
    ListFDs(Vec<i32>),
    TaskAllInfo(TaskAllInfo),
    TBSDInfo(BSDInfo),
    TaskInfo(TaskInfo),
    ThreadInfo(ThreadInfo),
    /// A list of Thread IDs
    ListThreads(Vec<i32>),
    RegionInfo(String),
    // String??
    RegionPathInfo(String),
    VNodePathInfo(String),
    ThreadPathInfo(String),
    /// The path of the executable being run as the process
    PathInfo(String),
    WorkQueueInfo(WorkQueueInfo),
}

// This trait is needed for polymorphism on listpidinfo types, also abstracting flavor in order to provide
// type-guaranteed flavor correctness
pub trait ListPIDInfo {
    type Item;
    fn flavor() -> PidInfoFlavor;
}

pub struct ListThreads;

impl ListPIDInfo for ListThreads {
    type Item = u64;
    fn flavor() -> PidInfoFlavor { PidInfoFlavor::ListThreads }
}

// this extern block links to the libproc library
// Original signatures of functions can be found at http://opensource.apple.com/source/Libc/Libc-594.9.4/darwin/libproc.c
#[cfg(target_os = "macos")]
#[link(name = "proc", kind = "dylib")]
extern {
    fn proc_listpids(proc_type: u32, typeinfo: u32, buffer: *mut c_void, buffersize: u32) -> c_int;

    fn proc_pidinfo(pid: c_int, flavor: c_int, arg: u64, buffer: *mut c_void, buffersize: c_int) -> c_int;

    fn proc_name(pid: c_int, buffer: *mut c_void, buffersize: u32) -> c_int;

    fn proc_regionfilename(pid: c_int, address: u64, buffer: *mut c_void, buffersize: u32) -> c_int;

    fn proc_pidpath(pid: c_int, buffer: *mut c_void, buffersize: u32) -> c_int;

    fn proc_libversion(major: *mut c_int, minor: *mut c_int) -> c_int;
}


/// Returns the PIDs of the processes active that match the ProcType passed in
///
/// # Examples
///
/// Get the list of running process IDs using `listpids`
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::proc_pid;
///
/// match proc_pid::listpids(proc_pid::ProcType::ProcAllPIDS) {
///     Ok(pids) => println!("Found {} processes using listpids()", pids.len()),
///     Err(err) => assert!(false, "Error listing pids")
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn listpids(proc_types: ProcType) -> Result<Vec<u32>, String> {
    let buffer_size = unsafe { proc_listpids(proc_types as u32, 0, ptr::null_mut(), 0) };
    if buffer_size <= 0 {
        return Err(helpers::get_errno_with_message(buffer_size));
    }

    let capacity = buffer_size as usize / mem::size_of::<u32>();
    let mut pids: Vec<u32> = Vec::with_capacity(capacity);
    let buffer_ptr = pids.as_mut_ptr() as *mut c_void;

    let ret = unsafe { proc_listpids(proc_types as u32, 0, buffer_ptr, buffer_size as u32) };

    if ret <= 0 {
        Err(helpers::get_errno_with_message(ret))
    } else {
        let items_count = ret as usize / mem::size_of::<u32>() - 1;
        unsafe {
            pids.set_len(items_count);
        }

        Ok(pids)
    }
}

#[cfg(target_os = "linux")]
pub fn listpids(proc_types: ProcType) -> Result<Vec<u32>, String> {
    match proc_types {
        ProcType::ProcAllPIDS => {
            let mut pids = Vec::<u32>::new();

            for entry in fs::read_dir("/proc").expect("Could not read '/proc'") {
                let path = entry.expect("Couldn't get /proc/ filename").path();
                let filename = path.file_name();
                if let Some(name) = filename {
                    if let Some(n) = name.to_str() {
                        if let Ok(pid) = n.parse::<u32>() {
                            pids.push(pid);
                        }
                    }
                }
            }

            Ok(pids)
        },
        _ => Err("Unsupported ProcType".to_owned())
    }
}

/// Get info about a process
///
/// arg - is "heavily not documented" and need to look at code for each flavour
/// [here](http://opensource.apple.com/source/xnu/xnu-1504.7.4/bsd/kern/proc_info.c)
/// to figure out what it's doing.
///
/// Pull-Requests welcome!
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::proc_pid::pidinfo;
/// use libproc::libproc::bsd_info::BSDInfo;
/// use std::process;
///
/// let pid = process::id() as i32;
///
/// match pidinfo::<BSDInfo>(pid, 0) {
///     Ok(info) => assert_eq!(info.pbi_pid as i32, pid),
///     Err(err) => assert!(false, "Error retrieving process info: {}", err)
/// };
/// ```
#[cfg(target_os = "macos")]
pub fn pidinfo<T: PIDInfo>(pid: i32, arg: u64) -> Result<T, String> {
    let flavor = T::flavor() as i32;
    let buffer_size = mem::size_of::<T>() as i32;
    let mut pidinfo = T::default();
    let buffer_ptr = &mut pidinfo as *mut _ as *mut c_void;
    let ret: i32;

    unsafe {
        ret = proc_pidinfo(pid, flavor, arg, buffer_ptr, buffer_size);
    };

    if ret <= 0 {
        Err(helpers::get_errno_with_message(ret))
    } else {
        Ok(pidinfo)
    }
}

#[cfg(not(target_os = "macos"))]
pub fn pidinfo<T: PIDInfo>(_pid: i32, _arg: u64) -> Result<T, String> {
    unimplemented!()
}

/// Get the filename associated with a memory region
///
/// # Examples
///
/// ```
/// use libproc::libproc::proc_pid::regionfilename;
///
/// // This checks that it can find the regionfilename of the region at address 0, of the init process with PID 1
/// match regionfilename(1, 0) {
///     Ok(regionfilename) => println!("Region Filename (at address = 0) of init process PID = 1 is '{}'", regionfilename),
///     Err(message) => assert!(true, message)
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn regionfilename(pid: i32, address: u64) -> Result<String, String> {
    let mut buf: Vec<u8> = Vec::with_capacity(PROC_PIDPATHINFO_MAXSIZE - 1);
    let buffer_ptr = buf.as_mut_ptr() as *mut c_void;
    let buffer_size = buf.capacity() as u32;
    let ret: i32;

    unsafe {
        ret = proc_regionfilename(pid, address, buffer_ptr, buffer_size);
    };

    helpers::check_errno(ret, &mut buf)
}

#[cfg(not(target_os = "macos"))]
pub fn regionfilename(_pid: i32, _address: u64) -> Result<String, String> {
    Err("'regionfilename' not implemented on linux".to_owned())
}

/// Get the path of the executable file being run for a process
///
/// # Examples
///
/// ```
/// use libproc::libproc::proc_pid::pidpath;
///
/// match pidpath(1) {
///     Ok(path) => println!("Path of init process with PID = 1 is '{}'", path),
///     Err(message) => assert!(false, message)
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn pidpath(pid: i32) -> Result<String, String> {
    let mut buf: Vec<u8> = Vec::with_capacity(PROC_PIDPATHINFO_MAXSIZE - 1);
    let buffer_ptr = buf.as_mut_ptr() as *mut c_void;
    let buffer_size = buf.capacity() as u32;
    let ret: i32;

    unsafe {
        ret = proc_pidpath(pid, buffer_ptr, buffer_size);
    };

    helpers::check_errno(ret, &mut buf)
}

#[cfg(target_os = "linux")]
pub fn pidpath(pid: i32) -> Result<String, String> {
    let exe_path = CString::new(format!("/proc/{}/exe", pid)).unwrap();
    let mut buf: Vec<u8> = Vec::with_capacity(PATH_MAX as usize - 1);
    let buffer_ptr = buf.as_mut_ptr() as *mut c_char;
    let buffer_size = buf.capacity();
    let ret = unsafe {
        readlink(exe_path.as_ptr(), buffer_ptr, buffer_size)
    };

    helpers::check_errno(ret as i32, &mut buf)
}

/// Get the major and minor version numbers of the native libproc library (Mac OS X)
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::proc_pid;
///
/// match proc_pid::libversion() {
///     Ok((major, minor)) => println!("Libversion: {}.{}", major, minor),
///     Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn libversion() -> Result<(i32, i32), String> {
    let mut major = 0;
    let mut minor = 0;
    let ret: i32;

    unsafe {
        ret = proc_libversion(&mut major, &mut minor);
    };

    // return value of 0 indicates success (inconsistent with other functions... :-( )
    if ret == 0 {
        Ok((major, minor))
    } else {
        Err(helpers::get_errno_with_message(ret))
    }
}

#[cfg(not(target_os = "macos"))]
pub fn libversion() -> Result<(i32, i32), String> {
    Err("Linux does not use a library, so no library version number".to_owned())
}

/// Get the name of a process
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::proc_pid;
///
/// match proc_pid::name(1) {
///     Ok(name) => println!("Name: {}", name),
///     Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn name(pid: i32) -> Result<String, String> {
    let mut namebuf: Vec<u8> = Vec::with_capacity(PROC_PIDPATHINFO_MAXSIZE - 1);
    let buffer_ptr = namebuf.as_ptr() as *mut c_void;
    let buffer_size = namebuf.capacity() as u32;
    let ret: i32;

    unsafe {
        ret = proc_name(pid, buffer_ptr, buffer_size);
    };

    if ret <= 0 {
        Err(helpers::get_errno_with_message(ret))
    } else {
        unsafe {
            namebuf.set_len(ret as usize);
        }

        match String::from_utf8(namebuf) {
            Ok(name) => Ok(name),
            Err(e) => Err(format!("Invalid UTF-8 sequence: {}", e))
        }
    }
}

/*
    A helper function for filding named fields in specific /proc FS files for processes
    This will be more useful when implementing more linux functions
*/
#[cfg(target_os = "linux")]
fn procfile_field(filename: &str, fieldname: &str) -> Result<String, String> {
    const SEPARATOR: &str = ":";
    let lineheader = format!("{}{}", fieldname, SEPARATOR);

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename)
        .expect(&format!("Could not open /proc file '{}'", filename));
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let line = line.expect("Could not read file contents");
        if line.starts_with(&lineheader) {
            let parts: Vec<&str> = line.split(SEPARATOR).collect();
            return Ok(parts[1].trim().to_owned());
        }
    }

    Err(format!("Could not find the field named '{}' in the /proc FS file name '{}'", fieldname, filename).to_owned())
}

#[cfg(target_os = "linux")]
pub fn name(pid: i32) -> Result<String, String> {
    procfile_field(&format!("/proc/{}/status", pid), "Name")
}

/// Get information on all running processes.
///
/// `max_len` is the maximum number of array to return.
/// The length of return value: `Vec<T::Item>` may be less than `max_len`.
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::proc_pid::{listpidinfo, pidinfo};
/// use libproc::libproc::task_info::TaskAllInfo;
/// use libproc::libproc::file_info::{ListFDs, ProcFDType};
/// use std::process;
///
/// let pid = process::id() as i32;
///
/// if let Ok(info) = pidinfo::<TaskAllInfo>(pid, 0) {
///     if let Ok(fds) = listpidinfo::<ListFDs>(pid, info.pbsd.pbi_nfiles as usize) {
///         for fd in &fds {
///             let fd_type = ProcFDType::from(fd.proc_fdtype);
///             println!("File Descriptor: {}, Type: {:?}", fd.proc_fd, fd_type);
///         }
///     }
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn listpidinfo<T: ListPIDInfo>(pid: i32, max_len: usize) -> Result<Vec<T::Item>, String> {
    let flavor = T::flavor() as i32;
    let buffer_size = mem::size_of::<T::Item>() as i32 * max_len as i32;
    let mut buffer = Vec::<T::Item>::with_capacity(max_len);
    let buffer_ptr = unsafe {
        buffer.set_len(max_len);
        buffer.as_mut_ptr() as *mut c_void
    };

    let ret: i32;

    unsafe {
        ret = proc_pidinfo(pid, flavor, 0, buffer_ptr, buffer_size);
    };

    if ret <= 0 {
        Err(helpers::get_errno_with_message(ret))
    } else {
        let actual_len = ret as usize / mem::size_of::<T::Item>();
        buffer.truncate(actual_len);
        Ok(buffer)
    }
}

#[cfg(not(target_os = "macos"))]
pub fn listpidinfo<T: ListPIDInfo>(_pid: i32, _max_len: usize) -> Result<Vec<T::Item>, String> {
    unimplemented!()
}

/// Gets the path of current working directory for the process with the provided pid.
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::proc_pid::pidcwd;
///
/// match pidcwd(1) {
///     Ok(cwd) => println!("The CWD of the process with pid=1 is '{}'", cwd.display()),
///     Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn pidcwd(_pid: pid_t) -> Result<PathBuf, String> {
    Err("pidcwd is not implemented for macos".into())
}

#[cfg(target_os = "linux")]
pub fn pidcwd(pid: pid_t) -> Result<PathBuf, String> {
    fs::read_link(format!("/proc/{}/cwd", pid)).map_err(|e| {
        e.to_string()
    })
}

/// Gets path of current working directory for the current process.
///
/// Just wraps rusts env::current_dir() function so not so useful.
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::proc_pid::cwdself;
///
/// match cwdself() {
///     Ok(cwd) => println!("The CWD of the current process is '{}'", cwd.display()),
///     Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
/// }
/// ```
pub fn cwdself() -> Result<PathBuf, String> {
    env::current_dir().map_err(|e| e.to_string())
}

/// Determine if the current user ID of this process is root
///
/// # Examples
///
/// ```
/// use libproc::libproc::proc_pid::am_root;
///
/// if am_root() {
///     println!("With great power comes great responsibility");
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn am_root() -> bool {
    // geteuid() is unstable still - wait for it or wrap this:
    // https://stackoverflow.com/questions/3214297/how-can-my-c-c-application-determine-if-the-root-user-is-executing-the-command
    unsafe { libc::getuid() == 0 }
}

#[cfg(target_os = "linux")]
pub fn am_root() -> bool {
    // when this becomes stable in rust libc then we can remove this function or combine for mac and linux
    unsafe { libc::geteuid() == 0 }
}

// run tests with 'cargo test -- --nocapture' to see the test output
#[cfg(test)]
mod test {
    #[cfg(target_os = "linux")]
    use std::process;
    use std::env;

    #[cfg(target_os = "macos")]
    use crate::libproc::bsd_info::BSDInfo;
    #[cfg(target_os = "macos")]
    use crate::libproc::file_info::ListFDs;
    #[cfg(target_os = "macos")]
    use crate::libproc::task_info::TaskAllInfo;

    #[cfg(target_os = "macos")]
    use super::{libversion, listpidinfo, ListThreads, pidinfo};
    use super::{name, cwdself, listpids, pidpath};
    #[cfg(target_os = "linux")]
    use super::pidcwd;
    use crate::libproc::proc_pid::ProcType;
    use super::am_root;

    #[cfg(target_os = "macos")]
    #[test]
    fn pidinfo_test() {
        use std::process;
        let pid = process::id() as i32;

        match pidinfo::<BSDInfo>(pid, 0) {
            Ok(info) => assert_eq!(info.pbi_pid as i32, pid),
            Err(err) => assert!(false, "Error retrieving process info: {}", err)
        };
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn listpidinfo_test() {
        use std::process;
        let pid = process::id() as i32;

        match pidinfo::<TaskAllInfo>(pid, 0) {
            Ok(info) => {
                match listpidinfo::<ListThreads>(pid, info.ptinfo.pti_threadnum as usize) {
                    Ok(threads) => assert!(!threads.is_empty()),
                    Err(err) => assert!(false, "Error retrieving process info: {}", err)
                }
                match listpidinfo::<ListFDs>(pid, info.pbsd.pbi_nfiles as usize) {
                    Ok(fds) => assert!(!fds.is_empty()),
                    Err(err) => assert!(false, "Error retrieving process info: {}", err)
                }
            }
            Err(err) => assert!(false, "Error retrieving process info: {}", err)
        };
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn libversion_test() {
        match libversion() {
            Ok((major, minor)) => println!("Major = {}, Minor = {}", major, minor),
            Err(message) => panic!(message)
        }
    }

    #[test]
    fn listpids_test() {
        match listpids(ProcType::ProcAllPIDS) {
            Ok(pids) => assert!(pids.len() > 1),
            Err(e) => assert!(false, "Error calling listpids(): {}", e)
        }
    }

    #[test]
    fn name_test() {
        #[cfg(target_os = "linux")]
        let expected_name = "systemd";
        #[cfg(target_os = "macos")]
        let expected_name = "launchd";

        if am_root() || cfg!(target_os = "linux") {
            match name(1) {
                Ok(name) => assert_eq!(expected_name, name),
                Err(err) => assert!(false, "Error retrieving process name: {}", err)
            }
        } else {
            println!("Cannot run name_test on macos unless run as root");
        }
    }

    #[test]
    // This checks that it cannot find the path of the process with pid -1 and returns correct error messaage
    fn pidpath_test_unknown_pid() {
        #[cfg(target_os = "macos")]
            let error_message = "No such process";
        #[cfg(target_os = "linux")]
            let error_message = "No such file or directory";

        match pidpath(-1) {
            Ok(path) => assert!(false, "It found the path of process Pwith ID = -1 (path = {}), that's not possible\n", path),
            Err(message) => assert!(message.contains(error_message)),
        }
    }

    #[test]
    #[cfg(target_os = "macos")]
    // TODO this seems to require root permission on linux
    // This checks that it cannot find the path of the process with pid 1
    fn pidpath_test() {
        #[cfg(target_os = "macos")]
            let expected_path = "/sbin/launchd";

        match pidpath(1) {
            Ok(path) => assert_eq!(expected_path, path),
            Err(message) => panic!(message),
        }
    }

    // Pretty useless test as it uses the exact same code as the function - but I guess we
    // should check it can be called and returns correct value
    #[test]
    fn test_cwd_self() {
        assert_eq!(env::current_dir().unwrap(), cwdself().unwrap());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn pidcwd_of_self_test() {
        assert_eq!(env::current_dir().unwrap(), pidcwd(process::id() as i32).unwrap());
    }

    #[test]
    fn test_if_root() {
        if am_root() {
            println!("You are root");
        } else {
            println!("You are not root");
        }
    }
}