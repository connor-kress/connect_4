use rand::Rng;
use crate::core::{ Player, Board, Color };

#[allow(dead_code)]
pub struct AIPlayer {
    name: String,
}

impl AIPlayer {
    #[allow(dead_code)]
    pub fn new(name: String) -> Self {
        AIPlayer {
            name,
        }
    }
}

impl Player for AIPlayer {
    #[allow(unused_variables)]
    fn get_column_index(&self, board: &Board, color: Color) -> Result<usize, String> {
        let mut rng = rand::thread_rng();
        loop {
            let col_index = rng.gen_range(0..board.num_columns);
            if board.available_column(col_index) {
                break Ok(col_index);
            }
        }
    }
}