extern crate libc;

use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};

use self::libc::{c_char, int32_t, uint64_t};

// from http://opensource.apple.com//source/xnu/xnu-1456.1.26/bsd/sys/proc_info.h
const MAXTHREADNAMESIZE : usize = 64;

#[repr(C)]
pub struct ThreadInfo {
    pub pth_user_time           : uint64_t,                     // user run time
    pub pth_system_time         : uint64_t,                     // system run time
    pub pth_cpu_usage           : int32_t,                      // scaled cpu usage percentage
    pub pth_policy              : int32_t,                      // scheduling policy in effect
    pub pth_run_state           : int32_t,                      // run state (see below)
    pub pth_flags               : int32_t,                      // various flags (see below)
    pub pth_sleep_time          : int32_t,                      // number of seconds that thread
    pub pth_curpri              : int32_t,                      // cur priority
    pub pth_priority            : int32_t,                      // priority
    pub pth_maxpriority         : int32_t,                      // max priority
    pub pth_name                : [c_char; MAXTHREADNAMESIZE]   // thread name, if any
}

impl PIDInfo for ThreadInfo {
    fn flavor() -> PidInfoFlavor { PidInfoFlavor::ThreadInfo }
}

impl Default for ThreadInfo {
    fn default() -> ThreadInfo {
        ThreadInfo {
            pth_user_time  : 0,
            pth_system_time: 0,
            pth_cpu_usage  : 0,
            pth_policy     : 0,
            pth_run_state  : 0,
            pth_flags      : 0,
            pth_sleep_time : 0,
            pth_curpri     : 0,
            pth_priority   : 0,
            pth_maxpriority: 0,
            pth_name       : [0; MAXTHREADNAMESIZE],
        }
    }
}