use crate::core::Board;
use super::Color;

pub struct Game {
    board: Board,
    current_color: Color,
    amount_to_win: usize,
    started: bool,
    ended: bool,
    // players: Vec<Player>,
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
            amount_to_win: 4,
            started: false,
            ended: false,
        }
    }

    fn switch_turn(&mut self) {
        self.current_color = match self.current_color {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        }
    }

    fn take_turn(&mut self) {
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
    }

    fn handle_win(&mut self, color: Color) {
        self.ended = true;
        println!("{:?}", self.board);
        println!("{:?} wins!", color);
    }

    fn handle_tie(&mut self) {
        self.ended = true;
        println!("{:?}", self.board);
        println!("Tie.");
    }

    pub fn resume(&mut self) -> Result<(), ()> {
        if self.ended | !self.started {
            return Err(());
        }
        loop {
            println!("{:?}", self.board);
            self.take_turn();
            match self.board.get_winning_color(self.amount_to_win) {
                Some(color) => {
                    self.handle_win(color);
                    break;
                },
                None => {},
            }
            if self.board.is_full() {
                self.handle_tie();
                break;
            }
            self.switch_turn();
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn start(&mut self) -> Result<(), ()> {
        if self.started | self.ended {
            return Err(());
        }
        self.started = true;
        self.resume()
    }
}