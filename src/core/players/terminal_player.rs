use crate::core::{ Player, Board, Color, clear_screen };

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
        let mut col_index;
        let mut error_msg: Option<String> = None;
        let mut index_str = " ".repeat(1 + ((board.column_width as f64 - 1.0) / 2.0).floor() as usize);
        let num_strs = (1..(board.num_columns + 1)).map(|num| num.to_string());
        for num_str in num_strs {
            index_str = index_str + &num_str + &" ".repeat(board.column_width + 1 - num_str.len());
        }
        loop {
            clear_screen();
            board.print()?;
            println!("{}", index_str);
            if let Some(msg) = error_msg {
                println!("{}", msg);
            }
            let mut line = String::new();
            println!("{} ({}):", color, self.get_name());
            let _ = std::io::stdin().read_line(&mut line);
            if let Ok(int) = line.trim().parse::<usize>() {
                col_index = int - 1;
            } else {
                error_msg = Some("Please input a valid integer.".into());
                continue;
            }
            if board.available_column(col_index) {
                return Ok(col_index);
            } else {
                error_msg = Some(
                    format!("Please input a valid column index (1-{}).", board.num_columns)
                );
                continue;
            }
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}
