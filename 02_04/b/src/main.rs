use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn main() {
    let file_path = "test_file";

    // Implement the read_file function
    let contents = read_file(file_path);
    println!("{}", contents);
}
