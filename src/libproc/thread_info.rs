extern crate libc;

use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};

use self::libc::{c_char};

// from http://opensource.apple.com//source/xnu/xnu-1456.1.26/bsd/sys/proc_info.h
const MAXTHREADNAMESIZE : usize = 64;

#[repr(C)]
pub struct ThreadInfo {
    pub pth_user_time           : u64,                     // user run time
    pub pth_system_time         : u64,                     // system run time
    pub pth_cpu_usage           : i32,                      // scaled cpu usage percentage
    pub pth_policy              : i32,                      // scheduling policy in effect
    pub pth_run_state           : i32,                      // run state (see below)
    pub pth_flags               : i32,                      // various flags (see below)
    pub pth_sleep_time          : i32,                      // number of seconds that thread
    pub pth_curpri              : i32,                      // cur priority
    pub pth_priority            : i32,                      // priority
    pub pth_maxpriority         : i32,                      // max priority
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