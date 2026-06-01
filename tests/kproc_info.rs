#![cfg(target_os = "macos")]
#![allow(clippy::cast_possible_wrap)]

//! Tests for the sysctl-based `kproc_info` API, which works for PID 0
//! (`kernel_task`) where `proc_pidinfo` fails, and without requiring root.

use libproc::libproc::proc_pid::{kproc_info, kproc_info_raw};

#[test]
fn kproc_info_pid0_is_kernel_task() {
    let info = kproc_info(0).expect("kproc_info(0) should succeed for kernel_task");
    assert_eq!(info.pid, 0);
    assert!(
        info.comm.starts_with("kernel_task"),
        "PID 0 comm should be kernel_task, got {:?}",
        info.comm
    );
}

#[test]
fn kproc_info_raw_pid0() {
    let raw = kproc_info_raw(0).expect("raw sysctl for PID 0 should succeed");
    assert_eq!(raw.kp_proc.p_pid, 0);
}

#[test]
fn kproc_info_self() {
    let me = std::process::id() as i32;
    let info = kproc_info(me).expect("kproc_info for self should succeed");
    assert_eq!(info.pid, me);
    assert!(!info.comm.is_empty(), "self comm should not be empty");
    // The test process always has a parent, so a correctly-read `eproc` layout
    // yields a non-zero ppid. (A wrong struct layout typically reads this as 0.)
    assert!(info.ppid > 0, "self should have a parent, got ppid {}", info.ppid);
    // Real UID should match what the OS reports for this process.
    let expected_uid = unsafe { libc::getuid() };
    assert_eq!(info.ruid, expected_uid, "ruid should match getuid()");
}

#[test]
fn kproc_info_nonexistent_pid_errors() {
    // A very high PID that should not exist.
    assert!(
        kproc_info(0x7FFF_FFFE).is_err(),
        "a non-existent PID should return an error"
    );
}
