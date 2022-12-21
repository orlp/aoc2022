fn main() {
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-search=native=z3/bin");
    }
}