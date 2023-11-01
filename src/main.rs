use clap::{command, Parser, Subcommand};
use install::install;
use list::list;
use models::{ModLoader, VersionChannel};
use search::search;
use tabled::Table;
use uninstall::uninstall;
use update::update_mods;

mod api;
mod cli_error;
mod client;
mod config;
mod install;
mod list;
mod models;
mod progress;
mod search;
mod uninstall;
mod update;

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "List installed mods")]
    List {
        #[arg(short = 'd', long = "directory", default_value = "mods")]
        directory: Option<String>,
    },
    #[command(about = "Search for a mod on Modrinth")]
    Search { _mod: Option<String> },
    #[command(about = "Install Mods")]
    Install {
        #[arg(short = 'd', long = "directory", default_value = "mods")]
        directory: Option<String>,
        mods: Option<Vec<String>>,
    },
    #[command(about = "Update Mods")]
    Update {
        #[arg(short = 'd', long = "directory", default_value = "mods")]
        directory: Option<String>,
    },
    #[command(about = "Uninstall Mods")]
    Uninstall {
        #[arg(short = 'd', long = "directory", default_value = "mods")]
        directory: Option<String>,
        mods: Option<Vec<String>>,
    },
}

#[derive(Parser, Debug)]
#[command(author = "Francesco Michele Barranca (kalairendev)", version = "1.1", about = "Minecraft Mods Package Manager for Modrinth", long_about = None)]
#[command(propagate_version = true)]
struct M3 {
    #[arg(short = 'm', long = "mc-version", default_value = "1.20.1")]
    minecraft_version: String,

    #[arg(short = 'l', long = "loader", default_value = "fabric")]
    loader: ModLoader,

    #[arg(short = 'c', long = "channel", default_value = "release")]
    channel: VersionChannel,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[tokio::main]
async fn main() {
    let args = M3::parse();

    match &args.command {
        Some(Commands::List { directory }) => {
            let dir = (directory.as_ref())
                .expect("Directory error on install! Could not use default value for some reason")
                .as_str();

            match list(dir) {
                Ok(res) => {
                    if !res.is_empty() {
                        let table = Table::new(res).to_string();

                        print!("{}", table);
                    } else {
                        println!("No mods installed");
                    }
                }
                Err(_) => eprintln!("Could not list mods"),
            };
        }
        Some(Commands::Search { _mod }) => match _mod {
            Some(_mod) => match search(_mod.as_ref()).await {
                Ok(result) => {
                    let table = Table::new(result).to_string();

                    print!("{}", table);
                }
                Err(err) => eprintln!("Search Error: {:?}", err),
            },
            None => eprintln!("No input for searching..."),
        },
        Some(Commands::Install { directory, mods }) => {
            let dir = (directory.as_ref())
                .expect("Directory error on install! Could not use default value for some reason")
                .as_str();
            match mods {
                Some(mods) => {
                    println!("Installing...");
                    match install(dir, mods, args.loader, args.channel, args.minecraft_version)
                        .await
                    {
                        Ok(_) => println!("Mods Successfully Installed"),
                        Err(err) => eprintln!("Install Error: {:?}", err),
                    }
                }
                None => eprintln!(
                    "Please provide some mods to install. Use the exact slug found by searching."
                ),
            }
        }
        Some(Commands::Update { directory }) => {
            let dir = (directory.as_ref())
                .expect("Directory error on install! Could not use default value for some reason")
                .as_str();
            println!("Updating...");
            match update_mods(dir).await {
                Ok(message) => println!("{}", message),
                Err(err) => eprintln!("Could not update mods: {:?}", err),
            }
        }
        Some(Commands::Uninstall { directory, mods }) => {
            let dir = (directory.as_ref())
                .expect("Directory error on install! Could not use default value for some reason")
                .as_str();

            match mods {
                Some(mods) => match uninstall(dir, mods) {
                    Ok(message) => println!("{}", message),
                    Err(err) => eprintln!("{:?}", err),
                },
                None => eprintln!(
                    "Please provide mods to uninstall. Use the exact slug found by searching."
                ),
            }
        }
        None => eprintln!("No Action Specified. Use -help to use m3 properly"),
    }
}
