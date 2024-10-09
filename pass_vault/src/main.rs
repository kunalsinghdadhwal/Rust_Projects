mod logic;

use std::io;

use crate::logic::prompt;
use crate::logic::read_password_from_file;
use crate::logic::ServiceInfo;

fn clr() {
    print!("{}[2J", 27 as char);
}

fn banner() {
    let asci = r#"
  

 ________  ________  ________   ________           ___      ___ ________  ___  ___  ___   _________   
|\   __  \|\   __  \|\   ____\ |\   ____\         |\  \    /  /|\   __  \|\  \|\  \|\  \ |\___   ___\ 
\ \  \|\  \ \  \|\  \ \  \___|_\ \  \___|_        \ \  \  /  / | \  \|\  \ \  \\\  \ \  \\|___ \  \_| 
 \ \   ____\ \   __  \ \_____  \\ \_____  \        \ \  \/  / / \ \   __  \ \  \\\  \ \  \    \ \  \  
  \ \  \___|\ \  \ \  \|____|\  \\|____|\  \        \ \    / /   \ \  \ \  \ \  \\\  \ \  \____\ \  \ 
   \ \__\    \ \__\ \__\____\_\  \ ____\_\  \        \ \__/ /     \ \__\ \__\ \_______\ \_______\ \__\
    \|__|     \|__|\|__|\_________\\_________\        \|__|/       \|__|\|__|\|_______|\|_______|\|__|
                       \|_________\|_________|                                                        
                                                                                                      
                                                                                                      


  "#;

    println!("{asci}");
}
fn main() {
    clr();
    banner();
    loop {
        println!("Password Generation Menu");
        println!("1.Add entry\n2.List entries\n3.Search entry\n4.Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Error Taking input");
        choice.trim();

        match choice {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service: "),
                    prompt("Username: "),
                    prompt("Password: "),
                );
                println!("\nEntry added Successfully");
                entry.write_to_file();
            }
            "2" => {
                let services = read_password_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading password: {}", err);
                    Vec::new();
                });
                for item in &services {
                    let mut i: i64 = 1;
                    println!(
                        "{}. Services: {}\n    Username: {}",
                        i, item.service, item.username
                    );
                    i += 1;
                }
            }
            "3" => {
                clr();
                let services = read_password_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new();
                });
                let search = prompt("Search: ");
                for item in &services {
                    if item.service.as_str() == search.as_str() {
                        println!(
                            "Service = {}\nUsername = {}\nPassword = {}",
                            item.service, item.username, item.password
                        );
                    }
                }
            }
            "4" => {
                clr();
                println!("Goodbye");
                break;
            }
            _ => {
                println!("Not a valid choice.\n");
            }
        }
        println!("\n\n");
    }
}
