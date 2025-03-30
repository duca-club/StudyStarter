mod file_checks;
mod file_parse;
mod repo;
mod directory_construction;

use studystarter::{Config, is_valid_unit};
use std::{path::PathBuf, process};
use clap::Parser;
use repo::get_unit_manifests;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    //collect environment arguments
    //TODO! check the parsed root directory for illegal characters and validate it.

    let args = Args::parse();

    for unit in args.codes.clone() {
        if !is_valid_unit(unit.clone()) {
            println!("{} is not a an available unit", unit);
            process::exit(0);
        }
    }

    let cfg: Config = Config { 
        output_dir: PathBuf::from(args.output_directory), 
        units: get_unit_manifests(args.codes).await?
    };


    Ok(())
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    
    #[arg(short, long, required=true, num_args = 1..)]    
    codes: Vec<String>,
    
    #[arg(short, long)]
    output_directory: PathBuf,
}