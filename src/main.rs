use anyhow::Result;
use std::{
    io::Write,
    process::{Command, Stdio},
};
use sysinfo::{ProcessExt, SystemExt};

const REGISTRY_PATH: &str = "~/.steam/registry.vdf";
const STEAM_COMMAND: &str = "steam-native";

fn main() -> Result<()> {
    print!("Enter username: ");
    std::io::stdout().flush()?;

    let mut username_input = String::new();
    std::io::stdin().read_line(&mut username_input)?;

    println!("Killing all steam processes...");
    kill_steam();
    println!("Steam killed! Modyfing registry.vdf...");

    modify_registry_file(username_input.trim())?;

    Command::new("bash")
        .arg("-c")
        .arg(format!("{} /dev/null 2>&1 &", STEAM_COMMAND))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()?;

    Ok(())
}

fn modify_registry_file(username: &str) -> Result<()> {
    let registry_file: String = std::fs::read_to_string(insert_home_dir(REGISTRY_PATH)?)
        .expect("Failed to read registry file");

    let mut tmp_registry_file: Vec<String> = vec![];
    for line in registry_file.lines() {
        if line.contains("AutoLoginUser") {
            let tabs_count = line.matches('\t').count() - 2;

            let auto_login_user: String = format!(
                "{}{}\t\t\"{}\"",
                "\t".repeat(tabs_count),
                "\"AutoLoginUser\"",
                username
            );

            let remember_password: String = format!(
                "{}{}\t\t\"{}\"",
                "\t".repeat(tabs_count),
                "\"RememberPassword\"",
                "1"
            );

            tmp_registry_file.push(auto_login_user);
            tmp_registry_file.push(remember_password);

            continue;
        } else if line.contains("RememberPassword") {
            continue;
        }

        tmp_registry_file.push(line.to_string());
    }

    let output_file: String = tmp_registry_file.join("\n");
    std::fs::write(insert_home_dir(REGISTRY_PATH)?, output_file)?;

    Ok(())
}

fn insert_home_dir(path: &str) -> Result<String> {
    let home_dir = std::env::var("HOME")?;
    Ok(path.replace('~', home_dir.as_str()))
}

fn kill_steam() {
    let mut system = sysinfo::System::new();
    system.refresh_all();

    for (pid, process) in system.processes() {
        if process.name().contains("steam") {
            println!("Killing: [{}] {}", pid, process.name());
            process.kill();
        }
    }
}
