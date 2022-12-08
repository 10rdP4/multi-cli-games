use std::io::{self, BufRead};

pub fn main_menu(){
    let game_list: Vec<_> = vec![
        "Tictactoe",
        "Hangman"
        ];

    println!("--------------------------\n\
            Welcome to multi-cli-games\n\
            --------------------------\n");
    
    println!("Please select a game:");
    for (pos, game) in game_list.iter().enumerate() {
        println!("{} -- {}", pos + 1, game);
    }
    println!("0 -- exit\n>>>");
    let mut selected_game = String::new();

    get_input(&mut selected_game).unwrap();

    match selected_game.as_str() {
        "0" => todo!(),
        "1" => todo!(),
        "2" => todo!(),
        "3" => todo!(),
        "4" => todo!(),
        _ => todo!()
        
    }

}

fn get_input(selection: &mut String) -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_line( selection)?;
    Ok(())
}