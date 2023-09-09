use crate::core::{ Board, Color };

pub trait Player {
    fn get_column_index(&self, board: &Board, color: Color) -> Result<usize, String>;
}