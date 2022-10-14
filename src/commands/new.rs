use super::*;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Template to use
    #[arg(index = 1)]
    template: Option<String>,
}

pub fn command(args: Args) -> Result<()> {
    unimplemented!("new command is not implemented yet");
    Ok(())
}
