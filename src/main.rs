// main.rs
mod genmove;
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
    let bb_layout = (1 << 0)  | // A1 (square 0)
                        (1 << 7)  | // H1 (square 7)
                        (1 << 27) | // D4 (square 27) (Rank 3, File 3)
                        (1 << 56) | // A8 (square 56) (Rank 7, File 0)
                        (1 << 63);  // H8 (square 63) (Rank 7, File 7)
    print_bitboard(bb_layout, "Layout Test:");
    let fen = "rnbqkbnr/pppppppp/8/8/4p3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1 ";
    let board = board::Board::parse_fen(fen);
    board.print(fen);
    
}