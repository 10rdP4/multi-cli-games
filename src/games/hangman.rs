use rand::prelude::*;
use std::char;
use std::fs;
use std::io::{self, Write};

struct Hangman {
    tries: u16,
    guess_word: String,
    correct_guesses: Vec<char>,
    failed_guesses: Vec<char>,
}

impl Hangman {
    fn new() -> Hangman {
        Hangman {
            tries: 0,
            guess_word: String::new(),
            correct_guesses: Vec::new(),
            failed_guesses: Vec::new(),
        }
    }
    fn set_max_tries(&mut self) {
        loop {
            let mut input = String::new();
            print!("Max number of tries: ");
            let _ = io::stdout().flush();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let result = input.trim().parse::<u16>();
            match result {
                Ok(result) => {
                    self.tries = result;
                    return;
                }
                Err(result) => {
                    println!("Invalid value: {}", result);
                }
            }
        }
    }
    fn set_guess_word(&mut self) {
        let contents = fs::read_to_string("src/assets/words_alpha.txt").expect("Path not found");
        let lines: Vec<&str> = contents.lines().collect();

        let mut rng = rand::thread_rng();
        let pos: usize = rng.gen_range(0..lines.len());
        self.guess_word = lines[pos].to_string().to_ascii_lowercase();
    }

    fn player_guess(&self) -> char {
        let hidden = self.hide_guess_word();
        println!("Word: {}", hidden);
        println!("Failed guesses: {:?}", self.failed_guesses);
        println!("Tries left: {}", self.tries);
        loop {
            let mut input = String::new();
            print!("Char: ");
            let _ = io::stdout().flush();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            print!("\n");
            let result = input.chars().next();
            if !result.is_none() {
                return result.unwrap();
            }
        }
    }

    fn hide_guess_word(&self) -> String {
        self.guess_word
            .chars()
            .map(|c| {
                if self.correct_guesses.contains(&c) {
                    c
                } else {
                    '_'
                }
            })
            .collect::<String>()
    }

    fn check_player_guess(&mut self, guess_char: char) -> bool {
        if self.guess_word.contains(guess_char) {
            if self.correct_guesses.contains(&guess_char) {
                println!("Already guessed {}", guess_char);
                self.tries = self.tries - 1;
            } else {
                self.correct_guesses.push(guess_char);
            }
        } else {
            if !self.failed_guesses.contains(&guess_char) {
                self.failed_guesses.push(guess_char);
            }
            self.tries = self.tries - 1;
        }

        let hidden = self.hide_guess_word();
        !hidden.contains('_')
    }
}

pub fn start_game() {
    let mut game = Hangman::new();
    game.set_max_tries();
    game.set_guess_word();

    loop {
        let guess_char = game.player_guess();
        if game.check_player_guess(guess_char) {
            println!("You guessed the word: {}", game.guess_word);
            break;
        }
        if game.tries == 0 {
            println!("Out of tries :(");
            println!("Word: {}", game.guess_word);
            break;
        }
    }
}
