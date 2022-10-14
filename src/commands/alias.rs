use crate::config::Configs;

use super::*;
use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
pub struct Args {
    /// Template name
    #[arg(index = 1)]
    name: String,

    /// Aliases
    #[arg(index = 2)]
    aliases: Option<Vec<String>>,

    /// Clear aliases
    #[arg(short, long)]
    clear: bool,
}

pub fn command(args: Args) -> Result<()> {
    let mut config = Configs::new()?;
    let local_bucket = config.get_local_bucket();

    let template = {
        if let Some(template) = local_bucket.aliases.get(&args.name) {
            local_bucket.templates.get_mut(&template.name).unwrap()
        } else {
            println!("Template {} not found", args.name.bold());
            return Ok(());
        }
    };

    if args.clear {
        template.aliases.clear();
    } else if let Some(aliases) = args.aliases {
        template.aliases.extend(aliases);
    }

    println!("{}: ", template.name.bold());
    template.aliases.iter().for_each(|alias| {
        println!("  {}", alias);
    });

    local_bucket.index_aliases();
    config.write()?;

    Ok(())
}
