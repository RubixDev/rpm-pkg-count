fn main() {
    println!("{:?} packages installed.", unsafe { rpm_pkg_count::count() });
}
