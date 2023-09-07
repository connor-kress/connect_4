mod core;
#[allow(unused_imports)]
use core::{ Color, Board, Game };

fn main() {
    let mut board = Board::new(6, 7);
    match board.drop_piece(Color::Red, 0) {
        Ok(_) => { },
        Err(_) => { },
    }
    let mut game = Game::new(Some(board));
    loop {
        game.take_turn();
    }
    // println!("{:?}", game.board.get_winning_color(4));
}
