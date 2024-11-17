#![allow(dead_code)]
/*----------------------------------------------------------------------------------------*/
use crate::package_manager::crates::get_crates;
use dialoguer::{theme::ColorfulTheme, Select};
use std::process::{exit, Command};

pub async fn rust_dialog_landing() {
    println!("🦀 Forge - Rust Project Manager");

    let options = vec![
        " New Project",
        " Install Dependencies",
        " Run Tests",
        " Build",
        " Publish",
        " Exit",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match options[choice] {
        " New Project" => new_project(),
        " Install Dependencies" => println!("Not supported yet"),
        /*install_dependencies().await*/
        " Run Tests" => run_tests(),
        " Build" => build(),
        " Publish" => publish(),
        " Exit" => exit(0),
        _ => println!("Invalid option"),
    };
}

fn publish() {
    println!("📦 Publishing project...");
    let output = Command::new("cargo")
        .arg("publish")
        .output()
        .expect("Failed to execute process");
    if output.status.success() {
        println!("🚀 Published successfully");
    } else {
        println!("🚀 Failed to publish");
    }
}

fn build() {
    println!("🔨 Building project...");
    let mut output = Command::new("cargo")
        .arg("build")
        .arg("--quiet")
        .arg("--release")
        .spawn()
        .expect("Failed to execute process");
    let status = output.wait().expect("Failed to wait on child process");
    if status.success() {
        println!("🔨 Built successfully");
    } else {
        println!("🔨 Failed to build");
    }
}

fn run_tests() {
    println!("🧪 Running tests...");
    let mut output = Command::new("cargo")
        .arg("test")
        .arg("--path")
        .arg(".")
        .spawn()
        .expect("Failed to execute process");
    let status = output.wait().expect("Failed to wait on child process");
    if status.success() {
        println!("🧪 Tests passed");
    } else {
        println!("🧪 Tests failed");
    }
}

async fn install_dependencies() {
    println!("🔧 Installing dependencies...");
    get_crates().await;
}

fn new_project() {
    println!("🚧 New project feature is not implemented yet");
}
