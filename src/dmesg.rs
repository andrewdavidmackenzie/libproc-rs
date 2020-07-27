//! A `dmesg` command that is a simple demonstration program for using the [`libproc`](../libproc/index.html) library
//!
//! Usage
//! =
//!
//! `> dmesg`
//!

extern crate libproc;
extern crate libc;

use crate::libproc::libproc::kmesg_buffer;

fn main() {
    match kmesg_buffer::kmsgbuf() {
        Ok(message) => print!("{}", message),
        Err(e) => eprintln!("{}", e)
    }
}