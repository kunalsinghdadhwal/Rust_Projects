use std::io::{self, Write};
use std::process::{exit, Command};

fn update_commit_push() {
    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to do git add");

    if !add_command.status.success() {
        eprintln!("Error: Failed to add files to repo");
        exit(1);
    }
    print!("Enter your commit message:- ");
    io::stdout().flush().unwrap();

    let mut commit_msg = String::new();
    io::stdin()
        .read_line(&mut commit_msg)
        .expect("Error taking input");
    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_msg)
        .output()
        .expect("git commit failed");

    if !commit_command.status.success() {
        eprintln!("Git commit failed");
        exit(1);
    }

    let push_command = Command::new("git")
        .arg("push")
        .arg("-u")
        .arg("origin")
        .arg("main")
        .output()
        .expect("Git push failed");

    if !push_command.status.success() {
        eprintln!("Git push failed");
        exit(1);
    }

    println!("Successfully did code add, commit, push to git");
}

fn main() {
    update_commit_push();
}
