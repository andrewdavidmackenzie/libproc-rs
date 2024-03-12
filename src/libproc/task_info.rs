extern crate libc;

use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};
use crate::libproc::bsd_info::BSDInfo;
pub use crate::osx_libproc_bindings::proc_taskinfo as TaskInfo;


impl PIDInfo for TaskInfo {
    fn flavor() -> PidInfoFlavor { PidInfoFlavor::TaskInfo }
}

/// Struct for info on all Tasks
#[repr(C)]
pub struct TaskAllInfo {
    /// `BSDInfo`
    pub pbsd : BSDInfo,
    /// `TaskInfo`
    pub ptinfo : TaskInfo
}

impl PIDInfo for TaskAllInfo {
    fn flavor() -> PidInfoFlavor { PidInfoFlavor::TaskAllInfo }
}
