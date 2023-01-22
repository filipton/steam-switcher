use anyhow::Result;
use std::io::Write;

use crate::steam::{kill_steam, launch_steam, modify_registry_file};

pub mod steam;

const STEAM_COMMAND: &str = "steam-native";

fn main() -> Result<()> {
    let res = show_menu()?;

    match res {
        MenuSelector::Account(username) => {
            println!("Killing all steam processes...");
            kill_steam();
            println!("Steam killed! Modyfing registry.vdf...");

            modify_registry_file(username)?;

            println!("Registry file modified! Starting steam...");
            launch_steam(STEAM_COMMAND)?;
        }
        MenuSelector::AddNew => {
            unimplemented!();
        }
        _ => {
            unimplemented!();
        }
    }

    Ok(())
}

enum MenuSelector {
    Account(String),
    AddNew,
    None,
}

fn show_menu() -> Result<MenuSelector> {
    let accounts: Vec<&str> = vec!["filipton", "filipton2"];

    for acc in 0..accounts.len() {
        println!("{}. {}", acc + 1, accounts[acc]);
    }

    println!("\nn. Add new");

    let mut menu_input = String::new();
    std::io::stdin().read_line(&mut menu_input)?;

    if menu_input.trim() == "n" {
        return Ok(MenuSelector::AddNew);
    }

    let account_index: usize = menu_input.trim().parse()?;
    if account_index - 1 < accounts.len() {
        return Ok(MenuSelector::Account(accounts[account_index - 1].to_string()));
    }

    return Ok(MenuSelector::None);
}
