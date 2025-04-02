use studystarter::Config;

#[cfg(target_os = "linux")]
const INVALID_FS_CHARS: &str = "/"; //characters forbidden in a linux folder or file name and for this program.

#[cfg(target_os = "windows")]
const INVALID_FS_CHARS: &str = "<>:\"/\\|?*"; //characters forbidden in a windows folder or file name and for this program.

#[cfg(target_os = "macos")]
const INVALID_FS_CHARS: &str = ":/"; //characters forbidden in a macos folder or file name and for this program.

pub fn is_valid_file(file: &str) -> bool {
    //strip and clean program specific chars.
    let file_lines: Vec<String> = file.split('\n').map(|x| x.to_owned()).collect();
    let file_lines: Vec<String> = file_lines.iter().map(|x| x.trim().to_owned()).collect();

    //check for invalid file syntax
    if !is_valid_syntax(&file_lines) {
        return false;
    }
    true
}

pub fn is_valid_config(config: &Config) -> bool {
    for unit in &config.units {
        if !is_valid_file(unit.manifest.as_str()) {
            return false
        }
        
    }
    true
}

///Check for program specific rule violations as discussed in the program instructions.
fn is_valid_syntax(files_lines: &Vec<String>) -> bool {
    let mut current_line_num: usize = 0;
    let mut cleaned_file_lines: Vec<String> = Vec::with_capacity(files_lines.len());

    for (index, line) in files_lines.iter().enumerate() {
        let mut new_line_num: usize = 0;
        for chr in line.chars() {
            if chr == '/' {
                new_line_num += 1;
            } else {
                break;
            }
        }

        //check to make sure that the user did not try to increase folder depth by more than one at a time (going from "/example" to "///example3").
        if current_line_num + 1 < new_line_num {
            let index = index + 1;
            eprintln!(
                "Invalid Syntax: Attempted to increase folder depth by more than 1 on line -> {}",
                index
            );
            return false;
        }
        current_line_num = new_line_num;
        cleaned_file_lines.push(line[new_line_num..].to_owned());
    }

    //check if the remain string (stripped of the program specific syntax) contains illegal characters.
    if contains_invalid_chars(&cleaned_file_lines) {
        return false;
    };
    true
}

fn contains_invalid_chars(cleaned_file: &[String]) -> bool {
    for (index, line) in cleaned_file.iter().enumerate() {
        for chr in INVALID_FS_CHARS.chars() {
            if line.contains(chr) {
                let index = index + 1;
                eprintln!("Invalid Character: illegal file system character detected!\n>Line: {index}\n>Character: \"{chr}\"");
                return true;
            }
        }
    }

    false
}

//NOTE: add option to log any errors found in the file to a file.
