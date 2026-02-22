use engine::*;
pub fn get_piece_at_square(board: &Board, square: Square) -> Option<ColoredPiece> {
    let (arr_r, arr_f) = square_to_array_indices(square);
    board.squares[arr_r][arr_f]
}

pub fn undo_move(_board: &mut Board, game_move: &Move) {
    println!("Undoing move from {} to {}", game_move.from, game_move.to);

}

fn main() {

    let mut board = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    //testing the en passant move on both type of piece color
//testing the castling rights both on the king and queen side
}