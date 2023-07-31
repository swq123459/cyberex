pub fn target_link_libraries(libs: Vec<(&'static str, &str)>) {
    for (t, lib) in libs {
        println!("cargo:rustc-link-lib={}={}", t, lib);
    }
}

pub fn target_link_directories<P>(p: P)
where
    P: IntoIterator,
    P::Item: AsRef<std::path::Path>,
{
    for path in p {
        println!(
            "cargo:rustc-link-search={}",
            path.as_ref().display()
        );
    }
}
