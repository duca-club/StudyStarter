mod file_checks;
mod file_parse;
mod repo;
mod directory_construction;

use colored::Colorize;
use tokio;
use anyhow;
use clap::Parser;

use std::{path::PathBuf, process};

use directory_construction::{build_fs_tree, generate_readmes};
use file_checks::is_valid_config;
use file_parse::parse_config;
use studystarter::{Config, is_valid_unit, print_status};
use repo::get_unit_manifests;



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //collect environment arguments
    //TODO! check the parsed root directory for illegal characters and validate it.

    let args = Args::parse();

    print_status("1".green(), "6".yellow(), "Checking Input".to_owned());

    for unit in args.codes.clone() {
        if !is_valid_unit(unit.clone()) {
            println!("{} is not a an available unit", unit);
            process::exit(0);
        }
    }

    print_status("2".green(), "6".yellow(), "Downloading Units");
    let cfg: Config = Config { 
        output_dir: PathBuf::from(args.output_directory), 
        units: get_unit_manifests(args.codes).await?
    };

    print_status("3", "6", "Checking Integrity For Manifest Files");
    // make the error here more specific
    if !is_valid_config(&cfg) {
        println!("{}: {}", "ERROR".bold().bright_red(), "Invalid syntax in manifest file");
        process::exit(0);
    }

    print_status("4", "6", "Parsing Manifest Files");
    let paths: Vec<PathBuf> = parse_config(&cfg)?;


    print_status("5", "6", "Building Unit Directories");
    build_fs_tree(paths)?;
    
    print_status("6", "6", "Generating Files");
    generate_readmes(&cfg)?;

    println!("\n{}", "Done!".purple().bold());

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