use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};

/// Structure for work queue items
#[derive(Default)]
pub struct WorkQueueInfo {
    /// total number of workqueue threads
    pub pwq_nthreads: u32,
    /// total number of running workqueue threads
    pub pwq_runthreads: u32,
    /// total number of blocked workqueue threads
    pub pwq_blockedthreads: u32,
    /// reserved for future use
    pub reserved: [u32; 1],
}

/// # Safety
///
/// `WorkQueueInfo` is the right size to be passed to `proc_pidinfo`.
unsafe impl PIDInfo for WorkQueueInfo {
    fn flavor() -> PidInfoFlavor {
        PidInfoFlavor::WorkQueueInfo
    }
}
