#[cfg(target_os = "macos")]
use crate::libproc::bsd_info::BSDInfo;
use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};
#[cfg(target_os = "macos")]
pub use crate::osx_libproc_bindings::proc_taskinfo as TaskInfo;

/// # Safety
///
/// `TaskInfo` is the right size to be passed to `proc_pidinfo`.
#[cfg(target_os = "macos")]
unsafe impl PIDInfo for TaskInfo {
    fn flavor() -> PidInfoFlavor {
        PidInfoFlavor::TaskInfo
    }
}

#[cfg(target_os = "macos")]
/// Struct for info on all Tasks
#[repr(C)]
pub struct TaskAllInfo {
    /// `BSDInfo`
    pub pbsd: BSDInfo,
    /// `TaskInfo`
    pub ptinfo: TaskInfo,
}

/// # Safety
///
/// `TaskInfoAll` is the right size to be passed to `proc_pidinfo`.
#[cfg(target_os = "macos")]
unsafe impl PIDInfo for TaskAllInfo {
    fn flavor() -> PidInfoFlavor {
        PidInfoFlavor::TaskAllInfo
    }
}
