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

# API
At the moment these methods are implemented
- pub fn pidpath(pid : i32) -> Result<String, String>

# Binaries
'cargo build' also builds a simple binary called 'procinfo' that takes a PID as an argument and returns information about the process on stdout

# Platforms
Initially just for Mac OS X. Once the API is complete then doing a Linux version with the same API would make sense.

# TODO
- Complete the API on Mac OS X to match that provided (on Mac OS X) at http://opensource.apple.com//source/Libc/Libc-498.1.1/darwin/libproc.c
- Add some documentation (including samples with documentation test)
- Implement a version for Linux with the same API...

# LICENSE
This code is licensed under MIT license (see LICENCE).

# CONTRIBUTING
You are welcome to fork this repo and make a pull request, or write an issue.
