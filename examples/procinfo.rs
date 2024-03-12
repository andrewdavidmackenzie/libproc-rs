//! `procinfo` is a simple program to demonstrate the use of the `libproc` library.
//!
//! It prints out info about a process specified by its pid, or the current process if no pid
//! specified.
//!
//! Usage
//! =
//!```
//! procinfo [pid]
//!
//!```
//!
//! Which will produce output similar to:
//! ```
//! Libversion: 1.1
//! Pid: 8484
//! Path: /Users/andrew/workspace/libproc-rs/target/debug/procinfo
//! Name: procinfo
//! Region Filename (at address 0): /Users/andrew/workspace/libproc-rs/target/debug/procinfo
//! There are currently 454 processes active
//! 8496
//! ...
//! ```

use libproc::pid_rusage::{pidrusage, RUsageInfoV0};
use libproc::proc_pid;
use libproc::processes;
use std::env;
use std::io::Write;
use std::process;

fn procinfo(pid: i32) {
    match proc_pid::libversion() {
        Ok((major, minor)) => println!("Libversion: {major}.{minor}"),
        Err(err) => writeln!(&mut std::io::stderr(), "Error: {err}").unwrap(),
    }

    println!("Pid: {pid}");

    match proc_pid::pidpath(pid) {
        Ok(path) => println!("Path: {path}"),
        Err(err) => writeln!(&mut std::io::stderr(), "Error: {err}").unwrap(),
    }

    match pidrusage::<RUsageInfoV0>(pid) {
        Ok(resource_usage) => println!("Memory Used: {} Bytes", resource_usage.ri_resident_size),
        Err(err) => writeln!(&mut std::io::stderr(), "Error: {err}").unwrap(),
    }

    match proc_pid::name(pid) {
        Ok(name) => println!("Name: {name}"),
        Err(err) => writeln!(&mut std::io::stderr(), "Error: {err}").unwrap(),
    }

    match proc_pid::regionfilename(pid, 0) {
        Ok(regionfilename) => println!("Region Filename (at address 0): {regionfilename}"),
        Err(err) => writeln!(&mut std::io::stderr(), "Error: {err}").unwrap(),
    }

    match processes::pids_by_type(processes::ProcFilter::All) {
        Ok(pids) => {
            println!("There are currently {} processes active", pids.len());
            for pid in pids {
                println!("{pid}");
            }
        }
        Err(err) => writeln!(&mut std::io::stderr(), "Error: {err}").unwrap(),
    }
}

/// Print out some information about the current process (if no arguments provided)
/// or a particular process (by passing that process's PID as the first argument
fn main() {
    let args: Vec<String> = env::args().collect();

    let pid = if args.len() == 1 {
        process::id()
    } else {
        args[1].clone().parse::<u32>().unwrap()
    };

    #[allow(clippy::cast_possible_wrap)]
    procinfo(pid as i32);
}
