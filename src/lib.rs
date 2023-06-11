pub mod classpath;
pub mod manifest;
pub mod rules;

use manifest::{read_manifest_from_file, JvmArgument};
use rules::is_all_rules_satisfied;
use std::{path::PathBuf, process::Command};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientBootstrapError {
    #[error("The game directory doesn't exist.")]
    GameDirNotExist,

    #[error("The java bin doesn't exist.")]
    JavaBinNotExist,

    #[error("The version file (.json) doesn't exist.")]
    VersionFileNotFound,

    #[error("An unexpected error has ocurred.")]
    UnknownError,

    #[error("{0}")]
    Json(#[from] serde_json::Error),
}

pub struct ClientAuth {
    pub access_token: Option<String>,
    pub username: String,
    pub uuid: Option<String>,
}

pub struct ClientVersion {
    pub version: String,
    pub version_type: String,
}

pub struct ClientSettings {
    pub assets: PathBuf,
    pub auth: ClientAuth,
    pub game_dir: PathBuf,
    pub java_bin: PathBuf,
    pub libraries_dir: PathBuf,
    pub manifest_file: PathBuf,
    pub natives_dir: PathBuf,
    pub version: ClientVersion,
    pub version_jar_file: PathBuf,
}

pub struct ClientBootstrap {
    pub settings: ClientSettings,
}

impl ClientBootstrap {
    pub fn new(settings: ClientSettings) -> Self {
        Self { settings }
    }

    pub fn get_assets_dir(&self) -> PathBuf {
        return self.settings.assets.clone();
    }

    pub fn get_game_dir(&self) -> PathBuf {
        return self.settings.game_dir.clone();
    }

    pub fn get_json_file(&self) -> PathBuf {
        return self.settings.manifest_file.clone();
    }

    pub fn get_jar_file(&self) -> PathBuf {
        return self.settings.version_jar_file.clone();
    }

    pub fn get_libs_dir(&self) -> PathBuf {
        return self.settings.libraries_dir.clone();
    }

    pub fn get_natives_dir(&self) -> PathBuf {
        return self.settings.natives_dir.clone();
    }

    pub fn build_args(&self) -> Result<Vec<String>, ClientBootstrapError> {
        let auth = &self.settings.auth;
        let assets_dir = self.get_assets_dir();
        let game_dir = self.get_game_dir();
        let java_bin = self.settings.java_bin.clone();
        let json_file = self.get_json_file();
        let natives_dir = self.get_natives_dir();
        let version = &self.settings.version;

        if !game_dir.is_dir() {
            return Err(ClientBootstrapError::GameDirNotExist);
        }

        if !java_bin.is_file() {
            return Err(ClientBootstrapError::JavaBinNotExist);
        }

        if !json_file.is_file() {
            return Err(ClientBootstrapError::VersionFileNotFound);
        }

        let manifest = read_manifest_from_file(json_file).unwrap();

        let assets_index = &manifest.asset_index.id;
        let classpath = classpath::create_classpath(
            self.get_jar_file(),
            self.get_libs_dir(),
            manifest.libraries,
        );

        let mut args: Vec<String> = vec![];

        for arg in manifest.arguments.jvm {
            match arg {
                JvmArgument::String(value) => {
                    args.push(value);
                }
                JvmArgument::Struct { value, rules, .. } => {
                    if !is_all_rules_satisfied(&rules) {
                        continue;
                    }

                    if let Some(value) = value.as_str() {
                        args.push(value.to_string());
                    } else if let Some(value_arr) = value.as_array() {
                        for value in value_arr {
                            if let Some(value) = value.as_str() {
                                args.push(value.to_string());
                            }
                        }
                    }
                }
            }
        }

        args.push(manifest.main_class);

        for arg in manifest.arguments.game {
            match arg {
                JvmArgument::String(value) => {
                    args.push(value);
                }
                JvmArgument::Struct { value, rules, .. } => {
                    if !is_all_rules_satisfied(&rules) {
                        continue;
                    }

                    if let Some(value) = value.as_str() {
                        args.push(value.to_string());
                    } else if let Some(value_arr) = value.as_array() {
                        for value in value_arr {
                            if let Some(value) = value.as_str() {
                                args.push(value.to_string());
                            }
                        }
                    }
                }
            }
        }

        args = args
            .iter()
            .map(|x| {
                x.replace("${assets_root}", &assets_dir.to_str().unwrap())
                    .replace("${game_directory}", &game_dir.to_str().unwrap())
                    .replace("${natives_directory}", &natives_dir.to_str().unwrap())
                    .replace("${launcher_name}", "minecraft-rs/bootstrap")
                    .replace("${launcher_version}", "0.1.1")
                    .replace(
                        "${auth_access_token}",
                        auth.access_token
                            .clone()
                            .unwrap_or("null".to_string())
                            .as_str(),
                    )
                    .replace("${auth_player_name}", auth.username.as_str())
                    .replace(
                        "${auth_uuid}",
                        auth.uuid.clone().unwrap_or("null".to_string()).as_str(),
                    )
                    .replace("${version_type}", &version.version_type)
                    .replace("${version_name}", &version.version)
                    .replace("${assets_index_name}", &assets_index)
                    .replace("${user_properties}", "{}")
                    .replace("${classpath}", &classpath)
            })
            .collect();

        return Ok(args);
    }

    pub fn launch(&self) -> Result<i32, ClientBootstrapError> {
        let args = self.build_args().unwrap();

        let mut process = Command::new(&self.settings.java_bin)
            .args(args)
            .spawn()
            .expect("command failed to start");

        let status = process.wait().unwrap().code().unwrap();
        return Ok(status);
    }
}
