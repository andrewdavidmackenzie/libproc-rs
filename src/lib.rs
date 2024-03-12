#![deny(missing_docs)]
#![warn(clippy::unwrap_used)]

//! `libproc` is a library for getting information about running processes on Mac and Linux.
//!
//! Not all methods are available on both Operating Systems yet, but more will be made
//! cross-platform over time.

extern crate errno;
extern crate libc;

/// Get information (such as name, path, process info, fd) about running processes by pid, process type, etc.
/// At the moment these methods have been implemented, most of which have examples in their docs:
///
/// # `libproc::libproc` (names that way for bad, historical reasons!)
/// ## Process / PID related
/// `pub fn listpids(proc_types: ProcType) -> Result<Vec<u32>, String> (macos) (linux)`
///
///  `pub fn listpidspath(proc_types: ProcType, path: &str) -> Result<Vec<u32>, String> (macos) (linux)`
///
///  `pub fn pidinfo<T: PIDInfo>(pid : i32, arg: u64) -> Result<T, String> (macos)`
///
///  `pub fn regionfilename(pid: i32, address: u64) -> Result<String, String> (macos)`
///
///  `pub fn pidpath(pid : i32) -> Result<String, String> (macos) (linux)`
///
///  `pub fn libversion() -> Result<(i32, i32), String> (macos)`
///
///  `pub fn name(pid: i32) -> Result<String, String> (linux) (macos)`
///
///  `pub fn listpidinfo<T: ListPIDInfo>(pid : i32, max_len: usize) -> Result<Vec<T::Item>, String> (macos)`
///
///  `pub fn pidcwd(pid: pid_t) -> Result<PathBuf, String> (linux)`
///
///  `pub fn cwdself() -> Result<PathBuf, String> (linux)`
///
///  ## File and FileDescriptor related
///  `pub fn pidfdinfo<T: PIDFDInfo>(pid : i32, fd: i32) -> Result<T, String> (macos)`
///
///  ## PID Resource Usage related
///  (Added in Mac OS X 10.9 - under "macosx_10_9" feature)
///  `pub fn pidrusage<T: PIDRUsage>(pid : i32) -> Result<T, String> (macos)`
///
///  ## Kernel Message Buffer - kmsgbuf
///  `pub fn kmsgbuf() -> Result<String, String>`
pub mod libproc;
/// List processes by type, path or by type and path.
///
///  `pub fn pids_by_type(filter: ProcFilter)` (macos) (linux)
///
///  `pub fn pids_by_path(path: &Path, is_volume: bool, exclude_event_only: bool)` (macos)
///
///  `pub fn pids_by_type_and_path(filter: ProcFilter, path: &Path, is_volume: bool, exclude_event_only: bool)` (macos)
pub mod processes;

#[cfg(target_os = "macos")]
#[allow(warnings, missing_docs)]
mod osx_libproc_bindings {
    include!(concat!(env!("OUT_DIR"), "/osx_libproc_bindings.rs"));
}
