extern crate libc;

use crate::libproc::file_info::{PIDFDInfo, PIDFDInfoFlavor};

use self::libc::{c_char, c_int, c_short, c_uchar, c_ushort, gid_t, IF_NAMESIZE, in6_addr, in_addr,
                 off_t, SOCK_MAXADDRLEN, sockaddr_un, uid_t};

#[repr(C)]
#[derive(Default)]
pub struct SocketFDInfo {
    pub pfi: ProcFileInfo,
    pub psi: SocketInfo,
}

#[repr(C)]
#[derive(Default)]
pub struct ProcFileInfo {
    pub fi_openflags: u32,
    pub fi_status   : u32,
    pub fi_offset   : off_t,
    pub fi_type     : i32,
    pub rfu_1       : i32,
}

impl PIDFDInfo for SocketFDInfo {
    fn flavor() -> PIDFDInfoFlavor { PIDFDInfoFlavor::SocketInfo }
}

#[derive(Copy, Clone, Debug)]
pub enum SocketInfoKind {
    Generic = 0,
    /// IPv4 and IPv6 Sockets
    In = 1,
    /// TCP Sockets
    Tcp = 2,
    /// Unix Domain Sockets
    Un = 3,
    /// PF_NDRV Sockets
    Ndrv = 4,
    /// Kernel Event Sockets
    KernEvent = 5,
    /// Kernel Control Sockets
    KernCtl = 6,
    /// Unknown
    Unknown,
}

impl From<c_int> for SocketInfoKind {
    fn from(value: c_int) -> SocketInfoKind {
        match value {
            0 => SocketInfoKind::Generic,
            1 => SocketInfoKind::In,
            2 => SocketInfoKind::Tcp,
            3 => SocketInfoKind::Un,
            4 => SocketInfoKind::Ndrv,
            5 => SocketInfoKind::KernEvent,
            6 => SocketInfoKind::KernCtl,
            _ => SocketInfoKind::Unknown,
        }
    }
}

#[repr(C)]
#[derive(Default)]
pub struct SocketInfo {
    pub soi_stat: VInfoStat,
    pub soi_so: u64,
    pub soi_pcb: u64,
    pub soi_type: c_int,
    pub soi_protocol: c_int,
    pub soi_family: c_int,
    pub soi_options: c_short,
    pub soi_linger: c_short,
    pub soi_state: c_short,
    pub soi_qlen: c_short,
    pub soi_incqlen: c_short,
    pub soi_qlimit: c_short,
    pub soi_timeo: c_short,
    pub soi_error: c_ushort,
    pub soi_oobmark: u32,
    pub soi_rcv: SockBufInfo,
    pub soi_snd: SockBufInfo,
    pub soi_kind: c_int,
    pub rfu_1: u32,
    pub soi_proto: SocketInfoProto,
}

#[repr(C)]
#[derive(Default)]
pub struct VInfoStat {
    pub vst_dev: u32,
    pub vst_mode: u16,
    pub vst_nlink: u16,
    pub vst_ino: u64,
    pub vst_uid: uid_t,
    pub vst_gid: gid_t,
    pub vst_atime: i64,
    pub vst_atimensec: i64,
    pub vst_mtime: i64,
    pub vst_mtimensec: i64,
    pub vst_ctime: i64,
    pub vst_ctimensec: i64,
    pub vst_birthtime: i64,
    pub vst_birthtimensec: i64,
    pub vst_size: off_t,
    pub vst_blocks: i64,
    pub vst_blksize: i32,
    pub vst_flags: u32,
    pub vst_gen: u32,
    pub vst_rdev: u32,
    pub vst_qspare: [i64; 2],
}

#[repr(C)]
#[derive(Default)]
pub struct SockBufInfo {
    pub sbi_cc: u32,
    pub sbi_hiwat: u32,
    pub sbi_mbcnt: u32,
    pub sbi_mbmax: u32,
    pub sbi_lowat: u32,
    pub sbi_flags: c_short,
    pub sbi_timeo: c_short,
}

#[repr(C)]
pub union SocketInfoProto {
    pub pri_in: InSockInfo,
    pub pri_tcp: TcpSockInfo,
    pub pri_un: UnSockInfo,
    pub pri_ndrv: NdrvInfo,
    pub pri_kern_event: KernEventInfo,
    pub pri_kern_ctl: KernCtlInfo,
}

impl Default for SocketInfoProto {
    fn default() -> SocketInfoProto {
        SocketInfoProto {
            pri_in: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct In4In6Addr {
    pub i46a_pad32: [u32; 3],
    pub i46a_addr4: in_addr,
}

impl Default for In4In6Addr {
    fn default() -> In4In6Addr {
        In4In6Addr {
            i46a_pad32: [0; 3],
            i46a_addr4: in_addr { s_addr: 0 },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct InSockInfo {
    pub insi_fport: c_int,
    pub insi_lport: c_int,
    pub insi_gencnt: u64,
    pub insi_flags: u32,
    pub insi_flow: u32,
    pub insi_vflag: u8,
    pub insi_ip_ttl: u8,
    pub rfu_1: u32,
    pub insi_faddr: InSIAddr,
    pub insi_laddr: InSIAddr,
    pub insi_v4: InSIV4,
    pub insi_v6: InSIV6,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct InSIV4 {
    pub in4_top: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct InSIV6 {
    pub in6_hlim: u8,
    pub in6_cksum: c_int,
    pub in6_ifindex: c_ushort,
    pub in6_hops: c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union InSIAddr {
    pub ina_46: In4In6Addr,
    pub ina_6: in6_addr,
}

impl Default for InSIAddr {
    fn default() -> InSIAddr {
        InSIAddr {
            ina_46: Default::default(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TcpSIState {
    /// Closed
    Closed = 0,
    /// Listening for connection
    Listen = 1,
    /// Active, have sent syn
    SynSent = 2,
    /// Have send and received syn
    SynReceived = 3,
    /// Established
    Established = 4,
    /// Rcvd fin, waiting for close
    CloseWait = 5,
    /// Have closed, sent fin
    FinWait1 = 6,
    /// Closed xchd FIN; await FIN ACK
    Closing = 7,
    /// Had fin and close; await FIN ACK
    LastAck = 8,
    /// Have closed, fin is acked
    FinWait2 = 9,
    /// In 2*msl quiet wait after close
    TimeWait = 10,
    /// Pseudo state: reserved
    Reserved = 11,
    /// Unknown
    Unknown,
}

impl From<c_int> for TcpSIState {
    fn from(value: c_int) -> TcpSIState {
        match value {
            0 => TcpSIState::Closed,
            1 => TcpSIState::Listen,
            2 => TcpSIState::SynSent,
            3 => TcpSIState::SynReceived,
            4 => TcpSIState::Established,
            5 => TcpSIState::CloseWait,
            6 => TcpSIState::FinWait1,
            7 => TcpSIState::Closing,
            8 => TcpSIState::LastAck,
            9 => TcpSIState::FinWait2,
            10 => TcpSIState::TimeWait,
            11 => TcpSIState::Reserved,
            _ => TcpSIState::Unknown,
        }
    }
}

const TSI_T_NTIMERS: usize = 4;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct TcpSockInfo {
    pub tcpsi_ini: InSockInfo,
    pub tcpsi_state: c_int,
    pub tcpsi_timer: [c_int; TSI_T_NTIMERS],
    pub tcpsi_mss: c_int,
    pub tcpsi_flags: u32,
    pub rfu_1: u32,
    pub tcpsi_tp: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct UnSockInfo {
    pub unsi_conn_so: u64,
    pub unsi_conn_pcb: u64,
    pub unsi_addr: UnSIAddr,
    pub unsi_caddr: UnSIAddr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union UnSIAddr {
    pub ua_sun: sockaddr_un,
    pub ua_dummy: [c_char; SOCK_MAXADDRLEN as usize],
}

impl Default for UnSIAddr {
    fn default() -> UnSIAddr {
        UnSIAddr {
            ua_dummy: [0; SOCK_MAXADDRLEN as usize],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct NdrvInfo {
    pub ndrvsi_if_family: u32,
    pub ndrvsi_if_unit: u32,
    pub ndrvsi_if_name: [c_char; IF_NAMESIZE],
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct KernEventInfo {
    pub kesi_vendor_code_filter: u32,
    pub kesi_class_filter: u32,
    pub kesi_subclass_filter: u32,
}

const MAX_KCTL_NAME: usize = 96;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KernCtlInfo {
    pub kcsi_id: u32,
    pub kcsi_reg_unit: u32,
    pub kcsi_flags: u32,
    pub kcsi_recvbufsize: u32,
    pub kcsi_sendbufsize: u32,
    pub kcsi_unit: u32,
    pub kcsi_name: [c_char; MAX_KCTL_NAME],
}

impl Default for KernCtlInfo {
    fn default() -> KernCtlInfo {
        KernCtlInfo {
            kcsi_id: 0,
            kcsi_reg_unit: 0,
            kcsi_flags: 0,
            kcsi_recvbufsize: 0,
            kcsi_sendbufsize: 0,
            kcsi_unit: 0,
            kcsi_name: [0; MAX_KCTL_NAME],
        }
    }
}