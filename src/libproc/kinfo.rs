//! `kinfo` module
//!
//! Process information obtained via `sysctl(KERN_PROC_PID)`. Unlike the
//! `proc_pidinfo`-based APIs, this works for PID 0 (`kernel_task`) and does not
//! require root for basic process information.
//!
//! Two types are provided:
//! - [`KProcInfo`](crate::libproc::kinfo::KProcInfo) — a friendly, owned wrapper
//!   with named scalar fields.
//! - [`KinfoProc`](crate::libproc::kinfo::KinfoProc) — a faithful `#[repr(C)]`
//!   mirror of the macOS `kinfo_proc` structure (`struct extern_proc` +
//!   `struct eproc`) for callers that need the raw kernel data.
//!
//! The raw structures are defined here (rather than relying on the `libc` crate)
//! because `libc` no longer exposes `kinfo_proc` for Apple targets. They mirror
//! the layout in `<sys/sysctl.h>` exactly so that `sysctl` fills them correctly.
//!
//! See [`crate::libproc::proc_pid::kproc_info`] and
//! [`crate::libproc::proc_pid::kproc_info_raw`].

use std::os::raw::c_void;

// Raw `#[repr(C)]` mirrors of the macOS kernel structures. Field names, types and
// order match `<sys/sysctl.h>` (as captured from the verified Apple definitions).
// Most fields are never read by consumers, so the raw structs are exempted from
// `missing_docs`, mirroring how this crate treats its other FFI bindings.

/// `struct timeval` (mirrored to avoid depending on a `libc` alias).
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Timeval {
    pub tv_sec: i64,
    pub tv_usec: i32,
}

/// `struct itimerval` (mirrored).
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Itimerval {
    pub it_interval: Timeval,
    pub it_value: Timeval,
}

/// Mirror of macOS `struct extern_proc` (the `kp_proc` member of `kinfo_proc`).
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternProc {
    // `union p_un` from the system header (process start time / scheduling links).
    // Represented by its 16-byte, 8-byte-aligned `timeval` member; never read here.
    pub p_un: Timeval,
    pub p_vmspace: *mut c_void,
    pub p_sigacts: usize,
    pub p_flag: i32,
    pub p_stat: i8,
    pub p_pid: i32,
    pub p_oppid: i32,
    pub p_dupfd: i32,
    pub user_stack: *mut c_void,
    pub exit_thread: *mut c_void,
    pub p_debugger: i32,
    pub sigwait: i32,
    pub p_estcpu: u32,
    pub p_cpticks: i32,
    pub p_pctcpu: u32,
    pub p_wchan: *mut c_void,
    pub p_wmesg: *mut c_void,
    pub p_swtime: u32,
    pub p_slptime: u32,
    pub p_realtimer: Itimerval,
    pub p_rtime: Timeval,
    pub p_uticks: u64,
    pub p_sticks: u64,
    pub p_iticks: u64,
    pub p_traceflag: i32,
    pub p_tracep: *mut c_void,
    pub p_siglist: i32,
    pub p_textvp: *mut c_void,
    pub p_holdcnt: i32,
    pub p_sigmask: u32,
    pub p_sigignore: u32,
    pub p_sigcatch: u32,
    pub p_priority: u8,
    pub p_usrpri: u8,
    pub p_nice: i8,
    pub p_comm: [i8; 17],
    pub p_pgrp: *mut c_void,
    pub p_addr: *mut c_void,
    pub p_xstat: u16,
    pub p_acflag: u16,
    pub p_ru: *mut c_void,
}

/// Mirror of macOS `struct _pcred` (process credentials).
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Pcred {
    pub pc_lock: [i8; 72],
    pub pc_ucred: *mut c_void,
    pub p_ruid: u32,
    pub p_svuid: u32,
    pub p_rgid: u32,
    pub p_svgid: u32,
    pub p_refcnt: i32,
}

/// Mirror of macOS `struct _ucred` (user credentials).
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ucred {
    pub cr_ref: i32,
    pub cr_uid: u32,
    pub cr_ngroups: i16,
    pub cr_groups: [u32; 16],
}

/// Mirror of macOS `struct vmspace` (opaque placeholder, as in the system header).
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmspace {
    pub dummy: i32,
    pub dummy2: *mut c_void,
    pub dummy3: [i32; 5],
    pub dummy4: [*mut c_void; 3],
}

/// Mirror of macOS `struct eproc` (the `kp_eproc` member of `kinfo_proc`).
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Eproc {
    pub e_paddr: *mut c_void,
    pub e_sess: *mut c_void,
    pub e_pcred: Pcred,
    pub e_ucred: Ucred,
    pub e_vm: Vmspace,
    pub e_ppid: i32,
    pub e_pgid: i32,
    pub e_jobc: i16,
    pub e_tdev: i32,
    pub e_tpgid: i32,
    pub e_tsess: *mut c_void,
    pub e_wmesg: [i8; 8],
    pub e_xsize: i32,
    pub e_xrssize: i16,
    pub e_xccount: i16,
    pub e_xswrss: i16,
    pub e_flag: i32,
    pub e_login: [i8; 12],
    pub e_spare: [i32; 4],
}

/// Faithful `#[repr(C)]` mirror of the macOS `kinfo_proc` structure returned by
/// `sysctl(KERN_PROC_PID)`.
///
/// Use [`crate::libproc::proc_pid::kproc_info`] for a friendly wrapper, or
/// [`crate::libproc::proc_pid::kproc_info_raw`] to obtain this raw structure.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct KinfoProc {
    pub kp_proc: ExternProc,
    pub kp_eproc: Eproc,
}

/// Friendly, owned process information obtained via `sysctl(KERN_PROC_PID)`.
///
/// Unlike the `proc_pidinfo`-based [`crate::libproc::bsd_info::BSDInfo`], this
/// works for PID 0 (`kernel_task`) and other processes that `proc_pidinfo`
/// cannot report.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct KProcInfo {
    /// Process ID (`kp_proc.p_pid`)
    pub pid: i32,
    /// Parent process ID (`kp_eproc.e_ppid`)
    pub ppid: i32,
    /// Process group ID (`kp_eproc.e_pgid`)
    pub pgid: i32,
    /// Real user ID (`kp_eproc.e_pcred.p_ruid`)
    pub ruid: u32,
    /// Nice value (`kp_proc.p_nice`)
    pub nice: i32,
    /// Process state / status code (`kp_proc.p_stat`)
    pub status: i32,
    /// Scheduling priority (`kp_proc.p_priority`)
    pub priority: i32,
    /// Short command name (`kp_proc.p_comm`), NUL-trimmed
    pub comm: String,
    /// Process flags (`kp_proc.p_flag`)
    pub flags: i32,
}

impl From<&KinfoProc> for KProcInfo {
    fn from(k: &KinfoProc) -> Self {
        KProcInfo {
            pid: k.kp_proc.p_pid,
            ppid: k.kp_eproc.e_ppid,
            pgid: k.kp_eproc.e_pgid,
            ruid: k.kp_eproc.e_pcred.p_ruid,
            nice: i32::from(k.kp_proc.p_nice),
            status: i32::from(k.kp_proc.p_stat),
            priority: i32::from(k.kp_proc.p_priority),
            comm: c_chars_to_string(&k.kp_proc.p_comm),
            flags: k.kp_proc.p_flag,
        }
    }
}

/// Convert a NUL-terminated C `char` buffer (`i8`) to a `String` (lossy UTF-8).
fn c_chars_to_string(buf: &[i8]) -> String {
    let bytes: Vec<u8> = buf
        .iter()
        .take_while(|&&c| c != 0)
        .map(|&c| c.to_le_bytes()[0])
        .collect();
    String::from_utf8_lossy(&bytes).into_owned()
}

#[cfg(test)]
mod layout_tests {
    use super::{Itimerval, Timeval};
    use std::mem::{align_of, offset_of, size_of};

    // The mirrored `Timeval` / `Itimerval` must match the system's `struct timeval`
    // / `struct itimerval` exactly, otherwise every field after `p_un` / `p_realtimer`
    // in `extern_proc` is read at the wrong offset and `sysctl` results are garbage.
    // `libc` exposes these two types on macOS (unlike `kinfo_proc`), so we use them
    // as the authoritative reference.

    #[test]
    fn timeval_matches_libc() {
        assert_eq!(
            size_of::<Timeval>(),
            size_of::<libc::timeval>(),
            "Timeval size must match libc::timeval"
        );
        assert_eq!(
            align_of::<Timeval>(),
            align_of::<libc::timeval>(),
            "Timeval alignment must match libc::timeval"
        );
        assert_eq!(offset_of!(Timeval, tv_sec), 0);
        assert_eq!(
            offset_of!(Timeval, tv_usec),
            size_of::<libc::time_t>(),
            "tv_usec must follow the full-width tv_sec"
        );
    }

    #[test]
    fn itimerval_matches_libc() {
        assert_eq!(
            size_of::<Itimerval>(),
            size_of::<libc::itimerval>(),
            "Itimerval size must match libc::itimerval"
        );
        assert_eq!(align_of::<Itimerval>(), align_of::<libc::itimerval>());
        assert_eq!(offset_of!(Itimerval, it_interval), 0);
        assert_eq!(
            offset_of!(Itimerval, it_value),
            size_of::<Timeval>(),
            "it_value must follow it_interval"
        );
    }
}
