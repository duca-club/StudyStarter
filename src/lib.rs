use std::{env, path::PathBuf};
pub struct Config {
    pub output_dir: PathBuf,
    pub units: Vec<Unit>,
}

pub struct Unit {
    name: String,
    manifest: Vec<String>
}

const UnitCodes: [&str; 3] = [
    "SIT221", 
    "SIT282", 
    "SIT384"
    ];

pub fn is_valid_unit(unit: String) -> bool {
    UnitCodes.contains(&unit.as_str())    
}
