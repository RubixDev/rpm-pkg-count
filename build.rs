#[cfg(not(feature = "compile-time"))]
fn main() {}

#[cfg(feature = "compile-time")]
fn main() {
    let lib = pkg_config::probe_library("rpm").unwrap();
    for include in &lib.include_paths {
        println!("cargo:root={}", include.display());
    }
}
