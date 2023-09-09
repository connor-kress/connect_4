mod core;
#[allow(unused_imports)]
use core::{ Color, Board, Game, Player, TerminalPlayer };

fn run_game() -> Result<(), String> {
    let players = vec![
        TerminalPlayer::new("Player 1".into()),
        TerminalPlayer::new("Player 2".into()),
    ];
    let player_refs = players.into_iter().map(|x| Box::new(x)).collect::<Vec<Box<TerminalPlayer>>>();
    let player_colors = vec![Color::Red, Color::Black];
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
