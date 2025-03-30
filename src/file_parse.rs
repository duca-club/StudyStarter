use std::{cmp::Ordering, path::PathBuf, usize};

pub fn parse_file<'a>(file: &'a [String], root_dir: &'a str) -> Result<Vec<PathBuf>, &'a str> {
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