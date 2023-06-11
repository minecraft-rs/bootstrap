use mc_bootstrap::manifest::read_manifest_from_str;

fn main() {
    let manifest =
        read_manifest_from_str(include_str!("./.minecraft/versions/1.19.4/1.19.4.json")).unwrap();

    println!("Asset index: {}", manifest.asset_index.id);
    println!("Assets: {}", manifest.assets);
    println!("Compilance Level: {}", manifest.compliance_level);
    println!("Jar: {}", manifest.downloads.client.url);
    println!("Java version: {}", manifest.java_version.major_version);
    println!("Libraries: {}", manifest.libraries.len());
    println!("Main class: {}", manifest.main_class);
    println!("Launcher version: {}", manifest.minimum_launcher_version);
    println!("Release time: {}", manifest.release_time);
    println!("Time: {}", manifest.time);
    println!("Type: {}", manifest.version_type);
}
