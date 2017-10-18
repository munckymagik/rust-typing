extern crate rand;
extern crate trivial_colours;

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::time::{Duration, Instant};

use rand::{Rng, ThreadRng};
use trivial_colours::{Colour, Reset};

const WORD_LIST_FILE: &str = "/usr/share/dict/words";
const TIMEOUT_SECS: u64 = 60;

fn main() {
    show_banner();

    let start_time = Instant::now();
    let timeout = Duration::from_secs(TIMEOUT_SECS);
    let words = load_words(WORD_LIST_FILE);
    let mut rng = rand::thread_rng();

    run(start_time, timeout, &mut rng, &words);
}

fn run(start_time: Instant, timeout: Duration, rng: &mut ThreadRng, words: &[String]) {
    let mut num_mistakes: i32 = 0;
    let mut num_words: i32 = 0;

    loop {
        if start_time.elapsed() >= timeout {
            println!("Time's up!");
            println!("{}{}{} words, {}{}{} mistakes",
                     Colour::Green,
                     num_words,
                     Reset,
                     Colour::Red,
                     num_mistakes,
                     Reset);
            break;
        }

        let mut buffer = String::new();
        let word = rng.choose(&words).unwrap();
        let mut attempts: i32 = 0;
        num_words += 1;

        while word != buffer.trim() {
            attempts += 1;
            let time_remaining = timeout.checked_sub(start_time.elapsed())
                                        .unwrap_or_default();

            show_prompt(word,
                        time_remaining,
                        num_words,
                        num_mistakes + attempts - 1);

            buffer.clear();
            io::stdin().read_line(&mut buffer).expect("Error reading input");
        }

        num_mistakes += attempts - 1;
    }
}

fn show_prompt(word: &str, time_remaining: Duration, num_words: i32, num_mistakes: i32) {
    if num_mistakes > 0 { print!("{}", Colour::Red) };
    print!("{}{}/{} ", num_mistakes, Reset, num_words);
    print!("{}{}s{} ", Colour::Magenta, time_remaining.as_secs(), Reset);
    print!("{}{}{}", Colour::Cyan, word, Reset);
    println!();
    print!("> ");
    io::stdout().flush().expect("Error flushing stdout");
}

fn show_banner() {
    println!("{}{} v{}{}",
             Colour::Blue,
             env!("CARGO_PKG_NAME"),
             env!("CARGO_PKG_VERSION"),
             Reset);
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
