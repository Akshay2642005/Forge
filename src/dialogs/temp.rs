use dialoguer::{theme::ColorfulTheme, Select};
use std::io::{self, *};
use std::process::Command;
pub fn rust_dialog() {
    println!("🦀 Forge - Rust Project Manager");

    let options = vec![
        "New Project",
        "Install Dependencies",
        "Run Tests",
        "Build",
        "Publish",
        "Exit",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match options[choice] {
        "New Project" => pr_init(),
        "Install Dependencies" => install_deps(),
        "Run Tests" => run_tests(),
        "Build" => build(),
        "Publish" => publish(),
        "Exit" => exit(0),
        _ => println!("Invalid option"),
    };
}

fn pr_init() {
    println!("🚀 Initializing project...");
    let options = vec!["Default", "Frameworks"];
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a template")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    match options[choice] {
        "Default" => bin(),
        "Frameworks" => framework(),
        _ => eprintln!("❌Invalid option"),
    };
}

fn bin() {
    let mut name = String::new();
    print!("📝 Enter the name of your project: ");
    io::stdin().read_line(&mut name).unwrap();
    io::stdout().flush().unwrap();
    let mut command = Command::new("cargo")
        .arg("new")
        .arg(name)
        .arg("--bin")
        .spawn()
        .unwrap()
        .expect("Failed to create project");
    println!("✅ Project created successfully");
}

fn framework() {
    let options = vec!["Actix", "Rocket", "Axum", "Warp"];
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a framework")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    match options[choice] {
        "Actix" => {
            let mut name = String::new();
            print!("📝 Enter the name of your project: ");
            io::stdin().read_line(&mut name).unwrap();
            io::stdout().flush().unwrap();
            let mut command = Command::new("cargo")
                .arg("new")
                .arg(name)
                .arg("--bin")
                .arg("--edition=2021")
                .arg("--no-default-features")
                .arg("--features=actix-web")
                .arg("--lib");
            command.spawn().unwrap();
            let mut command = Command::new("cargo").arg("add").arg("actix-web");
            command.spawn().unwrap();
        }
        "Rocket" => {
            let mut name = String::new();
            print!("📝 Enter the name of your project: ");
            io::stdin().read_line(&mut name).unwrap();
            io::stdout().flush().unwrap();
            let mut command = Command::new("cargo")
                .arg("new")
                .arg(name)
                .arg("--bin")
                .arg("--edition=2021")
                .arg("--no-default-features")
                .arg("--features=rocket")
                .arg("--lib");
            command.spawn().unwrap();
            let mut command = Command::new("cargo").arg("add").arg("rocket");
            command.spawn().unwrap();
        }
        "Axum" => {
            let mut name = String::new();
            print!("📝 Enter the name of your project: ");
            io::stdin().read_line(&mut name).unwrap();
            io::stdout().flush().unwrap();
            let mut command = Command::new("cargo")
                .arg("new")
                .arg(name)
                .arg("--bin")
                .arg("--edition=2021")
                .arg("--no-default-features")
                .arg("--features=axum")
                .arg("--lib");
            command.spawn().unwrap();
            let mut command = Command::new("cargo").arg("add").arg("axum");
            command.spawn().unwrap();
        }
        "Warp" => {
            let mut name = String::new();
            print!("📝 Enter the name of your project: ");
            io::stdin().read_line(&mut name).unwrap();
            io::stdout().flush().unwrap();
            let mut command = Command::new("cargo")
                .arg("new")
                .arg(name)
                .arg("--bin")
                .arg("--edition=2021")
                .arg("--no-default-features")
                .arg("--features=warp")
                .arg("--lib");
            command.spawn().unwrap();
            let mut command = Command::new("cargo").arg("add").arg("warp");
            command.spawn().unwrap();
        }
        _ => eprintln!("❌Invalid option"),
    };
}

fn install_deps() {
    println!("📦 Installing dependencies...");
    let mut command = Command::new("cargo")
        .arg("install")
        .arg("--path")
        .arg(".")
        .arg("--force")
        .spawn()
        .unwrap()
        .expect("Failed to install dependencies");
    println!("✅ Dependencies installed successfully");
}

fn run_tests() {
    println!("🧪 Running tests...");
    let mut command = Command::new("cargo")
        .arg("test")
        .arg("--path")
        .arg(".")
        .arg("--force")
        .spawn()
        .unwrap()
        .expect("Failed to run tests");
    println!("✅ Tests ran successfully");
}

fn build() {
    println!("🔨 Building project...");
    let mut command = Command::new("cargo")
        .arg("build")
        .arg("--path")
        .arg(".")
        .arg("--force")
        .spawn()
        .unwrap()
        .expect("Failed to build project");
    println!("✅ Project built successfully");
}

fn publish() {
    println!("📦 Publishing project...");
    let mut command = Command::new("cargo")
        .arg("publish")
        .arg("--path")
        .arg(".")
        .arg("--force")
        .spawn()
        .unwrap()
        .expect("Failed to publish project");
    println!("✅ Project published successfully");
}

fn exit(code: i32) {
    println!("👋 See you soon!");
    std::process::exit(code);
}
