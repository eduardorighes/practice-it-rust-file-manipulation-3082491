use std::io;
use std::fs;

fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

fn main() {
    let file_path = "test_file";
    
    // Implement the read_file function
    let contents = read_file(&file_path).expect(&format!("unable to read file <{}>", file_path));
    println!("{}", contents);
}
