mod core;
#[allow(unused_imports)]
use core::{ Color, Board, Game, Player, TerminalPlayer, AIPlayer };

fn run_game() -> Result<(), String> {
    let player_refs: Vec<Box<dyn Player>> = vec![
        Box::new(TerminalPlayer::new("Player 1".into())),
        Box::new(TerminalPlayer::new("Player 2".into())),
        Box::new(AIPlayer::new("Player 3".into())),
    ];
    let player_colors = vec![Color::Red, Color::Black, Color::Red];
    
    let mut game = Game::new(None, player_refs, player_colors)?;
    game.start()?;
    Ok(())
}

fn main() {
    match run_game() {
        Ok(_) => {},
        Err(e) => println!("[ERROR] {}", e),
    }
}
