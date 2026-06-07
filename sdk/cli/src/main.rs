mod cmd_build;
mod cmd_docs;
mod cmd_new;
mod cmd_run;

use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "codeos", about = "CodeOS Developer CLI", version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new CodeOS app from template
    New {
        name: String,
        #[arg(long, default_value = "rust-app")]
        template: String,
    },
    /// Build the current app into a .capp package
    Build {
        #[arg(long, default_value = ".")]
        path: String,
    },
    /// Run app on device or simulator
    Run {
        #[arg(long)]
        simulator: bool,
        #[arg(long, default_value = ".")]
        path: String,
    },
    /// Open SDK documentation
    Docs,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    match cli.command {
        Commands::New { name, template } => cmd_new::run(&name, &template),
        Commands::Build { path } => cmd_build::run(&path),
        Commands::Run { simulator, path } => cmd_run::run(&path, simulator),
        Commands::Docs => cmd_docs::run(),
    }
}
