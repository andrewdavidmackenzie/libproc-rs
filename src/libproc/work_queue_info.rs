extern crate libc;

use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};

use self::libc::uint32_t;

#[derive(Default)]
pub struct WorkQueueInfo {
    pub pwq_nthreads            : uint32_t,     // total number of workqueue threads
    pub pwq_runthreads          : uint32_t,     // total number of running workqueue threads
    pub pwq_blockedthreads      : uint32_t,     // total number of blocked workqueue threads
    pub reserved                : [uint32_t;1]  // reserved for future use
}

impl PIDInfo for WorkQueueInfo {
    fn flavor() -> PidInfoFlavor { PidInfoFlavor::WorkQueueInfo }
}