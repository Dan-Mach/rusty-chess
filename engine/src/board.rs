use crate::color::Color;
use crate::pieces::{Piece as PieceKindEnum, ColoredPiece, PROMOTION_PIECES};
use crate::genmove::{Move, Square, MoveList};
use crate::rank::Rank;
use crate::file::File;
use crate::coordinates::{array_indices_to_square, square_to_array_indices, square_to_rank_file_enums,rank_file_enums_to_square};
use crate::error::{Error, FenParseError}; 

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    InProgress,
    Checkmate(Color), // Color of the winner
    Stalemate,
}

#[derive(Debug, Clone, Copy)]
pub struct PreviousBoardState {
    pub captured_piece: Option<ColoredPiece>,
    pub previous_en_passant_target: Option<(usize, usize)>, 
    pub castling_kingside_white: bool,
    pub castling_queenside_white: bool,
    pub castling_kingside_black: bool,
    pub castling_queenside_black: bool,
    pub previous_halfmove_clock: u32,
}
#[derive(Clone, Debug)]
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
    pub move_history: Vec<(Move, PreviousBoardState)>,
    pub game_result: GameResult,
}

impl Board {
    pub fn new() -> Self { 
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
            move_history: Vec::new(),
            game_result: GameResult::InProgress,
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

    pub fn parse_fen(fen_string: &str) -> Result<Board, Error> {
        let parts: Vec<&str> = fen_string.split_whitespace().collect();
        if parts.len() != 6 {
            
            return Err(Error::FenParsing(FenParseError::NotEnoughParts));
        }

        let mut board = Board::new();
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
    pub fn make_move(&mut self, mv: &Move) -> PreviousBoardState{
        let (from_arr_r, from_arr_f) = square_to_array_indices(mv.from);
        let (to_arr_r, to_arr_f) = square_to_array_indices(mv.to);

        let piece_moved = self.squares[from_arr_r][from_arr_f]
            .expect("make_move: No piece at 'from' square.");
        let prev_state = PreviousBoardState {
            captured_piece: self.squares[to_arr_r][to_arr_f],
            previous_en_passant_target: self.en_passant_target,
            castling_kingside_white: self.castling_kingside_white,
            castling_queenside_white: self.castling_queenside_white,
            castling_kingside_black: self.castling_kingside_black,
            castling_queenside_black: self.castling_queenside_black,
            previous_halfmove_clock: self.halfmove_clock,
        };
        self.move_history.push((mv.clone(), prev_state));
        let mut actual_captured_piece_for_ep_logic = prev_state.captured_piece;
        self.en_passant_target = None; // Reset EP target after a move
        self.squares[to_arr_r][to_arr_f] = Some(piece_moved);
        self.squares[from_arr_r][from_arr_f] = None; // Clear the 'from' square

        if piece_moved.kind == PieceKindEnum::Pawn || prev_state.captured_piece.is_some() {
            self.halfmove_clock = 0;
        }else {
            self.halfmove_clock += 1;
        }

        if piece_moved.kind == PieceKindEnum::Pawn {
            if let Some(promotion_kind) = mv.promotion {
                self.squares[to_arr_r][to_arr_f] = Some(ColoredPiece {
                    kind: promotion_kind,
                    color: piece_moved.color,
                });
            }
        }

        let (from_r_enum, from_f_enum) = square_to_rank_file_enums(mv.from);
        let (to_r_enum, to_f_enum) = square_to_rank_file_enums(mv.to);
        if (to_r_enum.to_index() as i8 - from_f_enum.to_index() as i8).abs() == 2 {
            let ep_rank_val = if piece_moved.color == Color::White { from_r_enum.to_index() + 1 } else { from_r_enum.to_index() - 1 };
            let ep_square_idx = (ep_rank_val as u8 * 8 + from_f_enum.to_index() as u8) as Square;
             self.en_passant_target = Some(square_to_array_indices(ep_square_idx));
        }

        if prev_state.captured_piece.is_none() && (from_arr_f != to_arr_f) {
            if let Some(ep_indices_to_check) = prev_state.previous_en_passant_target {
                let ep_sq_to_check = array_indices_to_square(ep_indices_to_check.0, ep_indices_to_check.1);
                if mv.to == ep_sq_to_check { // Pawn landed on the previous EP target square
                    let captured_pawn_arr_r = if piece_moved.color == Color::White { to_arr_r + 1 } else { to_arr_r - 1 };
                    actual_captured_piece_for_ep_logic = self.squares[captured_pawn_arr_r][to_arr_f].take(); 
                }
                
            }
        }
       // Re-check halfmove clock if EP capture changed the capture status
        if piece_moved.kind == PieceKindEnum::Pawn || prev_state.captured_piece.is_some() || actual_captured_piece_for_ep_logic.is_some() {
            self.halfmove_clock = 0;
        }

        // Handle Castling 
        if piece_moved.kind == PieceKindEnum::King && (to_arr_f as i8 - from_arr_f as i8).abs() == 2 {
            let (rook_from_f_idx, rook_to_f_idx) = if to_arr_f > from_arr_f { (File::H.to_index(), File::F.to_index()) } else { (File::A.to_index(), File::D.to_index()) };
            if let Some(rook_piece) = self.squares[from_arr_r][rook_from_f_idx].take() {
                self.squares[from_arr_r][rook_to_f_idx] = Some(rook_piece);
            }
        }

        // Update Castling Rights after the move
        if piece_moved.kind == PieceKindEnum::King {
            if piece_moved.color == Color::White { self.castling_kingside_white = false; self.castling_queenside_white = false; } 
            else { self.castling_kingside_black = false; self.castling_queenside_black = false; }
        }
        if mv.from == rank_file_enums_to_square(Rank::First, File::A) { self.castling_queenside_white = false; }
        if mv.from == rank_file_enums_to_square(Rank::First, File::H) { self.castling_kingside_white = false; }
        if mv.from == rank_file_enums_to_square(Rank::Eighth, File::A) { self.castling_queenside_black = false; }
        if mv.from == rank_file_enums_to_square(Rank::Eighth, File::H) { self.castling_kingside_black = false; }
        // If a rook is captured on its starting square
        if let Some(cap_piece) = prev_state.captured_piece { // Check what was originally on mv.to
            if cap_piece.kind == PieceKindEnum::Rook {
                if mv.to == rank_file_enums_to_square(Rank::First, File::A) { self.castling_queenside_white = false; }
                if mv.to == rank_file_enums_to_square(Rank::First, File::H) { self.castling_kingside_white = false; }
                if mv.to == rank_file_enums_to_square(Rank::Eighth, File::A) { self.castling_queenside_black = false; }
                if mv.to == rank_file_enums_to_square(Rank::Eighth, File::H) { self.castling_kingside_black = false; }
            }
        }
        
        // Update Fullmove Number
        if self.active_color == Color::Black { self.fullmove_number += 1; }

        // Switch Active Color
        self.active_color = !self.active_color;

        prev_state 
    }
    pub(crate) fn find_king_square(&self, color: Color) -> Option<Square> {
        for r_idx in 0..8 {
            for f_idx in 0..8 {
                if let Some(piece) = self.squares[r_idx][f_idx] {
                    if piece.color == color && piece.kind == PieceKindEnum::King {
                        return Some(array_indices_to_square(r_idx, f_idx));
                    }
                }
            }
        }
        None 
    }
    pub fn undo_move(&mut self) -> Result<(), &'static str> {
        if let Some((mv, prev_state)) = self.move_history.pop() {
            self.unmake_move(&mv, &prev_state);
            Ok(())
        } else {
            Err("No move to undo")
        }
    }
    pub fn generate_legal_moves(&self) -> MoveList {
        let pseudo_legal_moves = self.generate_pseudo_legal_moves();
        let mut legal_moves = MoveList::new(); 
        let current_player_color = self.active_color;     
        let opponent_color = !current_player_color; 

        for mv in pseudo_legal_moves.iter() { // Iterate by reference as Move is Copy
            //Simulate the move on a temporary board
            let mut temp_board = self.clone(); // Requires Board to derive Clone
            temp_board.make_move(mv); // Use the full make_move method

            // Find the current player's king on this temporary board
            //    (It's the king of the player who just made the move `mv`)
            if let Some(king_sq_after_move) = temp_board.find_king_square(current_player_color) {
                // 3. Check if that king is attacked by the opponent on the temporary board
                if !temp_board.is_square_attacked(king_sq_after_move, opponent_color) {
                    legal_moves.push(*mv); 
                }
            } else {
                println!("Error: King not found for {:?}", current_player_color);
            }
        }
        legal_moves
    }
    
    /// Check if the current player is in check
    pub fn is_in_check(&self) -> bool {
        let current_player_color = self.active_color;
        let opponent_color = !current_player_color;
        
        if let Some(king_sq) = self.find_king_square(current_player_color) {
            self.is_square_attacked(king_sq, opponent_color)
        } else {
            false
        }
    }
    
    /// Check if the current position is checkmate
    pub fn is_checkmate(&self) -> bool {
        // Checkmate requires:
        // 1. The current player is in check
        // 2. The current player has no legal moves
        self.is_in_check() && self.generate_legal_moves().is_empty()
    }
    
    /// Check if the current position is stalemate
    pub fn is_stalemate(&self) -> bool {
        // Stalemate requires:
        // 1. The current player is NOT in check
        // 2. The current player has no legal moves
        !self.is_in_check() && self.generate_legal_moves().is_empty()
    }
    
    /// Check if the game is over (checkmate or stalemate)
    pub fn is_game_over(&self) -> bool {
        self.game_result != GameResult::InProgress
    }
    
    /// Update the game result based on the current position.
    /// This should be called after making a move to check for checkmate/stalemate.
    pub fn update_game_result(&mut self) {
        if self.game_result != GameResult::InProgress {
            // Game already over, don't update
            return;
        }
        
        if self.is_checkmate() {
            // The current active player is in checkmate
            // So the opponent (who just moved) wins
            let winner = !self.active_color;
            self.game_result = GameResult::Checkmate(winner);
        } else if self.is_stalemate() {
            self.game_result = GameResult::Stalemate;
        }
    }
    
    pub fn generate_pseudo_legal_moves(&self) -> MoveList {
        let mut moves = MoveList::new(); // MoveList is likely Vec<Move>
        let player_color = self.active_color;

        // Iterate over each square of the board
        for r_idx in 0..8 { // 0 is Rank 8 (FEN), 7 is Rank 1 (FEN) in our array
            for f_idx in 0..8 { // 0 is File A, 7 is File H
                if let Some(piece_on_square) = self.squares[r_idx][f_idx] {
                    // Check if the piece belongs to the current player
                    if piece_on_square.color == player_color {
                        let from_square: Square = array_indices_to_square(r_idx, f_idx);

                        // Dispatch to piece-specific move generation logic
                        match piece_on_square.kind {
                            PieceKindEnum::Pawn   => self.generate_pawn_moves(from_square, player_color, &mut moves),
                            PieceKindEnum::Knight => self.generate_knight_moves(from_square, player_color, &mut moves),
                            PieceKindEnum::Bishop => self.generate_bishop_moves(from_square, player_color, &mut moves),
                            PieceKindEnum::Rook   => self.generate_rook_moves(from_square, player_color, &mut moves),
                            PieceKindEnum::Queen  => self.generate_queen_moves(from_square, player_color, &mut moves),
                            PieceKindEnum::King   => self.generate_king_moves(from_square, player_color, &mut moves),
                        }
                    }
                }
            }
        }
        moves
    }
    
    pub fn to_fen_string(&self) -> String {
        let mut fen_parts: Vec<String> = Vec::with_capacity(6);

        // 1. Piece Placement
        let mut piece_placement_fen = String::new();
        for r_idx in 0..8 { // Iterate from array rank 0 (FEN Rank 8) to array rank 7 (FEN Rank 1)
            let mut empty_squares_count = 0;
            for f_idx in 0..8 { // Iterate from file 0 (File A) to file 7 (File H)
                if let Some(colored_piece) = self.squares[r_idx][f_idx] {
                    if empty_squares_count > 0 {
                        piece_placement_fen.push_str(&empty_squares_count.to_string());
                        empty_squares_count = 0;
                    }
                    piece_placement_fen.push(colored_piece.to_char()); // Relies on ColoredPiece::to_char()
                } else {
                    empty_squares_count += 1;
                }
            }
            // If the rank ended with empty squares
            if empty_squares_count > 0 {
                piece_placement_fen.push_str(&empty_squares_count.to_string());
            }
            // Add '/' separator if it's not the last rank
            if r_idx < 7 {
                piece_placement_fen.push('/');
            }
        }
        fen_parts.push(piece_placement_fen);

        // 2. Active Color
        fen_parts.push(if self.active_color == Color::White { "w".to_string() } else { "b".to_string() });

        // 3. Castling Availability
        let mut castling_fen = String::new();
        if self.castling_kingside_white { castling_fen.push('K'); }
        if self.castling_queenside_white { castling_fen.push('Q'); }
        if self.castling_kingside_black { castling_fen.push('k'); }
        if self.castling_queenside_black { castling_fen.push('q'); }
        if castling_fen.is_empty() {
            fen_parts.push("-".to_string());
        } else {
            fen_parts.push(castling_fen);
        }

        // 4. En Passant Target Square
        if let Some((ep_arr_r, ep_arr_f)) = self.en_passant_target {
            let ep_square = array_indices_to_square(ep_arr_r, ep_arr_f);
            let (ep_rank_enum, ep_file_enum) = square_to_rank_file_enums(ep_square);

            let file_char = (b'a' + ep_file_enum.to_index() as u8) as char;
            let rank_char = std::char::from_digit((ep_rank_enum.to_index() + 1) as u32, 10)
                .expect("Rank index out of range for FEN char conversion");
            fen_parts.push(format!("{}{}", file_char, rank_char));
        } else {
            fen_parts.push("-".to_string());
        }
        // 5. Halfmove Clock
        fen_parts.push(self.halfmove_clock.to_string());

        // 6. Fullmove Number
        fen_parts.push(self.fullmove_number.to_string());

        fen_parts.join(" ")
    }
}