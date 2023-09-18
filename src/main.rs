use std::env;

use install::install;
use search::search;
use update::update_mods;

mod client;
mod constants;
mod install;
mod project_json;
mod search;
mod search_json;
mod update;
mod version_json;

#[tokio::main]
async fn main() {
    // Getting user input
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        println!("Not enough arguments... Quitting");
    }

    let action = args[0].as_str();

    match action {
        "search" => {
            let query = &args[1];
            let results = search(query).await.expect("Could not search on Modrinth");

            println!("Mods Found:");
            println!("{:?}", results);
        }
        "install" => {
            let dir = &args[1];
            let mods_to_install: Vec<&String> = args.iter().skip(2).collect();

            install(dir, mods_to_install)
                .await
                .expect("Could not install mods");

            println!("Mods installed!")
        }
        "update" => {
            let dir = &args[1];

            update_mods(dir).await.expect("could not Update Mods");

            println!("Mods Updated");
        }
        _ => (),
    }
}
