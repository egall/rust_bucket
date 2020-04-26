// $ rustc read_file.rs
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn read_file_lines(filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let _filehandle = File::open(filename)?;
    Ok(io::BufReader::new(_filehandle).lines())

}

fn read_file(filename: &Path) {
    println!("File name is {}", filename.display());
    if let Ok(_lines) = read_file_lines(filename){
        for line in _lines {
            if let Ok(ln) = line {
                println!("{}", ln);
            }
        }
        println!("Opened file");
    } else {
        eprintln!("Error, failed to open file {}", std::io::Result());
    }  
}
fn main() {
    let test_filename_string = String::from("~/scratch/grateful_dead/blah5.config");
    let test_filename = Path::new(&test_filename_string);
    println!("Testing read_file()");
    read_file(&test_filename);
}

