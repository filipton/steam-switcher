use anyhow::Result;
use config::Config;
use std::io::{Read, Write};

use crate::steam::{kill_steam, launch_steam, modify_registry_file};

pub mod config;
pub mod steam;

fn main() -> Result<()> {
    let mut config: Config = Config::load()?;
    println!("{:?}", config);

    let res = show_menu(&mut config)?;

    match res {
        MenuSelector::Account(username) => {
            println!("Selected steam user: {}", username);
            println!("Killing all steam processes...");
            kill_steam();
            println!("Steam killed! Modyfing registry.vdf...");

            modify_registry_file(username)?;

            println!("Registry file modified! Starting steam...");
            launch_steam(config.steam_command.as_str())?;
        }
        MenuSelector::AddNew => {
            let mut username = String::new();
            std::io::stdin().read_line(&mut username)?;

            config.add_account(username.trim().to_string())?;
            println!("Added account: {}!", username.trim());

            main()?;
        }
        MenuSelector::Remove => {
            let mut remove_index = String::new();
            std::io::stdin().read_line(&mut remove_index)?;

            let remove_index: usize = remove_index.trim().parse()?;

            println!("Removed account: {}!", config.accounts[remove_index - 1]);
            config.remove_account_index(remove_index - 1)?;

            main()?;
        }
        _ => {}
    }

    Ok(())
}

enum MenuSelector {
    Account(String),
    Remove,
    AddNew,
    None,
}

fn show_menu(config: &mut Config) -> Result<MenuSelector> {
    let accounts: &Vec<String> = &config.accounts;

    for acc in 0..accounts.len() {
        println!("{}. {}", acc + 1, accounts[acc]);
    }

    println!("\nn. Add new");
    println!("d. Remove");
    println!("q. Exit");

    let mut menu_input = String::new();
    std::io::stdin().read_line(&mut menu_input)?;

    match menu_input.trim() {
        "n" => {
            return Ok(MenuSelector::AddNew);
        }
        "q" => {
            return Ok(MenuSelector::None);
        }
        "d" => {
            return Ok(MenuSelector::Remove);
        }
        _ => {}
    }

    let account_index: usize = menu_input.trim().parse()?;
    if account_index - 1 < accounts.len() {
        return Ok(MenuSelector::Account(
            accounts[account_index - 1].to_string(),
        ));
    }

    return Ok(MenuSelector::None);
}
