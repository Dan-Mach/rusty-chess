mod board;
mod pieces;
mod bitboard;

//use board::Board;
use bitboard::{count_bits, print_bitboard};
fn main () {
    // let board = Board::new();
    // board.print();
    print_bitboard(100);
    count_bits(1);


}