extern crate libc;

use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};
use crate::libproc::bsd_info::BSDInfo;

use self::libc::{int32_t, uint64_t};

// structures from http://opensource.apple.com//source/xnu/xnu-1456.1.26/bsd/sys/proc_info.h
#[repr(C)]
#[derive(Default)]
pub struct TaskInfo {
    pub pti_virtual_size        : uint64_t,     // virtual memory size (bytes)
    pub pti_resident_size       : uint64_t,     // resident memory size (bytes)
    pub pti_total_user          : uint64_t,     // total time
    pub pti_total_system        : uint64_t,
    pub pti_threads_user        : uint64_t,     // existing threads only
    pub pti_threads_system      : uint64_t,
    pub pti_policy              : int32_t,      // default policy for new threads
    pub pti_faults              : int32_t,      // number of page faults
    pub pti_pageins             : int32_t,      // number of actual pageins
    pub pti_cow_faults          : int32_t,      // number of copy-on-write faults
    pub pti_messages_sent       : int32_t,      // number of messages sent
    pub pti_messages_received   : int32_t,      // number of messages received
    pub pti_syscalls_mach       : int32_t,      // number of mach system calls
    pub pti_syscalls_unix       : int32_t,      // number of unix system calls
    pub pti_csw                 : int32_t,      // number of context switches
    pub pti_threadnum           : int32_t,      // number of threads in the task
    pub pti_numrunning          : int32_t,      // number of running threads
    pub pti_priority            : int32_t       // task priority
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