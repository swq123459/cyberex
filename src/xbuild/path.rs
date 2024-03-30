use crate::plat::dist::{plat_dist, PlatDist};
use crate::xpath::path::path_to_string;
use std::env;
use std::path::{Path, PathBuf};

fn get_dist_lib_name() -> String {
    match plat_dist() {
        PlatDist::Rh => "lib64",
        PlatDist::Debian => "lib",
        _ => "lib",
    }
    .to_string()
}

pub fn lib_path_of_root<Paths>(lib_root: Paths) -> String
where
    Paths: AsRef<std::path::Path>,
    Paths: std::fmt::Display,
{
    let p = Path::new(lib_root.as_ref());
    let lib_must = p.join(get_dist_lib_name());
    let lib_alt = if lib_must.ends_with("64") { "lib" } else { "lib64" };
    let out = if lib_must.exists() { lib_must } else { p.join(lib_alt) };
    path_to_string(out)
}

pub fn cargo_profile_dir() -> String {
    return PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .display()
        .to_string();
}
pub fn cargo_target_dir() -> String {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let mut target_dir = None;
    let mut sub_path = out_dir.as_path();
    while let Some(parent) = sub_path.parent() {
        if parent.ends_with("target") {
            target_dir = Some(parent);
            break;
        }
        sub_path = parent;
    }
    let target_dir = target_dir.unwrap();
    target_dir.to_path_buf().display().to_string()
}

pub fn cargo_target_bin_dir() -> String {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let profile = std::env::var("PROFILE").unwrap();
    let mut target_dir = None;
    let mut sub_path = out_dir.as_path();
    while let Some(parent) = sub_path.parent() {
        if parent.ends_with(&profile) {
            target_dir = Some(parent);
            break;
        }
        sub_path = parent;
    }
    let target_dir = target_dir.unwrap();
    target_dir.to_path_buf().display().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_case_lib_path_of_root() {
        if cfg!(windows) {
            assert_eq!(
                lib_path_of_root(
                    r#"D:\code\thirdlib\EaxLibrary\EaxComponent\gb_media\build\gb-media\thirdlib\ffmpeg\"#
                ),
                r#"D:\code\thirdlib\EaxLibrary\EaxComponent\gb_media\build\gb-media\thirdlib\ffmpeg\lib"#
            );
        } else if cfg!(linux) {
            assert_eq!(
                lib_path_of_root("/workspace/cyberex/thirdlib/catch2"),
                "/workspace/cyberex/thirdlib/catch2/lib"
            );
        }
    }

    #[test]
    fn test_cargo_target_dir() {
        std::env::set_var("PROFILE", "debug");
        assert_eq!(cargo_target_bin_dir(), "/workspace/cyberex/target/debug");
        assert_eq!(cargo_target_dir(), "/workspace/cyberex/target");
        assert_eq!(cargo_profile_dir(), "/workspace/cyberex");
    }
}
