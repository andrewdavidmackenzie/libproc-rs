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

use crate::libproc::libproc::kmesg_buffer;

fn main() {
    if let Ok(message) = kmesg_buffer::kmsgbuf() {
        println!("{}", message);
    }
}