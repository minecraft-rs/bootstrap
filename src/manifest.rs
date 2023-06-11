use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Rules {
    pub action: String,
    pub features: Option<Features>,
    pub os: Option<Os>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Features {
    pub is_demo_user: Option<bool>,
    pub has_custom_resolution: Option<bool>,
    pub is_quick_play_realms: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Os {
    pub arch: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Arguments {
    pub game: Vec<JvmArgument>,
    pub jvm: Vec<JvmArgument>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum JvmArgument {
    String(String),
    Struct {
        rules: Vec<Rules>,
        value: serde_json::Value,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    #[serde(rename = "totalSize")]
    pub total_size: u64,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Downloads {
    pub client: DownloadItem,
    pub client_mappings: DownloadItem,
    pub server: DownloadItem,
    pub server_mappings: DownloadItem,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DownloadItem {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JavaVersion {
    pub component: String,
    #[serde(rename = "majorVersion")]
    pub major_version: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Library {
    pub downloads: LibraryDownloads,
    pub name: String,
    pub rules: Option<Vec<Rules>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LibraryDownloads {
    pub artifact: LibraryArtifact,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LibraryArtifact {
    pub path: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Logging {
    pub client: ClientLogging,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientLogging {
    pub argument: String,
    pub file: ClientLogFile,
    #[serde(rename = "type")]
    pub log_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientLogFile {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub arguments: Arguments,
    #[serde(rename = "assetIndex")]
    pub asset_index: AssetIndex,
    pub assets: String,
    #[serde(rename = "complianceLevel")]
    pub compliance_level: u32,
    pub downloads: Downloads,
    pub id: String,
    #[serde(rename = "javaVersion")]
    pub java_version: JavaVersion,
    pub libraries: Vec<Library>,
    pub logging: Logging,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    #[serde(rename = "minimumLauncherVersion")]
    pub minimum_launcher_version: u32,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    pub time: String,
    #[serde(rename = "type")]
    pub version_type: String,
}

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("The game directory doesn't exist.")]
    GameDirNotExist,

    #[error("The java bin doesn't exist.")]
    JavaBinNotExist,

    #[error("An unexpected error has ocurred.")]
    UnknownError,

    #[error("{0}")]
    IO(#[from] std::io::Error),

    #[error("{0}")]
    Json(#[from] serde_json::Error),
}

pub fn read_manifest_from_str(string: &str) -> Result<Manifest, ManifestError> {
    let manifest: Manifest = serde_json::from_str(&string)?;
    return Ok(manifest);
}

pub fn read_manifest_from_file(file: PathBuf) -> Result<Manifest, ManifestError> {
    let raw = fs::read_to_string(file)?;
    let manifest: Manifest = read_manifest_from_str(&raw)?;
    return Ok(manifest);
}
