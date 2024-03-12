use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};
#[cfg(target_os = "macos")]
pub use crate::osx_libproc_bindings::proc_bsdinfo as BSDInfo;

#[cfg(target_os = "macos")]
impl PIDInfo for BSDInfo {
    fn flavor() -> PidInfoFlavor {
        PidInfoFlavor::TBSDInfo
    }
}
