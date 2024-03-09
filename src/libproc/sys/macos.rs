use std::os::unix::ffi::OsStrExt;
use std::{ffi, io, mem, path, ptr};

use libc::{c_char, c_void, c_int};

use crate::osx_libproc_bindings;
use crate::processes::ProcFilter;

impl ProcFilter {
    pub(crate) fn typeinfo(self) -> u32 {
        match self {
            ProcFilter::All => 0, // The Darwin kernel ignores the value, it doesn't matter what we pass in
            ProcFilter::ByProgramGroup { pgrpid } => pgrpid,
            ProcFilter::ByTTY { tty } => tty,
            ProcFilter::ByUID { uid } => uid,
            ProcFilter::ByRealUID { ruid } => ruid,
            ProcFilter::ByParentProcess { ppid } => ppid,
        }
    }
}

impl From<ProcFilter> for u32 {
    fn from(proc_type: ProcFilter) -> Self {
        match proc_type {
            ProcFilter::All => osx_libproc_bindings::PROC_ALL_PIDS,
            ProcFilter::ByProgramGroup { .. } => osx_libproc_bindings::PROC_PGRP_ONLY,
            ProcFilter::ByTTY { .. } => osx_libproc_bindings::PROC_TTY_ONLY,
            ProcFilter::ByUID { .. } => osx_libproc_bindings::PROC_UID_ONLY,
            ProcFilter::ByRealUID { .. } => osx_libproc_bindings::PROC_RUID_ONLY,
            ProcFilter::ByParentProcess { .. } => osx_libproc_bindings::PROC_PPID_ONLY,
        }
    }
}

// similar to list_pids_ret() below, there are two cases when 0 is returned, one when there are
// no pids, and the other when there is an error
fn check_listpid_ret(ret: c_int) -> io::Result<Vec<u32>> {
    let errno = io::Error::last_os_error().raw_os_error().unwrap_or(0);
    if ret < 0 || (ret == 0 && errno != 0) {
        return Err(io::Error::last_os_error());
    }

    let capacity = ret as usize / mem::size_of::<u32>();
    Ok(Vec::with_capacity(capacity))
}

// Common code for handling the special case of listpids return, where 0 is a valid return
// but is also used in the error case - so we need to look at errno to distringish between a valid
// 0 return and an error return
fn list_pids_ret(ret: c_int, mut pids: Vec<u32>) -> io::Result<Vec<u32>> {
    let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
    match ret {
        value if value < 0 || errno != 0 => Err(io::Error::last_os_error()),
        _ => {
            let items_count = ret as usize / mem::size_of::<u32>();
            unsafe {
                pids.set_len(items_count);
            }
            Ok(pids)
        }
    }
}

pub(crate) fn listpids(proc_type: ProcFilter) -> io::Result<Vec<u32>> {
    let buffer_size = unsafe {
        osx_libproc_bindings::proc_listpids(
            proc_type.into(),
            proc_type.typeinfo(),
            ptr::null_mut(),
            0,
        )
    };
    let mut pids = check_listpid_ret(buffer_size)?;
    let buffer_ptr = pids.as_mut_ptr() as *mut c_void;

    let ret = unsafe {
        osx_libproc_bindings::proc_listpids(
            proc_type.into(),
            proc_type.typeinfo(),
            buffer_ptr,
            buffer_size,
        )
    };

    list_pids_ret(ret, pids)
}

pub(crate) fn listpidspath(
    proc_type: ProcFilter,
    path: &path::Path,
    is_volume: bool,
    exclude_event_only: bool,
) -> io::Result<Vec<u32>> {
    let path_bytes = path.as_os_str().as_bytes();
    let c_path = ffi::CString::new(path_bytes)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "CString::new failed"))?;
    let mut pathflags: u32 = 0;
    if is_volume {
        pathflags |= osx_libproc_bindings::PROC_LISTPIDSPATH_PATH_IS_VOLUME;
    }
    if exclude_event_only {
        pathflags |= osx_libproc_bindings::PROC_LISTPIDSPATH_EXCLUDE_EVTONLY;
    }

    let buffer_size = unsafe {
        osx_libproc_bindings::proc_listpidspath(
            proc_type.into(),
            proc_type.typeinfo(),
            c_path.as_ptr() as *const c_char,
            pathflags,
            ptr::null_mut(),
            0,
        )
    };
    let mut pids = check_listpid_ret(buffer_size)?;
    let buffer_ptr = pids.as_mut_ptr() as *mut c_void;

    let ret = unsafe {
        osx_libproc_bindings::proc_listpidspath(
            proc_type.into(),
            proc_type.typeinfo(),
            c_path.as_ptr() as *const c_char,
            0,
            buffer_ptr,
            buffer_size,
        )
    };

    list_pids_ret(ret, pids)
}

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    use super::*;

    use crate::libproc::{bsd_info, proc_pid};

    fn get_all_pid_bsdinfo() -> io::Result<Vec<bsd_info::BSDInfo>> {
        let pids = listpids(ProcFilter::All)?;
        Ok(pids
            .iter()
            .filter_map(|pid| proc_pid::pidinfo::<bsd_info::BSDInfo>(*pid as i32, 0).ok())
            .collect())
    }

    #[test]
    fn test_listpids() -> io::Result<()> {
        let pid = std::process::id();
        let pids = listpids(ProcFilter::All)?;
        assert!(!pids.is_empty());
        assert!(pids.contains(&pid));
        Ok(())
    }

    // Compare the (filtered) PID lists with what manual filtering with BSDInfo
    // data. This won't be a 1:1 match as processes come and go, but it
    // shouldn't deviate hugely either. Each test is retried multiple times to
    // avoid random failures.

    const PROCESS_DIFF_TOLERANCE: usize = 15;

    #[test]
    fn test_listpids_pgid() {
        let mut bsdinfo_pgrps: HashMap<_, HashSet<_>> = HashMap::new();
        for info in get_all_pid_bsdinfo()
            .expect("Could not get all pids info") {
            if info.pbi_pgid == info.pbi_pid {
                continue;
            }
            bsdinfo_pgrps
                .entry(info.pbi_pgid)
                .and_modify(|pids| {
                    pids.insert(info.pbi_pid);
                })
                .or_insert_with(|| vec![info.pbi_pid].into_iter().collect());
        }
        let mut not_matched = 0;
        for (pgrp, bsdinfo_pids) in bsdinfo_pgrps.iter_mut() {
            if bsdinfo_pids.len() <= 1 {
                continue;
            }
            let pids =
                listpids(ProcFilter::ByProgramGroup { pgrpid: *pgrp })
                    .expect("Could not listpids");
            for pid in pids {
                if !bsdinfo_pids.remove(&pid) {
                    not_matched += 1;
                    break;
                }
            }
            if !bsdinfo_pids.is_empty() {
                not_matched += 1;
            }
        }
        assert!(not_matched <= PROCESS_DIFF_TOLERANCE);
    }

    const NODEV: u32 = u32::MAX;

    #[test]
    fn test_listpids_tty() {
        let mut bsdinfo_ttys: HashMap<_, HashSet<_>> = HashMap::new();
        for info in get_all_pid_bsdinfo()
            .expect("Could not get all pids info") {
            if info.e_tdev == NODEV || info.e_tpgid == info.pbi_pid {
                continue;
            }
            bsdinfo_ttys
                .entry(info.e_tdev)
                .and_modify(|pids| {
                    pids.insert(info.pbi_pid);
                })
                .or_insert_with(|| vec![info.pbi_pid].into_iter().collect());
        }
        let mut not_matched = 0;
        for (tty_nr, bsdinfo_pids) in bsdinfo_ttys.iter_mut() {
            if bsdinfo_pids.len() <= 1 {
                continue;
            }
            let pids = listpids(ProcFilter::ByTTY { tty: *tty_nr })
                .expect("Could not listpids");
            for pid in pids {
                if !bsdinfo_pids.remove(&pid) {
                    not_matched += 1;
                    break;
                }
            }
            if !bsdinfo_pids.is_empty() {
                not_matched += 1;
            }
        }
        assert!(not_matched <= PROCESS_DIFF_TOLERANCE);
    }

    #[test]
    fn test_listpids_uid() {
        let mut bsdinfo_uids: HashMap<_, HashSet<_>> = HashMap::new();
        for info in get_all_pid_bsdinfo()
            .expect("Could not get all pids info") {
            bsdinfo_uids
                .entry(info.pbi_uid)
                .and_modify(|pids| {
                    pids.insert(info.pbi_pid);
                })
                .or_insert_with(|| vec![info.pbi_pid].into_iter().collect());
        }
        let mut not_matched = 0;
        for (uid, bsdinfo_pids) in bsdinfo_uids.iter_mut() {
            if bsdinfo_pids.len() <= 1 {
                continue;
            }
            let pids = listpids(ProcFilter::ByUID { uid: *uid })
                .expect("Could not listpids");
            for pid in pids {
                if !bsdinfo_pids.remove(&pid) {
                    not_matched += 1;
                    break;
                }
            }
            if !bsdinfo_pids.is_empty() {
                not_matched += 1;
            }
        }
        assert!(not_matched <= PROCESS_DIFF_TOLERANCE);
    }

    #[test]
    fn test_listpids_real_uid() {
        let mut bsdinfo_ruids: HashMap<_, HashSet<_>> = HashMap::new();
        for info in get_all_pid_bsdinfo()
            .expect("Could not get all pids info"){
            bsdinfo_ruids
                .entry(info.pbi_ruid)
                .and_modify(|pids| {
                    pids.insert(info.pbi_pid);
                })
                .or_insert_with(|| vec![info.pbi_pid].into_iter().collect());
        }
        let mut not_matched = 0;
        for (ruid, bsdinfo_pids) in bsdinfo_ruids.iter_mut() {
            if bsdinfo_pids.len() <= 1 {
                continue;
            }
            let pids = listpids(ProcFilter::ByRealUID { ruid: *ruid })
                .expect("Could not listpids");
            for pid in pids {
                if !bsdinfo_pids.remove(&pid) {
                    not_matched += 1;
                    println!("pid {pid} not matched for ruid {ruid}");
                    break;
                }
            }
            // PROC_ALL_PIDS and PROC_RUID_ONLY are regulargy not agreeing, with PROC_ALL_PIDS
            // listing more than PROC_RUID_ONLY for the same ruid. Testing if bsdinfo_pids is
            // empty is futile here.
        }
        assert!(not_matched <= PROCESS_DIFF_TOLERANCE);
    }

    #[test]
    fn test_listpids_parent_pid() {
        let mut bsdinfo_ppids: HashMap<_, HashSet<_>> = HashMap::new();
        for info in get_all_pid_bsdinfo()
            .expect("Could not get all pids info") {
            bsdinfo_ppids
                .entry(info.pbi_ppid)
                .and_modify(|pids| {
                    pids.insert(info.pbi_pid);
                })
                .or_insert_with(|| vec![info.pbi_pid].into_iter().collect());
        }
        let mut not_matched = 0;
        for (ppid, bsdinfo_pids) in bsdinfo_ppids.iter_mut() {
            let pids =
                listpids(ProcFilter::ByParentProcess { ppid: *ppid })
                    .expect("Could not listpids by parent process");
            for pid in pids {
                if !bsdinfo_pids.remove(&pid) {
                    not_matched += 1;
                    break;
                }
            }
            // PROC_ALL_PIDS is consistently producing processes that are
            // not listed by PROC_PPID_ONLY, so we can't make assertions
            // about having matched all child processes. There is no
            // signal that I can see on why this is.
        }
        assert!(not_matched <= PROCESS_DIFF_TOLERANCE);
    }

    #[test]
    fn test_listpids_invalid_parent_pid() {
        let pids = listpids(ProcFilter::ByParentProcess { ppid: u32::MAX })
            .expect("Error requesting children of inexistant process");
        assert!(pids.is_empty());
    }

    // No point in writing test cases for all ProcFilter members, as the Darwin
    // implementation of proc_listpidspath is essentially a wrapper acound
    // proc_listpids with calls to proc_pidinfo to gather path information.
    // Tests here would simply repeat that work, and so in essence *test the
    // Darwin libproc library* and not our wrapping of that library.

    #[test]
    fn test_listpidspath() -> Result<(), io::Error> {
        let root = std::path::Path::new("/");
        let pids: Vec<u32> =
            listpidspath(ProcFilter::All, root, true, false).expect("Failed to load PIDs for path");
        assert!(!pids.is_empty());
        Ok(())
    }
}
