use std::fs;

fn print_wanted_lines_from_file(file_path: &str, wanted_string: &str) {
    let contents = fs::read_to_string(file_path).unwrap();
    for line in contents.lines() {
        if line.contains(wanted_string) {
            println!("found line: {}", line);
        }
    }
}

fn main() {
    let file_path = "file_with_lines";
    let wanted_string = "a";

    // Create the below function
    print_wanted_lines_from_file(file_path, wanted_string);
}
