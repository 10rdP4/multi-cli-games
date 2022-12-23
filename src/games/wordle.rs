use rand::prelude::*;
use std::fs;
use std::io::{self, Write};
use colored::Colorize;

struct Wordle {
    tries: u16,
    word_lenght: u16,
    guess_word: String,
    dict: Vec<String>,
}

impl Wordle {
    fn new() -> Wordle {
        Wordle { tries: 0, word_lenght: 0, guess_word: String::new(), dict: Vec::new() }
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

    fn set_word_lenght(&mut self) {
        loop {
            let mut input = String::new();
            print!("Set the word length: ");
            let _ = io::stdout().flush();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let result = input.trim().parse::<u16>();
            match result {
                Ok(result) => {
                    self.word_lenght = result;
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
        self.dict = contents.lines().filter(|w| w.len() == self.word_lenght.into()).map(|w| w.to_string()).collect();
        let mut rng = rand::thread_rng();
        let pos: usize = rng.gen_range(0..self.dict.len());
        self.guess_word = self.dict[pos].to_string().to_ascii_lowercase();
    }

    fn player_guess(&self) -> String {
        loop {
            let mut input = String::new();
            print!("{}:", self.tries);
            let _ = io::stdout().flush();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input = input.trim().to_string();
            if self.dict.contains(&input){
                return input;
            }else {
                println!("Invalid word");
            }
        }
    }
    fn check_player_guess(&mut self, word:String) -> bool {
        if word == self.guess_word {
            return true
        }else{
            let mut asdf = self.guess_word.chars();
            for c in word.chars() {
                if asdf.next().unwrap() == c {
                    print!("{}", format!("{}", c).on_green());
                }
                else if self.guess_word.contains(c){
                    print!("{}", format!("{}", c).on_yellow());
                }
                else {
                    print!("{}", format!("{}", c));
                }
            }
            print!("\n");
            self.tries -= 1;
        }
        false
    }
}

pub fn start_game() {
    let mut game = Wordle::new();
    game.set_max_tries();
    game.set_word_lenght();
    game.set_guess_word();

    loop {
        let guess_word = game.player_guess();
        if game.check_player_guess(guess_word) {
            println!("You guessed the word: {}", game.guess_word.on_green());
            break;
        }
        if game.tries == 0 {
            println!("Out of tries :(");
            println!("Word: {}", game.guess_word);
            break;
        }
    }
}