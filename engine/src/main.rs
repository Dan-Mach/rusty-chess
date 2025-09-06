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

fn main() {
    // Test 1: Undo a simple move
    let mut board = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let mv = Move::new(12, 28, None); // e2 to e4
    undo_move(&mut board, &mv);

    // Test 2: Undo a capture move
    let mut board = Board::parse_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1").unwrap();
    let mv = Move::new(28, 36, Some(Piece::Pawn)); // e4 captures d5
    undo_move(&mut board, &mv);

    // Test 3: Undo a castling move
    let mut board = Board::parse_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
    let mv = Move::new(4, 6, None); // White king-side castling
    undo_move(&mut board, &mv);

    // Test 4: Undo a promotion move
    let mut board = Board::parse_fen("8/P7/8/8/8/8/8/8 w - - 0 1").unwrap();
    let mv = Move::new(8, 0, Some(Piece::Queen)); // a7 to a8, promote to queen
    undo_move(&mut board, &mv);

    // Test 5: Undo an en passant move
    let mut board = Board::parse_fen("8/8/8/3pP3/8/8/8/8 w - d6 0 1").unwrap();
    let mv = Move::new(28, 19, Some(Piece::Pawn)); // e5 captures d6 en passant
    undo_move(&mut board, &mv);
}