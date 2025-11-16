use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(path: &str) -> Result<Vec<Vec<String>>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let result: Vec<Vec<String>> = reader
        .lines()
        .filter_map(|line| {
            if line.is_ok() {
                Some(line.unwrap())
            } else {
                None
            }
        })
        .map(|line| line.split_whitespace().map(|w| w.to_string()).collect())
        .collect();

    Ok(result)
}

fn main() {
    let file_path = "file_with_lines";
    let lines = read_file(&file_path).expect(&format!("Unable to read file <{}>", &file_path));
    println!("{:?}", lines);
}
