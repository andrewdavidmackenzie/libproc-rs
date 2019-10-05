extern crate libc;

use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};

#[derive(Default)]
pub struct WorkQueueInfo {
    pub pwq_nthreads            : u32,     // total number of workqueue threads
    pub pwq_runthreads          : u32,     // total number of running workqueue threads
    pub pwq_blockedthreads      : u32,     // total number of blocked workqueue threads
    pub reserved                : [u32;1]  // reserved for future use
}

impl PIDInfo for WorkQueueInfo {
    fn flavor() -> PidInfoFlavor { PidInfoFlavor::WorkQueueInfo }
}