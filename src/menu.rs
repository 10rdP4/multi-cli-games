use std::io;

use crate::games::{tictactoe, hangman};

pub fn main_menu() {
    let game_list: Vec<_> = vec!["Tictactoe", "Hangman"];

    loop {
        println!(
            "--------------------------\n\
            Welcome to multi-cli-games\n\
            --------------------------\n"
        );

        println!("Please select a game:");
        for (pos, game) in game_list.iter().enumerate() {
            println!("{} -- {}", pos + 1, game);
        }
        println!("0 -- exit\n>>>");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        match input {
            "0" => {
                println!("Bye :) ");
                std::process::exit(0);
            },
            "1" => {
                tictactoe::start_game();
            },
            "2" => {
                hangman::start_game();
            },
            "3" => todo!(),
            "4" => todo!(),
            _ => todo!(),
        }
    }
}
