use crate::config::{Bucket, Configs};

use super::*;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Bucket source
    #[arg(index = 1)]
    source: String,

    /// Bucket name
    #[arg(index = 2)]
    alias: String,
}

pub fn command(args: Args) -> Result<()> {
    let mut config = Configs::new()?;

    config.root_config.buckets.insert(
        args.alias.clone(),
        Bucket::new(args.alias.clone(), args.source.clone()),
    );

    config.index_all_aliases();
    config.write()?;

    Ok(())
}
