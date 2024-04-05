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

impl DevPath {
    pub fn get_shared(&self) -> Vec<PathBuf> {
        let mut netca_libs = Vec::new();
        if let Ok(paths) = std::fs::read_dir(&self.lib) {
            for path in paths.flatten() {
                let path = path.path();
                if path.is_file() && path.extension() == Some(get_dist_lib_suffix().as_ref()) {
                    netca_libs.push(path);
                }
            }
        }

        netca_libs
    }
}

fn get_dist_lib_name() -> String {
    match plat_dist() {
        PlatDist::Rh => "lib64",
        PlatDist::Debian => "lib",
        PlatDist::Other => "lib",
    }
    .to_string()
}

fn get_dist_lib_suffix() -> String {
    if cfg!(target_os = "linux") {
        "so".to_string()
    } else if cfg!(target_os = "windows") {
        "dll".to_string()
    } else {
        panic!("unkown os")
    }
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
        match plat_dist() {
            PlatDist::Rh => assert_eq!(lib_path_of_root("/noexist"), "/noexist/lib"),
            PlatDist::Debian => assert_eq!(lib_path_of_root("/noexist"), "/noexist/lib64"),
            PlatDist::Other => {},
        };

        if cfg!(target_os = "windows") {
            assert_eq!(
                lib_path_of_root(
                    r#"D:\code\thirdlib\EaxLibrary\EaxComponent\gb_media\build\gb-media\thirdlib\ffmpeg\"#
                ),
                r#"D:\code\thirdlib\EaxLibrary\EaxComponent\gb_media\build\gb-media\thirdlib\ffmpeg\lib"#
            );
        }
        if cfg!(target_os = "linux") {
            println!("fuck you ");
            let catch_root = "/workspace/cyberex/thirdlib/ffmpeg";
            let catch_lib = "/workspace/cyberex/thirdlib/ffmpeg/lib";
            let catch_include = "/workspace/cyberex/thirdlib/ffmpeg/include";

            assert_eq!(lib_path_of_root(catch_root), catch_lib);
            assert_eq!(include_path_of_root(catch_root), catch_include);
            let dev_path = dev_path_of_root(catch_root);
            assert_eq!(dev_path.lib.display().to_string(), catch_lib);
            assert_eq!(dev_path.include.display().to_string(), catch_include);
            assert_eq!(
                dev_path.get_shared(),
                Vec::from([
                    PathBuf::from("/workspace/cyberex/thirdlib/ffmpeg/lib/libavformat.so"),
                    PathBuf::from("/workspace/cyberex/thirdlib/ffmpeg/lib/libavutil.so"),
                    PathBuf::from("/workspace/cyberex/thirdlib/ffmpeg/lib/libavfilter.so"),
                    PathBuf::from("/workspace/cyberex/thirdlib/ffmpeg/lib/libavcodec.so"),
                    PathBuf::from("/workspace/cyberex/thirdlib/ffmpeg/lib/libswscale.so"),
                    PathBuf::from("/workspace/cyberex/thirdlib/ffmpeg/lib/libswresample.so"),
                    PathBuf::from("/workspace/cyberex/thirdlib/ffmpeg/lib/libavdevice.so")
                ])
            );

            let env_var = "FUCKYOU_ROOT";
            std::env::set_var(env_var, "/workspace/cyberex/thirdlib/catch2");
            let dev_path = dev_path_of_root_env(env_var);
            assert_eq!(
                dev_path.lib.display().to_string(),
                "/workspace/cyberex/thirdlib/catch2/lib"
            );
            assert_eq!(
                dev_path.include.display().to_string(),
                "/workspace/cyberex/thirdlib/catch2/include"
            );

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
