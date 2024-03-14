![Build Status](https://travis-ci.org/andrewdavidmackenzie/libproc-rs.svg?branch=master "Mac OS X")
[![codecov](https://codecov.io/gh/andrewdavidmackenzie/libproc-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/andrewdavidmackenzie/libproc-rs)

# libproc-rs
This is a library for getting information about running processes for Mac OS X and Linux.

Add it to your project's `Cargo.toml`:
```toml
libproc = "0.14.4"
```

And then use it in your code:
```rust
use libproc::libproc::proc_pid;

match proc_pid::pidpath(pid) {
    Ok(path) => println!("PID {}: has path {}", pid, path),
    Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
}
```

You can find the latest published release on [crates.io](https://crates.io/crates/libproc)

You can find the browseable docs for the latest release on [docs.rs](https://docs.rs/libproc/latest/libproc/).

NOTE: `master` branch (code and docs) can differ from those docs prior to a new release.

# Minimum rust version
The minimum rust version required, by version:
* libproc-rs: 0.14.6 --> 1.74.1 
* libproc-rs: 0.14.7 --> 1.72.0
This is tested in CI and must pass.

# Test Matrix
The Github Actions CI matrix is:

rust versions:
* `stable` (must pass)
* `beta` (must pass)
* `1.74.1` (currently the minimum rust version supported) (must pass)
* `nightly` (allowed to fail) 

on the following platforms:
* `ubuntu-latest`
* `macos-11` (Big Sur)
* `macos-12` (Monterey)
* `macos-13` (Ventura)
* `macos-14` (Sonoma)


## Mac OS X Versions
Calls were added to libproc in 10.9 (Mavericks) and they are under a rust "feature" switch called "macosx_10_9".
The default build includes the "macosx_10_9" feature.

To build for versions prior to Mac OS 10.9 disable the default features by passing `--no-default-features` to cargo.

# Examples
Two simple examples are included to show libproc-rs working.

- `procinfo` that takes a PID as an optional argument (uses it's own pid if none supplied) and returns
  information about the process on stdout
- `dmesg` is a version of dmesg implemented in rust that uses libproc-rs.

These can be ran thus:
`sudo cargo run --example procinfo` or 
`sudo cargo run --example dmesg`

# Contributing
You are welcome to fork this repo and make a pull request, or write an issue.

## Experiment in OSS funding
I am exploring the ideas around Open Source Software funding from [RadWorks Foundation]([https://radworks.org/) via the [Drips Project](https://www.drips.network/)

This project is in Drips [here](https://www.drips.network/app/projects/github/andrewdavidmackenzie/libproc-rs)

## Input Requested
* Suggestions for API, module re-org and cross-platform abstractions are welcome.
* How to do error reporting? Define own new Errors, or keep simple with Strings?
* Would like Path/PathBuf returned when it makes sense instead of String?

## TODO
See the [list of issues](https://github.com/andrewdavidmackenzie/libproc-rs/issues). 
I put the "help wanted" label where I need help from others.
 
- Look at what similar methods could be implemented as a starting point on Linux
- Complete the API on Mac OS X - figuring out all the Mac OS X / Darwin version mess....
- Add more documentation (including samples with documentation test)
- Add own custom error type and implement From::from to ease reporting of multiple error types in clients

## Build and Test Locally
If you're feeling lucky today, start with `make`
that will run `clippy`, `test` and will build docs also.

If you want to test locally as much of the test matrix as possible (different OS and
versions of rust), that you can use `make matrix`. On macos, if you have `act`
installed, it will use it to run the linux part of the matrix.

If you want to stay "pure rust" : `cargo test` will build and test as usual.

However, as some functions need to be run as `root` to work, CI tests are run as `root`.
So, when developing in local it's best if you use `sudo cargo test`.

[!NOTE] This can get you into permissions problems when switching back and for
between using `cargo test` and `sudo cargo test`.
To fix that run `sudo cargo clean` and then build or test as you prefer.

In order to have tests pass when run as `root` or not, some tests need to check if they are `root`
at run-time (using our own `am_root()` function is handy) and avoid failing if *not* run as `root`.

### Using "act" to run GH Actions CI workflows locally
If you develop on macos but want to ensure code builds and tests pass on linux while making changes,
you can use the [act](https://github.com/nektos/act) tool to run the Github Actions Workflows on
the test matrix.

Just install `act` (`brew install act`) (previously install docker if you don't have it already,
and make sure the daemon is running) then run `act -W .github/workflows/clippy_build_test.yml`
at the command line

### Macos: clang detection and header file finding
Newer versions of `bindgen` have improved the detection of `clang` and hence macos header files.
If you also have llvm/clang installed directly or via `brew` this may cause the build to fail saying it
cannot find `libproc.h`. This can be fixed by setting `CLANG_PATH="/usr/bin/clang"` so that `bindgen`
detects the Xcode version and hence can fidn the correct header files.

# LICENSE
This code is licensed under MIT license (see LICENCE).

