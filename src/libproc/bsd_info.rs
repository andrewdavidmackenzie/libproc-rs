use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};
pub use crate::osx_libproc_bindings::proc_bsdinfo as BSDInfo;

impl PIDInfo for BSDInfo {
    fn flavor() -> PidInfoFlavor { PidInfoFlavor::TBSDInfo }
}
