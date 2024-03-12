/// BSD specific information - very macos specific
#[cfg(target_os = "macos")]
pub mod bsd_info;
/// Information about Files and File Descriptors used by processes
#[cfg(target_os = "macos")]
pub mod file_info;
/// Get messages from the kernel message buffer
pub mod kmesg_buffer;
/// Information about Process Resource Usage - added in Mac OS X 10.9
pub mod pid_rusage;
/// Get basic information about processes by PID
pub mod proc_pid;
/// Information about Tasks - very macos specific
#[cfg(target_os = "macos")]
pub mod task_info;
/// Information about Threads running inside processes
#[cfg(target_os = "macos")]
pub mod thread_info;
/// Information about Work Queues - very macos specific
#[cfg(target_os = "macos")]
pub mod work_queue_info;

/// Information about Network usage by a process
#[cfg(target_os = "macos")]
pub mod net_info;

mod helpers;
pub(crate) mod sys;
