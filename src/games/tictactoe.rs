use std::io;

pub struct TicTacToe {
    grid: [[Option<char>; 3]; 3],
    current_player: char,
    outcome: Option<char>,
}

impl TicTacToe {
    fn new() -> TicTacToe {
        TicTacToe {
            grid: [[Option::None; 3]; 3],
            current_player: 'X',
            outcome: None,
        }
    }

    fn play_move(&mut self) {
        println!("Turn of player: {}", self.current_player);
        loop {
            let row = player_input("row");
            let col = player_input("col");

            if !self.grid[row][col].is_none() {
                println!("That cell is already occupied!!");
            } else {
                self.grid[row][col] = Some(self.current_player);
                self.current_player = if self.current_player == 'X' { 'O' } else { 'X' };
                break;
            }
        }
    }

    fn check_game_over(&mut self) {
        // Checks row
        for row in 0..3 {
            if self.grid[row][0] == self.grid[row][1] && self.grid[row][1] == self.grid[row][2] {
                self.outcome = self.grid[row][0];
                return;
            }
        }

        // Checks column
        for col in 0..3 {
            if self.grid[0][col] == self.grid[1][col] && self.grid[1][col] == self.grid[2][col] {
                self.outcome = self.grid[0][col];
                return;
            }
        }

        // Checks diagonal
        if (self.grid[0][0] == self.grid[1][1] && self.grid[1][1] == self.grid[2][2])
            || (self.grid[0][2] == self.grid[1][1] && self.grid[1][1] == self.grid[2][0])
        {
            self.outcome = self.grid[1][1];
            return;
        }
    }

    fn check_left_cells(&mut self) {
        for row in 0..3 {
            for col in 0..3 {
                if self.grid[row][col].is_none() {
                    return;
                }
            }
        }
        self.outcome = Some('D');
    }

    fn print_grid(&self) {
        println!("   0  1  2");
        for row in 0..3 {
            print!("{}  ", row);
            for col in 0..3 {
                print!("{}| ", self.grid[row][col].unwrap_or(' '));
            }
            println!("");
        }
        println!("\n");
    }
}

pub fn start_game() {
    let mut game = TicTacToe::new();

    while game.outcome.is_none() {
        game.print_grid();
        game.play_move();
        game.check_left_cells();
        game.check_game_over();
    }

    game.print_grid();
    if game.outcome.unwrap() == 'D' {
        println!("Draw!!!");
    } else {
        println!("Player {} wins!!\n\n", game.outcome.unwrap());
    }
}

fn player_input(pos: &str) -> usize {
    loop {
        let mut input = String::new();
        println!("{}: ", pos);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let result = input.trim().parse::<usize>();
        match result {
            Ok(result) => {
                if result > 2 {
                    println!("Invalid value: Select value between 0 and 2")
                } else {
                    return result;
                }
            }
            Err(result) => {
                println!("Invalid value: {}", result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::games::tictactoe::TicTacToe;

    #[test]
    fn player_x_wins() {
        let mut game = TicTacToe::new();
        game.grid = [
            [Some('X'), Some('O'), Some('O')],
            [Some('X'), Some('X'), Some('O')],
            [Some('O'), Some('O'), Some('X')],
        ];
        game.check_left_cells();
        game.check_game_over();
        assert_eq!(game.outcome, Some('X'));
    }

    #[test]
    fn player_o_wins() {
        let mut game = TicTacToe::new();
        game.grid = [
            [Some('X'), Some('O'), Some('O')],
            [Some('X'), Some('X'), Some('X')],
            [Some('O'), Some('O'), Some('O')],
        ];
        game.check_left_cells();
        game.check_game_over();
        assert_eq!(game.outcome, Some('X'));
    }

    #[test]
    fn player_draw() {
        let mut game = TicTacToe::new();
        game.grid = [
            [Some('X'), Some('O'), Some('O')],
            [Some('O'), Some('X'), Some('X')],
            [Some('O'), Some('X'), Some('O')],
        ];
        game.check_left_cells();
        game.check_game_over();
        assert_eq!(game.outcome, Some('D'));
    }
}
