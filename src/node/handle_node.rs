use crate::Commands;

use super::{commands::install, utils::utils::create_node_directory};

pub fn handle_node(command: Commands) {
    create_node_directory().expect("Cannot create node directory");
    match command {
        Commands::Install { version } => {
            match version {
                Some(version) => {
                    install::install(&version);
                }
                // dialoguer 
                None => {}
            };
        }
        Commands::Remove { version } => {
            let version = match version {
                Some(version) => version,
                None => "None".to_string(),
            };
            // node::remove(&version);
        }
        Commands::List => {
            // node::list()
        }
        Commands::Use { version } => {
            let version = match version {
                Some(version) => version,
                None => "None".to_string(),
            };
            // node::use_version(&version);
        }
        Commands::Current => {
            // node::current()
        }
    }
}
