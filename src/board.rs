use std::fmt;
use crate::color::Color;
use crate::pieces::{Piece as PieceKindEnum, ColoredPiece};
use crate::genmove::{Square, MoveList, Move}; 
use crate::error::{Error, FenParseError};
use crate::coordinates::{array_indices_to_square, square_to_array_indices, square_to_rank_file_enums};
use crate::file::File;
use crate::rank::Rank;
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
    pub fn generate_legal_move(&self) -> MoveList {
        let mut moves = MoveList::new();
        let player_color = self.active_color;

        for r_idx in 0..8 {
            for f_idx in 0..8 {
                if let  Some(piece_on_square) = self.squares[r_idx][f_idx] {
                    if piece_on_square.color == player_color {
                        let from_square: Square = array_indices_to_square(r_idx, f_idx);

                        match piece_on_square.kind {
                            PieceKindEnum::Pawn => self.generate_pawn_moves(from_square, player_color, &mut moves),
                            PieceKindEnum::Knight => self.generate_knight_moves(from_square, player_color, &mut moves),
                            PieceKindEnum::Bishop => self.generate_bishop_moves(from_square, player_color, &mut moves),
                            PieceKindEnum::Rook => self.generate_rook_moves(from_square, player_color, &mut moves),
                            PieceKindEnum::Queen => self.generate_queen_moves(from_square, player_color, &mut moves),
                            PieceKindEnum::King => self.generate_king_move(from_square, player_color, &mut moves),
                        }
                    }
                    
                }
            }
        }
        moves
    }
    fn generate_pawn_moves(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList) {
        let (from_rank_enum, from_file_enum) = square_to_rank_file_enums(from_sq);
        let from_rank_val = from_rank_enum.to_index() as i8;
        let from_file_val = from_file_enum.to_index() as i8;

        let forward_delta_rank: i8;
        let start_rank_val:i8;
        let promotion_rank_val: i8;

        if piece_color == Color::White {
            forward_delta_rank = 1;
            start_rank_val = Rank::Second.to_index() as i8;
            promotion_rank_val = Rank::Eighth.to_index() as i8;
        } else {
            forward_delta_rank = -1;
            start_rank_val = Rank::Seventh.to_index() as i8;
            promotion_rank_val = Rank::First.to_index() as i8;
        }

        let _add_pawn_move = |target_rannk_val:i8, target_file_val:i8, is_capture:bool, moves_list: &mut MoveList| {
            if target_rannk_val >= 0 && target_rannk_val <= 7 && target_file_val >= 0 && target_file_val <= 7 {
                let target_sq = (target_rannk_val as u8 * 8 + target_file_val as u8) as Square;
                if target_rannk_val == promotion_rank_val {
                    for &promo_piece_kind in crate::pieces::PROMOTION_PIECES.iter() {
                        moves_list.push(Move::new_promotion(from_sq, target_sq, promo_piece_kind));
                    }
                }else {
                    moves_list.push(Move::new_quiet(from_sq, target_sq))
                }
            }
        };

         // 1. Single Square Push
    let one_step_fwd_rank_val = from_rank_val + forward_delta_rank;
    if one_step_fwd_rank_val >= 0 && one_step_fwd_rank_val <= 7 {
        let target_sq_one_step: Square = (one_step_fwd_rank_val as u8 * 8 + from_file_val as u8) as Square;
        let (target_arr_r, target_arr_f) = square_to_array_indices(target_sq_one_step);
        if self.squares[target_arr_r][target_arr_f].is_none() { // Must be empty
            _add_pawn_move(one_step_fwd_rank_val, from_file_val, false, moves);

            // 2. Double Square Push (only if single push was possible and pawn is on start rank)
            if from_rank_val == start_rank_val {
                let two_steps_fwd_rank_val = from_rank_val + (2 * forward_delta_rank);
                // Boundary check for two_steps_fwd_rank_val is implicitly covered if one_step was valid
                // and start_rank allows two steps.
                let target_sq_two_steps: Square = (two_steps_fwd_rank_val as u8 * 8 + from_file_val as u8) as Square;
                let (target_arr_r_two, target_arr_f_two) = square_to_array_indices(target_sq_two_steps);
                if self.squares[target_arr_r_two][target_arr_f_two].is_none() { // Must also be empty
                    // Note: This move would set the en_passant_target in the board state when actually made.
                    // For generation, we just add the move.
                    moves.push(Move::new_quiet(from_sq, target_sq_two_steps));
                }
            }
        }
    }

    // 3. Diagonal Captures
    for df_capture in [-1i8, 1i8].iter() { // Delta files for capture: -1 (left) and 1 (right)
        let target_capture_rank_val = from_rank_val + forward_delta_rank;
        let target_capture_file_val = from_file_val + *df_capture;

        if target_capture_rank_val >= 0 && target_capture_rank_val <= 7 &&
           target_capture_file_val >= 0 && target_capture_file_val <= 7 {
            
            let target_capture_sq: Square = (target_capture_rank_val as u8 * 8 + target_capture_file_val as u8) as Square;
            let (target_arr_r, target_arr_f) = square_to_array_indices(target_capture_sq);

            if let Some(piece_on_target) = self.squares[target_arr_r][target_arr_f] {
                if piece_on_target.color != piece_color { // Must be an opponent's piece
                    _add_pawn_move(target_capture_rank_val, target_capture_file_val, true, moves);
                }
            }
        }
    }
    
    // 4. En Passant Capture
    if let Some(ep_target_sq_indices) = self.en_passant_target { // ep_target_sq_indices is (array_rank_idx, array_file_idx)
        let ep_target_sq = array_indices_to_square(ep_target_sq_indices.0, ep_target_sq_indices.1);
        let (ep_rank_enum, ep_file_enum) = square_to_rank_file_enums(ep_target_sq);
        let ep_rank_val = ep_rank_enum.to_index() as i8;
        // let ep_file_val = ep_file_enum.to_index() as i8; // Not strictly needed for this check logic

        // Check if pawn is on correct rank to perform en passant
        // White pawns on rank 5 (index 4), Black pawns on rank 4 (index 3)
        let can_ep_from_this_rank = if piece_color == Color::White {
            from_rank_val == Rank::Fifth.to_index() as i8 
        } else { // Black
            from_rank_val == Rank::Fourth.to_index() as i8
        };

        if can_ep_from_this_rank {
            // Check if the en_passant_target square is one of the diagonal forward squares for this pawn
            for df_capture in [-1i8, 1i8].iter() {
                let potential_ep_capture_rank_val = from_rank_val + forward_delta_rank;
                let potential_ep_capture_file_val = from_file_val + *df_capture;

                if potential_ep_capture_rank_val == ep_rank_val && 
                   potential_ep_capture_file_val == ep_file_enum.to_index() as i8 {
                    // This pawn can capture en passant onto ep_target_sq
                    // No promotion on en passant
                    moves.push(Move::new_quiet(from_sq, ep_target_sq)); // Mark as en-passant if Move struct supports it
                    break; // Found the en passant for this pawn
                }
            }
        }
    }
       

    
    }
    fn generate_knight_moves(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList) {
        const KNIGHT_OFFSETS: [(i8, i8); 8] = [
            (1, 2), (1, -2), (-1, 2), (-1, -2),
            (2, 1), (2, -1), (-2, 1), (-2, -1),

        ];
        let (from_rank_enum, from_fil_enum) = square_to_rank_file_enums(from_sq);
        let from_rank_val = from_rank_enum.to_index() as i8;
        let from_file_val = from_fil_enum.to_index() as i8;

        for (dr,df) in KNIGHT_OFFSETS.iter() {
            let target_rank_val = from_rank_val + dr;
            let target_file_val = from_file_val + df;

            if target_rank_val >= 0 && target_rank_val <= 7 && target_file_val >= 0 && target_file_val <= 7 {
                let target_sq = (target_rank_val as u8 * 8 + target_file_val as u8) as Square;
                let (target_arr_r, target_arr_f) = square_to_array_indices(target_sq);

                match self.squares[target_arr_r][target_arr_f] {
                    Some(piece_on_target_sq) => {
                        if piece_on_target_sq.color != piece_color {
                            moves.push(Move::new_quiet(from_sq, target_sq)); 
                        }
                    }
                    None => {
                        moves.push(Move::new(from_sq, target_sq)); // Non-capture move
                    }
                }
            }
        }
    }
    fn generate_bishop_moves(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList) {
        const BISHOP_DIRECTIONS: [(i8, i8); 4] = [
            (1, 1), (1, -1), (-1, 1), (-1, -1),
        ];
        let (from_rank_enum, from_file_enum) = square_to_rank_file_enums(from_sq);
        let initial_rank_val = from_rank_enum.to_index() as i8;
        let initial_file_val = from_file_enum.to_index() as i8;
        
        for (dr, df) in BISHOP_DIRECTIONS.iter() {
            let mut current_rank_val = initial_rank_val;
            let mut current_file_val = initial_file_val;

            loop {
                current_rank_val += dr;
                current_file_val += df;

                // Boundary check
                if current_rank_val >= 0 && current_rank_val <= 7 &&
                    current_file_val >= 0 && current_file_val <= 7 {
                
                    let target_sq: Square = (current_rank_val as u8 * 8 + current_file_val as u8) as Square;
                    let (target_arr_r, target_arr_f) = square_to_array_indices(target_sq);

                    match self.squares[target_arr_r][target_arr_f] {
                        None => {
                            // Empty square: add move and continue in this direction
                            moves.push(Move::new_quiet(from_sq, target_sq));
                        }
                        Some(piece_on_target_sq) => {
                            // Occupied square
                            if piece_on_target_sq.color != piece_color {
                                // Opponent piece: add capture and stop in this direction
                                moves.push(Move::new_quiet(from_sq, target_sq));
                            }
                            // Friendly piece or opponent piece (after adding capture): stop in this direction
                            break; 
                        }
                    }
                }else {
                    // Out of bounds: stop in this direction
                    break;
                }
            }
        }
    }
       
    fn generate_rook_moves(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList) {
        const ROOK_DIRECTIONS: [(i8, i8); 4] = [
             (0, 1), (0, -1),(1, 0),(-1, 0),
        ];
        let (from_rank_enum, from_file_enum) = square_to_rank_file_enums(from_sq);
        let initial_rank_val = from_rank_enum.to_index() as i8;
        let initial_file_val = from_file_enum.to_index() as i8;

        for (dr, df) in ROOK_DIRECTIONS.iter() {
            let mut current_rank_val = initial_rank_val;
            let mut current_file_val = initial_file_val;

            loop {
                current_rank_val += dr;
                current_file_val += df;

                // Boundary check
                if current_rank_val >= 0 && current_rank_val <= 7 &&
                    current_file_val >= 0 && current_file_val <= 7 {
                
                    let target_sq: Square = (current_rank_val as u8 * 8 + current_file_val as u8) as Square;
                    let (target_arr_r, target_arr_f) = square_to_array_indices(target_sq);

                    match self.squares[target_arr_r][target_arr_f] {
                        None => {
                            // Empty square: add move and continue in this direction
                            moves.push(Move::new_quiet(from_sq, target_sq));
                        }
                        Some(piece_on_target_sq) => {
                            // Occupied square
                            if piece_on_target_sq.color != piece_color {
                                // Opponent piece: add capture and stop in this direction
                                moves.push(Move::new_quiet(from_sq, target_sq));
                            }
                            // Friendly piece or opponent piece (after adding capture): stop in this direction
                            break; 
                        }
                    }
                }else {
                    // Out of bounds: stop in this direction
                    break;
                }
            }
        }
           
    }
    fn generate_queen_moves(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList) {
        const QUEEN_DIRECTIONS: [(i8, i8); 8] = [
            (1, 0), (0, 1), (-1, 0), (0, -1), // Rook-like moves
            (1, 1), (1, -1), (-1, 1), (-1, -1), // Bishop-like moves
        ];
        
        let (from_rank_enum, from_file_enum) = square_to_rank_file_enums(from_sq);
        let initial_rank_val = from_rank_enum.to_index() as i8;
        let initial_file_val = from_file_enum.to_index() as i8;

        for (dr, df) in QUEEN_DIRECTIONS.iter() {
            let mut current_rank_val = initial_rank_val;
            let mut current_file_val = initial_file_val;

            loop {
                current_rank_val += dr;
                current_file_val += df;

                // Boundary check
                if current_rank_val >= 0 && current_rank_val <= 7 &&
                    current_file_val >= 0 && current_file_val <= 7 {
                
                    let target_sq: Square = (current_rank_val as u8 * 8 + current_file_val as u8) as Square;
                    let (target_arr_r, target_arr_f) = square_to_array_indices(target_sq);

                    match self.squares[target_arr_r][target_arr_f] {
                        None => {
                            // Empty square: add move and continue in this direction
                            moves.push(Move::new_quiet(from_sq, target_sq));
                        }
                        Some(piece_on_target_sq) => {
                            // Occupied square
                            if piece_on_target_sq.color != piece_color {
                                // Opponent piece: add capture and stop in this direction
                                moves.push(Move::new_quiet(from_sq, target_sq));
                            }
                            // Friendly piece or opponent piece (after adding capture): stop in this direction
                            break; 
                        }
                    }
                }else {
                    // Out of bounds: stop in this direction
                    break;
                }
            }
        }
    }
    fn generate_king_move(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList) {
        const KING_OFFSETS: [(i8, i8); 8] = [
            (1, 0), (0, 1), (-1, 0), (0, -1),
            (1, 1), (1, -1), (-1, 1), (-1, -1),
        ];
        let (from_rank_enum, from_file_enum) = square_to_rank_file_enums(from_sq);
        let from_rank_val = from_rank_enum.to_index() as i8;
        let from_file_val = from_file_enum.to_index() as i8;

        for (dr, df) in KING_OFFSETS.iter() {
            let target_rank_val = from_rank_val + dr;
            let target_file_val = from_file_val + df;

            if target_rank_val >= 0 && target_rank_val <= 7 && target_file_val >= 0 && target_file_val <= 7 {
                let target_sq: Square = (target_rank_val as u8 * 8 + target_file_val as u8) as Square;
                let (target_arr_r, target_arr_f) = square_to_array_indices(target_sq);

                match self.squares[target_arr_r][target_arr_f] {
                    None => {
                        moves.push(Move::new(from_sq, target_sq)); // Non-capture move
                    }
                    Some(piece_on_target_sq) => {
                        if piece_on_target_sq.color != piece_color {
                            moves.push(Move::new_quiet(from_sq, target_sq)); 
                        }
                    }
                    
                }
            }
        }

        let home_array_rank = if piece_color == Color::White { 7 } else { 0 };
        let can_castle_ks = if piece_color == Color::White {
            self.castling_kingside_white
        } else {
            self.castling_kingside_black
        };
        if can_castle_ks {
            if self.squares[home_array_rank][5].is_none() && self.squares[home_array_rank][6].is_none() {
                // Check if the squares between the king and rook are empty
                let king_dest_sq_ks  = array_indices_to_square(home_array_rank, 6);
                moves.push(Move::new(from_sq, king_dest_sq_ks)); // Castling kingside move
               
            }
        }
        // Queenside Castling (O-O-O)
        // King moves from E file (index 4) to C file (index 2)
        let can_castle_qs = if piece_color == Color::White { self.castling_queenside_white } else { self.castling_queenside_black };
        if can_castle_qs {
            // Check squares D, C, and B are empty (indices 3, 2, and 1 on the home rank)
            // Note: Rook moves to D file (index 3)
            if self.squares[home_array_rank][1].is_none() && 
            self.squares[home_array_rank][2].is_none() &&
            self.squares[home_array_rank][3].is_none() {
                // TODO: Add check: king is not in check, D and C squares are not attacked.
                // (B square is not passed through by the king in O-O-O, king lands on C)
                // For now, just adding the pseudo-legal move.
                let king_dest_sq_qs = array_indices_to_square(home_array_rank, 2);
                moves.push(Move::new_quiet(from_sq, king_dest_sq_qs)); // Mark as castling later if needed
            }
        }
    }
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

// At the end of src/board.rs

#[cfg(test)]
mod tests {
    use super::*; // Imports everything from the parent module (board.rs)
                  // like Board, Color, etc.
    use crate::genmove::{Move, Square}; // Assuming MoveList is Vec<Move>
    // We don't strictly need PieceKindEnum or Color for this specific knight test's assertions 
    // if we just check from/to squares, but good to have for other tests.
    use std::collections::HashSet;

    // Helper function to convert algebraic square notation (e.g., "e4") to your Square (u8) type.
    // This assumes A1=0, B1=1, ..., H1=7, A2=8, ..., H8=63 (LERF mapping).
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
        let board = Board::parse_fen(fen).expect("Failed to parse FEN for knight test");
        
        // The knight is on d4. Calculate its Square index.
        // Rank '4' is rank_val 3 (0-indexed). File 'd' is file_val 3 (0-indexed).
        // Square index = rank_val * 8 + file_val = 3 * 8 + 3 = 24 + 3 = 27.
        let knight_from_sq: Square = sq("d4"); // Should be 27

        // 2. Call move generation
        let all_generated_moves = board.generate_legal_move();

        // 3. Filter for moves from our knight
        let knight_actual_moves: HashSet<Move> = all_generated_moves
            .into_iter()
            .filter(|m| m.from == knight_from_sq)
            .collect();

        // 4. Define expected moves
        // A knight on d4 (sq 27) on an empty board can move to 8 squares:
        // D4 (rank 3, file 3) ->
        // C2 (rank 1, file 2) -> sq 1*8+2 = 10
        // E2 (rank 1, file 4) -> sq 1*8+4 = 12
        // B3 (rank 2, file 1) -> sq 2*8+1 = 17
        // F3 (rank 2, file 5) -> sq 2*8+5 = 21
        // B5 (rank 4, file 1) -> sq 4*8+1 = 33
        // F5 (rank 4, file 5) -> sq 4*8+5 = 37
        // C6 (rank 5, file 2) -> sq 5*8+2 = 42
        // E6 (rank 5, file 4) -> sq 5*8+4 = 44
        let expected_moves: HashSet<Move> = [
            Move::new_quiet(knight_from_sq, sq("c2")), // to 10
            Move::new_quiet(knight_from_sq, sq("e2")), // to 12
            Move::new_quiet(knight_from_sq, sq("b3")), // to 17
            Move::new_quiet(knight_from_sq, sq("f3")), // to 21
            Move::new_quiet(knight_from_sq, sq("b5")), // to 33
            Move::new_quiet(knight_from_sq, sq("f5")), // to 37
            Move::new_quiet(knight_from_sq, sq("c6")), // to 42
            Move::new_quiet(knight_from_sq, sq("e6")), // to 44
        ]
        .iter()
        .cloned() // Since Move is Copy, cloned() works well here.
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

    // You should add more tests for knights:
    // - test_knight_moves_from_corner_a1()
    // - test_knight_moves_from_edge_h4()
    // - test_knight_moves_blocked_by_own_pieces()
    // - test_knight_moves_capturing_opponent_pieces()
}