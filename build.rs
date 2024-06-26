fn main() {
    #[cfg(target_os = "macos")]
    {
        use cc::Build;
        use std::path::Path;

        let path = Path::new("src")
            .join("target")
            .join("macos")
            .join("ffi")
            .join("lladdr.c");

        Build::new().file(path).compile("ffi");
    }
}
