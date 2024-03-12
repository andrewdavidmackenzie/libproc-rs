use std::io;
#[cfg(target_os = "macos")]
use std::path::Path;

use crate::libproc::sys::{listpids, listpidspath};

/// `ProcFilter` is used to filter process ids.
/// See [`pids_by_type`] and `pids_by_type_and_path` (macos only) for details.
#[derive(Copy, Clone)]
pub enum ProcFilter {
    /// All processes
    All,
    /// Filter by program group id
    ByProgramGroup {
        /// List PIDs that are members of this process group
        pgrpid: u32,
    },
    /// Filter by TTY
    ByTTY {
        /// List PIDs attached to the specific TTY
        tty: u32,
    },
    /// Filter by (effective) user ID
    ByUID {
        /// List PIDs of processes with the permissions of this specific user.
        uid: u32,
    },
    /// Filter by real user ID
    ByRealUID {
        /// List PIDs of processes started by this specific user.
        ruid: u32,
    },
    /// Filter by parent process ID
    ByParentProcess {
        /// List PIDs of processes that are children of this specific process.
        ppid: u32,
    },
}

/// Returns the PIDs of active processes that match the given [`ProcFilter`] filter.
///
/// # Errors
///
/// Will return an error if the pids matching the filter cannot be listed for some reason, as
/// returned in `errno` by Darwin's libproc.
///
/// # Examples
///
/// Get the list of all running process IDs using [`pids_by_type`] and [`ProcFilter::All`]:
///
/// ```
/// use std::io::Write;
/// use libproc::processes;
///
/// if let Ok(pids) = processes::pids_by_type(processes::ProcFilter::All) {
///     println!("There are {} processes running on this system", pids.len());
/// }
/// ```
///
/// Get a list of running process IDs that are children of the current process:
///
/// ```
/// use std::io::Write;
/// use std::process;
/// use libproc::processes;
///
/// let filter = processes::ProcFilter::ByParentProcess { ppid: process::id() };
/// if let Ok(pids) = processes::pids_by_type(filter) {
///     println!("Found {} child processes of this process", pids.len());
/// }
/// ```
pub fn pids_by_type(filter: ProcFilter) -> io::Result<Vec<u32>> {
    listpids(filter)
}

/// Returns the PIDs of active processes that reference reference an open file
/// with the given path or volume, with or without files opened with the
/// `O_EVTONLY` flag.
///
/// (Files opened with the `O_EVTONLY` flag will not prevent a volume from being
/// unmounted).
///
/// # Errors
///
/// Will return an error if:
///   * input `path` is invalid
///   * the pids matching the filter cannot be listed for some reason, as
///     returned in `errno` by Darwin's libproc.
///
/// # Examples
///
/// Get the list of all running process IDs that have a specific filename open:
///
/// ```
/// use std::path::Path;
/// use std::io::Write;
/// use libproc::processes;
///
/// let path = Path::new("/etc/hosts");
/// if let Ok(pids) = processes::pids_by_path(&path, false, false) {
///     println!("Found {} processes accessing {}", pids.len(), path.display());
/// }
/// ```
///
/// List all processes that have a file open on a specific volume; the path
/// argument is used to get the filesystem device ID, and processes that have
/// a file open on that same device ID match the filter:
/// ```
/// use std::path::Path;
/// use std::io::Write;
/// use libproc::processes;
///
/// let path = Path::new("/Volumes/MountedDrive");
/// if let Ok(pids) = processes::pids_by_path(&path, true, false) {
///     println!("Found {} processes accessing files on {}", pids.len(), path.display());
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn pids_by_path(
    path: &Path,
    is_volume: bool,
    exclude_event_only: bool,
) -> io::Result<Vec<u32>> {
    listpidspath(ProcFilter::All, path, is_volume, exclude_event_only)
}

/// Returns a filtered list of PIDs of active processes that reference reference
/// an open file with the given path or volume, with or without files opened
/// with the `O_EVTONLY` flag. Use a [`ProcFilter`] member to specify how to
/// filter the list of PIDs returned.
///
/// (Files opened with the `O_EVTONLY` flag will not prevent a volume from being
/// unmounted).
///
/// # Errors
///
/// Will return an error if:
///   * input `path` is invalid
///   * the pids matching the filter cannot be listed for some reason, as
///     returned in `errno` by Darwin's libproc.
///
/// # Examples
///
/// Get the list of process ids for child processes that have a specific filename open:
///
/// ```
/// use std::path::Path;
/// use std::process;
/// use std::io::Write;
/// use libproc::processes;
///
/// let path = Path::new("/etc/hosts");
/// let filter = processes::ProcFilter::ByParentProcess { ppid: process::id() };
/// if let Ok(pids) = processes::pids_by_type_and_path(filter, &path, false, false) {
///     println!("Found {} processes accessing {}", pids.len(), path.display());
/// }
/// ```
///
/// List all processes within the current process group that have a file open on
/// a specific volume; the path argument is used to get the filesystem device
/// ID, and processes that have a file open on that same device ID match the
/// filter:
/// ```
/// use std::path::Path;
/// use std::process;
/// use std::io::Write;
/// use libproc::processes;
///
/// let path = Path::new("/Volumes/MountedDrive");
/// let filter = processes::ProcFilter::ByParentProcess { ppid: process::id() };
/// if let Ok(pids) = processes::pids_by_type_and_path(filter, &path, true, false) {
///     println!("Found {} processes accessing files on {}", pids.len(), path.display());
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn pids_by_type_and_path(
    filter: ProcFilter,
    path: &Path,
    is_volume: bool,
    exclude_event_only: bool,
) -> io::Result<Vec<u32>> {
    listpidspath(filter, path, is_volume, exclude_event_only)
}
