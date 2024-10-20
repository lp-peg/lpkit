use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    name: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Test {
        #[arg(short, long)]
        list: bool,
    },
    Echo {
        #[arg(long)]
        text: String,
    },
}

fn main() {
    let cli = Cli::parse();
    if let Some(name) = cli.name.as_deref() {
        println!("Name: {}", name);
    }
    if let Some(config_path) = cli.config.as_deref() {
        println!("Config: {}", config_path.display());
    }
    match cli.debug {
        0 => println!("Debug: off"),
        1 => println!("Debug: kind of on"),
        2 => println!("Debug: on"),
        _ => println!("Don't be crazy"),
    }
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Test list");
            } else {
                println!("Test");
            }
        }
        Some(Commands::Echo { text }) => {
            println!("{}", lpkit::commands::echo::echo(text));
        }
        None => println!("No command"),
    }
}
