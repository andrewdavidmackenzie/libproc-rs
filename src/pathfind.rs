extern crate libproc;
use std::{env, str};
use std::io::Write;
use libproc::libproc::proc_pid;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let pid_arg = args[1].clone();
            match pid_arg.parse::<i32>() {
                Ok(pid) => {
                    match proc_pid::pidpath(pid) {
                        Ok(path) => {
                            println!("PID {}: has path {}", pid, path);
                        },
                        Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
                    }
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