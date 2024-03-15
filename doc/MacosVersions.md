# Macos Versions
This doc lists the different versions of macos released (and still supported and tested by `libproc-rs`)
along with the corresponding versions of `Darwin` and `XNU used in each.

It also lists the public (via `libproc.h` header file) functions provided by `libproc` in each version, and
shows in what version new functions were added.

## What is XNU
XNU kernel is part of the Darwin operating system for use in macOS and iOS operating systems.

[Here](https://github.com/apple-oss-distributions/xnu/tags) you can find the XNU version tags that apple has tagged in 
the GH project

## What is Darwin
Darwin is the core Unix operating system of macOS (previously OS X and Mac OS X), iOS, watchOS, tvOS, iPadOS, visionOS, 
and bridgeOS. It previously existed as an independent open-source operating system, first released by Apple Inc. 
in 2000. It is composed of code derived from NeXTSTEP, FreeBSD,[3] other BSD operating systems,[6] Mach, and other 
free software projects' code, as well as code developed by Apple.

# Mapping of macos versions to xnu versions
## macos 14
* 14.0 -> [xnu-10002.1.13](https://newosxbook.com/src.jl?tree=&file=/xnu-10002.1.13)
* 14.1 -> xnu-10002.41.9
* 14.1.1 -> xnu-10002.41.9
* 14.1.2 -> xnu-10002.41.9
* 14.2 -> xnu-10002.61.3
* 14.2.1 -> xnu-10002.61.3
* 14.3 -> xnu-10002.81.5
* 14.3.1 -> xnu-10002.81.5
* 14.4 -> xnu-10063.101.15

## macos 13
* 13.0 -> xnu-8792.41.9
* 13.1 -> xnu-8792.61.2
* 13.2 -> [xnu-8792.81.2](https://newosxbook.com/src.jl?tree=&file=/xnu-8792.81.2)
  * Added
    * `proc_terminate_all_rsr` (NOTE: Not implemented by libproc-rs)
* 13.2.1 -> xnu-8792.81.3
* 13.3 -> xnu-8796.101.5
* 13.4 -> xnu-8796.121.2
* 13.4.1 -> xnu-8796.121.3
* 13.5 -> xnu-8796.141.3
* 13.6 -> xnu-8796.141.3.700.8
* 13.6.1 -> xnu-8796.141.3.701.17
* 13.6.2 -> xnu-8796.141.3.701.17
* 13.6.3 -> xnu-8796.141.3.702.9
* 13.6.4 -> xnu-8796.141.3.703.2
* 13.6.5 -> xnu-8796.141.3.704.6

## macos 12
2021 - Darwin 21, macOS Monterey (Version 12.0)

* 12.0 -> [xnu-8019.30.61 (generic 8019)](https://newosxbook.com/src.jl?tree=&file=/xnu-8019)
  * Added
    * `proc_pidpath_audittoken` (NOTE: Not implemented by libproc-rs)
* 12.0.1 -> xnu-8019.41.5
* 12.1 -> xnu-8019.61.5
* 12.2 -> xnu-8019.80.24
* 12.3 -> xnu-8020.101.4
* 12.3.1 -> xnu-8020.101.4
* 12.4 -> xnu-8020.121.3
* 12.5 -> xnu-8020.140.41
* 12.5.1 -> xnu-8020.141.5
* 12.6 -> xnu-8020.140.49
* 12.6.1 -> xnu-8020.240.7
* 12.6.2 -> xnu-8020.240.14
* 12.6.3 -> xnu-8020.240.18
* 12.6.4 -> xnu-8020.240.18.700.8
* 12.6.6 -> xnu-8020.240.18.701.5
* 12.6.7 -> xnu-8020.240.18.701.6
* 12.6.8 -> xnu-8020.240.18.702.13
* 12.7 -> xnu-8020.240.18.703.5
* 12.7.1 -> xnu-8020.240.18.704.15
* 12.7.2 -> xnu-8020.240.18.705.10
* 12.7.3 -> xnu-8020.240.18.706.2
* 12.7.4 -> xnu-8020.240.18.707.4

## macos 11
2020 - Darwin 20, macOS Big Sur (Version 11.0)

* 11.0 -> xnu-7195.41.8
* 11.0.1 -> xnu-7195.50.7
* 11.1 -> xnu-7195.60.75
* 11.2 -> [xnu-7195.81.3](https://opensource.apple.com/source/xnu/xnu-7195.81.3/)
  * Added 
    * `proc_set_no_smt` (NO_SMT means that on an SMT CPU, this thread must be scheduled alone, 
  with the paired CPU idle. Set NO_SMT on the current proc (all existing and future threads).
  This attribute is inherited on fork and exec.  (NOTE: Not implemented by libproc-rs)
    * `proc_setthread_no_smt` (Set NO_SMT on the current thread)  (NOTE: Not implemented by libproc-rs)
    * `proc_set_csm` (CPU Security Mitigation APIs -  Set CPU security mitigation on the current proc 
  (all existing and future threads). This attribute is inherited on fork and exec)  (NOTE: Not implemented by libproc-rs)
    * `proc_setthread_csm (Set CPU security mitigation on the current thread)  (NOTE: Not implemented by libproc-rs)
* 11.3 -> xnu-7195.101.1
* 11.3.1 -> xnu-7195.101.2
* 11.4 -> xnu-7195.121.3
* 11.5 -> xnu-7195.141.2
* 11.6 -> xnu-7195.141.6
* 11.6.1 -> xnu-7195.141.8
* 11.6.2 -> xnu-7195.141.14
* 11.6.3 -> xnu-7195.141.19
* 11.6.5 -> xnu-7195.141.26
* 11.6.6 -> xnu-7195.141.29
* 11.6.8 -> xnu-7195.141.32
* 11.7 -> xnu-7195.141.39
* 11.7.1 -> xnu-7195.141.42
* 11.7.2 -> xnu-7195.141.46
* 11.7.3 -> xnu-7195.141.49
* 11.7.5 -> xnu-7195.141.49.700.6
* 11.7.7 -> xnu-7195.141.49.701.3
* 11.7.8 -> xnu-7195.141.49.701.4
* 11.7.9 -> xnu-7195.141.49.702.12

[!NOTE] Versions below here (prior to macOS 11) are not supported in GitHub Actions and hence are not tested
as part of `libproc-rs` CI process.

## macos 10.15
2019 - Darwin 19, macOS Catalina (Version 10.15)

10.15.1 -> [xnu-6153.41.3 (closest 6153.11.26)](https://newosxbook.com/src.jl?tree=&file=/xnu-6153.11.26)
10.15.2 -> xnu-6153.61.1
10.15.3 ->xnu-6153.81.5
10.15.4 -> xnu-6153.101.6
10.15.5 -> xnu-6153.121.1
10.15.6 -> [xnu-6153.141.1](https://opensource.apple.com/source/xnu/xnu-6153.141.1/)
10.15.7 -> xnu-6153.141.2

## macos 10.14 
2018 - Darwin 18, macOS Mojave (Version 10.14)

* 10.14.1 -> [xnu-4903.221.2](https://opensource.apple.com/source/xnu/xnu-4903.221.2/)
  * No additions, as xnu-4570.71.2 below
* 10.14.2 -> [xnu-4903.231.4](https://opensource.apple.com/source/xnu/xnu-4903.231.4/)
  * No additions, as xnu-4570.71.2 below
* 10.14.3 -> [xnu-4903.241.1](https://opensource.apple.com/source/xnu/xnu-4903.241.1/)
  * No additions, as xnu-4570.71.2 below
* 10.14.4 -> xnu-4903.251.3
* 10.14.5 -> xnu-4903.261.4
* 10.14.6 -> [xnu-4903.270.47](https://opensource.apple.com/source/xnu/xnu-4903.270.47/)
  * No additions, as xnu-4570.71.2 below

## macos 10.13
2017 - Darwin 17, macOS High Sierra (Version 10.13)

10.13.6 -> xnu-4570.71.46 - xnu-4570.71.82.8 (approx [xnu-4570.71.2](https://opensource.apple.com/source/xnu/xnu-4570.71.2/))

Methods inherited from previous versions of XNU, Darwin and macOS that
are present in 10.13.6:
* proc_listpidspath
* proc_listpids
* proc_listallpids
* proc_listpgrppids
* proc_listchildpids
* proc_pidinfo
* proc_pidfdinfo
* proc_pidfileportinfo
* proc_name
* proc_regionfilename
* proc_kmsgbuf
* proc_pidpath
* proc_libversion
* proc_pid_rusage
* proc_setpcontrol
* proc_track_dirty
* proc_set_dirty
* proc_get_dirty
* proc_clear_dirty
* proc_terminate
* proc_udata_info

# Reference docs used
* [Darwin page on operating-system.org](https://www.operating-system.org/betriebssystem/_english/bs-darwin.htm)
* [Darwin/XNU Github Project](https://github.com/apple/darwin-xnu)
* [Apple OS Distributions Github Project](https://github.com/apple-oss-distributions/xnu)
* [Darwin Wikipedia page](https://en.wikipedia.org/wiki/Darwin_(operating_system))