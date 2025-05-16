mod board;
mod pieces;
mod file;
mod error;
mod color;
mod rank;
mod bitboard;

//use crate::pieces::NUM_PIECES;
use crate::board::Board;
fn main () {
    let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";
    let board = Board::parse_fen(fen);
    board.print(fen);
    
}