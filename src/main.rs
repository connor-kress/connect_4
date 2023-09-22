mod core;
#[allow(unused_imports)]
use core::{ Color, Board, Game, Player, TerminalPlayer, AIPlayer };

fn run_game() -> Result<(), String> {
    let players: Vec<Box<dyn Player>> = vec![
        Box::new(TerminalPlayer::new("Player 1".into())),
        Box::new(AIPlayer::new("Bot 1".into())),
        Box::new(AIPlayer::new("Bot 2".into())),
        Box::new(AIPlayer::new("Bot 3".into())),
    ];
    let player_colors = vec![Color::Red, Color::Black, Color::Red, Color::Black];

    let mut game = Game::new(None, players, player_colors)?;
    game.start()?;
    Ok(())
}

fn main() {
    match run_game() {
        Ok(_) => {},
        Err(msg) => eprintln!("[ERROR] {}", msg),
    }
}
