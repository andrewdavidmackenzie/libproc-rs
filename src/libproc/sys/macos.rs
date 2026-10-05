use std::convert::TryFrom;
use std::os::unix::ffi::OsStrExt;
use std::{ffi, io, mem, path, ptr};

use libc::{c_char, c_int, c_void};

use crate::osx_libproc_bindings;
use crate::processes::ProcFilter;

// Darwin proc_listpids writes native int PIDs into the existing u32 representation.
const _: [(); mem::size_of::<u32>()] = [(); mem::size_of::<c_int>()];
const _: [(); mem::align_of::<u32>()] = [(); mem::align_of::<c_int>()];

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
// when `errno` is set to indicate an error in the input type, the return value is 0
fn check_listpid_ret(ret: c_int) -> io::Result<Vec<u32>> {
    if ret < 0 || (ret == 0 && io::Error::last_os_error().raw_os_error().unwrap_or(0) != 0) {
        return Err(io::Error::last_os_error());
    }

    // `ret` cannot be negative here - so no possible loss of sign
    #[allow(clippy::cast_sign_loss)]
    let capacity = ret as usize / mem::size_of::<u32>();
    Ok(Vec::with_capacity(capacity))
}

// Common code for handling the special case of listpids return, where 0 is a valid return
// but is also used in the error case - so we need to look at errno to distringish between a valid
// 0 return and an error return
// when `errno` is set to indicate an error in the input type, the return value is 0
fn list_pids_ret(ret: c_int, mut pids: Vec<u32>) -> io::Result<Vec<u32>> {
    match ret {
        value
            if value < 0
                || (ret == 0 && io::Error::last_os_error().raw_os_error().unwrap_or(0) != 0) =>
        {
            Err(io::Error::last_os_error())
        }
        _ => {
            // `ret` cannot be negative here - so no possible loss of sign
            #[allow(clippy::cast_sign_loss)]
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
    let buffer_ptr = pids.as_mut_ptr().cast::<c_void>();

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

pub(crate) fn listpids_into(proc_type: ProcFilter, pids: &mut [u32]) -> io::Result<usize> {
    let buffer_size = c_int::try_from(mem::size_of_val(pids)).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "PID buffer byte length exceeds native int",
        )
    })?;
    listpids_into_with_size(proc_type, pids, buffer_size)
}

fn listpids_into_with_size(
    proc_type: ProcFilter,
    pids: &mut [u32],
    buffer_size: c_int,
) -> io::Result<usize> {
    let bytes = usize::try_from(buffer_size).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "PID buffer byte length is negative",
        )
    })?;
    if bytes == 0 || bytes > mem::size_of_val(pids) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "PID buffer byte length is outside the supplied slice",
        ));
    }
    errno::set_errno(errno::Errno(0));
    // SAFETY: u32 has the same size and alignment as Darwin's native PID int,
    // as in the existing listpids buffer. The initialized exclusive slice stays
    // live for the synchronous call; the checked byte count never exceeds it.
    let ret = unsafe {
        osx_libproc_bindings::proc_listpids(
            proc_type.into(),
            proc_type.typeinfo(),
            pids.as_mut_ptr().cast::<c_void>(),
            buffer_size,
        )
    };
    let native_errno = errno::errno().0;
    if ret <= 0 && native_errno != 0 {
        return Err(io::Error::from_raw_os_error(native_errno));
    }
    let written = usize::try_from(ret).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "negative native PID byte count without errno",
        )
    })?;
    if written > bytes || written % mem::size_of::<u32>() != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "native PID byte count is outside the supplied capacity",
        ));
    }
    Ok(written / mem::size_of::<u32>())
}

pub(crate) fn listpidspath(
    proc_type: ProcFilter,
    path: &path::Path,
    is_volume: bool,
    exclude_event_only: bool,
) -> io::Result<Vec<u32>> {
    let path_bytes = path.as_os_str().as_bytes();
    let c_path =
        ffi::CString::new(path_bytes).map_err(|_| io::Error::other("CString::new failed"))?;
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
            c_path.as_ptr().cast::<c_char>(),
            pathflags,
            ptr::null_mut(),
            0,
        )
    };
    let mut pids = check_listpid_ret(buffer_size)?;
    let buffer_ptr = pids.as_mut_ptr().cast::<c_void>();

    let ret = unsafe {
        osx_libproc_bindings::proc_listpidspath(
            proc_type.into(),
            proc_type.typeinfo(),
            c_path.as_ptr().cast::<c_char>(),
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

    // Don't worry about > i32::MAX number of processes
    #[allow(clippy::cast_possible_wrap)]
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
        assert_ne!(pids, [] as [u32; 0]);
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
        for info in get_all_pid_bsdinfo().expect("Could not get all pids info") {
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
        for (pgrp, bsdinfo_pids) in &mut bsdinfo_pgrps {
            if bsdinfo_pids.len() <= 1 {
                continue;
            }
            let pids =
                listpids(ProcFilter::ByProgramGroup { pgrpid: *pgrp }).expect("Could not listpids");
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
        for info in get_all_pid_bsdinfo().expect("Could not get all pids info") {
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
        for (tty_nr, bsdinfo_pids) in &mut bsdinfo_ttys {
            if bsdinfo_pids.len() <= 1 {
                continue;
            }
            let pids = listpids(ProcFilter::ByTTY { tty: *tty_nr }).expect("Could not listpids");
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
        for info in get_all_pid_bsdinfo().expect("Could not get all pids info") {
            bsdinfo_uids
                .entry(info.pbi_uid)
                .and_modify(|pids| {
                    pids.insert(info.pbi_pid);
                })
                .or_insert_with(|| vec![info.pbi_pid].into_iter().collect());
        }
        let mut not_matched = 0;
        for (uid, bsdinfo_pids) in &mut bsdinfo_uids {
            if bsdinfo_pids.len() <= 1 {
                continue;
            }
            let pids = listpids(ProcFilter::ByUID { uid: *uid }).expect("Could not listpids");
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
        for info in get_all_pid_bsdinfo().expect("Could not get all pids info") {
            bsdinfo_ruids
                .entry(info.pbi_ruid)
                .and_modify(|pids| {
                    pids.insert(info.pbi_pid);
                })
                .or_insert_with(|| vec![info.pbi_pid].into_iter().collect());
        }
        let mut not_matched = 0;
        for (ruid, bsdinfo_pids) in &mut bsdinfo_ruids {
            if bsdinfo_pids.len() <= 1 {
                continue;
            }
            let pids = listpids(ProcFilter::ByRealUID { ruid: *ruid }).expect("Could not listpids");
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
        for info in get_all_pid_bsdinfo().expect("Could not get all pids info") {
            bsdinfo_ppids
                .entry(info.pbi_ppid)
                .and_modify(|pids| {
                    pids.insert(info.pbi_pid);
                })
                .or_insert_with(|| vec![info.pbi_pid].into_iter().collect());
        }
        let mut not_matched = 0;
        for (ppid, bsdinfo_pids) in &mut bsdinfo_ppids {
            let pids = listpids(ProcFilter::ByParentProcess { ppid: *ppid })
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
        assert_eq!(pids, [] as [u32; 0]);
    }

    // No point in writing test cases for all ProcFilter members, as the Darwin
    // implementation of proc_listpidspath is essentially a wrapper acound
    // proc_listpids with calls to proc_pidinfo to gather path information.
    // Tests here would simply repeat that work, and so in essence *test the
    // Darwin libproc library* and not our wrapping of that library.

    #[test]
    fn test_listpidspath() {
        let root = std::path::Path::new("/");
        let pids: Vec<u32> =
            listpidspath(ProcFilter::All, root, true, false).expect("Failed to load PIDs for path");
        assert_ne!(pids, [] as [u32; 0]);
    }

    mod bounded {
        use std::io::Write;
        use std::os::unix::process::{CommandExt, ExitStatusExt};
        use std::process::{Child, Command, ExitStatus, Stdio};
        use std::thread;
        use std::time::{Duration, Instant};

        use super::super::*;
        use crate::libproc::{bsd_info, proc_pid};
        use crate::processes::pids_by_type_into;

        fn command(program: &str) -> Command {
            let mut command = Command::new(program);
            command
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null());
            command
        }

        #[derive(Debug)]
        struct NativeRetirement {
            pid: u32,
            status: Option<ExitStatus>,
            kill_attempted: bool,
            errors: Vec<(&'static str, io::Error)>,
            deadline_exhausted: bool,
            report_error: Option<io::Error>,
        }

        struct FixtureChild {
            child: Child,
            retirement: Option<NativeRetirement>,
        }

        impl FixtureChild {
            fn spawn(phase: &str, command: &mut Command) -> io::Result<Self> {
                let spawned = command.spawn();
                let publication = report_primary(phase, &spawned);
                match spawned {
                    Ok(child) => {
                        let mut fixture = Self {
                            child,
                            retirement: None,
                        };
                        if publication.is_err() {
                            let retirement = fixture.retire();
                            assert!(
                                publication.is_ok(),
                                "native spawn succeeded; publication={publication:?}; retirement={retirement:?}", publication = publication, retirement = retirement
                            );
                        }
                        Ok(fixture)
                    }
                    Err(error) => {
                        assert!(
                            publication.is_ok(),
                            "native spawn primary={error:?}; publication={publication:?}; no child was acquired", error = error, publication = publication
                        );
                        Err(error)
                    }
                }
            }

            fn id(&self) -> u32 {
                self.child.id()
            }

            fn retire(&mut self) -> &NativeRetirement {
                if self.retirement.is_none() {
                    let mut retirement = finish_child(&mut self.child);
                    retirement.report_error = writeln!(
                        io::stderr().lock(),
                        "native-owned-child-retirement {retirement:?}"
                    )
                    .err();
                    self.retirement = Some(retirement);
                }
                self.retirement
                    .as_ref()
                    .expect("retirement was recorded before returning")
            }
        }

        impl Drop for FixtureChild {
            fn drop(&mut self) {
                // Early return/unwind still retires this same held child. An
                // unknown memoized result is never retried with its numeric PID.
                if self.retirement.is_none() {
                    self.retire();
                }
            }
        }

        fn finish_child(child: &mut Child) -> NativeRetirement {
            let deadline = Instant::now() + Duration::from_secs(2);
            let mut retirement = NativeRetirement {
                pid: child.id(),
                status: None,
                kill_attempted: false,
                errors: Vec::new(),
                deadline_exhausted: false,
                report_error: None,
            };
            match child.try_wait() {
                Ok(Some(status)) => {
                    retirement.status = Some(status);
                    return retirement;
                }
                Err(error) => {
                    retirement.errors.push(("initial_wait", error));
                    // A failed ownership observation does not authorize a later
                    // signal to a potentially reused numeric PID.
                    return retirement;
                }
                Ok(None) => {}
            }
            retirement.kill_attempted = true;
            if let Err(error) = child.kill() {
                retirement.errors.push(("kill", error));
            }
            loop {
                match child.try_wait() {
                    Ok(Some(status)) => {
                        retirement.status = Some(status);
                        return retirement;
                    }
                    Err(error) => {
                        retirement.errors.push(("post_kill_wait", error));
                        return retirement;
                    }
                    Ok(None) => {}
                }
                if Instant::now() >= deadline {
                    retirement.deadline_exhausted = true;
                    return retirement;
                }
                thread::sleep(Duration::from_millis(10));
            }
        }

        fn report_primary<T>(phase: &str, result: &io::Result<T>) -> io::Result<()> {
            match result {
                Ok(_) => writeln!(
                    io::stderr().lock(),
                    "native-primary {phase}: observed success"
                ),
                Err(error) => writeln!(
                    io::stderr().lock(),
                    "native-primary {phase}: error={error:?}; kind={:?}; raw_os_error={:?}",
                    error.kind(),
                    error.raw_os_error()
                ),
            }
        }

        fn assert_publication<T>(
            primary: &io::Result<T>,
            publication: &io::Result<()>,
            retirements: &[&NativeRetirement],
        ) {
            assert!(
                publication.is_ok(),
                "native primary={:?}; publication={publication:?}; retirements={retirements:?}",
                primary.as_ref().err()
            );
        }

        fn assert_retired<T>(primary: &io::Result<T>, retirement: &NativeRetirement) -> ExitStatus {
            assert!(
                retirement.report_error.is_none()
                    && retirement.errors.is_empty()
                    && !retirement.deadline_exhausted
                    && retirement.status.is_some(),
                "native primary={:?}; retirement={retirement:?}",
                primary.as_ref().err()
            );
            retirement
                .status
                .expect("actual status checked before successful retirement")
        }

        fn checked_pid(child: &mut FixtureChild) -> io::Result<i32> {
            let pid = i32::try_from(child.id()).map_err(io::Error::other);
            let publication = report_primary("positive-native-pid", &pid);
            if pid.is_err() || publication.is_err() {
                let retirement = child.retire();
                assert_publication(&pid, &publication, &[retirement]);
                assert_retired(&pid, retirement);
            }
            pid
        }

        #[test]
        fn native_bounded_group_prefix_and_unused_tail() -> io::Result<()> {
            let mut leader = FixtureChild::spawn(
                "group-leader-spawn",
                command("/bin/sleep").arg("10").process_group(0),
            )?;
            let group = checked_pid(&mut leader)?;
            let member_spawn = command("/bin/sleep").arg("10").process_group(group).spawn();
            let member_publication = report_primary("group-member-spawn", &member_spawn);
            let mut member = match member_spawn {
                Ok(child) => FixtureChild {
                    child,
                    retirement: None,
                },
                Err(error) => {
                    let primary: io::Result<()> = Err(error);
                    let retirement = leader.retire();
                    assert_publication(&primary, &member_publication, &[retirement]);
                    assert_retired(&primary, retirement);
                    return primary;
                }
            };
            let leader_pid = leader.id();
            let member_pid = member.id();
            let filter = ProcFilter::ByProgramGroup { pgrpid: leader_pid };
            let mut full = [u32::MAX; 1];
            let bounded = pids_by_type_into(filter, &mut full);
            let bounded_publication = report_primary("group-one-slot", &bounded);
            let mut spare = [u32::MAX; 3];
            let complete = pids_by_type_into(filter, &mut spare);
            let complete_publication = report_primary("group-spare-slot", &complete);
            let leader_cleanup = leader.retire();
            let member_cleanup = member.retire();
            let retirements = [leader_cleanup, member_cleanup];
            let acquired: io::Result<()> = Ok(());
            assert!(
                member_publication.is_ok()
                    && bounded_publication.is_ok()
                    && complete_publication.is_ok()
                    && retirements.iter().all(|retirement| {
                        retirement.report_error.is_none()
                            && retirement.errors.is_empty()
                            && !retirement.deadline_exhausted
                            && retirement.status.is_some()
                    }),
                "member-spawn={acquired:?}; publication={member_publication:?}; bounded={bounded:?}; publication={bounded_publication:?}; complete={complete:?}; publication={complete_publication:?}; retirements={retirements:?}", acquired = acquired, member_publication = member_publication, bounded = bounded, bounded_publication = bounded_publication, complete = complete, complete_publication = complete_publication, retirements = retirements
            );
            let leader_status = assert_retired(&bounded, leader_cleanup);
            let member_status = assert_retired(&complete, member_cleanup);
            assert_eq!(
                bounded?, 1,
                "a full buffer is a prefix, never completeness proof"
            );
            assert!(full[0] == leader_pid || full[0] == member_pid);
            assert_eq!(complete?, 2);
            assert!(spare[..2].contains(&leader_pid));
            assert!(spare[..2].contains(&member_pid));
            assert_eq!(
                spare[2],
                u32::MAX,
                "unwritten tail must not become an observation"
            );
            assert_eq!(leader_status.signal(), Some(libc::SIGKILL));
            assert_eq!(member_status.signal(), Some(libc::SIGKILL));
            Ok(())
        }

        #[test]
        fn native_terminal_unreaped_owner_uses_two_slots_and_repeats() -> io::Result<()> {
            let mut child = FixtureChild::spawn(
                "terminal-owner-spawn",
                command("/bin/sh").args(["-c", "exit 0"]).process_group(0),
            )?;
            let pid = child.id();
            let native_pid = checked_pid(&mut child)?;
            let deadline = Instant::now() + Duration::from_secs(2);
            let terminal = loop {
                match proc_pid::pidinfo::<bsd_info::BSDInfo>(native_pid, 1) {
                    Ok(info) if info.pbi_status == libc::SZOMB => break Ok(info),
                    Ok(_) if Instant::now() < deadline => thread::sleep(Duration::from_millis(10)),
                    Ok(_) => {
                        break Err(io::Error::new(
                            io::ErrorKind::TimedOut,
                            "native terminal owner not observed",
                        ));
                    }
                    Err(error) => break Err(io::Error::other(error)),
                }
            };
            let observed = (|| -> io::Result<_> {
                let info = terminal?;
                let filter = ProcFilter::ByProgramGroup { pgrpid: pid };
                let mut one = [u32::MAX; 1];
                let one_count = pids_by_type_into(filter, &mut one)?;
                let mut two = [u32::MAX; 2];
                let two_count = pids_by_type_into(filter, &mut two)?;
                let mut repeated = [u32::MAX; 2];
                let repeated_count = pids_by_type_into(filter, &mut repeated)?;
                Ok((
                    info,
                    one,
                    one_count,
                    two,
                    two_count,
                    repeated,
                    repeated_count,
                ))
            })();
            let publication = report_primary("terminal-owner-observation", &observed);
            let cleanup = child.retire();
            assert_publication(&observed, &publication, &[cleanup]);
            let status = assert_retired(&observed, cleanup);
            let (info, one, one_count, two, two_count, repeated, repeated_count) = observed?;
            assert_eq!(info.pbi_pid, pid);
            assert_eq!(info.pbi_status, libc::SZOMB);
            assert_eq!(
                one_count, 1,
                "one slot is full and cannot certify singleton"
            );
            assert_eq!(one, [pid]);
            assert_eq!(
                two_count, 1,
                "unused second slot establishes the short snapshot"
            );
            assert_eq!(two, [pid, u32::MAX]);
            assert_eq!(repeated_count, 1);
            assert_eq!(repeated, two);
            assert_eq!(status.code(), Some(0));
            Ok(())
        }

        #[test]
        fn native_empty_filter_clears_previous_thread_errno() -> io::Result<()> {
            // No process can have the negative native parent PID encoded by MAX.
            // This is a genuine native empty observation, not a fabricated error.
            let mut pids = [u32::MAX; 2];
            errno::set_errno(errno::Errno(libc::EPERM));
            let count =
                pids_by_type_into(ProcFilter::ByParentProcess { ppid: u32::MAX }, &mut pids)?;
            assert_eq!(count, 0);
            assert_eq!(pids, [u32::MAX; 2]);
            Ok(())
        }

        #[test]
        fn native_short_byte_buffer_preserves_actual_kernel_enomem() {
            let mut pids = [u32::MAX; 1];
            // The same private FFI/decoder gets an actually held buffer but a
            // native byte capacity below sizeof(int). Darwin refuses before copyout.
            let error = listpids_into_with_size(ProcFilter::All, &mut pids, 1)
                .expect_err("native kernel must refuse a one-byte PID buffer");
            assert_eq!(error.raw_os_error(), Some(libc::ENOMEM));
            assert_eq!(error.kind(), io::ErrorKind::OutOfMemory);
            assert_eq!(pids, [u32::MAX; 1]);
        }

        #[test]
        fn bounded_empty_buffer_refuses_before_native_read() {
            let error = pids_by_type_into(ProcFilter::All, &mut [])
                .expect_err("public API must refuse an empty caller buffer");
            assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
            assert_eq!(
                error.raw_os_error(),
                None,
                "input refusal is not a native errno"
            );
        }

        #[test]
        fn native_member_spawn_failure_keeps_primary_and_owned_retirement() -> io::Result<()> {
            let mut leader = FixtureChild::spawn(
                "spawn-refusal-leader",
                command("/bin/sleep").arg("10").process_group(0),
            )?;
            let group = checked_pid(&mut leader)?;
            let primary = command("/dev/null/libproc-bounded-member")
                .process_group(group)
                .spawn();
            let publication = report_primary("actual-member-spawn-refusal", &primary);
            // Even an unexpected successful spawn acquires a fixture before
            // refusal is asserted; both actual children are retired and reported.
            match primary {
                Err(error) => {
                    let primary: io::Result<()> = Err(error);
                    let cleanup = leader.retire();
                    assert_publication(&primary, &publication, &[cleanup]);
                    let status = assert_retired(&primary, cleanup);
                    assert!(
                        cleanup.kill_attempted,
                        "cleanup={cleanup:?}",
                        cleanup = cleanup
                    );
                    assert_eq!(status.signal(), Some(libc::SIGKILL));
                    Ok(())
                }
                Ok(child) => {
                    let mut acquired = FixtureChild {
                        child,
                        retirement: None,
                    };
                    let acquired_pid = acquired.id();
                    let leader_cleanup = leader.retire();
                    let acquired_cleanup = acquired.retire();
                    panic!(
                        "native member unexpectedly spawned pid={acquired_pid}; publication={publication:?}; leader={leader_cleanup:?}; acquired={acquired_cleanup:?}", acquired_pid = acquired_pid, publication = publication, leader_cleanup = leader_cleanup, acquired_cleanup = acquired_cleanup
                    );
                }
            }
        }

        #[test]
        fn native_kernel_refusal_keeps_primary_and_owned_retirement() -> io::Result<()> {
            let mut child = FixtureChild::spawn(
                "kernel-refusal-owner",
                command("/bin/sleep").arg("10").process_group(0),
            )?;
            let mut pids = [u32::MAX; 1];
            let primary = listpids_into_with_size(ProcFilter::All, &mut pids, 1);
            let publication = report_primary("actual-one-byte-kernel-refusal", &primary);
            let cleanup = child.retire();
            assert_publication(&primary, &publication, &[cleanup]);
            let status = assert_retired(&primary, cleanup);
            let error = primary.expect_err("actual native one-byte buffer must refuse");
            assert_eq!(error.raw_os_error(), Some(libc::ENOMEM));
            assert_eq!(error.kind(), io::ErrorKind::OutOfMemory);
            assert_eq!(pids, [u32::MAX; 1]);
            assert_eq!(status.signal(), Some(libc::SIGKILL));
            Ok(())
        }

        fn external_reap(pid: i32) -> io::Result<ExitStatus> {
            let deadline = Instant::now() + Duration::from_secs(2);
            loop {
                let mut status = 0;
                // SAFETY: exact PID belongs to this test's still-held Child;
                // status is initialized writable storage. WNOHANG never blocks.
                // This controlled external reap deliberately invalidates Child's
                // wait custody; the fixture may not signal after observing ECHILD.
                let reaped =
                    unsafe { libc::waitpid(pid, std::ptr::addr_of_mut!(status), libc::WNOHANG) };
                if reaped == -1 {
                    return Err(io::Error::last_os_error());
                }
                if reaped == pid {
                    return Ok(ExitStatus::from_raw(status));
                }
                if reaped != 0 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "native waitpid returned a different PID",
                    ));
                }
                if Instant::now() >= deadline {
                    return Err(io::Error::new(
                        io::ErrorKind::TimedOut,
                        "external native reap not observed within fixture deadline",
                    ));
                }
                thread::sleep(Duration::from_millis(10));
            }
        }

        #[test]
        fn native_external_reap_retains_primary_cleanup_cause_and_forbids_late_kill(
        ) -> io::Result<()> {
            let mut child = FixtureChild::spawn(
                "external-reap-owner",
                command("/bin/sh").args(["-c", "exit 0"]).process_group(0),
            )?;
            let pid = checked_pid(&mut child)?;
            let mut pids = [u32::MAX; 1];
            let primary = listpids_into_with_size(ProcFilter::All, &mut pids, 1);
            let primary_publication = report_primary("external-reap-primary-enomem", &primary);
            let externally_reaped = external_reap(pid);
            let reap_publication = report_primary("actual-external-waitpid", &externally_reaped);
            let cleanup = child.retire();
            assert!(
                primary_publication.is_ok()
                    && reap_publication.is_ok()
                    && cleanup.report_error.is_none(),
                "primary={:?}; publication={primary_publication:?}; external_reap={externally_reaped:?}; publication={reap_publication:?}; cleanup={cleanup:?}",
                primary.as_ref().err()
            );
            assert!(
                externally_reaped.is_ok(),
                "primary={:?}; external_reap={externally_reaped:?}; cleanup={cleanup:?}",
                primary.as_ref().err()
            );
            assert_eq!(externally_reaped?.code(), Some(0));
            assert!(
                cleanup.status.is_none(),
                "cleanup={cleanup:?}",
                cleanup = cleanup
            );
            assert!(
                !cleanup.kill_attempted,
                "cleanup={cleanup:?}",
                cleanup = cleanup
            );
            assert!(
                !cleanup.deadline_exhausted,
                "cleanup={cleanup:?}",
                cleanup = cleanup
            );
            assert_eq!(cleanup.pid, u32::try_from(pid).expect("positive owned PID"));
            assert_eq!(cleanup.errors.len(), 1, "cleanup={cleanup:?}",);
            assert_eq!(cleanup.errors[0].0, "initial_wait");
            assert_eq!(cleanup.errors[0].1.raw_os_error(), Some(libc::ECHILD));
            let error = primary.expect_err("actual native one-byte buffer must refuse");
            assert_eq!(error.raw_os_error(), Some(libc::ENOMEM));
            assert_eq!(pids, [u32::MAX; 1]);
            let first_record = std::ptr::from_ref::<NativeRetirement>(cleanup);
            let repeated = child.retire();
            assert!(std::ptr::eq(first_record, repeated));
            assert!(
                !repeated.kill_attempted,
                "repeated={repeated:?}",
                repeated = repeated
            );
            assert_eq!(repeated.errors[0].1.raw_os_error(), Some(libc::ECHILD));
            // The same unknown Child result is retained. Drop makes no second
            // wait/kill attempt; the actual external wait above reaped it.
            Ok(())
        }
    }
}
