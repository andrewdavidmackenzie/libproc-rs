extern crate libc;

use crate::libproc::file_info::{PIDFDInfo, PIDFDInfoFlavor};

use self::libc::{c_char, c_int, c_short, c_uchar, c_ushort, gid_t, IF_NAMESIZE, in6_addr, in_addr,
                 int32_t, int64_t, off_t, SOCK_MAXADDRLEN, sockaddr_un, uid_t, uint16_t, uint32_t,
                 uint64_t, uint8_t};

#[repr(C)]
#[derive(Default)]
pub struct SocketFDInfo {
    pub pfi: ProcFileInfo,
    pub psi: SocketInfo,
}

#[repr(C)]
#[derive(Default)]
pub struct ProcFileInfo {
    pub fi_openflags: uint32_t,
    pub fi_status   : uint32_t,
    pub fi_offset   : off_t,
    pub fi_type     : int32_t,
    pub rfu_1       : int32_t,
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
    pub soi_so: uint64_t,
    pub soi_pcb: uint64_t,
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
    pub soi_oobmark: uint32_t,
    pub soi_rcv: SockBufInfo,
    pub soi_snd: SockBufInfo,
    pub soi_kind: c_int,
    pub rfu_1: uint32_t,
    pub soi_proto: SocketInfoProto,
}

#[repr(C)]
#[derive(Default)]
pub struct VInfoStat {
    pub vst_dev: uint32_t,
    pub vst_mode: uint16_t,
    pub vst_nlink: uint16_t,
    pub vst_ino: uint64_t,
    pub vst_uid: uid_t,
    pub vst_gid: gid_t,
    pub vst_atime: int64_t,
    pub vst_atimensec: int64_t,
    pub vst_mtime: int64_t,
    pub vst_mtimensec: int64_t,
    pub vst_ctime: int64_t,
    pub vst_ctimensec: int64_t,
    pub vst_birthtime: int64_t,
    pub vst_birthtimensec: int64_t,
    pub vst_size: off_t,
    pub vst_blocks: int64_t,
    pub vst_blksize: int32_t,
    pub vst_flags: uint32_t,
    pub vst_gen: uint32_t,
    pub vst_rdev: uint32_t,
    pub vst_qspare: [int64_t; 2],
}

#[repr(C)]
#[derive(Default)]
pub struct SockBufInfo {
    pub sbi_cc: uint32_t,
    pub sbi_hiwat: uint32_t,
    pub sbi_mbcnt: uint32_t,
    pub sbi_mbmax: uint32_t,
    pub sbi_lowat: uint32_t,
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
    pub i46a_pad32: [uint32_t; 3],
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
    pub insi_gencnt: uint64_t,
    pub insi_flags: uint32_t,
    pub insi_flow: uint32_t,
    pub insi_vflag: uint8_t,
    pub insi_ip_ttl: uint8_t,
    pub rfu_1: uint32_t,
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
    pub in6_hlim: uint8_t,
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
    pub tcpsi_flags: uint32_t,
    pub rfu_1: uint32_t,
    pub tcpsi_tp: uint64_t,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct UnSockInfo {
    pub unsi_conn_so: uint64_t,
    pub unsi_conn_pcb: uint64_t,
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
    pub ndrvsi_if_family: uint32_t,
    pub ndrvsi_if_unit: uint32_t,
    pub ndrvsi_if_name: [c_char; IF_NAMESIZE],
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct KernEventInfo {
    pub kesi_vendor_code_filter: uint32_t,
    pub kesi_class_filter: uint32_t,
    pub kesi_subclass_filter: uint32_t,
}

const MAX_KCTL_NAME: usize = 96;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KernCtlInfo {
    pub kcsi_id: uint32_t,
    pub kcsi_reg_unit: uint32_t,
    pub kcsi_flags: uint32_t,
    pub kcsi_recvbufsize: uint32_t,
    pub kcsi_sendbufsize: uint32_t,
    pub kcsi_unit: uint32_t,
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