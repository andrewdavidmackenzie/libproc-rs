extern crate libproc;
extern crate libc;

#[cfg(target_os = "macos")]
use std::io::Write;
#[cfg(target_os = "macos")]
use crate::libproc::libproc::proc_pid;
#[cfg(target_os = "macos")]
use crate::libproc::libproc::kmesg_buffer;

/*
    A `dmesg` commands as a simple demonstration program of using libproc-rs
*/
#[cfg(target_os = "macos")]
fn main() {
    if proc_pid::am_root() {
        match kmesg_buffer::kmsgbuf() {
            Ok(message) => println!("{}", message),
            Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
        }
    } else {
        writeln!(&mut std::io::stderr(), "Must be run as root").unwrap()
    }
}

#[cfg(target_os = "linux")]
fn main() {
    unimplemented!()
}