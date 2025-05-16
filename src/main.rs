mod data;
mod structures;
mod board;

use structures::Board;
use board::print_board;

fn main () {
    let board = Board::new();
    print_board(&board);
}