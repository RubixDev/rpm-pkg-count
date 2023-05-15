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
//! The crate provides two cargo features, exactly **one** of them must be enabled.
//!
//! 1. `compile-time`: Link to librpm during compile-time using Rust's `extern "C"`
//!    functionality. This requires librpm to be installed on every target's system
//!    for a binary to run at all.
//! 2. `runtime`: Link to librpm during runtime using the
//!    [`libloading` crate](https://crates.io/crates/libloading). This way,
//!    `count()` simply returns `None` if librpm is not installed on the target
//!    system.
//!
//! The crate then exposes exactly _one_ public function which takes no arguments
//! and returns the package count as an `Option<u32>`. An example usage is shown
//! here:
//!
//! ```
//! use rpm_pkg_count::count;
//!
//! match unsafe { count() } {
//!     Some(count) => println!("{count} packages installed."),
//!     None => println!("packages could not be counted"),
//! }
//! ```
#![warn(rust_2018_idioms)]
#![deny(missing_docs)]

#[cfg(any(
    not(any(feature = "runtime", feature = "compile-time")),
    all(feature = "runtime", feature = "compile-time")
))]
compile_error!("Exactly one of the features `runtime` or `compile-time` is required.");

#[cfg(any(feature = "runtime", feature = "compile-time"))]
mod ffi_types;

#[cfg(feature = "compile-time")]
mod ffi;

/// Return the count of installed RPM packages as a `u32`.
///
/// Code is manually translated from C as used by `fastfetch`:
/// <https://github.com/LinusDierheimer/fastfetch/blob/e837e1e1d1e5a7eba02235748cd1a20a72bc28f9/src/detection/packages/packages_linux.c#L230-L264>
#[allow(clippy::missing_safety_doc)]
#[cfg(feature = "compile-time")]
pub unsafe fn count() -> Option<u32> {
    use ffi::*;
    use ffi_types::*;

    if rpmReadConfigFiles(std::ptr::null(), std::ptr::null()) != 0 {
        return None;
    }

    let ts = rpmtsCreate();
    if ts.is_null() {
        return None;
    }

    let mi = rpmtsInitIterator(ts, rpmDbiTag_e_RPMDBI_LABEL as i32, std::ptr::null(), 0);
    if mi.is_null() {
        rpmtsFree(ts);
        return None;
    }

    let count = rpmdbGetIteratorCount(mi);

    rpmdbFreeIterator(mi);
    rpmtsFree(ts);

    Some(count as u32)
}

/// Return the count of installed RPM packages as a `u32`.
///
/// Code is manually translated from C as used by `fastfetch`:
/// <https://github.com/LinusDierheimer/fastfetch/blob/e837e1e1d1e5a7eba02235748cd1a20a72bc28f9/src/detection/packages/packages_linux.c#L230-L264>
#[allow(clippy::missing_safety_doc, non_snake_case)]
#[cfg(feature = "runtime")]
pub unsafe fn count() -> Option<u32> {
    use ffi_types::*;
    use libloading::{Library, Symbol};

    // dynamically load the `rpm` library from the system
    let lib = Library::new(libloading::library_filename("rpm")).ok()?;

    // and the required function symbols
    let rpmReadConfigFiles: Symbol<
        '_,
        unsafe extern "C" fn(
            file: *const ::std::os::raw::c_char,
            target: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    > = lib.get(b"rpmReadConfigFiles").ok()?;
    let rpmdbFreeIterator: Symbol<
        '_,
        unsafe extern "C" fn(mi: rpmdbMatchIterator) -> rpmdbMatchIterator,
    > = lib.get(b"rpmdbFreeIterator").ok()?;
    let rpmdbGetIteratorCount: Symbol<
        '_,
        unsafe extern "C" fn(mi: rpmdbMatchIterator) -> ::std::os::raw::c_int,
    > = lib.get(b"rpmdbGetIteratorCount").ok()?;
    let rpmtsCreate: Symbol<'_, unsafe extern "C" fn() -> rpmts> = lib.get(b"rpmtsCreate").ok()?;
    let rpmtsFree: Symbol<'_, unsafe extern "C" fn(ts: rpmts) -> rpmts> =
        lib.get(b"rpmtsFree").ok()?;
    let rpmtsInitIterator: Symbol<
        '_,
        unsafe extern "C" fn(
            ts: rpmts,
            rpmtag: rpmDbiTagVal,
            keyp: *const ::std::os::raw::c_void,
            keylen: size_t,
        ) -> rpmdbMatchIterator,
    > = lib.get(b"rpmtsInitIterator").ok()?;

    if rpmReadConfigFiles(std::ptr::null(), std::ptr::null()) != 0 {
        return None;
    }

    let ts = rpmtsCreate();
    if ts.is_null() {
        return None;
    }

    let mi = rpmtsInitIterator(ts, rpmDbiTag_e_RPMDBI_LABEL as i32, std::ptr::null(), 0);
    if mi.is_null() {
        rpmtsFree(ts);
        return None;
    }

    let count = rpmdbGetIteratorCount(mi);

    rpmdbFreeIterator(mi);
    rpmtsFree(ts);

    Some(count as u32)
}
