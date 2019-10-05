extern crate libc;

use crate::libproc::helpers;

use self::libc::{c_void, uint64_t, c_int, uint8_t};

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
///
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

#[repr(C)]
#[derive(Default)]
pub struct RUsageInfoV0 {
    pub ri_uuid              : [uint8_t; 16],
    pub ri_user_time         : uint64_t,
    pub ri_system_time       : uint64_t,
    pub ri_pkg_idle_wkups    : uint64_t,
    pub ri_interrupt_wkups   : uint64_t,
    pub ri_pageins           : uint64_t,
    pub ri_wired_size        : uint64_t,
    pub ri_resident_size     : uint64_t,
    pub ri_phys_footprint    : uint64_t,
    pub ri_proc_start_abstime: uint64_t,
    pub ri_proc_exit_abstime : uint64_t,
}

impl PIDRUsage for RUsageInfoV0 {
    fn flavor() -> PidRUsageFlavor { PidRUsageFlavor::V0 }
}

#[repr(C)]
#[derive(Default)]
pub struct RUsageInfoV1 {
    pub ri_uuid                 : [uint8_t; 16],
    pub ri_user_time            : uint64_t,
    pub ri_system_time          : uint64_t,
    pub ri_pkg_idle_wkups       : uint64_t,
    pub ri_interrupt_wkups      : uint64_t,
    pub ri_pageins              : uint64_t,
    pub ri_wired_size           : uint64_t,
    pub ri_resident_size        : uint64_t,
    pub ri_phys_footprint       : uint64_t,
    pub ri_proc_start_abstime   : uint64_t,
    pub ri_proc_exit_abstime    : uint64_t,
    pub ri_child_user_time      : uint64_t,
    pub ri_child_system_time    : uint64_t,
    pub ri_child_pkg_idle_wkups : uint64_t,
    pub ri_child_interrupt_wkups: uint64_t,
    pub ri_child_pageins        : uint64_t,
    pub ri_child_elapsed_abstime: uint64_t,
}

impl PIDRUsage for RUsageInfoV1 {
    fn flavor() -> PidRUsageFlavor { PidRUsageFlavor::V1 }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct RUsageInfoV2 {
    pub ri_uuid                 : [uint8_t; 16],
    pub ri_user_time            : uint64_t,
    pub ri_system_time          : uint64_t,
    pub ri_pkg_idle_wkups       : uint64_t,
    pub ri_interrupt_wkups      : uint64_t,
    pub ri_pageins              : uint64_t,
    pub ri_wired_size           : uint64_t,
    pub ri_resident_size        : uint64_t,
    pub ri_phys_footprint       : uint64_t,
    pub ri_proc_start_abstime   : uint64_t,
    pub ri_proc_exit_abstime    : uint64_t,
    pub ri_child_user_time      : uint64_t,
    pub ri_child_system_time    : uint64_t,
    pub ri_child_pkg_idle_wkups : uint64_t,
    pub ri_child_interrupt_wkups: uint64_t,
    pub ri_child_pageins        : uint64_t,
    pub ri_child_elapsed_abstime: uint64_t,
    pub ri_diskio_bytesread     : uint64_t,
    pub ri_diskio_byteswritten  : uint64_t,
}

impl PIDRUsage for RUsageInfoV2 {
    fn flavor() -> PidRUsageFlavor { PidRUsageFlavor::V2 }
}

#[repr(C)]
#[derive(Default)]
pub struct RUsageInfoV3 {
    pub ri_uuid                         : [uint8_t; 16],
    pub ri_user_time                    : uint64_t,
    pub ri_system_time                  : uint64_t,
    pub ri_pkg_idle_wkups               : uint64_t,
    pub ri_interrupt_wkups              : uint64_t,
    pub ri_pageins                      : uint64_t,
    pub ri_wired_size                   : uint64_t,
    pub ri_resident_size                : uint64_t,
    pub ri_phys_footprint               : uint64_t,
    pub ri_proc_start_abstime           : uint64_t,
    pub ri_proc_exit_abstime            : uint64_t,
    pub ri_child_user_time              : uint64_t,
    pub ri_child_system_time            : uint64_t,
    pub ri_child_pkg_idle_wkups         : uint64_t,
    pub ri_child_interrupt_wkups        : uint64_t,
    pub ri_child_pageins                : uint64_t,
    pub ri_child_elapsed_abstime        : uint64_t,
    pub ri_diskio_bytesread             : uint64_t,
    pub ri_diskio_byteswritten          : uint64_t,
    pub ri_cpu_time_qos_default         : uint64_t,
    pub ri_cpu_time_qos_maintenance     : uint64_t,
    pub ri_cpu_time_qos_background      : uint64_t,
    pub ri_cpu_time_qos_utility         : uint64_t,
    pub ri_cpu_time_qos_legacy          : uint64_t,
    pub ri_cpu_time_qos_user_initiated  : uint64_t,
    pub ri_cpu_time_qos_user_interactive: uint64_t,
    pub ri_billed_system_time           : uint64_t,
    pub ri_serviced_system_time         : uint64_t,
}

impl PIDRUsage for RUsageInfoV3 {
    fn flavor() -> PidRUsageFlavor { PidRUsageFlavor::V3 }
}

#[repr(C)]
#[derive(Default)]
pub struct RUsageInfoV4 {
    pub ri_uuid                         : [uint8_t; 16],
    pub ri_user_time                    : uint64_t,
    pub ri_system_time                  : uint64_t,
    pub ri_pkg_idle_wkups               : uint64_t,
    pub ri_interrupt_wkups              : uint64_t,
    pub ri_pageins                      : uint64_t,
    pub ri_wired_size                   : uint64_t,
    pub ri_resident_size                : uint64_t,
    pub ri_phys_footprint               : uint64_t,
    pub ri_proc_start_abstime           : uint64_t,
    pub ri_proc_exit_abstime            : uint64_t,
    pub ri_child_user_time              : uint64_t,
    pub ri_child_system_time            : uint64_t,
    pub ri_child_pkg_idle_wkups         : uint64_t,
    pub ri_child_interrupt_wkups        : uint64_t,
    pub ri_child_pageins                : uint64_t,
    pub ri_child_elapsed_abstime        : uint64_t,
    pub ri_diskio_bytesread             : uint64_t,
    pub ri_diskio_byteswritten          : uint64_t,
    pub ri_cpu_time_qos_default         : uint64_t,
    pub ri_cpu_time_qos_maintenance     : uint64_t,
    pub ri_cpu_time_qos_background      : uint64_t,
    pub ri_cpu_time_qos_utility         : uint64_t,
    pub ri_cpu_time_qos_legacy          : uint64_t,
    pub ri_cpu_time_qos_user_initiated  : uint64_t,
    pub ri_cpu_time_qos_user_interactive: uint64_t,
    pub ri_billed_system_time           : uint64_t,
    pub ri_serviced_system_time         : uint64_t,
    pub ri_logical_writes               : uint64_t,
    pub ri_lifetime_max_phys_footprint  : uint64_t,
    pub ri_instructions                 : uint64_t,
    pub ri_cycles                       : uint64_t,
    pub ri_billed_energy                : uint64_t,
    pub ri_serviced_energy              : uint64_t,
    pub ri_interval_max_phys_footprint  : uint64_t,
    pub ri_unused                       : [uint64_t; 1],
}

impl PIDRUsage for RUsageInfoV4 {
    fn flavor() -> PidRUsageFlavor { PidRUsageFlavor::V4 }
}

#[cfg(test)]
mod test {
    use super::pidrusage;
    use super::RUsageInfoV2;

    #[test]
    fn pidrusage_test() {
        use std::process;
        let pid = process::id() as i32;

        let res = pidrusage::<RUsageInfoV2>(pid).unwrap();
        println!("UUID: {:?}, Disk Read: {}, Disk Write: {}", res.ri_uuid, res.ri_diskio_bytesread, res.ri_diskio_byteswritten);
    }
}