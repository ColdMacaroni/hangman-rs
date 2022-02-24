use rand::prelude::*;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// Feel free to hardcode your own list of words
const WORDS_FILE: &str = "/home/muffin/projects/programming/rust/hangman/src/words_alpha.txt";

// How many lines in the words file
const WORDS_LINES: usize = 370102;

fn main() {
    let mut solve = Vec::<char>::new();
    let mut word = Vec::<char>::new();

    let file = File::open(WORDS_FILE).expect("File not found, whats wrong?");
    let buffer = BufReader::new(file);

    // If i cant unwrap this twice its your fault
    let full_word = buffer
        .lines()
        .nth(thread_rng().gen_range(0..WORDS_LINES))
        .unwrap()
        .unwrap();

    for x in full_word.into_bytes() {
        solve.push('_');
        word.push(x as char);
    }

    let mut guesses = Vec::<char>::new();
    let mut guess = String::new();
    'game: loop {
        // TODO: Filter '_'
        let n_bad = guesses.len();

        // There's probably a better way to do this
        // 14 attempts. 15 and youre out.
        // Top bit
        println!("{}", if n_bad >= 7 { " _______" } else { "" });

        // Second line, more complex
        println!(
            "{}",
            if n_bad >= 8 {
                r"  \|/  |"
            } else if n_bad >= 6 {
                r"  \|/   "
            } else if n_bad >= 5 {
                r"   |/   "
            } else if n_bad >= 4 {
                r"   |    "
            } else {
                ""
            }
        );

        // With da head
        println!(
            "{}",
            if n_bad >= 9 {
                "   |   O"
            } else if n_bad >= 4 {
                "   |    "
            } else {
                ""
            }
        );

        // Le arms + txt
        print!(
            "{: <12}",
            if n_bad >= 12 {
                r"   |  /|\"
            } else if n_bad >= 11 {
                r"   |  /|"
            } else if n_bad >= 10 {
                r"   |   |"
            } else if n_bad >= 4 {
                r"   |    "
            } else {
                ""
            }
        );

        // Print word
        for c in &solve {
            print!("{} ", &c);
        }
        println!();

        // Leggo
        println!(
            "{}",
            if n_bad >= 14 {
                r"   |  / \"
            } else if n_bad >= 13 {
                r"   |  /  "
            } else if n_bad >= 4 {
                r"   |    "
            } else {
                ""
            }
        );

        // bot
        println!(
            "{}",
            if n_bad >= 4 {
                r"__/|\__"
            } else if n_bad >= 3 {
                r"__/_\__"
            } else if n_bad >= 2 {
                r"__/____"
            } else if n_bad >= 1 {
                r"_______"
            } else {
                ""
            }
        );

        // Print incorrect guesses
        println!();
        for c in &guesses {
            print!("{} ", &c);
        }
        println!();

        // Check if it is correct here so that it has a chance to print
        if solve == word {
            println!("You've guessed the word!");
            break 'game;
        } else if guesses.len() == 14 {
            print!("Oh lord you've killed him. The word was ");
            for c in &word {
                print!("{}", c)
            }

            println!();
            break 'game;
        }

        // Take guess from user
        guess.clear();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Couldn't open stdin");

        if let Ok(c) = guess.trim().parse::<char>() {
            // Already a guess
            if guesses.contains(&c) {
                continue 'game;

            // Not in word, incorrect
            } else if !word.contains(&c) {
                guesses.push(c);

            // Hasnt been guessed right yet
            } else if !solve.contains(&c) {
                for idx in 0..word.len() {
                    // Update all at correct position
                    if word[idx] == c {
                        solve[idx] = c;
                    }
                }
            }
        }
    }
}
