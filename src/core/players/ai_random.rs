use crate::core::{ Player, Board, Color };

#[allow(dead_code)]
pub struct AIPlayer {
    name: String,
}

impl AIPlayer {
    #[allow(dead_code)]
    fn new(name: String) -> Self {
        AIPlayer {
            name,
        }
    }
}

impl Player for AIPlayer {
    #[allow(unused_variables)]
    fn get_column_index(&self, board: &Board, color: Color) -> Result<usize, String> {
        Ok(3)
    }
}