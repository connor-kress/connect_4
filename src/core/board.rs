use crate::core::Color;

type Line = Vec<Option<Color>>;

#[derive(Debug, PartialEq)]
pub struct Board {
    pub data: Vec<Line>,
    pub num_rows: usize,
    pub num_columns: usize,
    pub column_width: usize,
    pub row_height: usize,
}

impl Board {
    #[allow(dead_code)]
    pub fn new(
        num_rows: usize,
        num_columns: usize,
        row_height: usize,
        column_width: usize,
    ) -> Self {
        let mut board = Board {
            data: Vec::new(),
            num_rows,
            num_columns,
            row_height,
            column_width,
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

    pub fn stringify(&self) -> Result<String, String> {
        let get_str_of = |item: Option<Color>| -> Result<String, String> {
            if let Some(color) = item {
                let color_str = color.to_string();
                if color_str.len() > self.column_width {
                    return Err("Inadequate column width.".into());
                }
                let remaining = self.column_width - color_str.len();
                let left = " ".repeat((remaining as f64 / 2.0).floor() as usize);
                let right = " ".repeat((remaining as f64 / 2.0).ceil() as usize);
                Ok(left + &color_str + &right)
            } else {
                Ok(" ".repeat(self.column_width))
            }
        };

        let mut bstr = "-".repeat(self.num_columns * (self.column_width + 1) + 1).to_string() + "\n";
        for row in self.data.iter() {
            let mut item_strs = Vec::new();
            for item in row {
                item_strs.push(get_str_of(*item)?);
            }
            bstr = bstr + &(
                ("|".to_string() + &(&(" ".repeat(self.column_width) + "|"))
                    .repeat(self.num_columns) + "\n")
                .repeat(((self.row_height - 1) as f64 / 2.0).floor() as usize)
            );
            bstr = bstr + &item_strs.iter().fold("|".to_string(), |acc, elem| acc + &elem + "|") + "\n";
            bstr = bstr + &(
                ("|".to_string() + &(&(" ".repeat(self.column_width) + "|"))
                    .repeat(self.num_columns) + "\n")
                .repeat(((self.row_height - 1) as f64 / 2.0).ceil() as usize)
            );
            bstr = bstr + &"-".repeat(self.num_columns * (self.column_width + 1) + 1).to_string() + "\n";
        }
        Ok(bstr.trim().into())
    }

    #[allow(dead_code)]
    pub fn print(&self) -> Result<(), String> {
        println!("{}", self.stringify()?);
        Ok(())
    }
}
