use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::xpath::path::path_to_string;
use anyhow::Result;
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

#[derive(Default)]
pub struct Target {
    pub name: String,
    pub files: String,
    pub include_dir: Option<String>,
    pub lib_dir: Option<String>,
    pub dep: Vec<String>,
    pub type_: String,
}
pub struct Module {
    module_name: String,
    targets: Vec<Target>,
    out_path: PathBuf,
    version: String,
}

impl Module {
    pub fn builder() -> ModuleBuilder {
        ModuleBuilder::default()
    }
    pub fn write(&self) -> Result<()> {
        create_dir_all(&self.out_path)?;
        self.write_version_file()?;
        self.write_target_file()?;
        self.write_config_file()?;
        Ok(())
    }

    fn get_target_file_name(&self) -> String {
        format!("{}Targets.cmake", self.module_name)
    }

    fn write_config_file(&self) -> Result<()> {
        let file_name = format!("{}Config.cmake", self.module_name);
        let mut file = File::create(self.out_path.join(file_name))?;
        file.write_all(
            format!(
                r#"include(${{CMAKE_CURRENT_LIST_DIR}}/{})"#,
                self.get_target_file_name()
            )
            .as_bytes(),
        )?;

        Ok(())
    }

    fn write_version_file(&self) -> Result<()> {
        let file_name = format!("{}ConfigVersion.cmake", self.module_name);
        let mut file = File::create(self.out_path.join(file_name))?;
        file.write_all(format!(r#"set(PACKAGE_VERSION "{}")"#, self.version).as_bytes())?;
        Ok(())
    }

    fn write_target_file(&self) -> Result<()> {
        let file_name = self.get_target_file_name();
        let mut file = File::create(self.out_path.join(file_name))?;
        file.write_all(
            r#"
get_filename_component(var_import_prefix "${CMAKE_CURRENT_LIST_FILE}" PATH)
get_filename_component(var_import_prefix "${var_import_prefix}" PATH)
"#
            .as_bytes(),
        )?;
        file.write_all(b"\n")?;
        for target in &self.targets {
            let sub_target_name = format!("{}::{}", self.module_name, target.name);
            file.write_all(format!(r#"add_library({} {} IMPORTED)"#, sub_target_name, target.type_).as_bytes())?;
            file.write_all(b"\n")?;

            file.write_all(format!(r#"set_target_properties({} PROPERTIES"#, sub_target_name).as_bytes())?;
            file.write_all(b"\n")?;

            {
                let mut prop_line = "".to_string();
                if let Some(include_dir) = &target.include_dir {
                    prop_line = format!(
                        r#"INTERFACE_INCLUDE_DIRECTORIES  "${{var_import_prefix}}/{}""#,
                        include_dir
                    );
                }
                file.write_all(prop_line.as_bytes())?;
                file.write_all(b"\n")?;
            }
            {
                let mut prop_line = "".to_string();
                if let Some(lib_dir) = &target.lib_dir {
                    prop_line = format!(
                        r#"IMPORTED_LOCATION  "${{var_import_prefix}}/{}/{}""#,
                        lib_dir, target.files
                    );
                }
                file.write_all(prop_line.as_bytes())?;
                file.write_all(b"\n")?;
            }
            {
                file.write_all(b"IMPORTED_NO_SONAME TRUE")?;
                file.write_all(b"\n")?;
            }
            {
                let dep_part = target.dep.join(";");
                file.write_all(format!(r#"INTERFACE_LINK_LIBRARIES "{}""#, dep_part).as_bytes())?;
                file.write_all(b"\n")?;
            }

            file.write_all(b")\n")?;

            file.write_all(r#"set(${CMAKE_FIND_PACKAGE_NAME}_FOUND TRUE)"#.to_string().as_bytes())?;
            file.write_all(b"\n")?;

            file.write_all(
                format!(r#"message(STATUS "Using {} ${{{}_VERSION}}")"#, self.module_name, self.module_name)
                    .to_string()
                    .as_bytes(),
            )?;
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct ModuleBuilder {
    target_name: Vec<Target>,
    out_path: Option<PathBuf>,
    module_name: Option<String>,
    version: Option<String>,
}
impl ModuleBuilder {
    pub fn add_target(mut self, target: Target) -> Self {
        self.target_name.push(target);
        self
    }
    pub fn module_name(mut self, module_name: impl Into<String>) -> Self {
        self.module_name = Some(module_name.into());
        self
    }
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
    pub fn out_path(mut self, out_path: impl AsRef<Path>) -> Self {
        self.out_path = Some(out_path.as_ref().to_path_buf());
        self
    }

    pub fn build(self) -> Module {
        Module {
            version: self.version.expect("Version must be set"),
            module_name: self.module_name.expect("Module name must be set"),
            targets: self.target_name,
            out_path: self.out_path.expect("Output path must be set"),
        }
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

    #[test]
    fn test_module_writer() {
        // Mod
        let m = Module::builder()
            .module_name("FUCK")
            .version("2.0")
            .out_path("/workspace/cyberex/target")
            .add_target(Target {
                name: "YOU".to_string(),
                files: "fuck.so".to_string(),
                include_dir: Some("include".to_string()),
                lib_dir: Some("lib".to_string()),
                dep: vec!["dl".to_string(), "ssl".to_string()],
                type_: "SHARED".to_string(),
            })
            .build();
        m.write().unwrap();
    }
}
