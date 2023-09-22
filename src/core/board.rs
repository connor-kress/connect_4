use crate::core::Color;

type Line = Vec<Option<Color>>;

/// Represents the game board in which pieces are dropped.
/// 
/// Adds several functionalities for checking and changing the state of the board.
#[derive(Debug, PartialEq)]
pub struct Board {
    /// The raw double nested `Vec` that holds the state of the board in the form
    /// of `Option<Color>`.
    pub data: Vec<Line>,
    /// The number of rows in the board.
    pub num_rows: usize,
    /// The number of columns in the board.
    pub num_columns: usize,
    /// The width in characters that each column should be printed as.
    pub column_width: usize,
    /// The height in lines that each row should be printed as.
    pub row_height: usize,
}

impl Board {
    /// Constructs an empty `Board` instance.
    /// 
    /// # Arguments
    /// 
    /// * `num_rows` - a `usize` integer denoting the number of rows in the board.
    /// 
    /// * `num_columns` - a `usize` integer denoting the number of columns in the board.
    /// 
    /// * `row_height` - a `usize` integer denoting the width in characters that each column should
    ///                  be printed as.
    /// 
    /// * `column_width` - a `usize` integer denoting the height in lines that each row should be
    ///                    printed as.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut default_board = Board::new(6, 7, 3, 7);
    /// ```
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

    /// Returns `true` if the column with index `col_index` is available else `false`.
    pub fn available_column(&self, col_index: usize) -> bool {
        if col_index >= self.num_columns {
            return false
        }
        match self.get_column(col_index)[0] {
            Some(_) => false,
            None => true,
        }
    }

    /// Returns a `Result` type with an `Ok` containing the row index of the highest available slot
    /// in the column with index `col_index` or an `Err` with a `String` containing an error
    /// message if the column with index `col_index` is unavailable.
    fn get_highest_index(&self, col_index: usize) -> Result<usize, String> {
        if !self.available_column(col_index) {
            return Err(format!("The column with index {} is not available.", col_index));
        }
        let mut highest = 0;
        for row_index in 0..self.num_rows {
            match self.data[row_index][col_index] {
                Some(_) => break,
                None => highest = row_index,
            }
        }
        Ok(highest)
    }

    /// Drops a game piece of `Color` `color` in the column wiht index `col_index`.
    /// 
    /// Returns a `Result` type with a unit `Ok` indicating success or an `Err` with a `String`
    /// containing an error message if the column with index `col_index` is unavailable.
    pub fn drop_piece(&mut self, color: Color, col_index: usize) -> Result<(), String> {
        let row_index = self.get_highest_index(col_index)?;
        self.data[row_index][col_index] = Some(color);
        Ok(())
    }

    /// Returns the column of the board with index `col_index`.
    fn get_column(&self, col_index: usize) -> Line {
        let mut column = Vec::new();
        for row_index in 0..self.num_rows {
            column.push(self.data[row_index][col_index]);
        }
        column
    }
    
    /// Returns the left leaning diagonal of the board with index from the top-right `diag_index`.
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

    /// Returns the right leaning diagonal of the board with index from the top-left `diag_index`.
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

    /// Returns a `bool` indicating whether the entire board is full of game pieces.
    pub fn is_full(&self) -> bool {
        for item in self.data[0].iter() {
            match item {
                Some(_) => {},
                None => return false,
            }
        }
        true
    }

    /// Returns an `Option` type containing `Some<Color>` if a team is in a winning condition or
    /// `None` if no teams are in a winning condition.
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

    /// Returns a `Result` type with an `Ok` containing a `String` representing the board state
    /// in a pretty printed format or an `Err` with a `String` containing an error message.
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

    /// Pretty prints a representation of the game state to the terminal.
    /// 
    /// Returns a `Result` type with a unit `Ok` indicating success or an `Err` with a `String`
    /// containing an error message.
    #[allow(dead_code)]
    pub fn print(&self) -> Result<(), String> {
        println!("{}", self.stringify()?);
        Ok(())
    }
}
