use std::fmt;
use crate::color::Color;
use crate::pieces::{Piece as PieceKindEnum, ColoredPiece};
use crate::genmove::Square;
use crate::coordinates::{square_to_array_indices, square_to_rank_file_enums, array_indices_to_square};
use crate::error::{Error, FenParseError};
#[derive(Clone, Debug)] // Board struct definition remains the same
pub struct Board {
    pub squares: [[Option<ColoredPiece>; 8]; 8],
    pub active_color: Color,
    pub castling_kingside_white: bool,
    pub castling_queenside_white: bool,
    pub castling_kingside_black: bool,
    pub castling_queenside_black: bool,
    pub en_passant_target: Option<(usize, usize)>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

impl Board {
    pub fn new_empty() -> Self { // Stays the same
        Board {
            squares: [[None; 8]; 8],
            active_color: Color::White,
            castling_kingside_white: false,
            castling_queenside_white: false,
            castling_kingside_black: false,
            castling_queenside_black: false,
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    pub fn fen_char_to_colored_piece(c: char) -> Option<ColoredPiece> { // Stays the same
        let color = if c.is_uppercase() { Color::White } else { Color::Black };
        let kind = match c.to_ascii_lowercase() {
            'p' => PieceKindEnum::Pawn,
            'n' => PieceKindEnum::Knight,
            'b' => PieceKindEnum::Bishop,
            'r' => PieceKindEnum::Rook,
            'q' => PieceKindEnum::Queen,
            'k' => PieceKindEnum::King,
            _ => return None,
        };
        Some(ColoredPiece::new(kind, color))
    }

    // parse_fen now returns Result<Board, crate::error::Error>
    pub fn parse_fen(fen_string: &str) -> Result<Board, Error> {
        let parts: Vec<&str> = fen_string.split_whitespace().collect();
        if parts.len() != 6 {
            // Wrap FenParseError in Error::FenParsing
            return Err(Error::FenParsing(FenParseError::NotEnoughParts));
        }

        let mut board = Board::new_empty();
        let piece_placement = parts[0];
        let mut rank_idx = 0;

        for rank_str in piece_placement.split('/') {
            if rank_idx >= 8 {
                return Err(Error::FenParsing(FenParseError::InvalidFormat("Too many ranks (more than 7 '/' separators)".to_string())));
            }
            let mut file_idx = 0;
            for char_code in rank_str.chars() {
                if file_idx >= 8 && !char_code.is_ascii_digit() {
                     return Err(Error::FenParsing(FenParseError::InvalidRankLength(format!("Rank {} has too many items before processing char '{}'", rank_idx + 1, char_code))));
                }

                if let Some(digit) = char_code.to_digit(10) {
                    if !(1..=8).contains(&digit) {
                        return Err(Error::FenParsing(FenParseError::InvalidFormat(format!("Invalid digit '{}' in piece placement.", char_code))));
                    }
                    if file_idx + (digit as usize) > 8 {
                        return Err(Error::FenParsing(FenParseError::InvalidRankLength(format!("Rank {} digit '{}' causes file overflow.", rank_idx + 1, digit))));
                    }
                    file_idx += digit as usize;
                } else {
                    if file_idx >= 8 {
                        return Err(Error::FenParsing(FenParseError::InvalidRankLength(format!("Rank {} trying to place piece at file {}, which is out of bounds.", rank_idx + 1, file_idx + 1))));
                    }
                    match Board::fen_char_to_colored_piece(char_code) {
                        Some(colored_piece) => {
                            board.squares[rank_idx][file_idx] = Some(colored_piece);
                            file_idx += 1;
                        }
                        None => return Err(Error::FenParsing(FenParseError::InvalidPiece(char_code))),
                    }
                }
            }
            if file_idx != 8 {
                return Err(Error::FenParsing(FenParseError::InvalidRankLength(format!("Rank {} (from top) did not sum to 8 files. Parsed: '{}'", rank_idx +1 , rank_str))));
            }
            rank_idx += 1;
        }
        if rank_idx != 8 {
            return Err(Error::FenParsing(FenParseError::InvalidFormat(format!("Expected 8 ranks, found {}", rank_idx))));
        }

        board.active_color = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(Error::FenParsing(FenParseError::InvalidActiveColor(parts[1].to_string()))),
        };

        let castling_str = parts[2];
        if castling_str != "-" {
            for c in castling_str.chars() {
                match c {
                    'K' => board.castling_kingside_white = true,
                    'Q' => board.castling_queenside_white = true,
                    'k' => board.castling_kingside_black = true,
                    'q' => board.castling_queenside_black = true,
                    _ => return Err(Error::FenParsing(FenParseError::InvalidCastlingRights(castling_str.to_string()))),
                }
            }
        }
        
        let en_passant_str = parts[3];
        if en_passant_str != "-" {
            if en_passant_str.len() != 2 {
                 return Err(Error::FenParsing(FenParseError::InvalidEnPassantTarget(en_passant_str.to_string())));
            }
            let mut chars = en_passant_str.chars();
            let file_char = chars.next().unwrap();
            let rank_char = chars.next().unwrap();

            let file = (file_char as u8).wrapping_sub(b'a') as usize;
            let rank_digit = rank_char.to_digit(10)
                .ok_or_else(|| Error::FenParsing(FenParseError::InvalidEnPassantTarget(en_passant_str.to_string())))?;

            if file >= 8 || !((rank_digit == 3 && board.active_color == Color::Black) || (rank_digit == 6 && board.active_color == Color::White)) {
                 return Err(Error::FenParsing(FenParseError::InvalidEnPassantTarget(format!("Invalid square or logic: {}",en_passant_str))));
            }
            let rank = 8 - rank_digit as usize;
            board.en_passant_target = Some((rank, file));
        }

        board.halfmove_clock = parts[4].parse()
            .map_err(|_| Error::FenParsing(FenParseError::InvalidHalfmoveClock(parts[4].to_string())))?;

        board.fullmove_number = parts[5].parse()
            .map_err(|_| Error::FenParsing(FenParseError::InvalidFullmoveNumber(parts[5].to_string())))?;
        if board.fullmove_number == 0 {
            return Err(Error::FenParsing(FenParseError::InvalidFullmoveNumber("Fullmove number cannot be 0".to_string())));
        }

        Ok(board)
    }
}

// impl fmt::Display for Board remains the same
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  +---+---+---+---+---+---+---+---+")?;
        for rank_idx in 0..8 {
            write!(f, "{} |", 8 - rank_idx)?;
            for file_idx in 0..8 {
                match self.squares[rank_idx][file_idx] {
                    Some(colored_piece) => write!(f, " {} |", colored_piece.to_char())?,
                    None => write!(f, "   |")?,
                }
            }
            writeln!(f)?;
            writeln!(f, "  +---+---+---+---+---+---+---+---+")?;
        }
        writeln!(f, "    a   b   c   d   e   f   g   h")?;
        writeln!(f)?;

        writeln!(f, "Active Color: {:?}", self.active_color)?;
        write!(f, "Castling: ")?;
        let mut castling_available = false;
        if self.castling_kingside_white { write!(f, "K")?; castling_available = true; }
        if self.castling_queenside_white { write!(f, "Q")?; castling_available = true; }
        if self.castling_kingside_black { write!(f, "k")?; castling_available = true; }
        if self.castling_queenside_black { write!(f, "q")?; castling_available = true; }
        if !castling_available {
            write!(f, "-")?;
        }
        writeln!(f)?;

        write!(f, "En Passant: ")?;
        if let Some((ep_rank_idx, ep_file_idx)) = self.en_passant_target {
            let file_char = (b'a' + ep_file_idx as u8) as char;
            let rank_char = (8 - ep_rank_idx).to_string();
            writeln!(f, "{}{}", file_char, rank_char)?;
        } else {
            writeln!(f, "-")?;
        }

        writeln!(f, "Halfmove Clock: {}", self.halfmove_clock)?;
        writeln!(f, "Fullmove Number: {}", self.fullmove_number)?;

        Ok(())
    }
}

// // At the end of src/board.rs

// #[cfg(test)]
// mod tests {
//     use super::*; // Imports everything from the parent module (board.rs)
//                   // like Board, Color, etc.
//     use crate::genmove::{Move, Square}; // Assuming MoveList is Vec<Move>
//     // We don't strictly need PieceKindEnum or Color for this specific knight test's assertions 
//     // if we just check from/to squares, but good to have for other tests.
//     use std::collections::HashSet;

//     // Helper function to convert algebraic square notation (e.g., "e4") to your Square (u8) type.
//     // This assumes A1=0, B1=1, ..., H1=7, A2=8, ..., H8=63 (LERF mapping).
//     fn sq(s: &str) -> Square {
//         assert_eq!(s.len(), 2, "Square string must be 2 characters, e.g., 'e4'. Input was: '{}'", s);
//         let file_char = s.chars().nth(0).unwrap_or_else(|| panic!("Empty string for square file: {}", s));
//         let rank_char = s.chars().nth(1).unwrap_or_else(|| panic!("Empty string for square rank: {}", s));

//         assert!(('a'..='h').contains(&file_char), "Invalid file character: {} in {}", file_char, s);
//         assert!(('1'..='8').contains(&rank_char), "Invalid rank character: {} in {}", rank_char, s);

//         let file_val = (file_char as u8) - b'a'; // a=0, b=1, ... h=7
//         let rank_val = (rank_char as u8) - b'1'; // 1=0, 2=1, ... 8=7
        
//         rank_val * 8 + file_val
//     }

//     #[test]
//     fn test_knight_moves_from_d4_on_empty_board_center() {
//         // 1. Setup: FEN for a board with only a white knight on d4
//         let fen = "8/8/8/8/3N4/8/8/8 w - - 0 1";
//         let board = Board::parse_fen(fen).expect("Failed to parse FEN for knight test");
        
//         // The knight is on d4. Calculate its Square index.
//         // Rank '4' is rank_val 3 (0-indexed). File 'd' is file_val 3 (0-indexed).
//         // Square index = rank_val * 8 + file_val = 3 * 8 + 3 = 24 + 3 = 27.
//         let knight_from_sq: Square = sq("d4"); // Should be 27

//         // 2. Call move generation
//         let all_generated_moves = board.generate_legal_moves();

//         // 3. Filter for moves from our knight
//         let knight_actual_moves: HashSet<Move> = all_generated_moves
//             .into_iter()
//             .filter(|m| m.from == knight_from_sq)
//             .collect();

//         // 4. Define expected moves
//         // A knight on d4 (sq 27) on an empty board can move to 8 squares:
//         // D4 (rank 3, file 3) ->
//         // C2 (rank 1, file 2) -> sq 1*8+2 = 10
//         // E2 (rank 1, file 4) -> sq 1*8+4 = 12
//         // B3 (rank 2, file 1) -> sq 2*8+1 = 17
//         // F3 (rank 2, file 5) -> sq 2*8+5 = 21
//         // B5 (rank 4, file 1) -> sq 4*8+1 = 33
//         // F5 (rank 4, file 5) -> sq 4*8+5 = 37
//         // C6 (rank 5, file 2) -> sq 5*8+2 = 42
//         // E6 (rank 5, file 4) -> sq 5*8+4 = 44
//         let expected_moves: HashSet<Move> = [
//             Move::new_quiet(knight_from_sq, sq("c2")), // to 10
//             Move::new_quiet(knight_from_sq, sq("e2")), // to 12
//             Move::new_quiet(knight_from_sq, sq("b3")), // to 17
//             Move::new_quiet(knight_from_sq, sq("f3")), // to 21
//             Move::new_quiet(knight_from_sq, sq("b5")), // to 33
//             Move::new_quiet(knight_from_sq, sq("f5")), // to 37
//             Move::new_quiet(knight_from_sq, sq("c6")), // to 42
//             Move::new_quiet(knight_from_sq, sq("e6")), // to 44
//         ]
//         .iter()
//         .cloned() // Since Move is Copy, cloned() works well here.
//         .collect();
        
//         // 5. Assert
//         assert_eq!(
//             knight_actual_moves.len(),
//             expected_moves.len(),
//             "Expected {} moves, but got {}. Generated: {:?}", 
//             expected_moves.len(), knight_actual_moves.len(), knight_actual_moves
//         );
//         assert_eq!(
//             knight_actual_moves, 
//             expected_moves,
//             "Generated knight moves from d4 do not match expected moves."
//         );
//     }

//     // You should add more tests for knights:
//     // - test_knight_moves_from_corner_a1()
//     // - test_knight_moves_from_edge_h4()
//     // - test_knight_moves_blocked_by_own_pieces()
//     // - test_knight_moves_capturing_opponent_pieces()
// }