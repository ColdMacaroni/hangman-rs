use rand::prelude::*;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const WORDS_FILE: &str = "words_alpha.txt";
const WORDS_LINES: usize = 370102;

fn main() {
    let mut attempted = Vec::<char>::new();

    let file = File::open(WORDS_FILE).expect("File not found, whats wrong?");
    let buffer = BufReader::new(file);

    // If i cant unwrap this twice its your fault
    let word = buffer
        .lines()
        .nth(thread_rng().gen_range(0..WORDS_LINES))
        .unwrap()
        .unwrap();

    println!("{}", word);

    // thread_rng().gen_range(0..=WORDS_LINES);
}
