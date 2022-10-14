use std::{collections::BTreeSet, fs, path::Path};

use crate::config::{Configs, Template};

use super::*;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Template name
    #[arg(short, long)]
    name: String,

    /// Template description
    #[arg(short, long)]
    description: Option<String>,

    /// Template source
    /// (can be a local path or a git repository)
    #[arg(index = 1, default_value = ".")]
    source: String,
}

pub fn command(args: Args) -> Result<()> {
    let mut config = Configs::new()?;
    let local_bucket = config.get_local_bucket();

    let source = if args.source.starts_with("http") || args.source.starts_with("git") {
        args.source
    } else {
        let path = fs::canonicalize(Path::new(&args.source))?;
        path.to_str().unwrap().to_string()
    };

    local_bucket.templates.insert(
        args.name.clone(),
        Template {
            name: args.name.clone(),
            description: args.description.unwrap_or_default(),
            source,
            aliases: BTreeSet::new(),
        },
    );

    local_bucket.index_aliases();
    config.write()?;

    Ok(())
}
