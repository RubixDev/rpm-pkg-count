# rpm-pkg-count

Counts installed RPM packages using `librpm`.

> Note: This crate does **not** make use of `librpm-sys` but links to the C
> library itself.

## Requirements

In order to compile this crate, one must have `librpm` installed on the system.
It is usually provided by package managers under the name `rpm-devel` (e.g.,
OpenSUSE), `rpm-tools` (e.g., Arch Linux) or `librpm-dev` (e.g., Debian).

## Usage

The crate provides two cargo features, exactly **one** of them must be enabled.

1. `compile-time`: Link to librpm during compile-time using Rust's `extern "C"`
   functionality. This requires librpm to be installed on every target's system
   for a binary to run at all.
2. `runtime`: Link to librpm during runtime using the
   [`libloading` crate](https://crates.io/crates/libloading). This way,
   `count()` simply returns `None` if librpm is not installed on the target
   system.

The crate then exposes exactly _one_ public function which takes no arguments
and returns the package count as an `Option<u32>`. An example usage is shown
here:

```rs
use rpm_pkg_count::count;

match unsafe { count() } {
    Some(count) => println!("{count} packages installed."),
    None => println!("packages could not be counted"),
}
```
