extern crate rand;

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use rand::Rng;

fn main() {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let f = File::open("/usr/share/dict/words").unwrap();
    let reader = BufReader::new(f);
    let mut words = vec![];

    for line in reader.lines() {
        words.push(line.unwrap().trim().to_owned());
    }

    let mut rng = rand::thread_rng();
    let mut input = String::new();
    loop {
        let word = rng.choose(&words).unwrap();
        while word != input.trim() {
            input.clear();
            println!("{}", word);
            print!("? ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
        }
    }
}
