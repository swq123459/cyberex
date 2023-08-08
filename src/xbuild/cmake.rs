pub enum LibKind {
    Shared(String),
    Static(String),
    Auto(String),
}

pub fn format_target_link_libraries(kind: LibKind) -> String {
    format!(
        "cargo:rustc-link-lib={}",
        match kind {
            LibKind::Shared(name) => "dylib=".to_string() + &name,
            LibKind::Static(name) => "static=".to_string() + &name,
            LibKind::Auto(name) => name,
        }
    )
}

pub fn target_link_libraries(kinds: Vec<LibKind>) {
    for kind in kinds {
        println!("{}", format_target_link_libraries(kind));
    }
}

pub fn target_link_directories<P>(p: P)
where
    P: IntoIterator,
    P::Item: AsRef<std::path::Path>,
{
    for path in p {
        println!("cargo:rustc-link-search={}", path.as_ref().display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_target_link_libraries() {
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
}
