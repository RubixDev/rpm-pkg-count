fn main() {
    let lib = pkg_config::probe_library("rpm").unwrap();
    for include in &lib.include_paths {
        println!("cargo:root={}", include.display());
    }
}
