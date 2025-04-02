use std::{cmp::Ordering, io::Error, path::{self, Path, PathBuf}, process::Output, usize};

use studystarter::Config;

pub fn parse_config(config: &Config) -> Result<Vec<PathBuf>, Error> {
    let mut paths: Vec<Vec<PathBuf>> = Vec::new();
    

    for unit in &config.units {
        let mut output: PathBuf = config.output_dir.clone();
        output.push(&unit.name);
        paths.push(
            parse_file(
                &unit.manifest.split("\n").collect(),
                &output 
            
            )
        ?)
    }

    Ok(paths.concat())
}

pub fn parse_file(file: &Vec<&str>, root_dir: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut current_path: PathBuf = PathBuf::new();
    current_path.push(root_dir);
    let mut current_depth: isize = -1;

    for line in file {
        //check depth of the line
        //
        //determine based on depth to move forwards/hold/backwards
        //
        let mut new_depth: isize = 0;
        for chr in line.chars() {
            if chr == '/' {
                new_depth += 1;
            } else {
                break;
            }
        }

        match new_depth.cmp(&current_depth) {
            Ordering::Less => {
                //push the current file path to the paths variable
                paths.push(current_path.clone());

                //determine using the current depth how many times to pop the current_path variable
                for _ in 0..(current_depth - new_depth + 1) {
                    current_path.pop();
                }
                current_path.push(&line[new_depth as usize..]);
            }
            Ordering::Equal => {
                paths.push(current_path.clone());
                current_path.pop();
                current_path.push(&line[new_depth as usize..]);
            }
            Ordering::Greater => {
                current_path.push(&line[new_depth as usize..]);
            }
        }
        current_depth = new_depth;
    }
    paths.push(current_path.clone());

    Ok(paths)
}