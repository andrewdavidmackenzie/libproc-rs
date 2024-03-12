#[cfg(any(target_os = "macos", doc))]
/// BSD specific information - very macos specific
pub mod bsd_info;

#[cfg(any(target_os = "macos", doc))]
/// Information about Files and File Descriptors used by processes
pub mod file_info;

/// Get messages from the kernel message buffer
pub mod kmesg_buffer;

/// Information about Process Resource Usage - added in Mac OS X 10.9
pub mod pid_rusage;

/// Get basic information about processes by PID
pub mod proc_pid;

#[cfg(any(target_os = "macos", doc))]
/// Information about Tasks - very macos specific
pub mod task_info;

#[cfg(any(target_os = "macos", doc))]
/// Information about Threads running inside processes
pub mod thread_info;

#[cfg(any(target_os = "macos", doc))]
/// Information about Work Queues - very macos specific
pub mod work_queue_info;

#[cfg(any(target_os = "macos", doc))]
/// Information about Network usage by a process
pub mod net_info;

mod helpers;
pub(crate) mod sys;
