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
Run "cargo doc" and then open "./target/doc/libproc/index.html".
More documentation will be added over time.

# API
At the moment these methods have been implemented:

## Process / PID related
```
pub fn listpids(proc_types: ProcType) -> Result<Vec<u32>, String>
```
```
pub fn pidinfo<T: PIDInfo>(pid : i32, arg: u64) -> Result<T, String> {
```
```
pub fn regionfilename(pid: i32, address: u64) -> Result<String, String>
```
```
pub fn pidpath(pid : i32) -> Result<String, String>
```
```
pub fn libversion() -> Result<(i32, i32), String>
```
```
pub fn name(pid: i32) -> Result<String, String>
```
```
pub fn listpidinfo<T: ListPIDInfo>(pid : i32, max_len: usize) -> Result<Vec<T::Item>, String>
```

## File and FileDescriptor related
```
pub fn pidfdinfo<T: PIDFDInfo>(pid : i32, fd: i32) -> Result<T, String>
```

## PID Resource Usage related
```
pub fn pidrusage<T: PIDRUsage>(pid : i32) -> Result<T, String>  
```

## kmsgbuf
I have also implemented this method - but the MAGIC_NUMBER returned is not correct, 
and upon investigation it seems that Apple/Darwin/Mach have changed totally how dmessage works in 
latest versions, moving away from using libproc to use kvm - with a total rewrite of dmesg.
I leave it in for now, but some serious revision of the library, with conditional compilation depending on
which version of Mac OS X and/or Darwin will be required to get a version that works broadly :-(

```
pub fn kmsgbuf() -> Result<String, String>
```

# Binaries
'cargo build' builds the following binaries:
- 'procinfo' that takes a PID as an optional argument (uses it's own pid if none supplied) and returns information about the process on stdout
- 'dmesg' is a version of dmesg implemented in rust that uses libproc-rs. This must be run as root. Currently fails (see above for explanation).

# Platforms
Implemented just for Mac OS X.
Got build and test passing with functionality disabled on Linux, now will start to look at
functions that could be implemented there also. Suggestions for API and cross-platform abstractions
are welcome.

# TODO
See the [list of issues](https://github.com/andrewdavidmackenzie/libproc-rs/issues). 
I put the "help wanted" label where I need help from others.
 
- Look at what similar methods could be implemented as a starting point on Linux
- Complete the API on Mac OS X - figuring out all the Mac OS X / Darwin version mess....
- Add more documentation (including samples with documentation test)
- Add own custom error type and implement From::from to ease reporting of multiple error types in clients

# LICENSE
This code is licensed under MIT license (see LICENCE).

# CONTRIBUTING
You are welcome to fork this repo and make a pull request, or write an issue.
