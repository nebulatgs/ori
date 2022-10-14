use crate::config::Configs;

use super::*;
use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
pub struct Args {}

pub fn command(_args: Args) -> Result<()> {
    let mut config = Configs::new()?;
    println!("Available templates:\n");
    config
        .root_config
        .buckets
        .iter()
        .for_each(|(name, bucket)| {
            println!("{}:", name.bold());
            bucket.templates.iter().for_each(|(name, template)| {
                println!("  {} - {}", name, template.description.italic().dimmed());
            });
        });
    config.index_all_aliases();
    config.write()?;

    Ok(())
}
