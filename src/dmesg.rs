//! A `dmesg` command that is a simple demonstration program for using the [`libproc`](../libproc/index.html) library
//!
//! Usage
//! =
//!
//! `> sudo dmesg`
//!
//! ---
//!
//! NOTE: This must be run as `root`
//!

extern crate libproc;
extern crate libc;

use std::io::Write;
use crate::libproc::libproc::proc_pid;
use crate::libproc::libproc::kmesg_buffer;

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