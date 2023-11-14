use std::{env, fs};


fn find_file(path: &str, file: &str) -> bool {
    let paths = fs::read_dir(path).expect("Error");
    for path in paths {
        let path_unwrapped = path.unwrap();
        if path_unwrapped.path().is_file() && path_unwrapped.file_name().to_str().expect("") == file {
            println!("Found full path: {}", path_unwrapped.path().display().to_string());
            return true;
        } else if !path_unwrapped.path().is_file() {
            if find_file(&path_unwrapped.path().display().to_string(), file) {
                return true;
            }
        }
    };
    return false;
}


fn recursive_output(p: &str) {
    let paths = fs::read_dir(p).expect("Error");
    for path in paths {
        let path_unwrapped = path.unwrap();
        if path_unwrapped.path().is_file() {
            println!("{}", path_unwrapped.path().display().to_string());
        } else {
            recursive_output(&path_unwrapped.path().display().to_string());
        }
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 4 && args[2] == "--find" {
        if !find_file(&args[1], &args[3]) {
            println!("File not found");
        }
    } else {
        recursive_output(&args[1]);
    }
}
