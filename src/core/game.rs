use crate::core::Board;
use super::Color;

pub struct Game {
    board: Board,
    current_color: Color,
    // players: Vec<u8>,
}

impl Game {
    pub fn new(board: Option<Board>) -> Self {
        Game {
            board: {
                match board {
                    Some(board) => board,
                    None => Board::new(6, 7),
                }
            },
            current_color: Color::Red,
        }
    }

    pub fn take_turn(&mut self) {
        loop {
            let col_index;
            loop {
                let mut line = String::new();
                println!("Pick a column ({:?}):", self.current_color);
                let _ = std::io::stdin().read_line(&mut line);
                match line.trim().parse() {
                    Ok(int) => {
                        col_index = int;
                        break;
                    },
                    Err(_) => continue,
                }
            }
            match self.board.drop_piece(self.current_color, col_index) {
                Ok(_) => break,
                Err(_) => continue,
            }
        }
        println!("{:?}", self.board);
    }
}