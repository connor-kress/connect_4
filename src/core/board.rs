use crate::core::Color;

type Line = Vec<Option<Color>>;

#[derive(Debug, PartialEq)]
pub struct Board {
    data: Vec<Line>,
    num_rows: usize,
    num_columns: usize,
}

impl Board {
    #[allow(dead_code)]
    pub fn new(num_rows: usize, num_columns: usize) -> Self {
        let mut board = Board {
            data: Vec::new(),
            num_rows,
            num_columns,
        };
        for _ in 0..num_rows {
            let mut row = Vec::new();
            for _ in 0..num_columns {
                row.push(None);
            }
            board.data.push(row);
        }
        board
    }

    pub fn available_column(&self, col_index: usize) -> bool {
        if col_index >= self.num_columns {
            return false
        }
        match self.get_column(col_index)[0] {
            Some(_) => false,
            None => true,
        }
    }

    fn get_highest_index(&self, col_index: usize) -> usize {
        let mut highest = 0;
        for row_index in 0..self.num_rows {
            match self.data[row_index][col_index] {
                Some(_) => break,
                None => highest = row_index,
            }
        }
        highest
    }

    pub fn drop_piece(&mut self, color: Color, col_index: usize) -> Result<(), String> {
        if self.available_column(col_index) {
            let row_index = self.get_highest_index(col_index);
            self.data[row_index][col_index] = Some(color);
            Ok(())
        } else {
            Err(format!("Column {} is not available.", col_index))
        }
    }

    fn get_column(&self, col_index: usize) -> Line {
        let mut column = Vec::new();
        for row_index in 0..self.num_rows {
            column.push(self.data[row_index][col_index]);
        }
        column
    }

    fn get_left_diagonal(&self, diag_index: usize) -> Line {
        let mut diagonal = Vec::new();
        for row_index in 0..self.num_rows {
            let col_index = {
                row_index as isize - diag_index as isize + self.num_columns as isize - 1
            };
            if col_index >= 0 && col_index <= self.num_columns as isize - 1 {
                diagonal.push(self.data[row_index][col_index as usize]);
            }
        }
        diagonal
    }

    fn get_right_diagonal(&self, diag_index: usize) -> Line {
        let mut diagonal = Vec::new();
        for row_index in 0..self.num_rows {
            let col_index = diag_index as isize - row_index as isize;
            if col_index >= 0 && col_index <= self.num_columns as isize - 1 {
                diagonal.push(self.data[row_index][col_index as usize]);
            }
        }
        diagonal
    }

    pub fn is_full(&self) -> bool {
        for item in self.data[0].iter() {
            match item {
                Some(_) => {},
                None => return false,
            }
        }
        true
    }

    pub fn get_winning_color(&self, amount_to_win: usize) -> Option<Color> {
        let check_line = |line: &Line| -> Option<Color> {
            let mut current: Option<Color> = None;
            let mut count = 0;
            for item in line {
                let item_color;
                if let Some(color) = item {
                    item_color = *color;
                } else {
                    current = None;
                    count = 0;
                    continue;
                }
                let current_color;
                if let Some(color) = current {
                    current_color = color;
                } else {
                    current = Some(item_color);
                    count = 1;
                    continue;
                }
                if item_color == current_color {
                    if count == amount_to_win - 1 {
                        return Some(current_color);
                    } else {
                        count += 1;
                    }
                } else {
                    current = Some(item_color);
                    count = 1;
                }
            }
            None
        };

        let mut lines = Vec::new();
        lines.extend(self.data.clone());
        for col_index in 0..self.num_columns {
            lines.push(self.get_column(col_index));
        }
        for diag_index in 0..self.num_rows + self.num_columns - 1 {
            lines.push(self.get_left_diagonal(diag_index));
            lines.push(self.get_right_diagonal(diag_index));
        }
        for line in lines.iter() {
            match check_line(line) {
                Some(color) => return Some(color),
                None => {},
            }
        }
        None
    }

    pub fn stringify(&self) -> String {
        format!("{:?}", self)
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("{}", self.stringify());
    }
}