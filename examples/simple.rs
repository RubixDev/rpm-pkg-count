fn main() {
    match unsafe { rpm_pkg_count::count() } {
        Some(count) => println!("{count} packages installed."),
        None => println!("packages could not be counted"),
    }
}
