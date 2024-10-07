use std::{fs, io, path};

use crate::processes::ProcFilter;

const FIRST_FIELD: isize = 2;

enum ProcStatField {
    // Commented out fields are skipped during parsing
    // or not used in this module. Numbers are the
    // offset from Status (2)
    // PID = 0,
    // Cmd = 1,
    // Status = 2,
    Ppid = 3 - FIRST_FIELD,
    Pgrp = 4 - FIRST_FIELD,
    // SID = 5,
    TtyNr = 6 - FIRST_FIELD,
    // rest ignored
}

/// Parse out a specific field from the stat file beloning to a path starting
/// with /proc/pid Expects the indicated (0-based) field to be parsable as a
/// u32 integer, and field must be > 2.  I/O errors are ignored, with the
/// assumption that the process has gone away.
fn proc_stat_field(proc_path: &path::Path, field: ProcStatField) -> Option<u32> {
    use io::BufRead;

    fs::File::open(proc_path.join("stat"))
        .and_then(|f| {
            // there should only be a single line, but best avoid reading more
            let mut buffer = io::BufReader::new(f);
            let mut line = String::new();
            buffer.read_line(&mut line).map(|_| line)
        })
        .map_or(None, |line| {
            line.rfind(')').and_then(|pos| {
                // Skip past the PID and command; the command is wrapped in (..)
                // and the closing parenthesis is the only such character in the
                // line if scanned from the end.
                line[pos + 2..]
                    .split_ascii_whitespace()
                    .nth(field as usize)
                    .and_then(|v| v.parse().ok())
            })
        })
}

/// Get the owner UID of a given path, as an option. Errors are ignored, it is
/// assumed the process went away.
fn file_owner_uid(path: &path::Path) -> Option<u32> {
    use std::os::unix::fs::MetadataExt;
    fs::metadata(path).map(|md| md.uid()).ok()
}

/// Reads process information from /proc/pid/{,stat} to enumerate PIDs matching the filter
pub fn listpids(proc_types: ProcFilter) -> io::Result<Vec<u32>> {
    let mut pids = Vec::<u32>::new();

    let proc_dir = fs::read_dir("/proc")?;

    for entry in proc_dir {
        let path = entry?.path();
        let filename = path.file_name();
        if let Some(name) = filename {
            if let Some(n) = name.to_str() {
                if let Ok(pid) = n.parse::<u32>() {
                    let matches = match proc_types {
                        ProcFilter::All => true,
                        ProcFilter::ByProgramGroup { pgrpid } => {
                            proc_stat_field(&path, ProcStatField::Pgrp) == Some(pgrpid)
                        }
                        ProcFilter::ByTTY { tty } => {
                            proc_stat_field(&path, ProcStatField::TtyNr) == Some(tty)
                        }
                        ProcFilter::ByUID { uid } => file_owner_uid(&path) == Some(uid),
                        ProcFilter::ByRealUID { ruid } => {
                            file_owner_uid(&path.join("stat")) == Some(ruid)
                        }
                        ProcFilter::ByParentProcess { ppid } => {
                            proc_stat_field(&path, ProcStatField::Ppid) == Some(ppid)
                        }
                    };
                    if matches {
                        pids.push(pid);
                    }
                }
            }
        }
    }

    Ok(pids)
}

#[cfg(test)]
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
mod test {
    use std::collections::{HashMap, HashSet};
    use std::io::{Error, Write};

    use super::*;

    #[test]
    fn test_proc_stat_field() -> Result<(), Error> {
        let tempdir = tempfile::tempdir()?;
        let path = tempdir.path();
        let mut test_file = fs::File::create(path.join("stat"))?;
        // PPID: 17, PGRP 23, Session 11 (ignored), TTY 4201, TGPID 7 (ignored)
        writeln!(
            test_file,
            "42 (libproc-rs-mock-process) T 17 23 11 4201 7 ..."
        )?;

        assert_eq!(proc_stat_field(path, ProcStatField::Ppid), Some(17));
        assert_eq!(proc_stat_field(path, ProcStatField::Pgrp), Some(23));
        assert_eq!(proc_stat_field(path, ProcStatField::TtyNr), Some(4201));

        Ok(())
    }

    #[test]
    fn test_proc_stat_field_errors() -> Result<(), Error> {
        let tempdir = tempfile::tempdir()?;
        let path = tempdir.path();
        let mut test_file = fs::File::create(path.join("stat"))?;
        // PPID: 17, PGRP 23, Session 11 (ignored), TTY 4201, TGPID 7 (ignored)
        writeln!(test_file, "garbage in\nerrors out")?;

        assert_eq!(proc_stat_field(path, ProcStatField::Ppid), None);
        assert_eq!(
            proc_stat_field(&path.join("nonesuch"), ProcStatField::Ppid),
            None
        );

        Ok(())
    }

    // Compare the (filtered) PID lists with what the procfs library sees. This
    // won't be a 1:1 match as processes come and go, but it shouldn't deviate
    // hugely either. Each test is retried multiple times to avoid random
    // failures.

    const PROCESS_DIFF_TOLERANCE: usize = 5;
    const MAX_RETRIES: usize = 5;

    #[test]
    fn test_listpids_all() -> Result<(), procfs::ProcError> {
        for _ in 0..MAX_RETRIES {
            let mut procfs_pids: HashSet<_> = procfs::process::all_processes()?
                .filter_map(|proc| proc.ok().map(|proc| proc.pid))
                .collect();
            let mut new_count = 0;
            let pids = listpids(ProcFilter::All).unwrap_or_default();
            for pid in pids {
                if !procfs_pids.remove(&(pid as i32)) {
                    new_count += 1;
                }
            }
            let gone_count = procfs_pids.len();
            if new_count < PROCESS_DIFF_TOLERANCE && gone_count < PROCESS_DIFF_TOLERANCE {
                return Ok(());
            }
        }
        panic!("Test failed");
    }

    #[test]
    fn test_listpids_pgid() -> Result<(), procfs::ProcError> {
        for _ in 0..MAX_RETRIES {
            let mut procfs_pgrps: HashMap<_, HashSet<_>> = HashMap::new();
            for proc in (procfs::process::all_processes()?).flatten() {
                if let Ok(stat) = proc.stat() {
                    procfs_pgrps
                        .entry(stat.pgrp)
                        .and_modify(|pids| {
                            pids.insert(stat.pid);
                        })
                        .or_insert_with(|| vec![stat.pid].into_iter().collect());
                }
            }
            let mut not_matched = 0;
            for (pgrp, procfs_pids) in &mut procfs_pgrps {
                if procfs_pids.len() <= 1 {
                    continue;
                }
                let pids = listpids(ProcFilter::ByProgramGroup {
                    pgrpid: *pgrp as u32,
                })
                    .unwrap_or_default();
                for pid in pids {
                    if !procfs_pids.remove(&(pid as i32)) {
                        not_matched += 1;
                        break;
                    }
                }
                if !procfs_pids.is_empty() {
                    not_matched += 1;
                }
            }
            if not_matched <= PROCESS_DIFF_TOLERANCE {
                return Ok(());
            }
        }
        panic!("Test failed");
    }

    #[test]
    fn test_listpids_tty() -> Result<(), procfs::ProcError> {
        for _ in 0..MAX_RETRIES {
            let mut procfs_ttys: HashMap<_, HashSet<_>> = HashMap::new();
            for proc in (procfs::process::all_processes()?).flatten() {
                if let Ok(stat) = proc.stat() {
                    procfs_ttys
                        .entry(stat.tty_nr)
                        .and_modify(|pids| {
                            pids.insert(stat.pid);
                        })
                        .or_insert_with(|| vec![stat.pid].into_iter().collect());
                }
            }
            let mut not_matched = 0;
            for (tty_nr, procfs_pids) in &mut procfs_ttys {
                if procfs_pids.len() <= 1 {
                    continue;
                }
                let pids = listpids(ProcFilter::ByTTY {
                    tty: *tty_nr as u32,
                })
                    .unwrap_or_default();
                for pid in pids {
                    if !procfs_pids.remove(&(pid as i32)) {
                        not_matched += 1;
                        break;
                    }
                }
                if !procfs_pids.is_empty() {
                    not_matched += 1;
                }
            }
            if not_matched <= PROCESS_DIFF_TOLERANCE {
                return Ok(());
            }
        }
        panic!("Test failed");
    }

    #[test]
    fn test_listpids_uid() -> Result<(), procfs::ProcError> {
        for _ in 0..MAX_RETRIES {
            let mut procfs_uids: HashMap<_, HashSet<_>> = HashMap::new();
            for proc in (procfs::process::all_processes()?).flatten() {
                if let Ok(status) = proc.status() {
                    procfs_uids
                        .entry(status.euid)
                        .and_modify(|pids| {
                            pids.insert(status.pid);
                        })
                        .or_insert_with(|| vec![status.pid].into_iter().collect());
                }
            }
            let mut not_matched = 0;
            for (uid, procfs_pids) in &mut procfs_uids {
                if procfs_pids.len() <= 1 {
                    continue;
                }
                let pids = listpids(ProcFilter::ByUID { uid: *uid }).unwrap_or_default();
                for pid in pids {
                    if !procfs_pids.remove(&(pid as i32)) {
                        not_matched += 1;
                        break;
                    }
                }
                if !procfs_pids.is_empty() {
                    not_matched += 1;
                }
            }
            if not_matched <= PROCESS_DIFF_TOLERANCE {
                return Ok(());
            }
        }
        panic!("Test failed");
    }

    #[test]
    fn test_listpids_real_uid() -> Result<(), procfs::ProcError> {
        for _ in 0..MAX_RETRIES {
            let mut procfs_ruids: HashMap<_, HashSet<_>> = HashMap::new();
            for proc in (procfs::process::all_processes()?).flatten() {
                if let Ok(status) = proc.status() {
                    procfs_ruids
                        .entry(status.ruid)
                        .and_modify(|pids| {
                            pids.insert(status.pid);
                        })
                        .or_insert_with(|| vec![status.pid].into_iter().collect());
                }
            }
            let mut not_matched = 0;
            for (ruid, procfs_pids) in &mut procfs_ruids {
                if procfs_pids.len() <= 1 {
                    continue;
                }
                let pids = listpids(ProcFilter::ByRealUID { ruid: *ruid }).unwrap_or_default();
                for pid in pids {
                    if !procfs_pids.remove(&(pid as i32)) {
                        not_matched += 1;
                        break;
                    }
                }
                if !procfs_pids.is_empty() {
                    not_matched += 1;
                }
            }
            if not_matched <= PROCESS_DIFF_TOLERANCE {
                return Ok(());
            }
        }
        panic!("Test failed");
    }

    #[test]
    fn test_listpids_parent_pid() -> Result<(), procfs::ProcError> {
        for _ in 0..MAX_RETRIES {
            let mut procfs_ppids: HashMap<_, HashSet<_>> = HashMap::new();
            for proc in (procfs::process::all_processes()?).flatten() {
                if let Ok(stat) = proc.stat() {
                    procfs_ppids
                        .entry(stat.ppid)
                        .and_modify(|pids| {
                            pids.insert(stat.pid);
                        })
                        .or_insert_with(|| vec![stat.pid].into_iter().collect());
                }
            }
            let mut not_matched = 0;
            for (ppid, procfs_pids) in &mut procfs_ppids {
                if procfs_pids.len() <= 1 {
                    continue;
                }
                let pids = listpids(ProcFilter::ByParentProcess { ppid: *ppid as u32 })
                    .unwrap_or_default();
                for pid in pids {
                    if !procfs_pids.remove(&(pid as i32)) {
                        not_matched += 1;
                        break;
                    }
                }
                if !procfs_pids.is_empty() {
                    not_matched += 1;
                }
            }
            if not_matched <= PROCESS_DIFF_TOLERANCE {
                return Ok(());
            }
        }
        panic!("Test failed");
    }
}
