use dialoguer::{theme::ColorfulTheme, Select};

use crate::{node::commands::use_version, AliasCommands, Commands};

use super::{
    commands::{alias, current, exec, install, list, remove},
    utils::{lts::LtsAliases, utils::create_node_directory},
};

use crate::node::types::LTS;

pub fn handle_node(command: Commands) {
    create_node_directory().expect("Cannot create node directory");
    match command {
        Commands::Install { version, debug } => {
            match version {
                Some(version) => {
                    install::install(&version, debug);
                }
                None => {
                    let selections_array: Vec<LTS> = vec![
                        LTS {
                            version: LtsAliases::get_alias(&LtsAliases::IRON).version,
                            alias: "Lts".to_string(),
                        },
                        LtsAliases::get_alias(&LtsAliases::ARGON),
                        LtsAliases::get_alias(&LtsAliases::BORON),
                        LtsAliases::get_alias(&LtsAliases::CARBON),
                        LtsAliases::get_alias(&LtsAliases::DUBNIUM),
                        LtsAliases::get_alias(&LtsAliases::ERBIUM),
                        LtsAliases::get_alias(&LtsAliases::FERMIUM),
                        LtsAliases::get_alias(&LtsAliases::GALLIUM),
                        LtsAliases::get_alias(&LtsAliases::HYDROGEN),
                        LtsAliases::get_alias(&LtsAliases::IRON),
                    ];
                    let selection: usize = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Select a version")
                        .default(0)
                        .items(&selections_array)
                        .interact()
                        .unwrap();

                    match &selections_array[selection] {
                        LTS { version, alias: _ } => {
                            let trim_version: &str = version.trim_start_matches("v");
                            install::install(&trim_version.to_string(), debug);
                        }
                    }
                }
            };
        }
        Commands::Remove { version, debug } => {
            let version: String = match version {
                Some(version) => version,
                None => "None".to_string(),
            };
            remove::remove(&version, debug);
        }
        Commands::List { debug } => {
            list::list(debug);
        }
        Commands::Use { version, debug } => {
            let version: String = match version {
                Some(version) => version,
                None => "None".to_string(),
            };
            use_version::use_version(&version, debug);
        }
        Commands::Current { debug } => {
            current::current(debug);
        }
        Commands::Exec {
            version,
            path,
            debug,
        } => {
            let version: String = match version {
                Some(version) => version,
                None => "None".to_string(),
            };
            exec::exec(&version, &path, debug);
        }
        Commands::Alias { command } => match command {
            AliasCommands::Add {
                version,
                alias,
                debug,
            } => alias::alias_add(&version, &alias, debug),
            AliasCommands::Remove { alias, debug } => alias::alias_remove(&alias, debug),
            AliasCommands::List { debug } => alias::alias_list(debug),
        },
    }
}
