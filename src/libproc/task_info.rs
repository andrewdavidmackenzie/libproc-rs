extern crate libc;

use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};
use crate::libproc::bsd_info::BSDInfo;

// structures from http://opensource.apple.com//source/xnu/xnu-1456.1.26/bsd/sys/proc_info.h
#[repr(C)]
#[derive(Default)]
pub struct TaskInfo {
    pub pti_virtual_size        : u64,     // virtual memory size (bytes)
    pub pti_resident_size       : u64,     // resident memory size (bytes)
    pub pti_total_user          : u64,     // total time
    pub pti_total_system        : u64,
    pub pti_threads_user        : u64,     // existing threads only
    pub pti_threads_system      : u64,
    pub pti_policy              : i32,      // default policy for new threads
    pub pti_faults              : i32,      // number of page faults
    pub pti_pageins             : i32,      // number of actual pageins
    pub pti_cow_faults          : i32,      // number of copy-on-write faults
    pub pti_messages_sent       : i32,      // number of messages sent
    pub pti_messages_received   : i32,      // number of messages received
    pub pti_syscalls_mach       : i32,      // number of mach system calls
    pub pti_syscalls_unix       : i32,      // number of unix system calls
    pub pti_csw                 : i32,      // number of context switches
    pub pti_threadnum           : i32,      // number of threads in the task
    pub pti_numrunning          : i32,      // number of running threads
    pub pti_priority            : i32       // task priority
}

impl PIDInfo for TaskInfo {
    fn flavor() -> PidInfoFlavor { PidInfoFlavor::TaskInfo }
}

#[repr(C)]
#[derive(Default)]
pub struct TaskAllInfo {
    pub pbsd : BSDInfo,
    pub ptinfo : TaskInfo
}

impl PIDInfo for TaskAllInfo {
    fn flavor() -> PidInfoFlavor { PidInfoFlavor::TaskAllInfo }
}