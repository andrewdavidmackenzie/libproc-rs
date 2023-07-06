// OS-specific implementations of process-related functions
#[cfg(any(target_os = "linux", target_os = "redox"))]
mod linux;
#[cfg(any(target_os = "linux", target_os = "redox"))]
pub(crate) use self::linux::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub(crate) use self::macos::*;
