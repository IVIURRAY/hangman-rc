use std::io;
use std::string::ToString;
use rand::Rng;


fn main() {
    let words = vec![
        "software".to_string(),
        "rust".to_string(),
        "programming".to_string(),
    ];

    let random_word_index = rand::thread_rng().gen_range(0..words.len());
    let word = &words[random_word_index];
    let mut display_word = vec!["_".to_string(); word.len()];
    let mut guesses_remaining = 6;
    let mut guessed_chars: Vec<String> = vec![];

    while guesses_remaining > 0 {
        println!("[ {} ]", display_word.join(" "));
        println!("What letter do you guess?: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unable to parse user input");
        let input = input.trim();

        if guessed_chars.contains(&input.to_string()) {
            println!("You have already guessed {}, try again.", input);
        } else if word.contains(input) {
            for (i, char) in word.chars().enumerate() {
                if char.to_string() == input {
                    display_word[i] = input.to_string();
                }
            }
        } else {
            guesses_remaining -= 1;
            println!("Incorrect - Guesses remaining {}", guesses_remaining);
        };

        guessed_chars.push(input.to_string());

        if !display_word.contains(&String::from("_")) {
            println!("You won! - You guessed `{}` with {} guesses remaining!", word, guesses_remaining);
            break;
        }
    }
}
