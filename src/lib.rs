use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub output_dir: PathBuf,
    pub units: Vec<Unit>,
}

#[derive(Debug)]
pub struct Unit {
    pub name: String,
    pub manifest: Vec<PathBuf>,
    pub readme: String,
}

const UnitCodes: [&str; 3] = [
    "SIT221", 
    "SIT282", 
    "SIT384"
    ];

pub fn is_valid_unit(unit: String) -> bool {
    UnitCodes.contains(&unit.as_str())    
}
