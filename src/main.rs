mod board;
mod input;

use crate::board::{Board, Opponent, Position};
use crate::input::{MoveInput, User};

fn test_board() {
    let mut x = Board::new();
    x.print();

    println!("{}", x[Position::POS_20]);
    x.set(Position::POS_20, Opponent::X).unwrap();
    x.set(Position::POS_11, Opponent::O).unwrap();


    // Expect PositionTakenError: print nice error message
    println!(
        "{}, as expected",
        x.set(Position::POS_11, Opponent::X).unwrap_err() // Should not panic
    );

    // Expect None from invalid index
    match Position::new(10, 0) {
        Some(_) => panic!("Unexpected Some"), // Should not panic
        None => println!("Invalid index, as expected"),
    }

    // x[Position::new(2, 0).unwrap()] = OpponentSpot(None); // Does not compile, as expected


    x.print();
    println!("{}", x[Position::new(2, 0).unwrap()]);
}

fn main() {
    let board = Board::new();
    let bob = User {
        name: "Bob".to_string(),
    };
    let pos = bob.get_move(&board);
    println!("\nYou have chosen position: {}", pos);
}
