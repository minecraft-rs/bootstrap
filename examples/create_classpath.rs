use std::path::PathBuf;

use mc_bootstrap::{classpath::create_classpath, manifest::read_manifest_from_file};

fn get_current_dir() -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir
}

fn get_mc_dir() -> PathBuf {
    return get_current_dir().join("examples").join(".minecraft");
}

fn main() {
    let libs = get_mc_dir().join("libraries");
    let jar = get_mc_dir().join("versions/1.19.4/1.19.4.jar");
    let json = get_mc_dir().join("versions/1.19.4/1.19.4.json");

    let manifest = read_manifest_from_file(json).unwrap();
    let classpath = create_classpath(jar, libs, manifest.libraries);
    println!("{}", classpath);
}
