use super::board::{Board, Position};
use std::io::{self, Write};

pub trait MoveInput {
    fn get_move(&self, board: &Board) -> Position;
}

#[derive(Debug)]
pub struct User {
    pub name: String,
}

impl MoveInput for User {
    fn get_move(&self, board: &Board) -> Position {
        println!("It is {}'s turn", self.name);
        println!("Here is the board:");
        board.print();
        println!("Input your move (1-9):");
        Board::print_positions();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut pos = String::new();
            io::stdin()
                .read_line(&mut pos)
                .expect("Failed to read line");

            let pos = match pos.trim().parse() {
                Ok(x) => x,
                Err(_) => {
                    println!("Invalid input: not a number");
                    continue;
                }
            };

            match Position::from_num(pos) {
                Some(x) => break x,
                None => {
                    println!("Invalid input: needs to be between 1 - 9");
                    continue;
                }
            }
        }
    }
}
