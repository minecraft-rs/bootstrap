use std::path::{Path, PathBuf};

use crate::{manifest::Library, rules::is_all_rules_satisfied};

pub fn should_use_library(lib: &Library) -> bool {
    let rules_opt = &lib.rules;
    if !rules_opt.is_some() {
        return true;
    }

    let rules = rules_opt.as_ref().unwrap();
    return is_all_rules_satisfied(rules);
}

#[cfg(not(target_os = "windows"))] // Linux and MacOS
pub const CLASSPATH_SEP: char = ':';

#[cfg(target_os = "windows")] // Windows
pub const CLASSPATH_SEP: char = ';';

pub fn create_classpath(
    jar_file: PathBuf,
    libraries_path: PathBuf,
    libraries: Vec<Library>,
) -> String {
    let mut classpath = jar_file.to_str().unwrap().to_string();

    for lib in libraries.iter() {
        let should_use = should_use_library(lib);
        if should_use {
            let artifact = &lib.downloads.artifact;
            let lib_path = artifact.path.clone();
            let fixed_lib_path = Path::new(&libraries_path).join(lib_path.replace("/", "\\"));
            classpath = format!("{}{}{}", classpath, CLASSPATH_SEP, fixed_lib_path.to_str().unwrap());
        }
    }

    return classpath;
}
