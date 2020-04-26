// $ rustc open_dir.rs
use std::path::Path;
use std::env;


fn traverse_dir(cur_dir: &Path) {
    println!("Directory is {}", cur_dir.display());
    for dirent in ::std::fs::read_dir(cur_dir).expect("Failed to open dir") {
        let file = dirent.expect("Didn't read file");
        let file_path = file.path();
        println!("Entry is {}", file_path.display());
        if file_path.is_dir() {
            println!("Traversing into {}", file_path.display());
            traverse_dir(&file_path);
        }
    }
}

fn help() {
    println!("usage: $ ./open_dir [path-to-dir]>");
}

fn main() {
    // Statements here are executed when the compiled binary is called

    let mut _path_string = String::new();
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("usage: $ ./open_dir [path-to-dir]>");
        },
        2 => {
            _path_string = args[1].clone();
        },
        _ => {
            help();
        }
    }

    if _path_string.is_empty() {
        _path_string = String::from("/home/egall/");
    } 

    let path = Path::new(&_path_string);
    if path.is_dir() {
        println!("{} is a directory", &_path_string);
        traverse_dir(path);
    } else {
        println!("{} is NOT a directory", &_path_string);
    }
}

