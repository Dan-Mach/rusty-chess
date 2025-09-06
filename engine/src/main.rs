use engine::*;
pub fn get_piece_at_square(board: &Board, square: Square) -> Option<ColoredPiece> {
    let (arr_r, arr_f) = square_to_array_indices(square);
    board.squares[arr_r][arr_f]
}

pub fn undo_move(board: &mut Board, game_move: &Move) {
    // This function should implement the logic to undo a move.
    // For now, we will just print the move being undone.
    println!("Undoing move from {} to {}", game_move.from, game_move.to);
    // Actual undo logic would go here.
}

fn main () {
    undo_move(
        &mut Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap(),
        &Move::new(0, 1, None),
    );
    
}