extern crate libc;

use crate::libproc::helpers;

use self::libc::{c_void, c_int};

// This trait is needed for polymorphism on pidrusage types, also abstracting flavor in order to provide
// type-guaranteed flavor correctness
pub trait PIDRUsage: Default {
    fn flavor() -> PidRUsageFlavor;
}

// From https://opensource.apple.com/source/xnu/xnu-4903.221.2/bsd/sys/resource.h
pub enum PidRUsageFlavor {
    V0 = 0,
    V1 = 1,
    V2 = 2,
    V3 = 3,
    V4 = 4,
}

// this extern block links to the libproc library
// Original signatures of functions can be found at http://opensource.apple.com/source/Libc/Libc-594.9.4/darwin/libproc.c
#[cfg(target_os = "macos")]
#[link(name = "proc", kind = "dylib")]
extern {
    fn proc_pid_rusage(pid: c_int, flavor: c_int, buffer: *mut c_void) -> c_int;
}

/// Returns the information about resources of the process that match pid passed in.
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::pid_rusage::{pidrusage, RUsageInfoV2};
///
/// fn pidrusage_test() {
///     use std::process;
///     let pid = process::id() as i32;
///
///     if let Ok(res) = pidrusage::<RUsageInfoV2>(pid) {
///         println!("UUID: {:?}, Disk Read: {}, Disk Write: {}", res.ri_uuid, res.ri_diskio_bytesread, res.ri_diskio_byteswritten);
///     }
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn pidrusage<T: PIDRUsage>(pid : i32) -> Result<T, String> {
    let flavor = T::flavor() as i32;
    let mut pidrusage = T::default();
    let buffer_ptr = &mut pidrusage as *mut _ as *mut c_void;
    let ret: i32;

    unsafe {
        ret = proc_pid_rusage(pid, flavor, buffer_ptr);
    };

    if ret < 0 {
        Err(helpers::get_errno_with_message(ret))
    } else {
        Ok(pidrusage)
    }
}

#[cfg(not(target_os = "macos"))]
pub fn pidrusage<T: PIDRUsage>(pid : i32) -> Result<T, String> {
    unimplemented!()
}

#[repr(C)]
#[derive(Default)]
pub struct RUsageInfoV0 {
    pub ri_uuid              : [u8; 16],
    pub ri_user_time         : u64,
    pub ri_system_time       : u64,
    pub ri_pkg_idle_wkups    : u64,
    pub ri_interrupt_wkups   : u64,
    pub ri_pageins           : u64,
    pub ri_wired_size        : u64,
    pub ri_resident_size     : u64,
    pub ri_phys_footprint    : u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime : u64,
}

impl PIDRUsage for RUsageInfoV0 {
    fn flavor() -> PidRUsageFlavor { PidRUsageFlavor::V0 }
}

#[repr(C)]
#[derive(Default)]
pub struct RUsageInfoV1 {
    pub ri_uuid                 : [u8; 16],
    pub ri_user_time            : u64,
    pub ri_system_time          : u64,
    pub ri_pkg_idle_wkups       : u64,
    pub ri_interrupt_wkups      : u64,
    pub ri_pageins              : u64,
    pub ri_wired_size           : u64,
    pub ri_resident_size        : u64,
    pub ri_phys_footprint       : u64,
    pub ri_proc_start_abstime   : u64,
    pub ri_proc_exit_abstime    : u64,
    pub ri_child_user_time      : u64,
    pub ri_child_system_time    : u64,
    pub ri_child_pkg_idle_wkups : u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins        : u64,
    pub ri_child_elapsed_abstime: u64,
}

impl PIDRUsage for RUsageInfoV1 {
    fn flavor() -> PidRUsageFlavor { PidRUsageFlavor::V1 }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct RUsageInfoV2 {
    pub ri_uuid                 : [u8; 16],
    pub ri_user_time            : u64,
    pub ri_system_time          : u64,
    pub ri_pkg_idle_wkups       : u64,
    pub ri_interrupt_wkups      : u64,
    pub ri_pageins              : u64,
    pub ri_wired_size           : u64,
    pub ri_resident_size        : u64,
    pub ri_phys_footprint       : u64,
    pub ri_proc_start_abstime   : u64,
    pub ri_proc_exit_abstime    : u64,
    pub ri_child_user_time      : u64,
    pub ri_child_system_time    : u64,
    pub ri_child_pkg_idle_wkups : u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins        : u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread     : u64,
    pub ri_diskio_byteswritten  : u64,
}

impl PIDRUsage for RUsageInfoV2 {
    fn flavor() -> PidRUsageFlavor { PidRUsageFlavor::V2 }
}

#[repr(C)]
#[derive(Default)]
pub struct RUsageInfoV3 {
    pub ri_uuid                         : [u8; 16],
    pub ri_user_time                    : u64,
    pub ri_system_time                  : u64,
    pub ri_pkg_idle_wkups               : u64,
    pub ri_interrupt_wkups              : u64,
    pub ri_pageins                      : u64,
    pub ri_wired_size                   : u64,
    pub ri_resident_size                : u64,
    pub ri_phys_footprint               : u64,
    pub ri_proc_start_abstime           : u64,
    pub ri_proc_exit_abstime            : u64,
    pub ri_child_user_time              : u64,
    pub ri_child_system_time            : u64,
    pub ri_child_pkg_idle_wkups         : u64,
    pub ri_child_interrupt_wkups        : u64,
    pub ri_child_pageins                : u64,
    pub ri_child_elapsed_abstime        : u64,
    pub ri_diskio_bytesread             : u64,
    pub ri_diskio_byteswritten          : u64,
    pub ri_cpu_time_qos_default         : u64,
    pub ri_cpu_time_qos_maintenance     : u64,
    pub ri_cpu_time_qos_background      : u64,
    pub ri_cpu_time_qos_utility         : u64,
    pub ri_cpu_time_qos_legacy          : u64,
    pub ri_cpu_time_qos_user_initiated  : u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time           : u64,
    pub ri_serviced_system_time         : u64,
}

impl PIDRUsage for RUsageInfoV3 {
    fn flavor() -> PidRUsageFlavor { PidRUsageFlavor::V3 }
}

#[repr(C)]
#[derive(Default)]
pub struct RUsageInfoV4 {
    pub ri_uuid                         : [u8; 16],
    pub ri_user_time                    : u64,
    pub ri_system_time                  : u64,
    pub ri_pkg_idle_wkups               : u64,
    pub ri_interrupt_wkups              : u64,
    pub ri_pageins                      : u64,
    pub ri_wired_size                   : u64,
    pub ri_resident_size                : u64,
    pub ri_phys_footprint               : u64,
    pub ri_proc_start_abstime           : u64,
    pub ri_proc_exit_abstime            : u64,
    pub ri_child_user_time              : u64,
    pub ri_child_system_time            : u64,
    pub ri_child_pkg_idle_wkups         : u64,
    pub ri_child_interrupt_wkups        : u64,
    pub ri_child_pageins                : u64,
    pub ri_child_elapsed_abstime        : u64,
    pub ri_diskio_bytesread             : u64,
    pub ri_diskio_byteswritten          : u64,
    pub ri_cpu_time_qos_default         : u64,
    pub ri_cpu_time_qos_maintenance     : u64,
    pub ri_cpu_time_qos_background      : u64,
    pub ri_cpu_time_qos_utility         : u64,
    pub ri_cpu_time_qos_legacy          : u64,
    pub ri_cpu_time_qos_user_initiated  : u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time           : u64,
    pub ri_serviced_system_time         : u64,
    pub ri_logical_writes               : u64,
    pub ri_lifetime_max_phys_footprint  : u64,
    pub ri_instructions                 : u64,
    pub ri_cycles                       : u64,
    pub ri_billed_energy                : u64,
    pub ri_serviced_energy              : u64,
    pub ri_interval_max_phys_footprint  : u64,
    pub ri_unused                       : [u64; 1],
}

impl PIDRUsage for RUsageInfoV4 {
    fn flavor() -> PidRUsageFlavor { PidRUsageFlavor::V4 }
}

#[cfg(test)]
mod test {
    use super::pidrusage;
    use super::RUsageInfoV2;

    #[cfg(target_os = "macos")]
    #[test]
    fn pidrusage_test() {
        use std::process;
        let pid = process::id() as i32;

        let res = pidrusage::<RUsageInfoV2>(pid).unwrap();
        println!("UUID: {:?}, Disk Read: {}, Disk Write: {}", res.ri_uuid, res.ri_diskio_bytesread, res.ri_diskio_byteswritten);
    }
}