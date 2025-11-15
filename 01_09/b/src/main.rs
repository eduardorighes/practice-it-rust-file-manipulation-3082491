use std::fs;

fn main() {
    let file_path = "my_file";
    let contents = "This is my new file! Congrats!!!".to_string();

    match fs::write(file_path, &contents) {
        Ok(()) => println!("contents written to {} successfully.", file_path),
        Err(e) => println!("failed to write to {}: {}", file_path, e),
    }
}