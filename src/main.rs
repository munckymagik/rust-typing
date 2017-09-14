extern crate rand;
extern crate trivial_colours;

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::time::{Duration, Instant};

use rand::Rng;
use trivial_colours::{Colour, Reset};

const WORD_LIST_FILE: &str = "/usr/share/dict/words";

fn main() {
    println!("{}{} v{}{}",
             Colour::Blue,
             env!("CARGO_PKG_NAME"),
             env!("CARGO_PKG_VERSION"),
             Reset);

    let start_time = Instant::now();
    let timeout = Duration::from_secs(60);

    let words = load_words(WORD_LIST_FILE);

    let mut rng = rand::thread_rng();
    let mut input = String::new();

    loop {
        if start_time.elapsed() >= timeout {
            println!("Time's up!");
            break;
        }

        let word = rng.choose(&words).unwrap();

        while word != input.trim() {
            let time_remaining = timeout.checked_sub(start_time.elapsed())
                                        .unwrap_or_default();

            print!("{}{}s{} ",
                   Colour::Magenta,
                   time_remaining.as_secs(),
                   Reset);
            println!("{}{}{}", Colour::Cyan, word, Reset);

            print!("> ");
            io::stdout().flush().expect("Error flushing stdout");

            input.clear();
            io::stdin().read_line(&mut input).expect("Error reading input");
        }
    }
}

fn load_words(word_list_file: &str) -> Vec<String> {
    let mut words = vec![];
    let file = File::open(word_list_file)
                    .expect(&(String::new() + word_list_file + " not found"));
    let reader = BufReader::new(file);

    for line in reader.lines() {
        words.push(line.unwrap().trim().to_owned());
    }

    words
}
