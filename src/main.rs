use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    io::{self},
    process::Command,
};
use std::path::PathBuf;
use directories::ProjectDirs;

#[derive(Parser)]
#[command(name = "Git Profile Manager")]
#[command(about = "A CLI tool for managing multiple git profiles")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Add a new Git profile")]
    Add,

    #[command(about = "Switch to an existing Git profile")]
    Switch,

    #[command(about = "Remove a specific Git profile (aliases: rm, -rm)")]
    #[command(alias = "rm")]
    #[command(alias = "-rm")]
    Remove,

    #[command(about = "Remove all Git profiles")]
    RemoveAll,

    #[command(about = "List all Git profiles")]
    List,

    #[command(about = "Check the configuration directory and settings (aliases: doc, -doc)")]
    #[command(alias = "doc")]
    #[command(alias = "-doc")]
    Doctor,

    #[command(about = "Show the current Git profile")]
    Current,

    #[command(about = "Get current CLI version (aliases: -v, --version)")]
    #[command(alias = "version")]
    #[command(alias = "--version")]
    #[command(alias = "-v")]
    Version,
}


#[derive(Serialize, Deserialize, Debug)]
struct Profile {
    profile_name: String,
    name: String,
    email: String,
    remote: String,
}

const PROFILE_FILE: &str = "profiles.json";
const VERSION: &str = "0.1.0";

fn version() {
    println!("{VERSION}")
}

fn get_profile_file_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "GitProjectManager", "gitpm") {
        let config_dir = proj_dirs.config_dir();
        if !config_dir.exists() {
            if let Err(e) = fs::create_dir_all(config_dir) {
                eprintln!("Failed to create config directory at {}: {}", config_dir.display(), e);
                return PathBuf::from(PROFILE_FILE);
            }
        }
        config_dir.join(PROFILE_FILE)
    } else {
        PathBuf::from(PROFILE_FILE)
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add => add_profile(),
        Commands::Switch => switch_profile(),
        Commands::Remove => remove_profile(),
        Commands::RemoveAll => remove_all_profiles(),
        Commands::List => list_profiles(),
        Commands::Doctor => doctor(),
        Commands::Current => show_current_profile(),
        Commands::Version => version(),
    }
}

fn load_profiles() -> HashMap<String, Profile> {
    let profile_path = get_profile_file_path();
    match fs::read_to_string(profile_path) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
        Err(_) => HashMap::new(),
    }
}

fn save_profiles(profiles: &HashMap<String, Profile>) {
    let profile_path = get_profile_file_path();
    let data = serde_json::to_string_pretty(&profiles).expect("Failed to serialize profiles");
    fs::write(profile_path, data).expect("Failed to write profiles to file");
}

fn add_profile() {
    println!("Enter profile name or select a profile service (1 - GitHub, 2 - GitLab, 3 - Other): ");
    let service_option = read_input();

    let service_name = match service_option.as_str() {
        "1" => "GitHub",
        "2" => "GitLab",
        "3" => {
            eprint!("Enter custom profile name: ");
            let custom_name = read_input();
            return add_custom_profile(custom_name);
        },
        _ => {
            let profile_name = service_option.clone();
            return add_custom_profile(profile_name);
        }
    };

    eprint!("Enter user name: ");
    let user_name = read_input();

    let profile_name = format!("{}_{}", service_name, user_name);

    eprint!("Enter email: ");
    let email = read_input();

    eprint!("Enter remote URL (optional): ");
    let remote = read_input();

    let mut profiles = load_profiles();
    profiles.insert(
        profile_name.clone(),
        Profile {
            profile_name,
            name: user_name,
            email,
            remote,
        },
    );

    save_profiles(&profiles);
    println!("Profile added successfully!");
}

fn add_custom_profile(profile_name: String) {
    eprint!("Enter user name: ");
    let user_name = read_input();

    eprint!("Enter email: ");
    let email = read_input();

    eprint!("Enter remote URL (optional): ");
    let remote = read_input();

    let mut profiles = load_profiles();
    profiles.insert(
        profile_name.clone(),
        Profile {
            profile_name,
            name: user_name,
            email,
            remote,
        },
    );

    save_profiles(&profiles);
    println!("Profile added successfully!");
}

fn switch_profile() {
    let profiles = load_profiles();

    if profiles.is_empty() {
        println!("No profiles found.");
        return;
    }

    println!("Select a profile to switch to:");
    let profile_name = select_profile(&profiles);

    if let Some(p) = profiles.get(&profile_name) {
        Command::new("git")
            .args(["config", "--global", "user.name", &p.name])
            .output()
            .expect("Failed to set git user name");

        Command::new("git")
            .args(["config", "--global", "user.email", &p.email])
            .output()
            .expect("Failed to set git user email");

        if !p.remote.is_empty() {
            Command::new("git")
                .args(["remote", "set-url", "origin", &p.remote])
                .output()
                .expect("Failed to set git remote URL");
        }

        println!("Switched to profile '{}'", profile_name);
    } else {
        println!("Profile '{}' not found!", profile_name);
    }
}

fn doctor() {
    use std::path::Path;
    let profile_path = get_profile_file_path();
    let config_dir = profile_path.parent().map(Path::to_path_buf).unwrap_or_else(PathBuf::new);

    println!("Configuration directory: {}", config_dir.display());
    println!("Configuration file: {}/{}", config_dir.display(), PROFILE_FILE);
}

fn show_current_profile() {
    // Получаем текущие настройки git
    let user_name = Command::new("git")
        .args(["config", "--global", "user.name"])
        .output()
        .expect("Failed to get git user name");
    let user_email = Command::new("git")
        .args(["config", "--global", "user.email"])
        .output()
        .expect("Failed to get git user email");

    let current_name = String::from_utf8_lossy(&user_name.stdout).trim().to_string();
    let current_email = String::from_utf8_lossy(&user_email.stdout).trim().to_string();

    if !current_name.is_empty() && !current_email.is_empty() {
        println!("Current Git Profile:\n---------------------");
        println!("User Name: {}", current_name);
        println!("Email: {}", current_email);
    } else {
        println!("No Git profile is currently set.");
    }
}

fn list_profiles() {
    let profiles = load_profiles();

    if profiles.is_empty() {
        println!("No profiles found.");
    } else {
        println!("\nYour Profiles:\n--------------");
        for (profile_key, profile) in profiles {
            println!(
                "Profile Name: {}\nService_User: {}\nUser Name: {}\nEmail: {}\nRemote: {}\n",
                profile_key, profile.profile_name, profile.name, profile.email, profile.remote
            );
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

#[allow(unused_mut)]
fn select_profile(profiles: &HashMap<String, Profile>) -> String {
    let mut names: Vec<&String> = profiles.keys().collect();
    for (i, name) in names.iter().enumerate() {
        println!("{}. {}", i + 1, name);
    }

    loop {
        eprint!("Enter the number of the profile: ");
        if let Ok(index) = read_input().parse::<usize>() {
            if index > 0 && index <= names.len() {
                return names[index - 1].clone();
            }
        }
        println!("Invalid selection. Please try again.");
    }
}


fn confirm(prompt: &str) -> bool {
    eprint!("{} (y/n): ", prompt);
    let response = read_input();
    response.to_lowercase() == "y"
}

fn remove_profile() {
    let mut profiles = load_profiles();

    if profiles.is_empty() {
        println!("No profiles found.");
        return;
    }

    println!("Select a profile to remove:");
    let profile_name = select_profile(&profiles);

    if confirm(&format!("Are you sure you want to remove the profile '{}'", profile_name)) {
        if profiles.remove(&profile_name).is_some() {
            save_profiles(&profiles);
            println!("Profile '{}' removed successfully!", profile_name);
        } else {
            println!("Profile '{}' not found!", profile_name);
        }
    } else {
        println!("Operation canceled.");
    }
}

fn remove_all_profiles() {
    if confirm("Are you sure you want to remove all profiles?") {
        let profiles: HashMap<String, Profile> = HashMap::new();
        save_profiles(&profiles);
        println!("All profiles have been removed successfully!");
    } else {
        println!("Operation canceled.");
    }
}