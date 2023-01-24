use anyhow::Result;
use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = "~/.rusty-switcher.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub accounts: Vec<String>,
    pub steam_command: String,
}

impl Config {
    pub fn load() -> Result<Config> {
        let mut config: Config = Config {
            accounts: vec![],
            steam_command: String::from("steam-native"),
        };

        match std::fs::read_to_string(insert_home_dir(CONFIG_PATH)?) {
            Ok(config_str) => {
                config = serde_json::from_str(&config_str)?;
            }
            Err(_) => {
                println!("Config file doesnt exists! Creating one...");
                config.save()?;
            }
        }

        return Ok(config);
    }

    pub fn add_account(&mut self, username: String) -> Result<()> {
        self.accounts.push(username);
        self.save()?;

        return Ok(());
    }

    pub fn remove_account_index(&mut self, index: usize) -> Result<()> {
        self.accounts.remove(index);
        self.save()?;

        return Ok(());
    }

    pub fn save(&self) -> Result<()> {
        let config_str = serde_json::to_string_pretty(self)?;
        std::fs::write(insert_home_dir(CONFIG_PATH)?, config_str)?;

        return Ok(());
    }
}

pub fn insert_home_dir(path: &str) -> Result<String> {
    let home_dir = std::env::var("HOME")?;
    Ok(path.replace('~', home_dir.as_str()))
}
