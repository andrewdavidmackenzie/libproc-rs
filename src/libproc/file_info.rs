extern crate libc;

use std::mem;

use crate::libproc::helpers;
use crate::libproc::proc_pid::{ListPIDInfo, PidInfoFlavor};

use self::libc::{c_int, c_void, int32_t, uint32_t};

// this extern block links to the libproc library
// Original signatures of functions can be found at http://opensource.apple.com/source/Libc/Libc-594.9.4/darwin/libproc.c
#[link(name = "proc", kind = "dylib")]
extern {
    fn proc_pidfdinfo(pid : c_int, fd : c_int, flavor : c_int, buffer : *mut c_void, buffersize : c_int) -> c_int;
}

pub enum PIDFDInfoFlavor {
    VNodeInfo       = 1,
    VNodePathInfo   = 2,
    SocketInfo      = 3,
    PSEMInfo        = 4,
    PSHMInfo        = 5,
    PipeInfo        = 6,
    KQueueInfo      = 7,
    ATalkInfo       = 8
}

pub struct ListFDs;

impl ListPIDInfo for ListFDs {
    type Item = ProcFDInfo;
    fn flavor() -> PidInfoFlavor { PidInfoFlavor::ListFDs }
}

#[repr(C)]
pub struct ProcFDInfo {
    pub proc_fd: int32_t,
    pub proc_fdtype: uint32_t,
}

#[derive(Copy, Clone, Debug)]
pub enum ProcFDType {
    /// AppleTalk
    ATalk    = 0,
    /// Vnode
    VNode    = 1,
    /// Socket
    Socket   = 2,
    /// POSIX shared memory
    PSHM     = 3,
    /// POSIX semaphore
    PSEM     = 4,
    /// Kqueue
    KQueue   = 5,
    /// Pipe
    Pipe     = 6,
    /// FSEvents
    FSEvents = 7,
    /// Unknown
    Unknown,
}

impl From<uint32_t> for ProcFDType {
    fn from(value: uint32_t) -> ProcFDType {
        match value {
            0 => ProcFDType::ATalk   ,
            1 => ProcFDType::VNode   ,
            2 => ProcFDType::Socket  ,
            3 => ProcFDType::PSHM    ,
            4 => ProcFDType::PSEM    ,
            5 => ProcFDType::KQueue  ,
            6 => ProcFDType::Pipe    ,
            7 => ProcFDType::FSEvents,
            _ => ProcFDType::Unknown ,
        }
    }
}

// This trait is needed for polymorphism on pidfdinfo types, also abstracting flavor in order to provide
// type-guaranteed flavor correctness
pub trait PIDFDInfo: Default {
    fn flavor() -> PIDFDInfoFlavor;
}

/// Returns the information about file descriptors of the process that match pid passed in.
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use std::net::TcpListener;
/// use libproc::libproc::proc_pid::{listpidinfo, pidinfo, ListThreads};
/// use libproc::libproc::bsd_info::{BSDInfo};
/// use libproc::libproc::net_info::{SocketFDInfo, SocketInfoKind};
/// use libproc::libproc::file_info::{pidfdinfo, ListFDs, ProcFDType};
/// use std::process;
///
/// let pid = process::id() as i32;
///
/// // Open TCP port:8000 to test.
/// let _listener = TcpListener::bind("127.0.0.1:8000");
///
/// if let Ok(info) = pidinfo::<BSDInfo>(pid, 0) {
///     if let Ok(fds) = listpidinfo::<ListFDs>(pid, info.pbi_nfiles as usize) {
///         for fd in &fds {
///             match fd.proc_fdtype.into() {
///                 ProcFDType::Socket => {
///                     if let Ok(socket) = pidfdinfo::<SocketFDInfo>(pid, fd.proc_fd) {
///                         match socket.psi.soi_kind.into() {
///                             SocketInfoKind::Tcp => {
///                                 // access to the member of `soi_proto` is unsafe becasuse of union type.
///                                let info = unsafe { socket.psi.soi_proto.pri_tcp };
///
///                                 // change endian and cut off because insi_lport is network endian and 16bit witdh.
///                                 let mut port = 0;
///                                 port |= info.tcpsi_ini.insi_lport >> 8 & 0x00ff;
///                                 port |= info.tcpsi_ini.insi_lport << 8 & 0xff00;
///
///                                 // access to the member of `insi_laddr` is unsafe becasuse of union type.
///                                 let s_addr = unsafe { info.tcpsi_ini.insi_laddr.ina_46.i46a_addr4.s_addr };
///
///                                 // change endian because insi_laddr is network endian.
///                                 let mut addr = 0;
///                                 addr |= s_addr >> 24 & 0x000000ff;
///                                 addr |= s_addr >> 8  & 0x0000ff00;
///                                 addr |= s_addr << 8  & 0x00ff0000;
///                                 addr |= s_addr << 24 & 0xff000000;
///
///                                 println!("{}.{}.{}.{}:{}", addr >> 24 & 0xff, addr >> 16 & 0xff, addr >> 8 & 0xff, addr & 0xff, port);
///                             },
///                             _ => (),
///                         }
///                     }
///                 },
///                 _ => (),
///             }
///         }
///     }
/// }
/// ```
///
pub fn pidfdinfo<T: PIDFDInfo>(pid : i32, fd: int32_t) -> Result<T, String> {
    let flavor = T::flavor() as i32;
    let buffer_size = mem::size_of::<T>() as i32;
    let mut pidinfo = T::default();
    let buffer_ptr = &mut pidinfo as *mut _ as *mut c_void;
    let ret: i32;

    unsafe {
        ret = proc_pidfdinfo(pid, fd, flavor, buffer_ptr, buffer_size);
    };

    if ret <= 0 {
        Err(helpers::get_errno_with_message(ret))
    } else {
        Ok(pidinfo)
    }
}

#[cfg(test)]
mod test {
    use crate::libproc::bsd_info::BSDInfo;
    use crate::libproc::file_info::{ListFDs, ProcFDType};
    use crate::libproc::net_info::{SocketFDInfo, SocketInfoKind};
    use crate::libproc::proc_pid::{listpidinfo, pidinfo};
    use super::pidfdinfo;

    #[test]
    fn pidfdinfo_test() {
        use std::process;
        use std::net::TcpListener;
        let pid = process::id() as i32;

        let _listener = TcpListener::bind("127.0.0.1:65535");

        let info = pidinfo::<BSDInfo>(pid, 0).unwrap();
        let fds = listpidinfo::<ListFDs>(pid, info.pbi_nfiles as usize).unwrap();
        for fd in fds {
            match fd.proc_fdtype.into() {
                ProcFDType::Socket => {
                    let socket = pidfdinfo::<SocketFDInfo>(pid, fd.proc_fd).unwrap();
                    match socket.psi.soi_kind.into() {
                        SocketInfoKind::Tcp => unsafe {
                            let info = socket.psi.soi_proto.pri_tcp;
                            assert_eq!(socket.psi.soi_protocol, libc::IPPROTO_TCP);
                            assert_eq!(info.tcpsi_ini.insi_lport as u32, 65535);
                        }
                        _ => (),
                    }
                },
                _ => (),
            }
        }
    }
}