use crate::xpath::path::path_to_string;

pub enum LibKind {
    Shared(String),
    Static(String),
    Auto(String),
}

fn libname_strip(lib_name: &str) -> String {
    if cfg!(target_os = "linux") {
        // Note: here remove the suffix in linux, beacause windows contain '*.a' as static library
        let temp = lib_name.strip_prefix("lib").unwrap_or(lib_name);
        path_to_string(std::path::Path::new(&temp).file_stem().unwrap_or(temp.as_ref()))
    } else {
        lib_name.to_string()
    }
}

pub fn format_target_link_libraries(kind: LibKind) -> String {
    format!(
        "cargo:rustc-link-lib={}",
        match kind {
            LibKind::Shared(name) => "dylib=".to_string() + &libname_strip(&name),
            LibKind::Static(name) => "static=".to_string() + &libname_strip(&name),
            LibKind::Auto(name) => libname_strip(&name),
        }
    )
}

pub fn target_link_libraries<Libs>(kinds: Libs)
where
    Libs: IntoIterator<Item = LibKind>,
{
    for kind in kinds {
        println!("{}", format_target_link_libraries(kind));
    }
}

pub fn target_link_directories<Paths>(p: Paths)
where
    Paths: IntoIterator,
    Paths::Item: AsRef<std::path::Path>,
{
    for path in p {
        let path_str = path.as_ref().display().to_string();
        if path_str.is_empty() {
            continue;
        }
        println!("cargo:rerun-if-changed={}", path_str);
        println!("cargo:rustc-link-search={}", path_str);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_target_link_libraries() {
        assert_eq!(
            format_target_link_libraries(LibKind::Shared("libm".into())),
            "cargo:rustc-link-lib=dylib=m"
        );
        assert_eq!(
            format_target_link_libraries(LibKind::Static("libm".into())),
            "cargo:rustc-link-lib=static=m"
        );
        assert_eq!(
            format_target_link_libraries(LibKind::Auto("libm".into())),
            "cargo:rustc-link-lib=m"
        );
        assert_eq!(
            format_target_link_libraries(LibKind::Auto("m.a".into())),
            "cargo:rustc-link-lib=m"
        );
        assert_eq!(
            format_target_link_libraries(LibKind::Auto("libm.a".into())),
            "cargo:rustc-link-lib=m"
        );
        assert_eq!(
            format_target_link_libraries(LibKind::Auto("libm.lib".into())),
            "cargo:rustc-link-lib=m"
        );
        assert_eq!(
            format_target_link_libraries(LibKind::Auto("libm.so".into())),
            "cargo:rustc-link-lib=m"
        );
        assert_eq!(
            format_target_link_libraries(LibKind::Shared("m".into())),
            "cargo:rustc-link-lib=dylib=m"
        );

        assert_eq!(
            format_target_link_libraries(LibKind::Static("m".into())),
            "cargo:rustc-link-lib=static=m"
        );
        assert_eq!(
            format_target_link_libraries(LibKind::Auto("m".into())),
            "cargo:rustc-link-lib=m"
        );
    }

    #[test]
    fn test_target_link_libraries() {
        target_link_libraries([LibKind::Shared("z".to_string())]);
        target_link_libraries(vec![LibKind::Shared("z".to_string())]);
    }

    #[test]
    fn test_target_link_directories() {
        target_link_directories([""]);
        target_link_directories(["path1"]);
        target_link_directories(vec!["path1"]);
    }
}
