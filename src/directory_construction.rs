// will handle the actual directory manipulation
use std::{fs, path::PathBuf};

pub fn build_fs_tree(fs_tree: Vec<PathBuf>) -> (std::io::Result<()>, String) {
    for path in fs_tree {
        //Recursively create directories
        let result = fs::create_dir_all(path.as_path());
        if result.is_err() {
            return (result, path.to_str().unwrap().to_owned());
        }
    }
    (Ok(()), "completed without detecting errors".to_owned())
}
