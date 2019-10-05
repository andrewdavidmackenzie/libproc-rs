extern crate libc;

use self::libc::{c_char, uid_t, gid_t};

use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};

// from http://opensource.apple.com//source/xnu/xnu-1504.7.4/bsd/sys/param.h
const MAXCOMLEN	: usize = 16;

#[repr(C)]
#[derive(Default)]
pub struct BSDInfo {
    pub pbi_flags               : u32,                 // 64bit; emulated etc
    pub pbi_status              : u32,
    pub pbi_xstatus             : u32,
    pub pbi_pid                 : u32,
    pub pbi_ppid                : u32,
    pub pbi_uid                 : uid_t,
    pub pbi_gid                 : gid_t,
    pub pbi_ruid                : uid_t,
    pub pbi_rgid                : gid_t,
    pub pbi_svuid               : uid_t,
    pub pbi_svgid               : gid_t,
    pub rfu_1                   : u32,                 // reserved
    pub pbi_comm                : [c_char; MAXCOMLEN],
    pub pbi_name                : [c_char; 2 * MAXCOMLEN],  // empty if no name is registered
    pub pbi_nfiles              : u32,
    pub pbi_pgid                : u32,
    pub pbi_pjobc               : u32,
    pub e_tdev                  : u32,                 // controlling tty dev
    pub e_tpgid                 : u32,                 // tty process group id
    pub pbi_nice                : u32,
    pub pbi_start_tvsec         : u64,
    pub pbi_start_tvusec        : u64
}

impl PIDInfo for BSDInfo {
    fn flavor() -> PidInfoFlavor { PidInfoFlavor::TBSDInfo }
}