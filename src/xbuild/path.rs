use crate::plat::dist::{plat_dist, PlatDist};
use crate::xpath::path::path_to_string;
use std::path::Path;

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
}
