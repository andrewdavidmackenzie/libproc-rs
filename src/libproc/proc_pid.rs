use std::env;
#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
use std::ffi::CString;
#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
use std::fs;
#[cfg(target_os = "macos")]
use std::mem;
use std::path::PathBuf;

#[cfg(target_os = "macos")]
use libc::c_int;
use libc::pid_t;

#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
use libc::PATH_MAX;

#[cfg(target_os = "macos")]
use crate::libproc::bsd_info::BSDInfo;
use crate::libproc::helpers;
#[cfg(target_os = "macos")]
use crate::libproc::task_info::{TaskAllInfo, TaskInfo};
#[cfg(target_os = "macos")]
use crate::libproc::thread_info::ThreadInfo;
#[cfg(target_os = "macos")]
use crate::libproc::work_queue_info::WorkQueueInfo;
#[cfg(target_os = "macos")]
use crate::osx_libproc_bindings::{
    proc_libversion, proc_name, proc_pidinfo, proc_pidpath, proc_regionfilename,
    PROC_PIDPATHINFO_MAXSIZE,
};

#[cfg(target_os = "macos")]
use libc::c_void;
#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
use libc::{c_char, readlink};

use crate::processes;

/// The `ProcType` type. Used to specify what type of processes you are interested
/// in in other calls, such as `listpids`.
#[derive(Copy, Clone)]
pub enum ProcType {
    /// All processes
    ProcAllPIDS = 1,
    /// Only PGRP Processes
    ProcPGRPOnly = 2,
    /// Only TTY Processes
    ProcTTYOnly = 3,
    /// Only UID Processes
    ProcUIDOnly = 4,
    /// Only RUID Processes
    ProcRUIDOnly = 5,
    /// Only PPID Processes
    ProcPPIDOnly = 6,
}

/// The `PIDInfo` trait is needed for polymorphism on pidinfo types, also abstracting flavor in order to provide
/// type-guaranteed flavor correctness
pub trait PIDInfo {
    /// Return the `PidInfoFlavor` of the implementing struct
    fn flavor() -> PidInfoFlavor;
}

/// An enum used to specify what type of information about a process is referenced
/// See <http://opensource.apple.com/source/xnu/xnu-1504.7.4/bsd/kern/proc_info.c>
pub enum PidInfoFlavor {
    /// List of File Descriptors
    ListFDs = 1,
    /// struct `proc_taskallinfo`
    TaskAllInfo = 2,
    /// struct `proc_bsdinfo`
    TBSDInfo = 3,
    /// struct `proc_taskinfo`
    TaskInfo = 4,
    /// struct `proc_threadinfo`
    ThreadInfo = 5,
    /// list thread ids
    ListThreads = 6,
    /// TBD what type `RegionInfo` is - string?
    RegionInfo = 7,
    /// Region Path info strings
    RegionPathInfo = 8,
    /// Strings
    VNodePathInfo = 9,
    /// Strings
    ThreadPathInfo = 10,
    /// Strings
    PathInfo = 11,
    /// struct `proc_workqueueinfo`
    WorkQueueInfo = 12,
}

/// The `PidInfo` enum contains a piece of information about a processes
#[allow(clippy::large_enum_variant)]
pub enum PidInfo {
    /// File Descriptors used by Process
    ListFDs(Vec<i32>),
    /// Get all Task Info
    #[cfg(target_os = "macos")]
    TaskAllInfo(TaskAllInfo),
    /// Get `TBSDInfo`
    #[cfg(target_os = "macos")]
    TBSDInfo(BSDInfo),
    /// Single Task Info
    #[cfg(target_os = "macos")]
    TaskInfo(TaskInfo),
    /// `ThreadInfo`
    #[cfg(target_os = "macos")]
    ThreadInfo(ThreadInfo),
    /// A list of Thread IDs
    ListThreads(Vec<i32>),
    /// `RegionInfo`
    RegionInfo(String),
    /// `RegionPathInfo`
    RegionPathInfo(String),
    /// `VNodePathInfo`
    VNodePathInfo(String),
    /// `ThreadPathInfo`
    ThreadPathInfo(String),
    /// `PathInfo` of the executable being run as the process
    PathInfo(String),
    /// `WorkQueueInfo`
    #[cfg(target_os = "macos")]
    WorkQueueInfo(WorkQueueInfo),
}

/// The `ListPIDInfo` trait is needed for polymorphism on listpidinfo types, also abstracting flavor in order to provide
/// type-guaranteed flavor correctness
pub trait ListPIDInfo {
    /// Item
    type Item;
    /// Return the `PidInfoFlavor` of the implementing struct
    fn flavor() -> PidInfoFlavor;
}

/// Struct for List of Threads
pub struct ListThreads;

impl ListPIDInfo for ListThreads {
    type Item = u64;
    fn flavor() -> PidInfoFlavor {
        PidInfoFlavor::ListThreads
    }
}

/// Map `ProcType` to the new `ProcFilter` enum; the values match the now
/// deprecated implementation of `listpids`
impl From<ProcType> for processes::ProcFilter {
    fn from(proc_type: ProcType) -> Self {
        use processes::ProcFilter;

        match proc_type {
            ProcType::ProcAllPIDS => ProcFilter::All,
            ProcType::ProcPGRPOnly => ProcFilter::ByProgramGroup { pgrpid: 0 },
            ProcType::ProcTTYOnly => ProcFilter::ByTTY { tty: 0 },
            ProcType::ProcUIDOnly => ProcFilter::ByUID { uid: 0 },
            ProcType::ProcRUIDOnly => ProcFilter::ByRealUID { ruid: 0 },
            ProcType::ProcPPIDOnly => ProcFilter::ByParentProcess { ppid: 0 },
        }
    }
}

/// Returns the PIDs of the active processes that match the `proc_types` parameter
///
/// # Note
///
/// This function is deprecated in favor of
/// [`libproc::processes::pids_by_type()`][crate::processes::pids_by_type],
/// which lets you specify what PGRP / TTY / UID / RUID / PPID you want to filter by
#[allow(clippy::missing_errors_doc)]
#[deprecated(
    since = "0.13.0",
    note = "Please use `libproc::processes::pids_by_type()` instead."
)]
pub fn listpids(proc_types: ProcType) -> Result<Vec<u32>, String> {
    processes::pids_by_type(proc_types.into()).map_err(|e| {
        e.raw_os_error()
            .map_or_else(|| e.to_string(), helpers::get_errno_with_message)
    })
}

/// Search through the current processes looking for open file references which match
/// a specified path or volume.
///
/// # Note
///
/// This function is deprecated in favor of
/// [`libproc::processes::pids_by_type_and_path()`][crate::processes::pids_by_type_and_path],
/// which lets you specify what PGRP / TTY / UID / RUID / PPID you want to
/// filter by.
///
#[cfg(target_os = "macos")]
#[allow(clippy::missing_errors_doc)]
#[deprecated(
    since = "0.13.0",
    note = "Please use `libproc::processes::pids_by_type_and_path()` instead."
)]
pub fn listpidspath(proc_types: ProcType, path: &str) -> Result<Vec<u32>, String> {
    processes::pids_by_type_and_path(proc_types.into(), &PathBuf::from(path), false, false).map_err(
        |e| {
            e.raw_os_error()
                .map_or_else(|| e.to_string(), helpers::get_errno_with_message)
        },
    )
}

/// Get info about a process, task, thread or work queue by specifying the appropriate type for `T`:
/// - `BSDInfo`
/// - `TaskInfo`
/// - `TaskAllInfo`
/// - `ThreadInfo`
/// - `WorkQueueInfo`
///
/// # Errors
///
/// Will return an error if underlying Darwin `proc_pidinfo` returns an error or sets `errno`
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
/// // Get the `BSDInfo` for Process of pid 0
/// match pidinfo::<BSDInfo>(pid, 0) {
///     Ok(info) => assert_eq!(info.pbi_pid as i32, pid),
///     Err(err) => eprintln!("Error retrieving process info: {}", err)
/// };
/// ```
#[cfg(target_os = "macos")]
pub fn pidinfo<T: PIDInfo>(pid: i32, arg: u64) -> Result<T, String> {
    let flavor = T::flavor() as i32;
    // No type `T` will be bigger than `i32::MAX`!!
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    let buffer_size = mem::size_of::<T>() as c_int;
    let mut pidinfo = unsafe { mem::zeroed() };
    #[allow(clippy::pedantic)]
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

#[cfg(any(target_os = "macos", doc))]
/// Get the filename associated with a memory region
///
/// # Errors
///
/// Will return an error if underlying Darwin function `proc_regionfilename` returns an error.
///
/// # Examples
///
/// ```
/// use libproc::libproc::proc_pid::regionfilename;
///
/// // This checks that it can find the regionfilename of the region at address 0, of the init process with PID 1
/// use libproc::libproc::proc_pid::am_root;
///
/// if am_root() {
///     match regionfilename(1, 0) {
///         Ok(regionfilename) => println!("Region Filename (at address = 0) of init process PID = 1 is '{}'", regionfilename),
///         Err(err) => eprintln!("Error: {}", err)
///     }
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn regionfilename(pid: i32, address: u64) -> Result<String, String> {
    let mut buf: Vec<u8> = Vec::with_capacity((PROC_PIDPATHINFO_MAXSIZE - 1) as _);
    let buffer_ptr = buf.as_mut_ptr().cast::<c_void>();
    // PROC_PIDPATHINFO_MAXSIZE will be smaller than `u32::MAX`
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    let buffer_size = buf.capacity() as u32;
    let ret: i32;

    unsafe {
        ret = proc_regionfilename(pid, address, buffer_ptr, buffer_size);
    };

    helpers::check_errno(ret, &mut buf)
}

#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
pub fn regionfilename(_pid: i32, _address: u64) -> Result<String, String> {
    Err("'regionfilename' not implemented on linux".to_owned())
}

/// Get the path of the executable file being run for a process
///
/// # Errors
///
/// Will return an error if underlying Darwin function `proc_pidpath` returns an error.
///
/// # Examples
///
/// ```
/// use libproc::libproc::proc_pid::pidpath;
///
/// match pidpath(1) {
///     Ok(path) => println!("Path of init process with PID = 1 is '{}'", path),
///     Err(err) => eprintln!("Error: {}", err)
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn pidpath(pid: i32) -> Result<String, String> {
    let mut buf: Vec<u8> = Vec::with_capacity((PROC_PIDPATHINFO_MAXSIZE - 1) as _);
    let buffer_ptr = buf.as_mut_ptr().cast::<c_void>();
    // PROC_PIDPATHINFO_MAXSIZE will be smaller than `u32::MAX`
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    let buffer_size = buf.capacity() as u32;
    let ret: i32;

    unsafe {
        ret = proc_pidpath(pid, buffer_ptr, buffer_size as _);
    };

    helpers::check_errno(ret, &mut buf)
}

/// Get the path of the executable file being run for a process
///
/// # Errors
///
/// Will return `Err` if not run as root or the underlying linux `readlink` method returns
/// a non-zero value and sets `errno`
///
/// # Examples
///
/// ```
/// use libproc::libproc::proc_pid::{pidpath, am_root};
///
/// match pidpath(1) {
///     Ok(path) => println!("Path of init process with PID = 1 is '{}'", path),
///     Err(_) if !am_root() => println!("pidpath() needs to be run as root"),
///     Err(err) if am_root() => eprintln!("Error: {}", err),
///     _ => panic!("Unknown error")
/// }
/// ```
#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
pub fn pidpath(pid: i32) -> Result<String, String> {
    let exe_path =
        CString::new(format!("/proc/{pid}/exe")).map_err(|_| "Could not create CString")?;
    let mut buf: Vec<u8> = Vec::with_capacity(PATH_MAX as usize - 1);
    let buffer_ptr = buf.as_mut_ptr().cast::<c_char>();
    let buffer_size = buf.capacity();
    let ret = unsafe { readlink(exe_path.as_ptr(), buffer_ptr, buffer_size) };

    #[allow(clippy::cast_possible_truncation)]
    helpers::check_errno(ret as i32, &mut buf)
}

#[cfg(any(target_os = "macos", doc))]
/// Get the major and minor version numbers of the native libproc library (Mac OS X)
///
/// # Errors
///
/// Should never return an error, but the `Result` return type is used for consistency with
/// other methods, and potential future use.
///
/// # Examples
///
/// ```
/// use libproc::libproc::proc_pid;
///
/// match proc_pid::libversion() {
///     Ok((major, minor)) => println!("Libversion: {}.{}", major, minor),
///     Err(err) => eprintln!("Error: {}", err)
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

#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
pub fn libversion() -> Result<(i32, i32), String> {
    Err("Linux does not use a library, so no library version number".to_owned())
}

/// Get the name of a process, using it's process id (pid)
///
/// # Errors
///
/// Will return an error if Darwin's `proc_pidinfo` returns 0
///
/// # Examples
///
/// ```
/// use libproc::libproc::proc_pid;
///
/// match proc_pid::name(1) {
///     Ok(name) => println!("Name: {}", name),
///     Err(err) => eprintln!("Error: {}", err)
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn name(pid: i32) -> Result<String, String> {
    let mut namebuf: Vec<u8> = Vec::with_capacity((PROC_PIDPATHINFO_MAXSIZE - 1) as _);
    let buffer_ptr = namebuf.as_ptr() as *mut c_void;
    // No type `T` will be bigger than `i32::MAX`!!
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    let buffer_size = namebuf.capacity() as u32;
    let ret: c_int;

    unsafe {
        ret = proc_name(pid, buffer_ptr, buffer_size);
    };

    if ret <= 0 {
        Err(helpers::get_errno_with_message(ret))
    } else {
        unsafe {
            // `ret` must be greater than 0 here, so no sign-loss
            #[allow(clippy::cast_sign_loss)]
            namebuf.set_len(ret as usize);
        }

        match String::from_utf8(namebuf) {
            Ok(name) => Ok(name),
            Err(e) => Err(format!("Invalid UTF-8 sequence: {e}")),
        }
    }
}

/// Get the name of a process, using it's process id (pid)
///
/// # Errors
///
/// An `Err` is returned if the information cannot be read from the procfs file system
#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
pub fn name(pid: i32) -> Result<String, String> {
    helpers::procfile_field(&format!("/proc/{pid}/status"), "Name")
}

/// Get information on all running processes.
///
/// `max_len` is the maximum number of array to return.
/// The length of return value: `Vec<T::Item>` may be less than `max_len`.
///
/// # Errors
///
/// Will return an error if Darwin's `proc_pidinfo` returns 0
///
/// # Examples
///
/// ```
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
    // No type `T` will be bigger than `c_int::MAX`!!
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    let buffer_size = mem::size_of::<T::Item>() as c_ * max_len as c_int;
    let mut buffer = Vec::<T::Item>::with_capacity(max_len);
    let buffer_ptr = unsafe {
        buffer.set_len(max_len);
        buffer.as_mut_ptr().cast::<c_void>()
    };

    let ret: i32;

    unsafe {
        ret = proc_pidinfo(pid, flavor, 0, buffer_ptr, buffer_size);
    };

    if ret <= 0 {
        Err(helpers::get_errno_with_message(ret))
    } else {
        // `ret` must be greater than 0 here, so no sign-loss
        #[allow(clippy::cast_sign_loss)]
        let actual_len = ret as usize / mem::size_of::<T::Item>();
        buffer.truncate(actual_len);
        Ok(buffer)
    }
}

#[cfg(target_os = "macos")]
/// Gets the path of current working directory for the process with the provided pid.
///
/// # Errors
///
/// Currently always returns an error as this is not implemented yet for macos
///
/// # Examples
///
/// ```
/// use libproc::libproc::proc_pid::pidcwd;
///
/// match pidcwd(1) {
///     Ok(cwd) => println!("The CWD of the process with pid=1 is '{}'", cwd.display()),
///     Err(err) => eprintln!("Error: {}", err)
/// }
/// ```
pub fn pidcwd(_pid: pid_t) -> Result<PathBuf, String> {
    Err("pidcwd is not implemented for macos".into())
}

#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
/// Gets the path of current working directory for the process with the provided pid.
///
/// # Errors
///
/// An `Err` is returned if process with PID `pid` does not exist, or the information
/// about it in the procfs file system cannot be read or parsed
///
/// # Examples
///
/// ```
/// use libproc::libproc::proc_pid::pidcwd;
///
/// match pidcwd(1) {
///     Ok(cwd) => println!("The CWD of the process with pid=1 is '{}'", cwd.display()),
///     Err(err) => eprintln!("Error: {}", err)
/// }
/// ```
pub fn pidcwd(pid: pid_t) -> Result<PathBuf, String> {
    fs::read_link(format!("/proc/{pid}/cwd")).map_err(|e| e.to_string())
}

/// Gets path of current working directory for the current process.
///
/// Just wraps rust's `env::current_dir()` function so not so useful.
///
/// # Errors
///
/// Returns an Err if the current working directory value is invalid. Possible cases:
///   * Current directory does not exist.
///   * There are insufficient permissions to access the current directory.
///
/// # Examples
///
/// ```
/// use libproc::libproc::proc_pid::cwdself;
///
/// match cwdself() {
///     Ok(cwd) => println!("The CWD of the current process is '{}'", cwd.display()),
///     Err(err) => eprintln!("Error: {}", err)
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
#[must_use]
pub fn am_root() -> bool {
    // geteuid() is unstable still - wait for it or wrap this:
    // https://stackoverflow.com/questions/3214297/how-can-my-c-c-application-determine-if-the-root-user-is-executing-the-command
    unsafe { libc::getuid() == 0 }
}

/// Return true if the calling process is being run by the root user, false otherwise
#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
#[must_use]
pub fn am_root() -> bool {
    // when this becomes stable in rust libc then we can remove this function or combine for mac and linux
    unsafe { libc::geteuid() == 0 }
}

// run tests with 'cargo test -- --nocapture' to see the test output
#[cfg(test)]
// Don't worry about wrapping in tests
#[allow(clippy::cast_possible_wrap)]
mod test {
    use std::env;
    use std::process;

    #[cfg(target_os = "macos")]
    use crate::libproc::bsd_info::BSDInfo;
    #[cfg(target_os = "macos")]
    use crate::libproc::file_info::ListFDs;
    #[cfg(target_os = "macos")]
    use crate::libproc::task_info::TaskAllInfo;

    use super::am_root;
    #[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
    use super::pidcwd;
    use super::{cwdself, name, pidpath};
    #[cfg(target_os = "macos")]
    use super::{libversion, listpidinfo, pidinfo, ListThreads};
    #[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
    use crate::libproc::helpers;
    #[cfg(target_os = "macos")]
    use crate::libproc::task_info::TaskInfo;
    #[cfg(target_os = "macos")]
    use crate::libproc::thread_info::ThreadInfo;
    #[cfg(target_os = "macos")]
    use crate::libproc::work_queue_info::WorkQueueInfo;

    #[cfg(target_os = "macos")]
    #[test]
    fn pidinfo_test() {
        let pid = process::id() as i32;

        match pidinfo::<BSDInfo>(pid, 0) {
            Ok(info) => assert_eq!(info.pbi_pid as i32, pid),
            Err(e) => panic!("Error retrieving BSDInfo: {}", e),
        };
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn taskinfo_test() {
        let pid = process::id() as i32;

        match pidinfo::<TaskInfo>(pid, 0) {
            Ok(info) => assert!(info.pti_virtual_size > 0),
            Err(e) => panic!("Error retrieving TaskInfo: {}", e),
        };
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn taskallinfo_test() {
        let pid = process::id() as i32;

        match pidinfo::<TaskAllInfo>(pid, 0) {
            Ok(info) => assert!(info.ptinfo.pti_virtual_size > 0),
            Err(e) => panic!("Error retrieving TaskAllInfo: {}", e),
        };
    }

    #[ignore]
    #[cfg(target_os = "macos")]
    #[test]
    fn threadinfo_test() {
        let pid = process::id() as i32;

        match pidinfo::<ThreadInfo>(pid, 0) {
            Ok(info) => assert!(info.pth_user_time > 0),
            Err(e) => panic!("Error retrieving ThreadInfo: {}", e),
        };
    }

    #[ignore]
    #[cfg(target_os = "macos")]
    #[test]
    fn workqueueinfo_test() {
        let pid = process::id() as i32;

        match pidinfo::<WorkQueueInfo>(pid, 0) {
            Ok(info) => assert!(info.pwq_nthreads > 0),
            Err(e) => panic!("{}: {}", "Error retrieving WorkQueueInfo", e),
        };
    }

    #[cfg(target_os = "macos")]
    #[test]
    #[allow(clippy::cast_sign_loss)]
    fn listpidinfo_test() {
        let pid = process::id() as i32;

        if let Ok(info) = pidinfo::<TaskAllInfo>(pid, 0) {
            if let Ok(threads) = listpidinfo::<ListThreads>(pid, info.ptinfo.pti_threadnum as usize)
            {
                assert!(!threads.is_empty());
            }
            if let Ok(fds) = listpidinfo::<ListFDs>(pid, info.pbsd.pbi_nfiles as usize) {
                assert!(!fds.is_empty());
            };
        }
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn libversion_test() {
        libversion().expect("libversion() failed");
    }

    #[test]
    fn name_test() {
        if am_root()
            || cfg!(any(
                target_os = "linux",
                target_os = "redox",
                target_os = "android"
            ))
        {
            assert!(
                &name(process::id() as i32)
                    .expect("Could not get the process name")
                    .starts_with("libproc"),
                "Incorrect process name"
            );
        } else {
            println!("Cannot run 'name_test' on macos unless run as root");
        }
    }

    #[test]
    // This checks that it cannot find the path of the process with pid -1 and returns correct error message
    fn pidpath_test_unknown_pid_test() {
        #[cfg(target_os = "macos")]
        let error_message = "No such process";
        #[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
        let error_message = "No such file or directory";

        match pidpath(-1) {
            Ok(path) => panic!(
                "It found the path of process with ID = -1 (path = {}), that's not possible\n",
                path
            ),
            Err(message) => assert!(message.contains(error_message)),
        }
    }

    #[test]
    #[cfg(target_os = "macos")]
    // This checks that it cannot find the path of the process with pid 1
    fn pidpath_test() {
        assert_eq!("/sbin/launchd", pidpath(1).expect("pidpath() failed"));
    }

    // Pretty useless test as it uses the exact same code as the function - but I guess we
    // should check it can be called and returns correct value
    #[test]
    fn cwd_self_test() {
        assert_eq!(
            env::current_dir().expect("Could not get current directory"),
            cwdself().expect("cwdself() failed")
        );
    }

    #[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
    #[test]
    fn pidcwd_of_self_test() {
        assert_eq!(
            env::current_dir().expect("Could not get current directory"),
            pidcwd(process::id() as i32).expect("pidcwd() failed")
        );
    }

    #[test]
    fn am_root_test() {
        if am_root() {
            println!("You are root");
        } else {
            println!("You are not root");
        }
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
    fn procfile_field_test() {
        if am_root() {
            assert!(helpers::procfile_field("/proc/1/status", "invalid").is_err());
        }
    }
}
