use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};
#[cfg(target_os = "macos")]
pub use crate::osx_libproc_bindings::proc_threadinfo as ThreadInfo;

#[cfg(target_os = "macos")]
impl PIDInfo for ThreadInfo {
    fn flavor() -> PidInfoFlavor {
        PidInfoFlavor::ThreadInfo
    }
}
