use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(path: &str) -> Result<Vec<Vec<String>>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let words: Vec<String> = line?
            .split_whitespace()
            .map(|word| word.to_string())
            .collect();
        result.push(words)
    }
    Ok(result)
}

fn main() {
    let file_path = "file_with_lines";
    let lines = read_file(&file_path).expect(&format!("Unable to read file <{}>", &file_path));
    println!("{:?}", lines);
}
