mod cli;
mod log;
mod core;

use clap::Parser;
use cli::args::Args;

use log::log::{ Log, LogF };

use cli::matches::match_cli;

use crate::core::{errors::MyWayError, filemanager::{Fiman, ReturnReadType}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // file manager
    let mut fileman: Fiman = Fiman::new()?;
    let _ = fileman.setup();

    let log: Log = Log::new(); // logs
    let cli: Args = Args::parse(); // cli

    // read myway_projects.json and convert to Vec<Project>
    let mut data = match fileman.read(&fileman.mw_path.clone())? {
        ReturnReadType::GenericList(p) => p,
        _ => return Err(MyWayError::InvalidInput("File projects corrupted".to_string()).into())
    };

    let mut data_graveyard = match fileman.read(&fileman.graveyard_path.clone())? {
        ReturnReadType::GenericList(p) => p,
        _ => return Err(MyWayError::InvalidInput("File graveyard corrupted".to_string()).into())
    };

    let mut user_data = match fileman.read(&fileman.user_path.clone())? {
        ReturnReadType::User(p) => p,
        _ => return Err(MyWayError::InvalidInput("File user corrupted".to_string()).into())
    };

    // initiate the match to analize command delivered
    match_cli(
        &cli.command, 
        log, 
        &mut fileman, 
        &mut data, 
        &mut data_graveyard,
        &mut user_data
    )?;

    Ok(())

}
