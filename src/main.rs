mod core;
#[allow(unused_imports)]
use core::{ Color, Board, Game };

fn main() {
    let mut game = Game::new(None);
    game.start().unwrap();
}
