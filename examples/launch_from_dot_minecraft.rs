use std::path::PathBuf;

use mc_bootstrap::{ClientAuth, ClientBootstrap, ClientSettings, ClientVersion};

fn get_mc_dir() -> PathBuf {
    return PathBuf::from("E:\\Workspaces\\downloader\\.minecraft");
}

fn main() {
    let bootstrap = ClientBootstrap::new(ClientSettings {
        assets: get_mc_dir().join("assets"),
        auth: ClientAuth {
            username: "Sammwy_".to_string(),
            access_token: None,
            uuid: None,
        },
        game_dir: get_mc_dir(),
        java_bin: PathBuf::from(
            "C:\\Program Files\\Eclipse Adoptium\\jdk-17.0.7.7-hotspot\\bin\\java.exe",
        ),
        libraries_dir: get_mc_dir().join("libraries"),
        manifest_file: get_mc_dir()
            .join("versions")
            .join("1.19.4")
            .join("1.19.4.json"),
        natives_dir: get_mc_dir().join("versions").join("1.19.4").join("natives"),
        version: ClientVersion {
            version: "1.19.4".to_string(),
            version_type: "release".to_string(),
        },
        version_jar_file: get_mc_dir()
            .join("versions")
            .join("1.19.4")
            .join("1.19.4.jar"),
    });

    bootstrap.launch().unwrap();
}
