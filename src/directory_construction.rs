// will handle the actual directory manipulation
use std::{fs::{create_dir_all, File}, io::Write, path::PathBuf};
use anyhow::{self};
use studystarter::Config;

pub fn build_fs_tree(fs_tree: Vec<PathBuf>) -> anyhow::Result<(), > {
    for path in fs_tree {
        //Recursively create directories
        create_dir_all(path.as_path())?;
    }
    Ok(())
}

pub fn generate_readmes(config: &Config) -> anyhow::Result<()> {

    for unit in &config.units {
        let mut path = config.output_dir.clone();
        path.push(&unit.name);
        path.push("README.md");

        let mut file = File::create_new(path)?;

        file.write_all(unit.readme.as_bytes())?;
    }

    Ok(())
}
