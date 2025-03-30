use studystarter::{Config, Unit, is_valid_unit};
use std::{env, fs, path::PathBuf, process};
mod file_checks;
mod file_parse;
use file_parse::parse_file;
mod directory_construction;
use clap::{Error, Parser, error::{ContextKind, ContextValue}};
mod repo;
use repo::get_unit_manifests;


fn main() {
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
        units: get_unit_manifests(args.codes) 
    };


}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    
    #[arg(short, long, required=true, num_args = 1..)]    
    codes: Vec<String>,
    
    #[arg(short, long)]
    output_directory: PathBuf,
}