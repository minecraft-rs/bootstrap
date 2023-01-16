pub mod classpath;
pub mod manifest;

use manifest::read_manifest_from_file;
use std::{path::Path, process::Command};
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

pub struct ClientBootstrap {
    access_token: String,
    game_dir: String,
    java_bin: String,
    username: String,
    uuid: String,
    version: String,
    version_type: String,
}

impl ClientBootstrap {
    pub fn new(
        access_token: &str,
        game_dir: &str,
        java_bin: &str,
        username: &str,
        uuid: &str,
        version: &str,
        version_type: &str,
    ) -> Self {
        Self {
            access_token: access_token.to_string(),
            game_dir: game_dir.to_string(),
            java_bin: java_bin.to_string(),
            username: username.to_string(),
            uuid: uuid.to_string(),
            version: version.to_string(),
            version_type: version_type.to_string(),
        }
    }

    pub fn get_assets_dir(&self) -> String {
        return Path::new(&self.game_dir)
            .join("assets")
            .to_str()
            .unwrap()
            .to_string();
    }

    pub fn get_libs_dir(&self) -> String {
        return Path::new(&self.game_dir)
            .join("libraries")
            .to_str()
            .unwrap()
            .to_string();
    }

    pub fn get_json_file(&self) -> String {
        return Path::new(&self.game_dir)
            .join("versions")
            .join(&self.version)
            .join(format!("{}.json", self.version))
            .to_str()
            .unwrap()
            .to_string();
    }

    pub fn get_jar_file(&self) -> String {
        return Path::new(&self.game_dir)
            .join("versions")
            .join(&self.version)
            .join(format!("{}.jar", self.version))
            .to_str()
            .unwrap()
            .to_string();
    }

    pub fn get_natives_dir(&self) -> String {
        return Path::new(&self.game_dir)
            .join("versions")
            .join(&self.version)
            .join("natives")
            .to_str()
            .unwrap()
            .to_string();
    }

    pub fn build_args(&self) -> Result<Vec<String>, ClientBootstrapError> {
        if !Path::new(&self.game_dir).is_dir() {
            return Err(ClientBootstrapError::GameDirNotExist);
        }

        if !Path::new(&self.java_bin).is_file() {
            return Err(ClientBootstrapError::JavaBinNotExist);
        }

        let manifest_file = &self.get_json_file();
        if !Path::new(manifest_file).is_file() {
            return Err(ClientBootstrapError::VersionFileNotFound);
        }

        let manifest = read_manifest_from_file(manifest_file).unwrap();
        let classpath = classpath::create_classpath(
            self.get_jar_file(),
            self.get_libs_dir(),
            manifest.libraries,
        );

        let args: Vec<String> = vec![
            format!("-Djava.library.path={}", self.get_natives_dir()),
            format!("-Dminecraft.launcher.brand={}", "mc_bootstrap"),
            format!("-Dminecraft.launcher.version={}", "1.0.0"),
            format!("-cp"),
            format!("{}", classpath),
            format!("{}", manifest.main_class),
            format!("--accessToken"),
            format!("{}", self.access_token),
            format!("--assetsDir"),
            format!("{}", self.get_assets_dir()),
            format!("--assetsIndex"),
            format!("{}", manifest.asset_index.id),
            format!("--gameDir"),
            format!("{}", self.game_dir),
            format!("--userType"),
            format!("{}", "mojang"),
            format!("--username"),
            format!("{}", self.username),
            format!("--uuid"),
            format!("{}", self.uuid),
            format!("--version"),
            format!("{}", self.version),
            format!("--versionType"),
            format!("{}", self.version_type),
        ];

        return Ok(args);
    }

    pub fn launch(&self) -> Result<i32, ClientBootstrapError> {
        let args = self.build_args().unwrap();

        let mut process = Command::new(&self.java_bin)
            .args(args)
            .spawn()
            .expect("command failed to start");

        let status = process.wait().unwrap().code().unwrap();
        return Ok(status);
    }
}
