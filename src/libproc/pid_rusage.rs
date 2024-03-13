#[cfg(target_os = "macos")]
use crate::libproc::helpers;

#[cfg(target_os = "macos")]
use libc::c_void;

#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
use crate::libproc::helpers::{parse_memory_string, procfile_field};
#[cfg(target_os = "macos")]
use crate::osx_libproc_bindings::proc_pid_rusage;

/// The `PIDRUsage` trait is needed for polymorphism on pidrusage types, also abstracting flavor in order to provide
/// type-guaranteed flavor correctness
pub trait PIDRUsage: Default {
    /// Return the `PidRUsageFlavor` for the implementing struct
    fn flavor() -> PidRUsageFlavor;
    /// Memory used in bytes
    fn memory_used(&self) -> u64;
    /// Memory used in bytes
    fn set_memory_used(&mut self, used: u64);
}

/// `PidRUsageFlavor` From <https://opensource.apple.com/source/xnu/xnu-4903.221.2/bsd/sys/resource.h>
pub enum PidRUsageFlavor {
    /// Version 0
    V0 = 0,
    /// Version 1
    V1 = 1,
    /// Version 2
    V2 = 2,
    /// Version 3
    V3 = 3,
    /// Version 4
    V4 = 4,
}

/// C struct for Resource Usage Version 0
#[repr(C)]
#[derive(Default)]
pub struct RUsageInfoV0 {
    /// Unique user id
    pub ri_uuid: [u8; 16],
    /// User time used
    pub ri_user_time: u64,
    /// System time used
    pub ri_system_time: u64,
    /// Wakeups from idle
    pub ri_pkg_idle_wkups: u64,
    /// Interrupt wakeups
    pub ri_interrupt_wkups: u64,
    /// Number of pageins
    pub ri_pageins: u64,
    /// Wired size
    pub ri_wired_size: u64,
    /// Resident size
    pub ri_resident_size: u64,
    /// Physical footprint
    pub ri_phys_footprint: u64,
    /// Process start time
    pub ri_proc_start_abstime: u64,
    /// Process exit time
    pub ri_proc_exit_abstime: u64,
}

impl PIDRUsage for RUsageInfoV0 {
    fn flavor() -> PidRUsageFlavor {
        PidRUsageFlavor::V0
    }

    fn memory_used(&self) -> u64 {
        self.ri_resident_size
    }

    fn set_memory_used(&mut self, used: u64) {
        self.ri_resident_size = used;
    }
}

/// C struct for Resource Usage Version 1
#[repr(C)]
#[derive(Default)]
pub struct RUsageInfoV1 {
    /// Unique user id
    pub ri_uuid: [u8; 16],
    /// User time used
    pub ri_user_time: u64,
    /// System time used
    pub ri_system_time: u64,
    /// Wakeups from idle
    pub ri_pkg_idle_wkups: u64,
    /// Interrupt wakeups
    pub ri_interrupt_wkups: u64,
    /// Number of pageins
    pub ri_pageins: u64,
    /// Wired size
    pub ri_wired_size: u64,
    /// Resident size
    pub ri_resident_size: u64,
    /// Physical footprint
    pub ri_phys_footprint: u64,
    /// Process start time
    pub ri_proc_start_abstime: u64,
    /// Process exit time
    pub ri_proc_exit_abstime: u64,
    /// Child user time
    pub ri_child_user_time: u64,
    /// Child system time
    pub ri_child_system_time: u64,
    /// Child wakeups from idle
    pub ri_child_pkg_idle_wkups: u64,
    /// Child interrupt wakeups
    pub ri_child_interrupt_wkups: u64,
    /// Child pageins
    pub ri_child_pageins: u64,
    /// Child elapse time
    pub ri_child_elapsed_abstime: u64,
}

impl PIDRUsage for RUsageInfoV1 {
    fn flavor() -> PidRUsageFlavor {
        PidRUsageFlavor::V1
    }

    fn memory_used(&self) -> u64 {
        self.ri_resident_size
    }

    fn set_memory_used(&mut self, used: u64) {
        self.ri_resident_size = used;
    }
}

/// C struct for Resource Usage Version 2
#[repr(C)]
#[derive(Debug, Default)]
pub struct RUsageInfoV2 {
    /// Unique user id
    pub ri_uuid: [u8; 16],
    /// User time used
    pub ri_user_time: u64,
    /// System time used
    pub ri_system_time: u64,
    /// Wakeups from idle
    pub ri_pkg_idle_wkups: u64,
    /// Interrupt wakeups
    pub ri_interrupt_wkups: u64,
    /// Number of pageins
    pub ri_pageins: u64,
    /// Wired size
    pub ri_wired_size: u64,
    /// Resident size
    pub ri_resident_size: u64,
    /// Physical footprint
    pub ri_phys_footprint: u64,
    /// Process start time
    pub ri_proc_start_abstime: u64,
    /// Process exit time
    pub ri_proc_exit_abstime: u64,
    /// Child user time
    pub ri_child_user_time: u64,
    /// Child system time
    pub ri_child_system_time: u64,
    /// Child wakeups from idle
    pub ri_child_pkg_idle_wkups: u64,
    /// Child interrupt wakeups
    pub ri_child_interrupt_wkups: u64,
    /// Child pageins
    pub ri_child_pageins: u64,
    /// Child elapse time
    pub ri_child_elapsed_abstime: u64,
    /// Disk IO bytes read
    pub ri_diskio_bytesread: u64,
    /// Disk IO bytes written
    pub ri_diskio_byteswritten: u64,
}

impl PIDRUsage for RUsageInfoV2 {
    fn flavor() -> PidRUsageFlavor {
        PidRUsageFlavor::V2
    }

    fn memory_used(&self) -> u64 {
        self.ri_resident_size
    }

    fn set_memory_used(&mut self, used: u64) {
        self.ri_resident_size = used;
    }
}

/// C struct for Resource Usage Version 3
#[repr(C)]
#[derive(Default)]
pub struct RUsageInfoV3 {
    /// Unique user id
    pub ri_uuid: [u8; 16],
    /// User time used
    pub ri_user_time: u64,
    /// System time used
    pub ri_system_time: u64,
    /// Wakeups from idle
    pub ri_pkg_idle_wkups: u64,
    /// Interrupt wakeups
    pub ri_interrupt_wkups: u64,
    /// Number of pageins
    pub ri_pageins: u64,
    /// Wired size
    pub ri_wired_size: u64,
    /// Resident size
    pub ri_resident_size: u64,
    /// Physical footprint
    pub ri_phys_footprint: u64,
    /// Process start time
    pub ri_proc_start_abstime: u64,
    /// Process exit time
    pub ri_proc_exit_abstime: u64,
    /// Child user time
    pub ri_child_user_time: u64,
    /// Child system time
    pub ri_child_system_time: u64,
    /// Child wakeups from idle
    pub ri_child_pkg_idle_wkups: u64,
    /// Child interrupt wakeups
    pub ri_child_interrupt_wkups: u64,
    /// Child pageins
    pub ri_child_pageins: u64,
    /// Child elapse time
    pub ri_child_elapsed_abstime: u64,
    /// Disk IO bytes read
    pub ri_diskio_bytesread: u64,
    /// Disk IO bytes written
    pub ri_diskio_byteswritten: u64,
    /// CPU time QOS default
    pub ri_cpu_time_qos_default: u64,
    /// CPU time QOS maintenance
    pub ri_cpu_time_qos_maintenance: u64,
    /// CPU time QOS background
    pub ri_cpu_time_qos_background: u64,
    /// CPU time QOS utility
    pub ri_cpu_time_qos_utility: u64,
    /// CPU time QOS legacy
    pub ri_cpu_time_qos_legacy: u64,
    /// CPU time QOS user initiated
    pub ri_cpu_time_qos_user_initiated: u64,
    /// CPU tim QOS user interactive
    pub ri_cpu_time_qos_user_interactive: u64,
    /// Billed system time
    pub ri_billed_system_time: u64,
    /// Serviced system time
    pub ri_serviced_system_time: u64,
}

impl PIDRUsage for RUsageInfoV3 {
    fn flavor() -> PidRUsageFlavor {
        PidRUsageFlavor::V3
    }

    fn memory_used(&self) -> u64 {
        self.ri_resident_size
    }

    fn set_memory_used(&mut self, used: u64) {
        self.ri_resident_size = used;
    }
}

/// C struct for Resource Usage Version 4
#[repr(C)]
#[derive(Default)]
pub struct RUsageInfoV4 {
    /// Unique user id
    pub ri_uuid: [u8; 16],
    /// User time used
    pub ri_user_time: u64,
    /// System time used
    pub ri_system_time: u64,
    /// Wakeups from idle
    pub ri_pkg_idle_wkups: u64,
    /// Child interrupt wakeups
    pub ri_interrupt_wkups: u64,
    /// Number of pageins
    pub ri_pageins: u64,
    /// Wired size
    pub ri_wired_size: u64,
    /// Resident size
    pub ri_resident_size: u64,
    /// Physical footprint
    pub ri_phys_footprint: u64,
    /// Process start time
    pub ri_proc_start_abstime: u64,
    /// Process exit time
    pub ri_proc_exit_abstime: u64,
    /// Child user time
    pub ri_child_user_time: u64,
    /// Child system time
    pub ri_child_system_time: u64,
    /// Child wakeups from idle
    pub ri_child_pkg_idle_wkups: u64,
    /// Child interrupt wakeups
    pub ri_child_interrupt_wkups: u64,
    /// Child pageins
    pub ri_child_pageins: u64,
    /// Child elapse time
    pub ri_child_elapsed_abstime: u64,
    /// Disk IO bytes read
    pub ri_diskio_bytesread: u64,
    /// Disk IO bytes written
    pub ri_diskio_byteswritten: u64,
    /// CPU time QOS default
    pub ri_cpu_time_qos_default: u64,
    /// CPU time QOS maintenance
    pub ri_cpu_time_qos_maintenance: u64,
    /// CPU time QOS background
    pub ri_cpu_time_qos_background: u64,
    /// CPU time QOS utility
    pub ri_cpu_time_qos_utility: u64,
    /// CPU time QOS legacy
    pub ri_cpu_time_qos_legacy: u64,
    /// CPU time QOS user initiated
    pub ri_cpu_time_qos_user_initiated: u64,
    /// CPU tim QOS user interactive
    pub ri_cpu_time_qos_user_interactive: u64,
    /// Billed system time
    pub ri_billed_system_time: u64,
    /// Serviced system time
    pub ri_serviced_system_time: u64,
    /// Logical writes
    pub ri_logical_writes: u64,
    /// Lifetime maximum physical footprint
    pub ri_lifetime_max_phys_footprint: u64,
    /// instructions
    pub ri_instructions: u64,
    /// cycles
    pub ri_cycles: u64,
    /// billed energy
    pub ri_billed_energy: u64,
    /// services energy
    pub ri_serviced_energy: u64,
    /// interval maximum physical footprint
    pub ri_interval_max_phys_footprint: u64,
    /// unused
    pub ri_unused: [u64; 1],
}

impl PIDRUsage for RUsageInfoV4 {
    fn flavor() -> PidRUsageFlavor {
        PidRUsageFlavor::V4
    }

    fn memory_used(&self) -> u64 {
        self.ri_resident_size
    }

    fn set_memory_used(&mut self, used: u64) {
        self.ri_resident_size = used;
    }
}

#[cfg(target_os = "macos")]
#[cfg(feature = "macosx_10_9")]
/// Returns the information about resources of the process that match pid passed in.
///
/// # Errors
///
/// Will return an `Err` if Darwin's underlying method `proc_pid_rusage` returns an error and
/// set `errno`
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
/// #[cfg(target_os = "macos")]
///     if let Ok(res) = pidrusage::<RUsageInfoV2>(pid) {
///         println!("UUID: {:?}, Disk Read: {}, Disk Write: {}", res.ri_uuid, res.ri_diskio_bytesread, res.ri_diskio_byteswritten);
///     }
/// }
/// ```
pub fn pidrusage<T: PIDRUsage>(pid: i32) -> Result<T, String> {
    let flavor = T::flavor() as i32;
    let mut pidrusage = T::default();
    #[allow(clippy::ptr_as_ptr, clippy::borrow_as_ptr, clippy::ref_as_ptr)]
    let buffer_ptr = &mut pidrusage as *mut _ as *mut c_void;
    let ret: i32;

    unsafe {
        ret = proc_pid_rusage(pid, flavor, buffer_ptr.cast());
    };

    if ret < 0 {
        Err(helpers::get_errno_with_message(ret))
    } else {
        Ok(pidrusage)
    }
}

#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
/// Returns the information about resources of the process that match pid passed in.
///
/// # Errors
///
/// Will return `Err` if no process with PID `pid` esists, if the procfs file system cannot be
/// read or the information `VmSize` cannot be read from it for the process in question
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use libproc::libproc::pid_rusage::{pidrusage, RUsageInfoV2, RUsageInfoV0, PIDRUsage};
///
/// fn pidrusage_test() {
///     use std::process;
///     let pid = process::id() as i32;
///
///     if let Ok(res) = pidrusage::<RUsageInfoV0>(pid) {
///         println!("VmSize (resident_size): {}", res.memory_used() );
///     }
/// }
/// ```
pub fn pidrusage<T: PIDRUsage>(pid: i32) -> Result<T, String> {
    let mut pidrusage = T::default();
    let vm_size = procfile_field(&format!("/proc/{pid}/status"), "VmSize")?;
    pidrusage.set_memory_used(parse_memory_string(&vm_size)?);

    Ok(pidrusage)
}

#[cfg(test)]
#[allow(clippy::cast_possible_wrap)]
mod test {
    use super::pidrusage;
    use crate::libproc::pid_rusage::RUsageInfoV0;

    #[test]
    fn pidrusage_test() {
        let usage: RUsageInfoV0 = pidrusage(std::process::id() as i32).expect("pidrusage() failed");
        assert!(usage.ri_resident_size > 0, "Resident size reports 0");
    }
}
