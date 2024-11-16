use std::io::{self, Write};
use std::process::Command;

pub fn build() {
    let mut cmd = Command::new("cargo")
        .arg("run")
        .spawn()
        .expect("failed to execute process");
    let status = cmd.wait().expect("failed to wait on child");
    if !status.success() {
        eprintln!("cargo run failed with status: {}", status);
        return;
    }
}

pub fn create() {
    let mut name = String::new();
    print!("  Enter the name of the project: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    let name = name.trim();
    if name.is_empty() {
        eprintln!("project name cannot be empty");
        return;
    }

    let mut cmd = Command::new("cargo")
        .arg("init")
        .arg("--bin")
        .arg(name)
        .spawn()
        .expect("failed to execute process");
    let status = cmd.wait().expect("failed to wait on child");
    if !status.success() {
        eprintln!("cargo init failed with status: {}", status);
        return;
    }

    println!("project created successfully!");
    println!("cd {}", name);
    println!("cargo run");
}
