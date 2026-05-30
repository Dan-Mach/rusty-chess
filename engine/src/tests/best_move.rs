use crate::board::Board;

#[test]
fn test_make_undo_roundtrip_startpos() {
    let mut board = Board::parse_fen(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    ).unwrap();

    let original_fen = board.to_fen_string();
    let moves = board.generate_legal_moves();

    for mv in moves {
        board.make_move(&mv);
        board.undo_move().unwrap();

        let fen_after = board.to_fen_string();
        assert_eq!(
            original_fen, fen_after,
            "Board mismatch after roundtrip for move {:?}",
            mv
        );
    }
}


#[test]
fn test_two_ply_roundtrip() {
    let mut board = Board::parse_fen(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    ).unwrap();

    let original_fen = board.to_fen_string();
    let moves1 = board.generate_legal_moves();

    for mv1 in moves1 {
        board.make_move(&mv1);

        let moves2 = board.generate_legal_moves();
        for mv2 in moves2 {
            board.make_move(&mv2);
            board.undo_move().unwrap();
        }

        board.undo_move().unwrap();

        let fen_after = board.to_fen_string();
        assert_eq!(
            original_fen, fen_after,
            "Board mismatch after nested roundtrip for move {:?}",
            mv1
        );
    }
}