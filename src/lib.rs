//! # rpm-pkg-count
//!
//! Counts installed RPM packages using `librpm`.
//!
//! > Note: This crate does **not** make use of `librpm-sys` but links to the C
//! > library itself.
//!
//! ## Requirements
//!
//! In order to compile this crate, one must have `librpm` installed on the system.
//! It is usually provided by package managers under the name `rpm-devel` (e.g.,
//! OpenSUSE), `rpm-tools` (e.g., Arch Linux) or `librpm-dev` (e.g., Debian).
//!
//! ## Usage
//!
//! The crate exposes exaclty _one_ public function which takes no arguments and
//! returns the package count as a `u32`. An example usage is shown here:
//!
//! ```
//! use rpm_pkg_count::count;
//!
//! println!("{} packages installed", unsafe { count() });
//! ```
#![warn(rust_2018_idioms)]
#![deny(missing_docs)]

mod ffi;
use ffi::*;

/// Return the count of installed RPM packages as a `u32`.
///
/// Code is manually translated from C as used by `fastfetch`:
/// <https://github.com/LinusDierheimer/fastfetch/blob/e837e1e1d1e5a7eba02235748cd1a20a72bc28f9/src/detection/packages/packages_linux.c#L230-L264>
#[allow(clippy::missing_safety_doc)]
pub unsafe fn count() -> u32 {
    if rpmReadConfigFiles(std::ptr::null::<i8>(), std::ptr::null::<i8>()) != 0 {
        return 0;
    }

    let ts = rpmtsCreate();
    if ts.is_null() {
        return 0;
    }

    let mi = rpmtsInitIterator(
        ts,
        rpmDbiTag_e_RPMDBI_LABEL as i32,
        std::ptr::null::<::std::os::raw::c_void>(),
        0,
    );
    if mi.is_null() {
        rpmtsFree(ts);
        return 0;
    }

    let count = rpmdbGetIteratorCount(mi);

    rpmdbFreeIterator(mi);
    rpmtsFree(ts);

    count as u32
}
