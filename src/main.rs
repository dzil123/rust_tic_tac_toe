mod board {
    use std::fmt::{self, Display, Formatter};
    use std::ops::{Index, IndexMut};


    #[derive(Debug, Clone, Copy)]
    pub enum Opponent {
        X,
        O,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct OpponentSpot(pub Option<Opponent>);

    impl Display for OpponentSpot {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}",
                match self.0 {
                    Some(Opponent::X) => "X",
                    Some(Opponent::O) => "O",
                    None => "_",
                }
            )
        }
    }


    #[derive(Debug, Clone, Copy)]
    pub struct Position(usize, usize);

    impl Position {
        pub const POS_00: Position = Position(0, 0);
        pub const POS_01: Position = Position(0, 1);
        pub const POS_02: Position = Position(0, 2);
        pub const POS_10: Position = Position(1, 0);
        pub const POS_11: Position = Position(1, 1);
        pub const POS_12: Position = Position(1, 2);
        pub const POS_20: Position = Position(2, 0);
        pub const POS_21: Position = Position(2, 1);
        pub const POS_22: Position = Position(2, 2);

        pub fn new(x: usize, y: usize) -> Option<Position> {
            if (0..3).contains(&x) && (0..3).contains(&y) {
                Some(Position(x, y))
            } else {
                None
            }
        }

        pub fn from_num(num: u8) -> Option<Position> {
            let num = num.checked_sub(1)?;
            Position::new((num % 3).into(), (num / 3).into())
        }
    }

    impl fmt::Display for Position {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }


    #[derive(Debug)]
    pub struct PositionTakenError {
        pos: Position,
        opponent: OpponentSpot,
    }

    impl fmt::Display for PositionTakenError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Position {} is already taken by '{}'",
                self.pos, self.opponent
            )
        }
    }

    impl std::error::Error for PositionTakenError {}

    pub type SetBoardResult = std::result::Result<(), PositionTakenError>;


    #[derive(Debug)]
    struct BoardImpl([[OpponentSpot; 3]; 3]);

    impl BoardImpl {
        fn new() -> BoardImpl {
            BoardImpl([[OpponentSpot(None); 3]; 3])
        }
    }

    impl Index<Position> for BoardImpl {
        type Output = OpponentSpot;

        fn index(&self, pos: Position) -> &Self::Output {
            &self.0[pos.0][pos.1]
        }
    }

    impl IndexMut<Position> for BoardImpl {
        fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
            &mut self.0[pos.0][pos.1]
        }
    }


    #[derive(Debug)]
    pub struct Board(BoardImpl);

    impl Index<Position> for Board {
        type Output = OpponentSpot;

        fn index(&self, pos: Position) -> &Self::Output {
            &self.0[pos]
        }
    }

    impl Board {
        pub fn new() -> Board {
            Board(BoardImpl::new())
        }

        pub fn set(&mut self, pos: Position, opponent: Opponent) -> SetBoardResult {
            match self[pos] {
                OpponentSpot(None) => {
                    self.0[pos] = OpponentSpot(Some(opponent));
                    SetBoardResult::Ok(())
                }
                opponent => SetBoardResult::Err(PositionTakenError { pos, opponent }),
            }
        }

        pub fn print(&self) {
            println!("{}", self);
        }

        pub fn print_positions() {
            println!("[[1, 2, 3],\n [4, 5, 6],\n [7, 8, 9]]");
        }
    }

    impl Display for Board {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "[[{}, {}, {}],\n [{}, {}, {}],\n [{}, {}, {}]]",
                self[Position::POS_00],
                self[Position::POS_01],
                self[Position::POS_02],
                self[Position::POS_10],
                self[Position::POS_11],
                self[Position::POS_12],
                self[Position::POS_20],
                self[Position::POS_21],
                self[Position::POS_22]
            )
        }
    }
}


mod input {
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
}

use board::{Board, Opponent, Position};
use input::{MoveInput, User};

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
        None => println!("Invalid Index, as expected"),
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
