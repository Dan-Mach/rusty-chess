// main.rs

mod board;
mod pieces;
mod file;
mod error;
mod color;
mod rank;
mod bitboard; // Your bitboard module

// Import necessary functions from the bitboard module
use crate::bitboard::{print_bitboard, pop_bit, count_bits};
// Note: bit_table() is not directly used in this main if pop_bit is updated,
// but it's good to keep the module structure clean.

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/4p3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1 ";
    let board = board::Board::parse_fen(fen);
    board.print(fen);
    
}