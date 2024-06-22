use dialoguer::{theme::ColorfulTheme, Select};

use crate::{node::commands::use_version, Commands};

use super::{
    commands::{current, exec, install, list, remove},
    utils::utils::{create_node_directory, get_selection_array},
};

use crate::node::types::LTS;

pub fn handle_node(command: Commands) {
    create_node_directory().expect("Cannot create node directory");
    match command {
        Commands::Install { version } => {
            match version {
                Some(version) => {
                    install::install(&version);
                }
                None => {
                    let selections_array: Vec<LTS> = get_selection_array();
                    let selection: usize = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Select a version")
                        .default(0)
                        .items(&selections_array)
                        .interact()
                        .unwrap();

                    match &selections_array[selection] {
                        LTS { version, alias: _ } => {
                            let trim_version: &str = version.trim_start_matches("v");
                            install::install(&trim_version.to_string());
                        }
                    }
                }
            };
        }
        Commands::Remove { version } => {
            let version: String = match version {
                Some(version) => version,
                None => "None".to_string(),
            };
            remove::remove(&version);
        }
        Commands::List => {
            list::list();
        }
        Commands::Use { version } => {
            let version: String = match version {
                Some(version) => version,
                None => "None".to_string(),
            };
            use_version::use_version(&version);
        }
        Commands::Current => {
            current::current();
        }
        Commands::Exec { version, path } => {
            let version: String = match version {
                Some(version) => version,
                None => "None".to_string(),
            };
            exec::exec(&version, &path);
        }
    }
}
