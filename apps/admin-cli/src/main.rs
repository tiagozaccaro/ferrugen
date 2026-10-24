use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "saas-admin",
    about = "Local administration CLI for the library"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Show the CLI version.
    Version,
    /// Print the deployment readiness checklist.
    Doctor,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Version => println!("saas-admin {}", env!("CARGO_PKG_VERSION")),
        Command::Doctor => {
            println!(
                "Not production-ready: database migrations, auth persistence, and backup checks remain to be implemented."
            );
        }
    }
}
