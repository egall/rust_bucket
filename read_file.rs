// $ rustc read_file.rs
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn print_lines(filelines: io::Lines<io::BufReader<File>>) {
    for line in filelines {
        if let Ok(ip) = line {
              println!("{}", ip);
         }
    } 
}

fn open_file(filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let _filehandle = File::open(filename)?;
    Ok(io::BufReader::new(_filehandle).lines())

}

fn read_file(filename: &Path) {
    println!("File name is {}", filename.display());
    let _lines = match open_file(filename){
        Ok (_lines) => 
             print_lines(_lines),
        Err(_e) =>
            eprintln!("Error, failed to open file {}", _e),
    };
}

fn main() {
    let test_filename_string = String::from("/home/egall/scratch/greatful_dead/blah5.config");
    let test_filename = Path::new(&test_filename_string);
    println!("Testing read_file()");
    read_file(&test_filename);
}

