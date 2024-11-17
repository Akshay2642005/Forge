#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
/*----------------------------------------------------------------------------------------*/
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::process::Command;
use tokio;

#[derive(Debug, Deserialize)]
pub struct Crate {
    name: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CrateSearchResult {
    crates: Vec<Crate>,
}

pub async fn search_crates(query: &str) -> Result<Vec<Crate>, Box<dyn Error>> {
    let url = format!(
        "https://crates.io/api/v1/cioXFuV7pnvZh99KyyVExzWCGikuWzkO2pb/crates?q={}&per_page=10",
        query
    );
    let client = Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let result: CrateSearchResult = response.json().await?;
        Ok(result.crates)
    } else {
        Err("Failed to fetch data from crates.io".into())
    }
}

pub async fn get_crates() {
    // Step 1: Get search query from user
    let search_query: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the crate name to search")
        .interact_text()
        .unwrap();

    // Step 2: Fetch crates from crates.io
    match search_crates(&search_query).await {
        Ok(crates) if crates.is_empty() => {
            println!("❌ No crates found for '{}'", search_query);
        }
        Ok(crates) => {
            // Step 3: Display results and let the user select crates
            let crate_names: Vec<String> = crates
                .iter()
                .map(|c| format!("{} - {}", c.name, c.description.clone().unwrap_or_default()))
                .collect();

            let selected = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Select crates to install")
                .items(&crate_names)
                .interact()
                .unwrap();

            if selected.is_empty() {
                println!("⚠️ No crates selected.");
                return;
            }

            // Step 4: Install selected crates using `cargo add`
            for index in selected {
                let crate_name = &crates[index].name;
                install_crate(crate_name).await;
            }
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }
}

/// Function to install a crate using `cargo add`
async fn install_crate(crate_name: &str) {
    println!("🔧 Installing crate: {}", crate_name);
    let output = Command::new("cargo")
        .arg("add")
        .arg(crate_name)
        .output()
        .expect("Failed to install crate");

    if output.status.success() {
        println!("✅ Successfully installed {}", crate_name);
    } else {
        eprintln!("❌ Failed to install {}", crate_name);
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}
