use clap::Parser;

use crate::cli::commands::Commands;

#[derive(Parser, Debug)]
#[command(author = "Gabriel 'dotxav' Xavier")]
#[command(version = "v0.1.5")]
#[command(about = None)]
#[command(long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands
}