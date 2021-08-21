extern crate libc;

use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};
use crate::libproc::bsd_info::BSDInfo;

/// TaskInfo structure see http://opensource.apple.com//source/xnu/xnu-1456.1.26/bsd/sys/proc_info.h
#[repr(C)]
#[derive(Default)]
pub struct TaskInfo {
    /// virtual memory size (bytes)
    pub pti_virtual_size        : u64,
    /// resident memory size (bytes)
    pub pti_resident_size       : u64,
    /// total user time
    pub pti_total_user          : u64,
    /// total system time
    pub pti_total_system        : u64,
    /// existing threads only
    pub pti_threads_user        : u64,
    /// number of system threads
    pub pti_threads_system      : u64,
    /// default policy for new threads
    pub pti_policy              : i32,
    /// number of page faults
    pub pti_faults              : i32,
    /// number of actual pageins
    pub pti_pageins             : i32,
    /// number of copy-on-write faults
    pub pti_cow_faults          : i32,
    /// number of messages sent
    pub pti_messages_sent       : i32,
    /// number of messages received
    pub pti_messages_received   : i32,
    /// number of mach system calls
    pub pti_syscalls_mach       : i32,
    /// number of unix system calls
    pub pti_syscalls_unix       : i32,
    /// number of context switches
    pub pti_csw                 : i32,
    /// number of threads in the task
    pub pti_threadnum           : i32,
    /// number of running threads
    pub pti_numrunning          : i32,
    /// task priority
    pub pti_priority            : i32
}

impl PIDInfo for TaskInfo {
    fn flavor() -> PidInfoFlavor { PidInfoFlavor::TaskInfo }
}

/// Struct for info on all Tasks
#[repr(C)]
#[derive(Default)]
pub struct TaskAllInfo {
    /// BSDInfo
    pub pbsd : BSDInfo,
    /// TaskInfo
    pub ptinfo : TaskInfo
}

impl PIDInfo for TaskAllInfo {
    fn flavor() -> PidInfoFlavor { PidInfoFlavor::TaskAllInfo }
}