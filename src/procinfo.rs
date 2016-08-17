extern crate libproc;

use std::{env, str};
use std::io::Write;
use libproc::libproc::proc_pid;

fn procinfo(pid : i32) {
    match proc_pid::libversion() {
        Ok((major, minor)) => println!("Libversion: {}.{}", major, minor),
        Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
    }

    println!("Pid: {}", pid);

    match proc_pid::pidpath(pid) {
        Ok(path) => println!("Path: {}", path),
        Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
    }

    match proc_pid::name(pid) {
        Ok(name) => println!("Name: {}", name),
        Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
    }

    match proc_pid::regionfilename(pid, 0) {
        Ok(regionfilename) => println!("Region Filename (at address 0): {}", regionfilename),
        Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let pid_arg = args[1].clone();
            match pid_arg.parse::<i32>() {
                Ok(pid) => {
                    procinfo(pid);
                },
                Err(err) => {
                    writeln!(&mut std::io::stderr(), "Error: Could not parse a valid PID from the argument '{}'. Error message = '{}'", pid_arg, err).unwrap();
                }
            }
        },
        _ => {
            writeln!(&mut std::io::stderr(), "Error: Please supply one process PID as an argument").unwrap();
        }
    }
}