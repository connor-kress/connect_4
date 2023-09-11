use crate::core::{ Board, Color, Player, clear_screen };

pub struct Game {
    board: Board,
    players: Vec<Box<dyn Player>>,
    player_colors: Vec<Color>,
    current_player_index: usize,
    amount_to_win: usize,
    started: bool,
    ended: bool,
}

impl Game {
    #[allow(dead_code)]
    pub fn new(board: Option<Board>,
               players: Vec<Box<dyn Player>>,
               player_colors: Vec<Color>) -> Result<Self, String> {
        if players.len() != player_colors.len() {
            return Err(
                "Player list and player color list's lengths do not match".to_string()
            );
        }
        Ok(Game {
            board: {
                match board {
                    Some(board) => board,
                    None => Board::new(6, 7, 3, 7),
                }
            },
            players,
            player_colors,
            current_player_index: 0,
            amount_to_win: 4,
            started: false,
            ended: false,
        })
    }

    fn get_current_player(&self) -> &Box<dyn Player> {
        &self.players[self.current_player_index]
    }
    
    fn get_current_color(&self) -> Color {
        self.player_colors[self.current_player_index]
    }

    fn switch_turn(&mut self) {
        if self.current_player_index == self.players.len() - 1 {
            self.current_player_index = 0;
        } else {
            self.current_player_index += 1;
        }
    }

    fn take_turn(&mut self) -> Result<(), String> {
        let color = self.get_current_color();
        let col_index = self.get_current_player().get_column_index(&self.board, color)?;
        self.board.drop_piece(color, col_index)
    }

    fn handle_win(&mut self, color: Color) -> Result<(), String> {
        self.ended = true;
        clear_screen();
        self.board.print()?;
        println!("{:?} wins!", color);
        Ok(())
    }

    fn handle_tie(&mut self) -> Result<(), String> {
        self.ended = true;
        clear_screen();
        self.board.print()?;
        println!("Tie.");
        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), String> {
        if !self.started {
            return Err("Attempted to resume an uninstantiated game.".to_string());
        } else if self.ended {
            return Err("Attempted to resume an ended game.".to_string());
        }
        loop {
            self.take_turn()?;
            if let Some(color) = self.board.get_winning_color(self.amount_to_win) {
                self.handle_win(color)?;
                break;
            }
            if self.board.is_full() {
                self.handle_tie()?;
                break;
            }
            self.switch_turn();
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn start(&mut self) -> Result<(), String> {
        if self.started {
            return Err("Attempted to start a instantiated game.".to_string());
        } else if self.ended {
            return Err("Attempted to start an ended game.".to_string());
        }
        self.started = true;
        self.resume()
    }
}
