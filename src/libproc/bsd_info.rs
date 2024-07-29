use crate::libproc::proc_pid::{PIDInfo, PidInfoFlavor};
#[cfg(target_os = "macos")]
pub use crate::osx_libproc_bindings::proc_bsdinfo as BSDInfo;

/// # Safety
///
/// `BSDInfo` is correctly sized and flavor indicates the right struct.
#[cfg(target_os = "macos")]
unsafe impl PIDInfo for BSDInfo {
    fn flavor() -> PidInfoFlavor {
        PidInfoFlavor::TBSDInfo
    }
}
