use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod config;

/// Ori - Git Scaffolding
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scaffold a new project
    New(commands::new::Args),

    /// List available templates
    List(commands::list::Args),

    /// Create a new template
    Create(commands::create::Args),

    /// Add a bucket
    Add(commands::add::Args),

    /// Add or list aliases
    Alias(commands::alias::Args),
}

fn main() -> Result<()> {
    let cli = Args::parse();

    match cli.command {
        Commands::New(args) => commands::new::command(args)?,
        Commands::List(args) => commands::list::command(args)?,
        Commands::Create(args) => commands::create::command(args)?,
        Commands::Add(args) => commands::add::command(args)?,
        Commands::Alias(args) => commands::alias::command(args)?,
    }

    Ok(())
}
