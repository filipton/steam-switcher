const REGISTRY_PATH: &str = "~/.steam/registry.vdf";

fn main() {
    let config_lines: Vec<String> = std::fs::read_to_string(insert_home_dir(REGISTRY_PATH))
        .expect("Failed to read registry file")
        .lines()
        .map(|x| x.to_string())
        .collect();

    config_lines.iter().for_each(|d| println!("{}", d));
}

fn insert_home_dir(path: &str) -> String {
    let home_dir = std::env::var("HOME").unwrap();

    return path.replace('~', home_dir.as_str());
}
