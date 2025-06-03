use std::fmt; // For Display trait
use crate::color::Color;
use crate::pieces::{Piece as PieceKindEnum, ColoredPiece}; 
#[derive(Clone, Debug)]
pub struct Board {
    pub squares: [[Option<ColoredPiece>; 8]; 8],
    pub active_color: Color,
    pub castling_kingside_white: bool,
    pub castling_queenside_white: bool,
    pub castling_kingside_black: bool,
    pub castling_queenside_black: bool,
    pub en_passant_target: Option<(usize, usize)>, // (rank_idx, file_idx)
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenParseError {
    InvalidFormat(String),
    InvalidPiece(char),
    InvalidRankLength(String),
    TooManyParts,
    NotEnoughParts,
    InvalidActiveColor(String),
    InvalidCastlingRights(String),
    InvalidEnPassantTarget(String),
    InvalidHalfmoveClock(String),
    InvalidFullmoveNumber(String),
}

impl std::fmt::Display for FenParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FenParseError::InvalidFormat(s) => write!(f, "Invalid FEN format: {}", s),
            FenParseError::InvalidPiece(c) => write!(f, "Invalid piece character in FEN: '{}'", c),
            FenParseError::InvalidRankLength(s) => write!(f, "Invalid rank in FEN (files do not sum to 8): '{}'", s),
            FenParseError::TooManyParts => write!(f, "Too many parts in FEN string"),
            FenParseError::NotEnoughParts => write!(f, "Not enough parts in FEN string (expected 6)"),
            FenParseError::InvalidActiveColor(s) => write!(f, "Invalid active color: {}", s),
            FenParseError::InvalidCastlingRights(s) => write!(f, "Invalid castling rights: {}", s),
            FenParseError::InvalidEnPassantTarget(s) => write!(f, "Invalid en passant target: {}", s),
            FenParseError::InvalidHalfmoveClock(s) => write!(f, "Invalid halfmove clock: {}", s),
            FenParseError::InvalidFullmoveNumber(s) => write!(f, "Invalid fullmove number: {}", s),
        }
    }
}

impl std::error::Error for FenParseError {}

impl Board {
    fn new_empty() -> Self {
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
    fn fen_char_to_colored_piece(c:char) -> Option<ColoredPiece> {
        let color = if c.is_uppercase() { Color::White } else { Color::Black };
        let kind = match c.to_ascii_lowercase() {
            'p' => PieceKindEnum::Pawn,
            'n' => PieceKindEnum::Knight,
            'b' => PieceKindEnum::Bishop,
            'r' => PieceKindEnum::Rook,
            'q' => PieceKindEnum::Queen,
            'k' => PieceKindEnum::King,
            _ => return None, // Invalid piece character
        };
        Some(ColoredPiece::new(kind, color))
    }
    pub fn parse_fen(fen_string: &str) -> Result<Board, FenParseError> {
        let parts: Vec<&str> = fen_string.split_whitespace().collect();
        if parts.len() != 6 {
            return Err(FenParseError::NotEnoughParts);
        }

        let mut board = Board::new_empty();
        let piece_placement = parts[0];
        let mut rank_idx = 0;
        for rank_str in piece_placement.split('/') {
            if rank_idx >= 8 {
                return Err(FenParseError::InvalidFormat("Too many ranks (more than 7 '/' separators)".to_string()));
            }
            let mut file_idx = 0;
            for char_code in rank_str.chars() {
                if file_idx >= 8 && !char_code.is_ascii_digit() {
                     return Err(FenParseError::InvalidRankLength(format!("Rank {} has too many items before processing char '{}'", rank_idx + 1, char_code)));
                }

                if let Some(digit) = char_code.to_digit(10) {
                    if !(1..=8).contains(&digit) {
                        return Err(FenParseError::InvalidFormat(format!("Invalid digit '{}' in piece placement.", char_code)));
                    }
                    if file_idx + (digit as usize) > 8 {
                        return Err(FenParseError::InvalidRankLength(format!("Rank {} digit '{}' causes file overflow.", rank_idx + 1, digit)));
                    }
                    file_idx += digit as usize;
                } else {
                    if file_idx >= 8 {
                        return Err(FenParseError::InvalidRankLength(format!("Rank {} trying to place piece at file {}, which is out of bounds.", rank_idx + 1, file_idx + 1)));
                    }
                    match Board::fen_char_to_colored_piece(char_code) {
                        Some(colored_piece) => {
                            board.squares[rank_idx][file_idx] = Some(colored_piece);
                            file_idx += 1;
                        }
                        None => return Err(FenParseError::InvalidPiece(char_code)),
                    }
                }
            }
            if file_idx != 8 {
                return Err(FenParseError::InvalidRankLength(format!("Rank {} (from top) did not sum to 8 files. Parsed: '{}'", rank_idx +1 , rank_str)));
            }
            rank_idx += 1;
        }
        if rank_idx != 8 {
            return Err(FenParseError::InvalidFormat(format!("Expected 8 ranks, found {}", rank_idx)));
        }

        board.active_color = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(FenParseError::InvalidActiveColor(parts[1].to_string())),
        };

        // Part 3: Castling availability
        let castling_str = parts[2];
        if castling_str != "-" {
            for c in castling_str.chars() {
                match c {
                    'K' => board.castling_kingside_white = true,
                    'Q' => board.castling_queenside_white = true,
                    'k' => board.castling_kingside_black = true,
                    'q' => board.castling_queenside_black = true,
                    _ => return Err(FenParseError::InvalidCastlingRights(castling_str.to_string())),
                }
            }
        }

        // Part 4: En passant target square
        let en_passant_str = parts[3];
        if en_passant_str != "-" {
            if en_passant_str.len() != 2 {
                return Err(FenParseError::InvalidEnPassantTarget(en_passant_str.to_string()));
            }
            let mut chars = en_passant_str.chars();
            let file_char = chars.next().unwrap();
            let rank_char = chars.next().unwrap();

            let file = (file_char as u8).wrapping_sub(b'a') as usize;
            let rank_digit = rank_char.to_digit(10).ok_or_else(|| FenParseError::InvalidEnPassantTarget(en_passant_str.to_string()))?;

            if file >= 8 || !((rank_digit == 3 && board.active_color == Color::Black) || (rank_digit == 6 && board.active_color == Color::White)) {
                 return Err(FenParseError::InvalidEnPassantTarget(format!("Invalid en passant square or logic: {}",en_passant_str)));
            }
            let rank = 8 - rank_digit as usize;
            board.en_passant_target = Some((rank, file));
        }

        // Part 5: Halfmove clock
        board.halfmove_clock = parts[4].parse().map_err(|_| FenParseError::InvalidHalfmoveClock(parts[4].to_string()))?;

        // Part 6: Fullmove number
        board.fullmove_number = parts[5].parse().map_err(|_| FenParseError::InvalidFullmoveNumber(parts[5].to_string()))?;
        if board.fullmove_number == 0 {
            return Err(FenParseError::InvalidFullmoveNumber("Fullmove number cannot be 0".to_string()));
        }

        Ok(board)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  +---+---+---+---+---+---+---+---+")?;
        for rank_idx in 0..8 {
            write!(f, "{} |", 8 - rank_idx)?;
            for file_idx in 0..8 {
                match self.squares[rank_idx][file_idx] {
                    Some(piece) => write!(f, " {} |", piece.to_char())?,
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