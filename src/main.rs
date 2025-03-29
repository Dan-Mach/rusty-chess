mod board;
mod pieces;
mod file;
mod error;
mod color;
mod rank;
mod bitboard;

use crate::pieces::NUM_PIECES;
use crate::board::Board;
fn main () {
    let fen = "rnbqkbnr/pppppppp/8/8/4p3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1 ";
    let board = Board::new(fen);
    board.print(fen);

    
}