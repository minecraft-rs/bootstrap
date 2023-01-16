use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ManifestAssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: i32,
    pub total_size: i32,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ManifestComponent {
    pub component: String,
    pub major_version: i8,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ManifestFile {
    pub path: Option<String>,
    pub sha1: String,
    pub size: i32,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct ManifestDownloads {
    pub client: ManifestFile,
    pub client_mappings: Option<ManifestFile>,
    pub server: ManifestFile,
    pub server_mappings: Option<ManifestFile>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ManifestRule {
    pub action: String,
    pub os: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ManifestLibraryDownloads {
    pub artifact: Option<ManifestFile>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ManifestLibrary {
    pub downloads: ManifestLibraryDownloads,
    pub name: String,
    pub rules: Option<Vec<ManifestRule>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Manifest {
    pub asset_index: ManifestAssetIndex,
    pub assets: String,
    pub compliance_level: i8,
    pub downloads: ManifestDownloads,
    pub id: String,
    pub java_version: ManifestComponent,
    pub libraries: Vec<ManifestLibrary>,
    pub main_class: String,
    pub minimum_launcher_version: i8,
    pub release_time: String,
    pub time: String,
    #[serde(rename(deserialize = "type"))]
    pub type_: String,
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

pub fn read_manifest_from_file(file: &str) -> Result<Manifest, ManifestError> {
    let raw = fs::read_to_string(file)?;
    let manifest: Manifest = read_manifest_from_str(&raw)?;
    return Ok(manifest);
}
