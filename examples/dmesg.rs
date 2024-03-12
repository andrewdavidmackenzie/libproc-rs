//! A `dmesg` command that is a simple demonstration program for using
//! the `libproc` library
//!
//! Usage
//! =
//!
//! `> dmesg`
//!

use libproc::kmesg_buffer;

fn main() {
    match kmesg_buffer::kmsgbuf() {
        Ok(message) => print!("{message}"),
        Err(e) => eprintln!("{e}"),
    }
}
