#![deny(missing_docs)]
#![warn(clippy::unwrap_used)]

//! `libproc` is a library for getting information about running processes on Mac and Linux.
//!
//! Not all methods are available on both Operating Systems yet, but more will
//! be made cross-platform over time.
//!
extern crate libc;
extern crate errno;

pub mod libproc;