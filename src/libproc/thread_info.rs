use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};
#[cfg(target_os = "macos")]
pub use crate::osx_libproc_bindings::proc_threadinfo as ThreadInfo;

/// # Safety
///
/// `ThreadInfo` is the right size to be passed to `proc_pidinfo`.
#[cfg(target_os = "macos")]
unsafe impl PIDInfo for ThreadInfo {
    fn flavor() -> PidInfoFlavor {
        PidInfoFlavor::ThreadInfo
    }
}
