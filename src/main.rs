mod rust;
use dialoguer::{theme::ColorfulTheme, Select};
use rust::r_handler::*;
use std::process::exit;
fn main() {
    // Define the available languages and options
    let langs = [" Rust", " Go", " JavaScript", " Exit"];
    let rust_options = [" Run", " Create"];

    // Step 1: Prompt the user to select a language
    let lang_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a language")
        .items(&langs)
        .default(0)
        .interact()
        .expect("Failed to select language");

    // Handle language selection
    match langs[lang_selection] {
        " Rust" => {
            // Step 2: If Rust is selected, prompt the user to select an action
            let option_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select an action for Rust")
                .items(&rust_options)
                .default(0)
                .interact()
                .expect("Failed to select option");

            // Handle Rust options
            match rust_options[option_selection] {
                " Run" => build(),
                " Create" => create(),
                _ => println!("Invalid option selected."),
            }
        }
        " Go" => {
            println!("Go support is not yet available.");
        }
        " JavaScript" => {
            println!("JavaScript support is not yet available.");
        }
        " Exit" => {
            println!("Goodbye!");
            exit(0);
        }
        _ => println!("Invalid language selected."),
    }
}
