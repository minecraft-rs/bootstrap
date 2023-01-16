use std::{env, path::Path};

use crate::manifest::{ManifestLibrary, ManifestRule};

pub fn is_rule_satisfied(rule: &ManifestRule) -> bool {
    if rule.os.is_some() {
        let os = rule.os.as_ref().unwrap();

        let current_os = env::consts::OS;
        let target_os = &os["name"];

        if current_os != target_os {
            return false;
        }
    }

    return true;
}

pub fn should_use_library(lib: &ManifestLibrary) -> bool {
    let rules_opt = &lib.rules;
    if !rules_opt.is_some() {
        return true;
    }

    let rules = rules_opt.as_ref().unwrap();
    for rule in rules.iter() {
        let satisfied = is_rule_satisfied(rule);
        let use_lib = rule.action == "allow";

        if satisfied && !use_lib {
            return false;
        } else if !satisfied && use_lib {
            return false;
        }
    }

    return true;
}

pub fn create_classpath(
    jar_file: String,
    libraries_path: String,
    libraries: Vec<ManifestLibrary>,
) -> String {
    let mut classpath: String = format!("{}", jar_file);

    for lib in libraries.iter() {
        let should_use = should_use_library(lib);
        if should_use {
            let artifact_opt = &lib.downloads.artifact;

            if artifact_opt.is_some() {
                let artifact = artifact_opt.as_ref().unwrap();
                let lib_path = artifact.path.as_ref().unwrap();
                let fixed_lib_path = Path::new(&libraries_path).join(lib_path.replace("/", "\\"));
                classpath = format!("{};{}", classpath, fixed_lib_path.to_str().unwrap());
            }
        }
    }

    return classpath;
}
