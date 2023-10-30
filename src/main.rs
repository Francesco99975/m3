use clap::{command, Parser, Subcommand};
use install::install;
use models::{ModLoader, VersionChannel};
use search::search;
use uninstall::uninstall;
use update::update_mods;

mod api;
mod cli_error;
mod client;
mod config;
mod constants;
mod install;
mod models;
mod project_json;
mod search;
mod search_json;
mod uninstall;
mod update;
mod version_json;

#[derive(Subcommand, Debug)]
enum Commands {
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
#[command(author = "Francesco", version = "1.0", about = "Minecraft Mods Package Manager for Modrinth", long_about = None)]
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
        Some(Commands::Search { _mod }) => {
            match search((_mod.as_ref()).expect("Could not Search for it").as_str()).await {
                Ok(result) => println!("{:?}", result),
                Err(err) => eprintln!("Search Error: {:?}", err),
            }
        }
        Some(Commands::Install { directory, mods }) => {
            let dir = (directory.as_ref())
                .expect("Directory error on install")
                .as_str();
            let _mods = (mods.as_ref()).expect("List Error");

            match install(
                dir,
                _mods,
                args.loader,
                args.channel,
                args.minecraft_version,
            )
            .await
            {
                Ok(_) => println!("Mods Successfully Installed"),
                Err(err) => eprintln!("{:?}", err),
            }
        }
        Some(Commands::Update { directory }) => {
            let dir = (directory.as_ref())
                .expect("Directory error on install")
                .as_str();

            match update_mods(dir).await {
                Ok(message) => println!("{}", message),
                Err(err) => eprintln!("Could not update mods: {:?}", err),
            }
        }
        Some(Commands::Uninstall { directory, mods }) => {
            let dir = (directory.as_ref())
                .expect("Directory error on uninstall")
                .as_str();
            let _mods = (mods.as_ref()).expect("List Error");

            match uninstall(dir, _mods) {
                Ok(message) => println!("{}", message),
                Err(err) => eprintln!("{:?}", err),
            }
        }
        None => eprintln!("No Action Specified"),
    }
}
