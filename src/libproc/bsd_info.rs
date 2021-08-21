extern crate libc;

use self::libc::{c_char, uid_t, gid_t};

use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};

// from http://opensource.apple.com//source/xnu/xnu-1504.7.4/bsd/sys/param.h
const MAXCOMLEN	: usize = 16;

/// Struct for BSDInfo about a process
#[repr(C)]
#[derive(Default)]
pub struct BSDInfo {
    /// 64bit; emulated etc
    pub pbi_flags               : u32,
    /// status
    pub pbi_status              : u32,
    /// x status
    pub pbi_xstatus             : u32,
    /// PID
    pub pbi_pid                 : u32,
    /// PPID
    pub pbi_ppid                : u32,
    /// UID - User ID
    pub pbi_uid                 : uid_t,
    /// GID - Group ID
    pub pbi_gid                 : gid_t,
    /// RUID
    pub pbi_ruid                : uid_t,
    /// RGID
    pub pbi_rgid                : gid_t,
    /// RGID
    pub pbi_svuid               : uid_t,
    /// SVUID
    pub pbi_svgid               : gid_t,
    /// reserved for future use
    pub rfu_1                   : u32,
    /// Comm
    pub pbi_comm                : [c_char; MAXCOMLEN],
    /// empty if no name is registered
    pub pbi_name                : [c_char; 2 * MAXCOMLEN],
    /// nfile - Number of files
    pub pbi_nfiles              : u32,
    /// PGID
    pub pbi_pgid                : u32,
    /// PJOBC
    pub pbi_pjobc               : u32,
    /// controlling tty dev
    pub e_tdev                  : u32,
    /// tty process group id
    pub e_tpgid                 : u32,
    /// Nice
    pub pbi_nice                : u32,
    /// Start tv sec
    pub pbi_start_tvsec         : u64,
    /// Start tv micro sec
    pub pbi_start_tvusec        : u64
}

impl PIDInfo for BSDInfo {
    fn flavor() -> PidInfoFlavor { PidInfoFlavor::TBSDInfo }
}