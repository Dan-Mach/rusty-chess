// tests/move_generation_tests.rs

// Import necessary items from the parent crate
use crate::{Board, Move, Square, Color};
// If Move struct used PieceKind for promotion, you'd import:
// use engine::pieces::Piece as PieceKind; 
use std::collections::HashSet;

// Helper function to convert algebraic square notation (e.g., "e4") to your Square (u8) type.
// This assumes A1=0, B1=1, ..., H1=7, A2=8, ..., H8=63 (LERF mapping).
// This helper is local to this test crate.
fn sq(s: &str) -> Square {
    assert_eq!(s.len(), 2, "Square string must be 2 characters, e.g., 'e4'. Input was: '{}'", s);
    let file_char = s.chars().nth(0).unwrap_or_else(|| panic!("Empty string for square file: {}", s));
    let rank_char = s.chars().nth(1).unwrap_or_else(|| panic!("Empty string for square rank: {}", s));

    assert!(('a'..='h').contains(&file_char), "Invalid file character: {} in {}", file_char, s);
    assert!(('1'..='8').contains(&rank_char), "Invalid rank character: {} in {}", rank_char, s);

    let file_val = (file_char as u8) - b'a'; // a=0, b=1, ... h=7
    let rank_val = (rank_char as u8) - b'1'; // 1=0, 2=1, ... 8=7
    
    rank_val * 8 + file_val
}

#[test]
fn test_knight_moves_from_d4_on_empty_board_center() {
    // 1. Setup: FEN for a board with only a white knight on d4
    let fen = "8/8/8/8/3N4/8/8/8 w - - 0 1";
    // Board::parse_fen is part of your library's public API
    let board = Board::parse_fen(fen).expect("Failed to parse FEN for knight test");
    
    let knight_from_sq: Square = sq("d4");

    // 2. Call move generation (generate_pseudo_legal_moves must be a public method of Board)
    let all_generated_moves = board.generate_pseudo_legal_moves();

    // 3. Filter for moves from our knight
    let knight_actual_moves: HashSet<Move> = all_generated_moves
        .into_iter()
        .filter(|m| m.from == knight_from_sq)
        .collect();

    // 4. Define expected moves
    // Your Move struct has `from`, `to`, and `promotion: Option<PieceKind>`.
    // Move::new_quiet sets promotion to None.
    let expected_moves: HashSet<Move> = [
        Move::new_quiet(knight_from_sq, sq("c2")),
        Move::new_quiet(knight_from_sq, sq("e2")),
        Move::new_quiet(knight_from_sq, sq("b3")),
        Move::new_quiet(knight_from_sq, sq("f3")),
        Move::new_quiet(knight_from_sq, sq("b5")),
        Move::new_quiet(knight_from_sq, sq("f5")),
        Move::new_quiet(knight_from_sq, sq("c6")),
        Move::new_quiet(knight_from_sq, sq("e6")),
    ]
    .iter()
    .cloned()
    .collect();
    
    // 5. Assert
    assert_eq!(
        knight_actual_moves.len(),
        expected_moves.len(),
        "Expected {} moves, but got {}. Generated: {:?}", 
        expected_moves.len(), knight_actual_moves.len(), knight_actual_moves
    );
    assert_eq!(
        knight_actual_moves, 
        expected_moves,
        "Generated knight moves from d4 do not match expected moves."
    );
}

// Add more tests here:
// #[test]
// fn test_knight_moves_from_a1_corner() { /* ... */ }
//
// #[test]
// fn test_rook_moves_on_empty_rank() { /* ... */ }
// etc.