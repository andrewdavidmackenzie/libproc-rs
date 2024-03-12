#![deny(missing_docs)]
#![warn(clippy::unwrap_used)]

//! `libproc` is a library for getting information about running processes on Mac and Linux.
//!
//! Not all methods are available on both Operating Systems yet, but more will be made
//! cross-platform over time.
//!
//! Get information (such as name, path, process info, fd) about running processes by pid, process type, etc.
//!
//! At the moment these methods have been implemented, most of which have examples in their docs:
//!

/// List processes by type, path or by type and path.
pub mod processes;

#[doc(inline)]
/// Get information about processes using mainly the `pid`
pub use libproc::proc_pid;

#[doc(inline)]
/// Read messages from the Kernel Message Buffer
pub use libproc::kmesg_buffer;

#[doc(inline)]
/// Get information about resource usage of processes
pub use libproc::pid_rusage;

#[cfg(any(target_os = "macos", doc))]
#[doc(inline)]
/// Get information specific to BSD/Darwin on macos
pub use libproc::bsd_info;

#[cfg(any(target_os = "macos", doc))]
#[doc(inline)]
/// Get information about a process's use of different types of file descriptors
pub use libproc::file_info;

#[cfg(any(target_os = "macos", doc))]
#[doc(inline)]
/// Get information about a processes use of network, sockets etc.
pub use libproc::net_info;

#[cfg(any(target_os = "macos", doc))]
#[doc(inline)]
/// Get information about a process's BSD Tasks
pub use libproc::task_info;

#[cfg(any(target_os = "macos", doc))]
#[doc(inline)]
/// Get information about threads within a process
pub use libproc::thread_info;

#[cfg(any(target_os = "macos", doc))]
#[doc(inline)]
/// Get information about Work Queues
pub use libproc::work_queue_info;

// Not documenting this as this export is legacy, and replaced by all the re-exports of
// sub-modules above
#[doc(hidden)]
pub mod libproc;

#[cfg(target_os = "macos")]
#[allow(warnings, missing_docs)]
mod osx_libproc_bindings {
    include!(concat!(env!("OUT_DIR"), "/osx_libproc_bindings.rs"));
}
