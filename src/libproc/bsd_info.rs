extern crate libc;

use self::libc::{uint32_t, int32_t, uint64_t, c_char, uid_t, gid_t};

use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};

// from http://opensource.apple.com//source/xnu/xnu-1504.7.4/bsd/sys/param.h
const MAXCOMLEN	: usize = 16;

#[repr(C)]
#[derive(Default)]
pub struct BSDInfo {
    pub pbi_flags               : uint32_t,                 // 64bit; emulated etc
    pub pbi_status              : uint32_t,
    pub pbi_xstatus             : uint32_t,
    pub pbi_pid                 : uint32_t,
    pub pbi_ppid                : uint32_t,
    pub pbi_uid                 : uid_t,
    pub pbi_gid                 : gid_t,
    pub pbi_ruid                : uid_t,
    pub pbi_rgid                : gid_t,
    pub pbi_svuid               : uid_t,
    pub pbi_svgid               : gid_t,
    pub rfu_1                   : uint32_t,                 // reserved
    pub pbi_comm                : [c_char; MAXCOMLEN],
    pub pbi_name                : [c_char; 2 * MAXCOMLEN],  // empty if no name is registered
    pub pbi_nfiles              : uint32_t,
    pub pbi_pgid                : uint32_t,
    pub pbi_pjobc               : uint32_t,
    pub e_tdev                  : uint32_t,                 // controlling tty dev
    pub e_tpgid                 : uint32_t,                 // tty process group id
    pub pbi_nice                : int32_t,
    pub pbi_start_tvsec         : uint64_t,
    pub pbi_start_tvusec        : uint64_t
}

impl PIDInfo for BSDInfo {
    fn flavor() -> PidInfoFlavor { PidInfoFlavor::TBSDInfo }
}