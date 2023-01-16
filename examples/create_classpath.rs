use mc_bootstrap::{classpath::create_classpath, manifest::read_manifest_from_file};

fn main() {
    let libs = "C:\\Users\\sammwy\\AppData\\Roaming\\.minecraft\\libraries";
    let jar = "C:\\Users\\sammwy\\AppData\\Roaming\\.minecraft\\versions\\1.19.3\\1.19.3.jar";
    let json = "C:\\Users\\sammwy\\AppData\\Roaming\\.minecraft\\versions\\1.19.3\\1.19.3.json";

    let manifest = read_manifest_from_file(json).unwrap();
    let classpath = create_classpath(jar.to_string(), libs.to_string(), manifest.libraries);
    println!("{}", classpath);
}
