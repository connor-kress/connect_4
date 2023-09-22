use crate::core::{ Board, Color, Player, clear_screen };

/// Represents a singular game with players and a board which can be played.
pub struct Game {
    /// The `Board` instance that the game is being played on.
    board: Board,
    /// The players participating in the game.
    players: Vec<Box<dyn Player>>,
    /// The color team that the players in the corresponding index are on.
    player_colors: Vec<Color>,
    /// The index that describes who the game's current active player is.
    current_player_index: usize,
    /// The number of pieces a team must get in a row to win the game (4 by default).
    amount_to_win: usize,
    /// Whether or not the game has started.
    started: bool,
    /// Whether or not the game has ended.
    ended: bool,
    /// A list containing the indices of all players on the winning team of an ended game.
    winner_indices: Option<Vec<usize>>,
}

impl Game {
    /// Creates a game instance which can be started to be played.
    /// 
    /// # Arguments
    /// 
    /// * `board` - pass `Some(board)` to play with a custom sized or preinstantiated board
    ///             or pass `None` for an automatic default sized board.
    /// 
    /// * `players` - a `Vec` of `Box<dyn Player>`s which will play in their present order
    ///                in the list.
    /// 
    /// * `player_colors` - a `Vec` of `Color`s of the same length as `players` which denotes
    ///                     the team of each player at the corresponding index in `players`.
    /// 
    /// Returns a `Result` type with a `Ok` containing the `Game` instance to indicate a success or
    /// an `Err` with a `String` containing an error message if invalid inputs are passed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let players = vec![
    ///     Box::new(TerminalPlayer::new("Player 1".into())),
    ///     Box::new(TerminalPlayer::new("Player 2".into())),
    /// ];
    /// let player_colors = vec![Color::Red, Color::Black];
    /// 
    /// let mut game = Game::new(None, players, player_colors);
    /// ...
    /// ```
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
            winner_indices: None,
        })
    }

    /// Returns a reference to the current active player.
    fn get_current_player(&self) -> &Box<dyn Player> {
        &self.players[self.current_player_index]
    }

    /// Returns a reference to the player at `index` in the game's queue.
    fn get_player(&self, index: usize) -> &Box<dyn Player> {
        &self.players[index]
    }

    /// Returns the `Color` corresponding with the game's current active player.
    fn get_current_color(&self) -> Color {
        self.player_colors[self.current_player_index]
    }

    /// Returns the indices of all players in the game with the given `Color`.
    fn get_player_indices_with_color(&self, color: Color) -> Vec<usize> {
        self.player_colors
            .iter()
            .enumerate()
            .filter(|(_, player_color)| **player_color == color)
            .map(|(i, _)| i)
            .collect()
    }

    /// Increments the `current_player_index` which switches the game's active player.
    fn switch_turn(&mut self) {
        if self.current_player_index == self.players.len() - 1 {
            self.current_player_index = 0;
        } else {
            self.current_player_index += 1;
        }
    }

    /// Prompts the active player for a column index and drops a piece corresponding with the
    /// active player's `Color` in the `Board` at that column index.
    /// 
    /// Returns a `Result` type with a unit `Ok` to indicate a success or an `Err` with a `String`
    /// containing an error message.
    fn take_turn(&mut self) -> Result<(), String> {
        let color = self.get_current_color();
        let col_index = self.get_current_player().get_column_index(&self.board, color)?;
        self.board.drop_piece(color, col_index)
    }

    /// Ends the game in a winning condition.
    /// 
    /// Is passed the winning `Color` to determine behavior.
    /// 
    /// Returns a `Result` type with a unit `Ok` to indicate a success or an `Err` with a `String`
    /// containing an error message.
    fn handle_win(&mut self, color: Color) -> Result<(), String> {
        self.ended = true;
        clear_screen();
        self.board.print()?;
        let winner_indices = self.get_player_indices_with_color(color);
        let num_winners = winner_indices.len();
        let winners = winner_indices.iter().map(|i| self.get_player(*i)).collect::<Vec<&Box<dyn Player>>>();
        let winners_str = {
            if num_winners == 1 {
                winners[0].get_name()
            } else if num_winners == 2 {
                winners[0].get_name() + " and " + &winners[1].get_name()
            } else {
                winners.iter()
                    .map(|player| player.get_name())
                    .enumerate()
                    .fold(String::new(), |acc, (i, name)| {
                            acc + if i == 0 {
                                ""
                            } else if i == num_winners - 1 {
                                ", and "
                            } else {
                                ", "
                            }
                            + &name
                    })
            }
        };
        if num_winners == 1 {
            println!("{} ({}) wins!", color, winners_str);
        } else {
            println!("{} team ({}) wins!", color, winners_str);
        }
        self.winner_indices = Some(winner_indices);
        Ok(())
    }

    /// Ends the game in a tied condition (the board being full).
    /// 
    /// Returns a `Result` type with a unit `Ok` to indicate a success or an `Err` with a `String`
    /// containing an error message.
    fn handle_tie(&mut self) -> Result<(), String> {
        self.ended = true;
        clear_screen();
        self.board.print()?;
        println!("Tie.");
        Ok(())
    }

    /// Starts a primary game loop from the current state of the `Game` instance.
    /// 
    /// This must be called on a game which is neither **unstarted** nor already **ended**.
    /// 
    /// Returns a `Result` type with a unit `Ok` to indicate a success or an `Err` with a `String`
    /// containing an error message.
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

    /// Begins the game and starts a primary game loop.
    /// 
    /// This must be called on a game which is neither already **started** nor already **ended**.
    /// 
    /// Returns a `Result` type with a unit `Ok` to indicate a success or an `Err` with a `String`
    /// containing an error message.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut game = Game::new(None, players, player_colors);
    /// 
    /// match game.start() {
    ///     Ok(_) => println!("Game ran successfully"),
    ///     Err(msg) => eprintln!("[ERROR] {}", msg),
    /// }
    /// ```
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
