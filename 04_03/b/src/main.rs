use std::collections::HashMap;

fn get_words(text: &str) -> Vec<String> {
    text.split_whitespace().map(|w| w.to_string()).collect()
}

fn word_count(words: &Vec<String>) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in words {
        map.entry(word.to_lowercase())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    map
}

fn main() {
    let contents = String::from("This is the first line\nThe second line is a little longer\nLine 3 is short\nThe 4th line is the first non-prime\nThe 5th line has the starting five");
    let words = get_words(&contents);
    println!("{:#?}", words);
    let counter = word_count(&words);
    println!("{:#?}", counter);
}
