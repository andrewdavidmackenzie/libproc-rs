TODO
= 
These are just notes I've gathered and a list of the status of different calls
and ideas for others.

Cross-Platform
==
* am_root - libc::geteuid() is unstable still.
* name - Returns the name of the process with the specified pid
* listpids - Only implements listing all pid types
* libversion - Just returns error message on linux as no lib used
* cwdself - just wraps env::current_dir() of rust so not so useful
* pidpath - Returns the path of the file being run as the process with specified pid
* kmsgbuf - get the contents of the kernel message buffer

Pending
==
Mac OS
=== 
* pidcwd                      Can't see yet how to do this in Mac OS
                
Linux
===
* listpidinfo
* regionfilename
* pidinfo
* pidfdinfo

Ideas
==
Here is a lits of things I see would be easy to do on Linux. I still need
to look into how they could be done on Mac.

* command line arguments
* environment vars
* uid running a process
* parent pid (ppid)