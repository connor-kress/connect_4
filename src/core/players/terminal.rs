use crate::core::{ Player, Board, Color };

#[allow(dead_code)]
pub struct TerminalPlayer {
    name: String,
}

impl TerminalPlayer {
    #[allow(dead_code)]
    pub fn new(name: String) -> Self {
        TerminalPlayer {
            name,
        }
    }
}

impl Player for TerminalPlayer {
    fn get_column_index(&self, board: &Board, color: Color) -> Result<usize, String> {
        board.print();
        let mut col_index;
        loop {
            let mut line = String::new();
            println!("{:?} ({:?}):", color, self.name);
            let _ = std::io::stdin().read_line(&mut line);
            if let Ok(int) = line.trim().parse() {
                col_index = int;
            } else {
                continue;
            }
            if board.available_column(col_index) {
                return Ok(col_index);
            } else {
                continue;
            }
        }
    }
}
