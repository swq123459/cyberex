use crate::plat::dist::{plat_dist, PlatDist};
use crate::xpath::path::path_to_string;
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct DevPath {
    pub include: PathBuf,
    pub lib: PathBuf,
}

fn get_dist_lib_name() -> String {
    match plat_dist() {
        PlatDist::Rh => "lib64",
        PlatDist::Debian => "lib",
        PlatDist::Other => "lib",
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
pub fn include_path_of_root<Paths>(lib_root: Paths) -> String
where
    Paths: AsRef<std::path::Path>,
    Paths: std::fmt::Display,
{
    let out = lib_root.as_ref().join("include");
    path_to_string(out)
}

pub fn dev_path_of_root<Paths>(lib_root: Paths) -> DevPath
where
    Paths: AsRef<std::path::Path>,
    Paths: std::fmt::Display,
{
    DevPath {
        include: PathBuf::from(include_path_of_root(&lib_root)),
        lib: PathBuf::from(lib_path_of_root(lib_root)),
    }
}
pub fn dev_path_of_root_env<K>(lib_root_key: K) -> DevPath
where
    K: AsRef<OsStr>,
{
    match env::var(lib_root_key) {
        Err(_) => DevPath::default(),
        Ok(lib_root) => dev_path_of_root(lib_root),
    }
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
            let catch_root = "/workspace/cyberex/thirdlib/catch2";
            let catch_lib = "/workspace/cyberex/thirdlib/catch2/lib";
            let catch_include = "/workspace/cyberex/thirdlib/catch2/include";

            assert_eq!(lib_path_of_root(catch_root), catch_lib);
            assert_eq!(include_path_of_root(catch_root), catch_include);
            let dev_path = dev_path_of_root(catch_root);
            assert_eq!(dev_path.lib.display().to_string(), catch_lib);
            assert_eq!(dev_path.include.display().to_string(), catch_include);

            std::env::set_var("FUCKYOU_ROOT", "debug");
            let dev_path = dev_path_of_root_env(catch_root);
            assert_eq!(dev_path.lib.display().to_string(), "debug/lib");
            assert_eq!(dev_path.include.display().to_string(), "debug/include");


            let dev_path = dev_path_of_root_env("noexist");
            assert_eq!(dev_path, DevPath::default());

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
