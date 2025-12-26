use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u32,
    name: String,
    age: u8,
}

#[derive(Parser)]
#[command(name = "user-management", about = "a simple user management with cli")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { name: String, age: u8 },
    List,
    Delete { id: u32, name: String },
}

fn main() {
    let cli = Cli::parse();
    let db_path = "users.json";

    // Load existing users or start with empty Vec
    let mut users: Vec<User> = fs::read_to_string(db_path)
        .map(|data| serde_json::from_str(&data).unwrap_or_default())
        .unwrap_or_default();

    match &cli.command {
        Commands::Add { name, age } => {
            let id = users.last().map(|u| u.id + 1).unwrap_or(1);
            users.push(User {
                id,
                name: name.clone(),
                age: age.clone(),
            });
            println!("{} added at the age of {}, with ID: '{}'", name, age, id);
        }
        Commands::List => {
            if users.is_empty() {
                println!("No users found")
            } else {
                for user in &users {
                    println!("[{}] {} {}yrs old", user.id, user.name, user.age)
                }
            }
        }
        Commands::Delete { id, name } => {
            let initial_length = users.len();
            users.retain(|user| user.id != *id);
            if users.len() < initial_length {
                println!("{} with ID: {} has been deleted", name, id)
            } else {
                print!("User with ID {} cannot be found", id);
            }
        }
    }
    let json = serde_json::to_string_pretty(&users).unwrap();
    fs::write(db_path, json).expect("Unable to save data")
}
