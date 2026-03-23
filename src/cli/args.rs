use clap::Parser;

use crate::cli::commands::Commands;

#[derive(Parser, Debug)]
#[command(author = "Gabriel 'dotxav' Xavier")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = None)]
#[command(long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands
}