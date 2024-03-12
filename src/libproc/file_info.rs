extern crate libc;

use std::mem;

use crate::libproc::helpers;
use crate::libproc::proc_pid::{ListPIDInfo, PidInfoFlavor};

#[cfg(target_os = "macos")]
use self::libc::c_void;
#[cfg(target_os = "macos")]
use crate::osx_libproc_bindings::proc_pidfdinfo;

/// Flavor of Pid `FileDescriptor` info for different types of File Descriptors
pub enum PIDFDInfoFlavor {
    /// `VNodeInfo`
    VNodeInfo = 1,
    /// `VNodePathInfo`
    VNodePathInfo = 2,
    /// `SocketInfo`
    SocketInfo = 3,
    /// `PSEMInfo`
    PSEMInfo = 4,
    /// `PSHMInfo`
    PSHMInfo = 5,
    /// `PipeInfo`
    PipeInfo = 6,
    /// `KQueueInfo`
    KQueueInfo = 7,
    /// `AppleTalkInfo`
    ATalkInfo = 8,
}

/// Struct for Listing File Descriptors
pub struct ListFDs;

impl ListPIDInfo for ListFDs {
    type Item = ProcFDInfo;
    fn flavor() -> PidInfoFlavor {
        PidInfoFlavor::ListFDs
    }
}

/// Struct to hold info about a Processes `FileDescriptor` Info
#[repr(C)]
pub struct ProcFDInfo {
    /// `FileDescriptor`
    pub proc_fd: i32,
    /// `FileDescriptor` type
    pub proc_fdtype: u32,
}

/// Enum for different `FileDescriptor` types
#[derive(Copy, Clone, Debug)]
pub enum ProcFDType {
    /// `AppleTalk`
    ATalk = 0,
    /// Vnode
    VNode = 1,
    /// Socket
    Socket = 2,
    /// POSIX shared memory
    PSHM = 3,
    /// POSIX semaphore
    PSEM = 4,
    /// Kqueue
    KQueue = 5,
    /// Pipe
    Pipe = 6,
    /// `FSEvents`
    FSEvents = 7,
    /// `NetPolicy`
    NetPolicy = 9,
    /// Unknown
    Unknown,
}

impl From<u32> for ProcFDType {
    fn from(value: u32) -> ProcFDType {
        match value {
            0 => ProcFDType::ATalk,
            1 => ProcFDType::VNode,
            2 => ProcFDType::Socket,
            3 => ProcFDType::PSHM,
            4 => ProcFDType::PSEM,
            5 => ProcFDType::KQueue,
            6 => ProcFDType::Pipe,
            7 => ProcFDType::FSEvents,
            _ => ProcFDType::Unknown,
        }
    }
}

/// The `PIDFDInfo` trait is needed for polymorphism on pidfdinfo types, also abstracting flavor in order to provide
/// type-guaranteed flavor correctness
pub trait PIDFDInfo: Default {
    /// Return the Pid File Descriptor Info flavor of the implementing struct
    fn flavor() -> PIDFDInfoFlavor;
}

/// Returns the information about file descriptors of the process that match pid passed in.
///
/// # Errors
///
/// Will return `Err`if the underlying Darwin method `proc_pidfdinfo` returns 0
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
///                             }
///                             _ => (),
///                         }
///                     }
///                 }
///                 _ => (),
///             }
///         }
///     }
/// }
/// ```
///
#[cfg(target_os = "macos")]
pub fn pidfdinfo<T: PIDFDInfo>(pid: i32, fd: i32) -> Result<T, String> {
    let flavor = T::flavor() as i32;
    // No `T` will have size greater than `i32::MAX` so no truncation
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    let buffer_size = mem::size_of::<T>() as i32;
    let mut pidinfo = T::default();
    let buffer_ptr = std::ptr::from_mut::<T>(&mut pidinfo).cast::<c_void>();
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

#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
pub fn pidfdinfo<T: PIDFDInfo>(_pid: i32, _fd: i32) -> Result<T, String> {
    unimplemented!()
}

#[cfg(all(test, target_os = "macos"))]
mod test {
    use crate::libproc::bsd_info::BSDInfo;
    use crate::libproc::file_info::{ListFDs, ProcFDType};
    use crate::libproc::net_info::{SocketFDInfo, SocketInfoKind};
    use crate::libproc::proc_pid::{listpidinfo, pidinfo};

    use super::pidfdinfo;

    #[test]
    #[allow(clippy::cast_possible_wrap)]
    fn pidfdinfo_test() {
        use std::net::TcpListener;
        use std::process;
        let pid = process::id() as i32;

        let _listener = TcpListener::bind("127.0.0.1:65535");

        let info = pidinfo::<BSDInfo>(pid, 0).expect("pidinfo() failed");
        let fds =
            listpidinfo::<ListFDs>(pid, info.pbi_nfiles as usize).expect("listpidinfo() failed");
        for fd in fds {
            if let ProcFDType::Socket = fd.proc_fdtype.into() {
                let socket =
                    pidfdinfo::<SocketFDInfo>(pid, fd.proc_fd).expect("pidfdinfo() failed");
                if let SocketInfoKind::Tcp = socket.psi.soi_kind.into() {
                    unsafe {
                        let info = socket.psi.soi_proto.pri_tcp;
                        assert_eq!(socket.psi.soi_protocol, libc::IPPROTO_TCP);
                        assert_eq!(info.tcpsi_ini.insi_lport, 65535);
                    }
                }
            }
        }
    }
}
