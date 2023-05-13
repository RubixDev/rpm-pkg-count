# rpm-pkg-count

Counts installed RPM packages using `librpm`.

> Note: This crate does **not** make use of `librpm-sys` but links to the C
> library itself.

## Requirements

In order to compile this crate, one must have `librpm` installed on the system.
It is usually provided by package managers under the name `rpm-devel` (e.g.,
OpenSUSE), `rpm-tools` (e.g., Arch Linux) or `librpm-dev` (e.g., Debian).

## Usage

The crate exposes exaclty _one_ public function which takes no arguments and
returns the package count as a `u32`. An example usage is shown here:

```rs
use rpm_pkg_count::count;

fn main() {
    println!("{} packages installed", unsafe { count() });
}
```
