use std::io;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut text = String::new();

    println!("Enter the text: ");

    io::stdin().read_line(&mut text).expect("Couldn't read from stdin!");

    text = text.trim().to_string();

    let words = text.split(" ");
    let mut result = vec![];

    for word in words {
        let graphemes = word.graphemes(true).collect::<Vec<&str>>();

        let starts_with_vowel = match graphemes[0] {
            "a" | "A" | "e" | "E" | "o" | "O" | "u" | "U" | "i" | "I" => true,
            _ => false
        };

        let suffix = if starts_with_vowel { "-hay".to_string() } else { format!("-{}ay", graphemes[0]) };

        let mut pig_word = String::new();

        for grapheme in if starts_with_vowel { &graphemes[..] } else { &graphemes[1..] } {
            pig_word.push_str(grapheme);
        };

        pig_word.push_str(suffix.as_str());

        result.push(pig_word);
    }

    for word in result {
        print!("{} ", word);
    }
}
