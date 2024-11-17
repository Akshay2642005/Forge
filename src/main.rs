mod dialogs;
mod package_manager;
use crate::dialogs::rs_dialog::rust_dialog_landing;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

#[tokio::main]
async fn main() {
    println!("👾 Forge - Project Manager");
    let languages = vec![" Rust", " Go", " JavaScript"];
    let choices = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a language")
        .items(&languages)
        .default(0)
        .interact()
        .unwrap();

    match languages[choices] {
        " Rust" => rust_dialog_landing().await,
        " Go" => println!("No Go support yet"),
        " JavaScript" => println!("No JavaScript support yet"),
        _ => println!("Invalid choice"),
    }
}
