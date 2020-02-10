mod logic {
    use std;
    use std::fmt::{self, Display, Formatter};

    pub use board::*;
    use error::*;
    pub use opponent::*;
    pub use position::*;

    mod opponent {
        use super::*;

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
    }

    mod position {
        use super::*;

        #[derive(Debug, Clone, Copy)]
        pub struct Position(pub(super) usize, pub(super) usize);

        impl Position {
            pub fn new(x: usize, y: usize) -> Option<Position> {
                if (0..3).contains(&x) && (0..3).contains(&y) {
                    Some(Position(x, y))
                } else {
                    None
                }
            }
        }

        impl fmt::Display for Position {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "({}, {})", self.0, self.1)
            }
        }
    }

    mod error {
        use super::*;

        #[derive(Debug)]
        pub struct PositionTakenError {
            pub(super) pos: Position,
            pub(super) opponent: OpponentSpot,
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

        pub type SetResult = std::result::Result<(), PositionTakenError>;
    }

    mod board {
        use super::*;

        #[derive(Debug)]
        pub struct Board([[OpponentSpot; 3]; 3]);

        impl Board {
            pub fn new() -> Board {
                Board([[OpponentSpot(None); 3]; 3])
            }

            pub fn get(&self, pos: Position) -> OpponentSpot {
                self.0[pos.0][pos.1]
            }

            // pub fn taken(&self, pos: &Position) -> bool {
            //     self.0.get(10);
            //     match self.get(pos) {
            //         OpponentSpot(None) => false,
            //         _ => true,
            //     }
            // }

            pub fn set(&mut self, pos: Position, opponent: Opponent) -> SetResult {
                match self.get(pos) {
                    OpponentSpot(None) => {
                        self.0[pos.0][pos.1] = OpponentSpot(Some(opponent));
                        SetResult::Ok(())
                    }
                    opponent => SetResult::Err(PositionTakenError { pos, opponent }),
                }
            }

            pub fn print(&self) {
                println!("{}", self);
            }
        }

        impl Display for Board {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "[[{}, {}, {}],\n [{}, {}, {}],\n [{}, {}, {}]]",
                    self.0[0][0],
                    self.0[0][1],
                    self.0[0][2],
                    self.0[1][0],
                    self.0[1][1],
                    self.0[1][2],
                    self.0[2][0],
                    self.0[2][1],
                    self.0[2][2]
                )
            }
        }
    }
}

use logic::*;

fn main() {
    let mut x = Board::new();
    x.print();
    x.set(Position::new(2, 0).unwrap(), Opponent::X).unwrap();
    x.set(Position::new(1, 1).unwrap(), Opponent::O).unwrap();
    x.print();
}
