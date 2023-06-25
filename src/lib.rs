use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("{0:?} is currently occupied")]
    IndexError(Square),
}

#[derive(Debug, PartialEq)]
pub enum Mark {
    Cross,
    Naught,
}

impl Mark {
    pub fn other(&self) -> Mark {
        match self {
            Mark::Cross => Mark::Naught,
            Mark::Naught => Mark::Cross,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Mark::Cross => "X",
            Mark::Naught => "O",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Square {
    A1 = 0x80080080,
    A2 = 0x40008000,
    A3 = 0x20000808,
    B1 = 0x08040000,
    B2 = 0x04004044,
    B3 = 0x02000400,
    C1 = 0x00820002,
    C2 = 0x00402000,
    C3 = 0x00200220,
}

#[derive(Debug, PartialEq)]
pub enum Triple {
    RowA,
    RowB,
    RowC,
    Col1,
    Col2,
    Col3,
    Diag1,
    Diag2,
}

impl From<u32> for Triple {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::RowA,
            1 => Self::RowB,
            2 => Self::RowC,
            3 => Self::Col1,
            4 => Self::Col2,
            5 => Self::Col3,
            6 => Self::Diag1,
            7 => Self::Diag2,
            _ => panic!("This should never happen!"),
        }
    }
}

#[derive(Clone, Copy)]
struct Board {
    xboard: u32,
    oboard: u32,
}

impl Board {
    pub fn new() -> Self {
        Self {
            xboard: 0x0,
            oboard: 0x0,
        }
    }

    pub fn calculate_winner(&self) -> Option<(Mark, Triple)> {
        let xboard = self.xboard & (self.xboard << 1) & (self.xboard >> 1);
        let oboard = self.oboard & (self.oboard << 1) & (self.oboard >> 1);
        if xboard >= 1 {
            Some((Mark::Cross, Triple::from(xboard.leading_zeros() - 1 >> 2)))
        } else if oboard >= 1 {
            Some((Mark::Naught, Triple::from(oboard.leading_zeros() - 1 >> 2)))
        } else {
            None
        }
    }

    fn check_index(&self, square: &Square) -> Result<(), ErrorKind> {
        if (*square as u32 & (self.xboard | self.oboard)) == 0 {
            Ok(())
        } else {
            Err(ErrorKind::IndexError(*square))
        }
    }

    pub fn make_move(&self, mark: &Mark, square: Square) -> Result<Self, ErrorKind> {
        self.check_index(&square)?;
        Ok(match mark {
            Mark::Cross => Self {
                xboard: self.xboard | square as u32,
                oboard: self.oboard,
            },
            Mark::Naught => Self {
                xboard: self.xboard,
                oboard: self.oboard | square as u32,
            },
        })
    }
}

pub struct Game {
    current_mark: Mark,
    board: Board,
}

impl Game {
    pub fn new(starting_mark: Mark) -> Game {
        Game {
            current_mark: starting_mark,
            board: Board::new(),
        }
    }

    pub fn make_move(&self, square: Square) -> Result<Self, ErrorKind> {
        let current_state = self.board.make_move(&self.current_mark, square)?;
        Ok(Game {
            current_mark: self.current_mark.other(),
            board: current_state,
        })
    }

    pub fn calculate_winner(&self) -> Option<(Mark, Triple)> {
        self.board.calculate_winner()
    }
}

impl Default for Game {
    fn default() -> Game {
        Game {
            current_mark: Mark::Cross,
            board: Board::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_calculate_winner() {
        // | |O|X|
        // |X|X|O|
        // |X| |O|

        let game = Game::default();
        let game = game.make_move(Square::B2).unwrap();
        let game = game.make_move(Square::A2).unwrap();
        let game = game.make_move(Square::B1).unwrap();
        let game = game.make_move(Square::B3).unwrap();
        let game = game.make_move(Square::C1).unwrap();
        let game = game.make_move(Square::C3).unwrap();
        let game = game.make_move(Square::A3).unwrap();

        assert_eq!(
            game.calculate_winner().unwrap(),
            (Mark::Cross, Triple::Diag2)
        );
    }

    #[test]
    fn test_game_make_move_ok() {
        let game = Game::default();
        let game = game.make_move(Square::A2);
        assert!(game.is_ok());
    }

    #[test]
    fn test_game_make_move_err() {
        let game = Game::default();
        let game = game.make_move(Square::B2).unwrap();
        let game = game.make_move(Square::B2);
        assert!(game.is_err());
    }
}
