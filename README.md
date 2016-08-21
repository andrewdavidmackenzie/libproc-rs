![Build Status](https://travis-ci.org/andrewdavidmackenzie/libproc-rs.svg?branch=master "Mac OS X")

# libproc-rs
This is a rust wrapper for libproc (supplied on Mac OS X as a native library) for getting information about running processes.

# Using it
```
extern crate libproc;
use libproc::libproc::proc_pid;

...

match proc_pid::pidpath(pid) {
    Ok(path) => {
        println!("PID {}: has path {}", pid, path);
    },
    Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
}
```

# Documentation
See Documentation published at crates.io (soon)

# API
At the moment these methods are implemented:
- pub fn listpids(proc_types: ProcType) -> Result<Vec<u32>, String>
- pub fn regionfilename(pid: i32, address: u64) -> Result<String, String>
- pub fn pidpath(pid : i32) -> Result<String, String>
- pub fn libversion() -> Result<(i32, i32), String>
- pub fn name(pid: i32) -> Result<String, String>

# Binaries
'cargo build' builds the following binaries:
- 'procinfo' that takes a PID as an optional argument (uses it's own pid if none supplied) and returns information about the process on stdout
- 'dmesg' is a version of dmesg implemented in rust that uses libproc-rs. This must be run as root.

# Platforms
Initially just for Mac OS X.

# TODO
- Complete the API on Mac OS X
- Add some documentation (including samples with documentation test)
- Once the API is complete then doing a Linux version with the same API would make sense.

# LICENSE
This code is licensed under MIT license (see LICENCE).

# CONTRIBUTING
You are welcome to fork this repo and make a pull request, or write an issue.
