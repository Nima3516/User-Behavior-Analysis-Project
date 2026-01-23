
mod agent;
mod network;
mod logger;
mod report;
mod screen;
mod dns;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    GenerateReport,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::GenerateReport) => {
            report::generate();
        }
        None => {
            agent::run();
        }
    }
}
