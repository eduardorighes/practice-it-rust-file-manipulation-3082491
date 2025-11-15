use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    let file = BufReader::new(file);

    let result: Vec<String> = file.lines()
        .filter(|f| f.is_ok())
        .map(|f| f.unwrap())
        .collect();

    Ok(result)
}

fn main() {
    let file_path = "file_with_lines";

    // Implement the read_file function
    let lines: Vec<String> = read_file(&file_path).expect(&format!("Error reading file <{}>", &file_path));
    println!("{:#?}", lines);
}
