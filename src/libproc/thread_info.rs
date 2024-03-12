use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};
pub use crate::osx_libproc_bindings::proc_threadinfo as ThreadInfo;

impl PIDInfo for ThreadInfo {
    fn flavor() -> PidInfoFlavor {
        PidInfoFlavor::ThreadInfo
    }
}
