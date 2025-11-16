use std::collections::HashMap;
use std::fs;

fn replace_x_with_y_in_place(
    mut contents: Vec<Vec<String>>,
    replacement_map: &HashMap<String, String>,
) -> Vec<Vec<String>> {
    for line in contents.iter_mut() {
        for word in line.iter_mut() {
            if let Some(new_word) = replacement_map.get(word) {
                *word = new_word.clone();
            }
        }
    }
    contents
}

fn read_file(path: &str) -> Result<Vec<Vec<String>>, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    let contents: Vec<Vec<String>> = contents
        .lines()
        .map(|line| line.split_whitespace().map(|w| w.to_string()).collect())
        .collect();
    Ok(contents)
}

fn write_file(path: &str, contents: &str) -> Result<(), std::io::Error> {
    fs::write(path, contents)?;
    Ok(())
}

fn prepare_contents(contents: &Vec<Vec<String>>) -> String {
    let lines: Vec<String> = contents.iter().map(|line| line.join(" ")).collect();
    lines.join("\n")
}

fn main() {
    let contents = read_file("alice_chapter_1").expect("cannot read file");
    let replacement_map = HashMap::from([
        ("herself".to_string(), "himself".to_string()),
        ("herself,".to_string(), "himself,".to_string()),
        ("her.".to_string(), "him.".to_string()),
        ("she".to_string(), "he".to_string()),
        ("(she".to_string(), "(he".to_string()),
        ("her".to_string(), "his".to_string()),
        ("Alice's".to_string(), "Eduardo's".to_string()),
        ("Alice!".to_string(), "Eduardo!".to_string()),
        ("Alice,".to_string(), "Eduardo,".to_string()),
        ("Alice;".to_string(), "Eduardo;".to_string()),
        ("She".to_string(), "He".to_string()),
        ("(Alice".to_string(), "(Eduardo".to_string()),
        ("Alice,)".to_string(), "Eduardo,)".to_string()),
        ("she'll".to_string(), "he'll".to_string()),
        ("she’ll".to_string(), "he’ll".to_string()),
        ("Alice".to_string(), "Eduardo".to_string()),
        ("her,".to_string(), "him,".to_string()),
        ("Alice’s".to_string(), "Eduardo’s".to_string()),
        ("girl".to_string(), "boy".to_string()),
    ]);
    let new_contents = replace_x_with_y_in_place(contents, &replacement_map);
    let buffer = prepare_contents(&new_contents);
    write_file("eduardo_chapter_1", &buffer).expect("cannot write file");
}
