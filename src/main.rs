extern crate rand;
extern crate trivial_colours;

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::time::{Duration, Instant};

use rand::Rng;
use trivial_colours::{Colour, Reset};

fn main() {
    println!("{}{} v{}{}",
             Colour::Blue,
             env!("CARGO_PKG_NAME"),
             env!("CARGO_PKG_VERSION"),
             Reset);

    let start_time = Instant::now();
    let timeout = Duration::from_secs(60);

    let f = File::open("/usr/share/dict/words").unwrap();
    let reader = BufReader::new(f);
    let mut words = vec![];

    for line in reader.lines() {
        words.push(line.unwrap().trim().to_owned());
    }

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
            println!("{}Time remaining {}s{}",
                     Colour::Magenta,
                     time_remaining.as_secs(),
                     Reset);

            println!("{}{}{}", Colour::Cyan, word, Reset);
            print!("? ");
            io::stdout().flush().unwrap();

            input.clear();
            io::stdin().read_line(&mut input).unwrap();
        }
    }
}
