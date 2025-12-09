// Tests for checkmate and stalemate detection

use crate::{Board, GameResult, Color};

// Helper function to convert algebraic square notation to Square (u8)
fn sq(s: &str) -> u8 {
    assert_eq!(s.len(), 2);
    let file_char = s.chars().nth(0).unwrap();
    let rank_char = s.chars().nth(1).unwrap();
    let file_val = (file_char as u8) - b'a';
    let rank_val = (rank_char as u8) - b'1';
    rank_val * 8 + file_val
}

#[test]
fn test_fools_mate() {
    // Fool's mate - fastest checkmate in chess
    // 1. f3 e5 2. g4 Qh4#
    let mut board = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .expect("Failed to parse starting position");
    
    // White plays f3
    let legal_moves = board.generate_legal_moves();
    let f3_move = legal_moves.iter().find(|m| m.from == sq("f2") && m.to == sq("f3"))
        .expect("f3 move should be legal");
    board.make_move(f3_move);
    board.update_game_result();
    assert_eq!(board.game_result, GameResult::InProgress);
    
    // Black plays e5
    let legal_moves = board.generate_legal_moves();
    let e5_move = legal_moves.iter().find(|m| m.from == sq("e7") && m.to == sq("e5"))
        .expect("e5 move should be legal");
    board.make_move(e5_move);
    board.update_game_result();
    assert_eq!(board.game_result, GameResult::InProgress);
    
    // White plays g4
    let legal_moves = board.generate_legal_moves();
    let g4_move = legal_moves.iter().find(|m| m.from == sq("g2") && m.to == sq("g4"))
        .expect("g4 move should be legal");
    board.make_move(g4_move);
    board.update_game_result();
    assert_eq!(board.game_result, GameResult::InProgress);
    
    // Black plays Qh4# (checkmate)
    let legal_moves = board.generate_legal_moves();
    let qh4_move = legal_moves.iter().find(|m| m.from == sq("d8") && m.to == sq("h4"))
        .expect("Qh4 move should be legal");
    board.make_move(qh4_move);
    board.update_game_result();
    
    // Verify checkmate
    assert_eq!(board.game_result, GameResult::Checkmate(Color::Black));
    assert!(board.is_checkmate());
    assert!(board.is_game_over());
}

#[test]
fn test_back_rank_mate() {
    // Back rank mate position - White delivers checkmate with rook
    let mut board = Board::parse_fen("6k1/5ppp/8/8/8/8/5PPP/R5K1 w - - 0 1")
        .expect("Failed to parse position");
    
    // White plays Ra8# (checkmate)
    let legal_moves = board.generate_legal_moves();
    let ra8_move = legal_moves.iter().find(|m| m.from == sq("a1") && m.to == sq("a8"))
        .expect("Ra8 move should be legal");
    board.make_move(ra8_move);
    board.update_game_result();
    
    // Verify checkmate
    assert_eq!(board.game_result, GameResult::Checkmate(Color::White));
    assert!(board.is_checkmate());
    assert!(board.is_game_over());
}

#[test]
fn test_stalemate() {
    // Stalemate position - Black king has no legal moves but is not in check
    // King on a8, White king on c7 (controls b8, b7), White queen on b6 (controls a8-a1 and a7)
    let board = Board::parse_fen("k7/2K5/1Q6/8/8/8/8/8 b - - 0 1")
        .expect("Failed to parse stalemate position");
    
    // Verify stalemate
    assert!(board.is_stalemate());
    assert!(!board.is_checkmate());
    assert!(!board.is_in_check());
}

#[test]
fn test_position_with_legal_moves_not_checkmate() {
    // Normal starting position - not checkmate
    let board = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .expect("Failed to parse starting position");
    
    assert!(!board.is_checkmate());
    assert!(!board.is_stalemate());
    assert!(!board.is_in_check());
    assert_eq!(board.game_result, GameResult::InProgress);
}

#[test]
fn test_check_but_not_checkmate() {
    // Position where White king is in check but can escape
    let board = Board::parse_fen("4k3/8/8/8/8/8/4r3/4K3 w - - 0 1")
        .expect("Failed to parse check position");
    
    assert!(board.is_in_check());
    assert!(!board.is_checkmate());
    assert!(!board.is_stalemate());
    assert_eq!(board.game_result, GameResult::InProgress);
}

#[test]
fn test_scholars_mate() {
    // Scholar's mate - 1. e4 e5 2. Bc4 Nc6 3. Qh5 Nf6 4. Qxf7#
    let mut board = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .expect("Failed to parse starting position");
    
    // 1. e4
    let legal_moves = board.generate_legal_moves();
    let e4_move = legal_moves.iter().find(|m| m.from == sq("e2") && m.to == sq("e4"))
        .expect("e4 move should be legal");
    board.make_move(e4_move);
    board.update_game_result();
    
    // 1... e5
    let legal_moves = board.generate_legal_moves();
    let e5_move = legal_moves.iter().find(|m| m.from == sq("e7") && m.to == sq("e5"))
        .expect("e5 move should be legal");
    board.make_move(e5_move);
    board.update_game_result();
    
    // 2. Bc4
    let legal_moves = board.generate_legal_moves();
    let bc4_move = legal_moves.iter().find(|m| m.from == sq("f1") && m.to == sq("c4"))
        .expect("Bc4 move should be legal");
    board.make_move(bc4_move);
    board.update_game_result();
    
    // 2... Nc6
    let legal_moves = board.generate_legal_moves();
    let nc6_move = legal_moves.iter().find(|m| m.from == sq("b8") && m.to == sq("c6"))
        .expect("Nc6 move should be legal");
    board.make_move(nc6_move);
    board.update_game_result();
    
    // 3. Qh5
    let legal_moves = board.generate_legal_moves();
    let qh5_move = legal_moves.iter().find(|m| m.from == sq("d1") && m.to == sq("h5"))
        .expect("Qh5 move should be legal");
    board.make_move(qh5_move);
    board.update_game_result();
    
    // 3... Nf6
    let legal_moves = board.generate_legal_moves();
    let nf6_move = legal_moves.iter().find(|m| m.from == sq("g8") && m.to == sq("f6"))
        .expect("Nf6 move should be legal");
    board.make_move(nf6_move);
    board.update_game_result();
    
    // 4. Qxf7# (checkmate)
    let legal_moves = board.generate_legal_moves();
    let qxf7_move = legal_moves.iter().find(|m| m.from == sq("h5") && m.to == sq("f7"))
        .expect("Qxf7 move should be legal");
    board.make_move(qxf7_move);
    board.update_game_result();
    
    // Verify checkmate
    assert_eq!(board.game_result, GameResult::Checkmate(Color::White));
    assert!(board.is_checkmate());
    assert!(board.is_game_over());
}
