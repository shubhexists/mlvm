mod node;
mod utils;
use clap::{Parser, Subcommand};
use node::handle_node::handle_node;

#[derive(Parser)]
#[clap(
    name = "Multi-Language Version Manager",
    version = "1.0",
    author = "Shubham Singh"
)]
struct CLI {
    #[command(subcommand)]
    language: Language,
}

#[derive(Subcommand)]
enum Language {
    #[clap(name = "node")]
    Node {
        #[command(subcommand)]
        command: Commands,
    },
}

#[derive(Subcommand)]
enum Commands {
    #[clap(name = "install", alias = "i")]
    /// Install a version
    Install {
        /// Version to install
        version: Option<String>,
        /// debug flag, to print debug information (-d, --debug)
        #[clap(short, long)]
        debug: bool,
    },
    #[clap(name = "remove", alias = "rm")]
    /// Remove a version
    Remove {
        /// Version to remove
        version: Option<String>,
        /// debug flag, to print debug information (-d, --debug)
        #[clap(short, long)]
        debug: bool,
    },
    #[clap(name = "list", alias = "ls")]
    /// List installed versions
    List {
        /// debug flag, to print debug information (-d, --debug)
        #[clap(short, long)]
        debug: bool,
    },
    #[clap(name = "use")]
    /// Set a version to use
    Use {
        /// Version to use
        version: Option<String>,
        /// debug flag, to print debug information (-d, --debug)
        #[clap(short, long)]
        debug: bool,
    },
    #[clap(name = "current")]
    /// Get current version
    Current {
        /// debug flag, to print debug information (-d, --debug)
        #[clap(short, long)]
        debug: bool,
    },
    #[clap(name = "exec")]
    /// Execute a file with a specific version
    Exec {
        /// Version to use
        version: Option<String>,
        /// file to execute
        path: String,
        /// debug flag, to print debug information (-d, --debug)
        #[clap(short, long)]
        debug: bool,
    },
    #[clap(name = "alias")]
    /// Add aliases for node versions
    Alias {
        #[command(subcommand)]
        command: AliasCommands,
    },
}

#[derive(Subcommand)]
enum AliasCommands {
    #[clap(name = "add")]
    /// Add an alias
    Add {
        /// Version to alias
        version: String,
        /// Alias to madd
        alias: String,
        /// debug flag, to print debug information (-d, --debug)
        #[clap(short, long)]
        debug: bool,
    },
    #[clap(name = "remove", alias = "rm")]
    /// Remove an alias
    Remove {
        /// Alias to remove
        alias: String,
        /// debug flag, to print debug information (-d, --debug)
        #[clap(short, long)]
        debug: bool,
    },
    #[clap(name = "list")]
    /// List aliases
    List {
        /// debug flag, to print debug information (-d, --debug)
        #[clap(short, long)]
        debug: bool,
    },
}

fn main() {
    utils::create_mvm_directory().expect("Cannot create mvm directory");
    let args: CLI = CLI::parse();
    match args.language {
        Language::Node { command } => handle_node(command),
    }
}
