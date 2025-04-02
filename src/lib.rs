use std::{fmt::Display, path::PathBuf};
use colored::Colorize;

#[derive(Debug)]
pub struct Config {
    pub output_dir: PathBuf,
    pub units: Vec<Unit>,
}

#[derive(Debug)]
pub struct Unit {
    pub name: String,
    pub manifest: String,
    pub readme: String,
}

const UNIT_CODES: [&str; 5] = [
    "SIT221", 
    "SIT282", 
    "SIT384",
    "SIT192",
    "SIT202",
    ];

pub fn is_valid_unit(unit: String) -> bool {
    UNIT_CODES.contains(&unit.as_str())    
}

pub fn print_status<T,K,L>(x: T, y: K, message: L, ) where T: Display, K: Display, L: Display {
    println!("{} {}", format!("[{}/{}]", x, y).bold(), message);
}