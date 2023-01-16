use mc_bootstrap::ClientBootstrap;

fn main() {
    let bootstrap = ClientBootstrap::new(
        "null",
        "C:\\Users\\sammwy\\AppData\\Roaming\\.minecraft",
        "E:\\Programs\\openjdk-8\\bin\\java.exe",
        "Sammwy_",
        "null",
        "1.8.8",
        "release",
    );

    bootstrap.launch().unwrap();
}
