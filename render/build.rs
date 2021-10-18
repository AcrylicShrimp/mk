fn main() {
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    {
        println!("cargo:rustc-cfg=gl33");
    }
}
