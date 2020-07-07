use std::io; // import the io library from the standard library

fn main() {
    println!("Enter string to convert to pig latin:");
    println!();

    loop {
        let mut user_word = String::new(); // define a mutable variable using the mut keyword
        io::stdin()
            .read_line(&mut user_word) // pass a reference to the guess variable using & and make it mutable because referenc are immutable by default
            .expect("Failed to read line");
        let user_word: String = match user_word.trim().parse() {
            Ok(word) => word,
            Err(_) => continue,
        };

        let v: Vec<&str> = user_word.split(' ').collect();
        let mut new_string: String = String::new();

        for string in &v {
            let new_word = pig_latin(&string);
            new_string.push_str(&new_word);
            new_string.push_str(" ");
        }

        println!("{}", new_string);
        println!();
    }
}

fn pig_latin(word: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut char_iter = word.chars();
    let first_letter = char_iter.next().unwrap();
    if vowels.contains(
        &first_letter
            .to_lowercase()
            .to_string()
            .chars()
            .next()
            .unwrap(),
    ) {
        format!("{}hay", &word)
    } else {
        let remaining: String = char_iter.take(word.len()).collect();
        format!("{}{}ay", &remaining, first_letter)
    }
}

fn convert_pig_lation_to_word(word: &str) -> String {
    let last_three: String = word.chars().rev().take(3).collect(); // get the last three words
                                                                   // check if the first letter starts with H
                                                                   // if it does, remove the hay at the end of the word and return the word
                                                                   // if it does not, split the word, taking the first letter and returning it. Throw away the last 2 letters
                                                                   // append the  first letter to the beginning of the word and return the word.

    return last_three;
}
