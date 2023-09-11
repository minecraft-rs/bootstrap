# MC Bootstrap

Launch minecraft from rust.

## Usage

Launch from .minecraft folder

```rust
use mc_bootstrap::{ClientAuth, ClientBootstrap, ClientSettings, ClientVersion};

fn get_mc_dir() -> PathBuf {
    return PathBuf::from("/home/sammwy/.minecraft");
}

fn get_java_path() -> PathBuf {
    return PathBuf::from("/usr/lib/jvm/java-16-openjdk/bin/java");
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
        java_bin: get_java_path(),
        libraries_dir: get_mc_dir().join("libraries"),
        manifest_file: get_mc_dir().join("versions").join("1.19.4").join("1.19.4.json"),
        natives_dir: get_mc_dir().join("versions").join("1.19.4").join("natives"),
        version: ClientVersion {
            version: "1.19.4".to_string(),
            version_type: "release".to_string(),
        },
        version_jar_file: get_mc_dir().join("versions").join("1.19.4").join("1.19.4.jar"),
    });

    bootstrap.launch().unwrap();
}
```

## Contribution

Feel free to contribute to the development of the library.
