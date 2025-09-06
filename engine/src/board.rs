use crate::color::Color;
use crate::pieces::{Piece as PieceKindEnum, ColoredPiece, PROMOTION_PIECES};
use crate::genmove::{Move, Square, MoveList};
use crate::rank::Rank;
use crate::file::File;
use crate::coordinates::{array_indices_to_square, square_to_array_indices, square_to_rank_file_enums,rank_file_enums_to_square};
use crate::error::{Error, FenParseError}; 

#[derive(Debug, Clone, Copy)]
pub struct PreviousBoardState {
    pub captured_piece: Option<ColoredPiece>,
    pub previous_en_passant_target: Option<(usize, usize)>, // Stores (array_rank_idx, array_file_idx)
    pub castling_kingside_white: bool,
    pub castling_queenside_white: bool,
    pub castling_kingside_black: bool,
    pub castling_queenside_black: bool,
    pub previous_halfmove_clock: u32,
}
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
    pub move_history: Vec<(Move, PreviousBoardState)>,
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
            move_history: Vec::new(),
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
       // (Re-check halfmove clock if EP capture changed the capture status)
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
    pub(crate) fn generate_knight_moves(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList) {
        const KNIGHT_OFFSETS: [(i8, i8); 8] = [
            (1, 2), (1, -2), (-1, 2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1),
        ];
        let (from_rank_enum, from_file_enum) = square_to_rank_file_enums(from_sq);
        let from_rank_val = from_rank_enum.to_index() as i8;
        let from_file_val = from_file_enum.to_index() as i8;

        for (dr, df) in KNIGHT_OFFSETS.iter() {
            let target_rank_val = from_rank_val + dr;
            let target_file_val = from_file_val + df;

            if target_rank_val >= 0 && target_rank_val <= 7 &&
               target_file_val >= 0 && target_file_val <= 7 {
                let target_sq: Square = (target_rank_val as u8 * 8 + target_file_val as u8) as Square;
                let (target_arr_r, target_arr_f) = square_to_array_indices(target_sq);
                match self.squares[target_arr_r][target_arr_f] {
                    Some(piece_on_target_sq) => {
                        if piece_on_target_sq.color != piece_color {
                            moves.push(Move::new_quiet(from_sq, target_sq));
                        }
                    }
                    None => {
                        moves.push(Move::new_quiet(from_sq, target_sq));
                    }
                }
            }
        }
    }
    pub(crate) fn generate_pawn_moves(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList) {
        let (from_rank_enum, from_file_enum) = square_to_rank_file_enums(from_sq);
        let from_rank_val = from_rank_enum.to_index() as i8;
        let from_file_val = from_file_enum.to_index() as i8;

        let forward_delta_rank: i8;
        let start_rank_val: i8;
        let promotion_rank_val: i8;

        if piece_color == Color::White {
            forward_delta_rank = 1;
            start_rank_val = Rank::Second.to_index() as i8;
            promotion_rank_val = Rank::Eighth.to_index() as i8;
        } else { // Black
            forward_delta_rank = -1;
            start_rank_val = Rank::Seventh.to_index() as i8;
            promotion_rank_val = Rank::First.to_index() as i8;
        }

        let add_move_with_promotion_check = |current_from_sq: Square, target_rank_val: i8, target_file_val: i8, moves_list: &mut MoveList| {
            // Check boundary before creating target_sq to avoid panic with invalid rank/file values
            if target_rank_val >=0 && target_rank_val <= 7 && target_file_val >=0 && target_file_val <=7 {
                let target_sq = (target_rank_val as u8 * 8 + target_file_val as u8) as Square;
                if target_rank_val == promotion_rank_val {
                    for &promo_piece_kind in PROMOTION_PIECES.iter() {
                        moves_list.push(Move::new_promotion(current_from_sq, target_sq, promo_piece_kind));
                    }
                } else {
                    moves_list.push(Move::new_quiet(current_from_sq, target_sq));
                }
            }
        };
        
        // Single Square Push
        let one_step_fwd_rank_val = from_rank_val + forward_delta_rank;
        if one_step_fwd_rank_val >= 0 && one_step_fwd_rank_val <= 7 { // Check rank boundary
            let target_sq_one_step: Square = (one_step_fwd_rank_val as u8 * 8 + from_file_val as u8) as Square;
            let (target_arr_r, target_arr_f) = square_to_array_indices(target_sq_one_step);
            if self.squares[target_arr_r][target_arr_f].is_none() {
                add_move_with_promotion_check(from_sq, one_step_fwd_rank_val, from_file_val, moves);
                // Double Square Push
                if from_rank_val == start_rank_val {
                    let two_steps_fwd_rank_val = from_rank_val + (2 * forward_delta_rank);
                    // No need to check boundary for two_steps_fwd_rank_val if one_step was valid from start rank
                    let target_sq_two_steps: Square = (two_steps_fwd_rank_val as u8 * 8 + from_file_val as u8) as Square;
                    let (target_arr_r_two, target_arr_f_two) = square_to_array_indices(target_sq_two_steps);
                    if self.squares[target_arr_r_two][target_arr_f_two].is_none() {
                        moves.push(Move::new_quiet(from_sq, target_sq_two_steps));
                    }
                }
            }
        }

        //Diagonal Captures
        for df_capture in [-1i8, 1i8].iter() {
            let target_capture_rank_val = from_rank_val + forward_delta_rank;
            let target_capture_file_val = from_file_val + *df_capture;

            if target_capture_rank_val >= 0 && target_capture_rank_val <= 7 &&
               target_capture_file_val >= 0 && target_capture_file_val <= 7 {
                let target_capture_sq: Square = (target_capture_rank_val as u8 * 8 + target_capture_file_val as u8) as Square;
                let (target_arr_r, target_arr_f) = square_to_array_indices(target_capture_sq);

                if let Some(piece_on_target) = self.squares[target_arr_r][target_arr_f] {
                    if piece_on_target.color != piece_color {
                        add_move_with_promotion_check(from_sq, target_capture_rank_val, target_capture_file_val, moves);
                    }
                }
            }
        }
        
        //En Passant Capture
        if let Some(ep_target_sq_indices) = self.en_passant_target { // ep_target_sq_indices is (array_rank_idx, array_file_idx)
            let ep_target_sq = array_indices_to_square(ep_target_sq_indices.0, ep_target_sq_indices.1);
            let (ep_rank_enum, ep_file_enum) = square_to_rank_file_enums(ep_target_sq);
            let ep_rank_val = ep_rank_enum.to_index() as i8;

            let can_ep_from_this_rank = if piece_color == Color::White {
                from_rank_val == Rank::Fifth.to_index() as i8 
            } else { // Black
                from_rank_val == Rank::Fourth.to_index() as i8
            };

            if can_ep_from_this_rank {
                for df_capture in [-1i8, 1i8].iter() {
                    let potential_ep_capture_rank_val = from_rank_val + forward_delta_rank;
                    let potential_ep_capture_file_val = from_file_val + *df_capture;

                    if potential_ep_capture_rank_val == ep_rank_val && 
                       potential_ep_capture_file_val == ep_file_enum.to_index() as i8 {
                        moves.push(Move::new_quiet(from_sq, ep_target_sq)); // EP move target is the EP square
                        break;
                    }
                }
            }
        }
    }
    pub(crate) fn generate_bishop_moves(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList) {
        const BISHOP_DIRECTIONS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
        self.generate_sliding_piece_moves(from_sq, piece_color, moves, &BISHOP_DIRECTIONS);
    }

    pub(crate) fn generate_rook_moves(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList) {
        const ROOK_DIRECTIONS: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        self.generate_sliding_piece_moves(from_sq, piece_color, moves, &ROOK_DIRECTIONS);
    }

    pub(crate) fn generate_queen_moves(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList) {
        const QUEEN_DIRECTIONS: [(i8, i8); 8] = [
            (0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1),
        ];
        self.generate_sliding_piece_moves(from_sq, piece_color, moves, &QUEEN_DIRECTIONS);
    }
    
    /// Helper for sliding pieces (Rook, Bishop, Queen)
    fn generate_sliding_piece_moves(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList, directions: &[(i8,i8)]) {
        let (from_rank_enum, from_file_enum) = square_to_rank_file_enums(from_sq);
        let initial_rank_val = from_rank_enum.to_index() as i8;
        let initial_file_val = from_file_enum.to_index() as i8;

        for (dr, df) in directions.iter() {
            let mut current_rank_val = initial_rank_val;
            let mut current_file_val = initial_file_val;

            loop {
                current_rank_val += *dr;
                current_file_val += *df;

                if current_rank_val >= 0 && current_rank_val <= 7 &&
                   current_file_val >= 0 && current_file_val <= 7 {
                    
                    let target_sq: Square = (current_rank_val as u8 * 8 + current_file_val as u8) as Square;
                    let (target_arr_r, target_arr_f) = square_to_array_indices(target_sq);

                    match self.squares[target_arr_r][target_arr_f] {
                        None => {
                            moves.push(Move::new_quiet(from_sq, target_sq));
                        }
                        Some(piece_on_target_sq) => {
                            if piece_on_target_sq.color != piece_color {
                                moves.push(Move::new_quiet(from_sq, target_sq));
                            }
                            break; 
                        }
                    }
                } else {
                    break; 
                }
            }
        }
    }

    pub(crate) fn generate_king_moves(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList) {
        const KING_DIRECTIONS: [(i8, i8); 8] = [
            (0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1),
        ];
        let (from_rank_enum, from_file_enum) = square_to_rank_file_enums(from_sq);
        let initial_rank_val = from_rank_enum.to_index() as i8;
        let initial_file_val = from_file_enum.to_index() as i8;

        for (dr, df) in KING_DIRECTIONS.iter() {
            let target_rank_val = initial_rank_val + dr;
            let target_file_val = initial_file_val + df;

            if target_rank_val >= 0 && target_rank_val <= 7 &&
               target_file_val >= 0 && target_file_val <= 7 {
                let target_sq: Square = (target_rank_val as u8 * 8 + target_file_val as u8) as Square;
                let (target_arr_r, target_arr_f) = square_to_array_indices(target_sq);
                match self.squares[target_arr_r][target_arr_f] {
                    None => { moves.push(Move::new_quiet(from_sq, target_sq)); }
                    Some(piece_on_target_sq) => {
                        if piece_on_target_sq.color != piece_color {
                            moves.push(Move::new_quiet(from_sq, target_sq));
                        }
                    }
                }
            }
        }
    }
    pub fn unmake_move(&mut self, mv: &Move, prev_state: &PreviousBoardState) {
        let (from_arr_r, from_arr_f) = square_to_array_indices(mv.from);
        let (to_arr_r, to_arr_f) = square_to_array_indices(mv.to);

        // 1. Restore Active Color (to the player who made the move)
        self.active_color = !self.active_color; 

        // 2. Restore Fullmove Number (if black's move was just unmade)
        if self.active_color == Color::Black { // Now it's Black's turn again, meaning Black's move was unmade
            self.fullmove_number -= 1;
        }
        
        // 3. Restore Castling Rights
        self.castling_kingside_white = prev_state.castling_kingside_white;
        self.castling_queenside_white = prev_state.castling_queenside_white;
        self.castling_kingside_black = prev_state.castling_kingside_black;
        self.castling_queenside_black = prev_state.castling_queenside_black;

        // 4. Restore En Passant Target
        self.en_passant_target = prev_state.previous_en_passant_target;
        
        // 5. Restore Halfmove Clock
        self.halfmove_clock = prev_state.previous_halfmove_clock;

        // 6. Undo Piece Movement
        // 6a. Get the piece that moved (it's currently on mv.to)
        let piece_that_moved = self.squares[to_arr_r][to_arr_f]
            .expect("unmake_move: No piece at 'to' square, should have been moved by make_move.");

        // 6b. Handle Pawn Demotion (if it was a promotion)
        if mv.promotion.is_some() {
            // Change the piece back to a pawn of its color
            self.squares[from_arr_r][from_arr_f] = Some(ColoredPiece {
                kind: PieceKindEnum::Pawn,
                color: piece_that_moved.color, // Color of the promoted piece is the pawn's color
            });
        } else {
            // Move the piece back to its original square
            self.squares[from_arr_r][from_arr_f] = Some(piece_that_moved);
        }
        self.squares[to_arr_r][to_arr_f] = prev_state.captured_piece;

        if piece_that_moved.kind == PieceKindEnum::Pawn &&
           (from_arr_f != to_arr_f) && // Diagonal pawn move
           prev_state.captured_piece.is_none() && // Landed on an empty square
           prev_state.previous_en_passant_target.map_or(false, |ep_indices| 
               mv.to == array_indices_to_square(ep_indices.0, ep_indices.1)
           )
        {
            let captured_pawn_arr_r = if piece_that_moved.color == Color::White {
                to_arr_r + 1 // White captured a black pawn on rank below mv.to (array index perspective)
            } else {
                to_arr_r - 1 // Black captured a white pawn on rank above mv.to
            };
            // The captured pawn's color is the opponent's color
            let opponent_color = !piece_that_moved.color;
            self.squares[captured_pawn_arr_r][to_arr_f] = Some(ColoredPiece {
                kind: PieceKindEnum::Pawn,
                color: opponent_color,
            });
        }

        // 7b. Undo Castling (Move the Rook back)
        if piece_that_moved.kind == PieceKindEnum::King &&
           (to_arr_f as i8 - from_arr_f as i8).abs() == 2 { // King moved two squares
            
            let (rook_original_f_idx, rook_current_f_idx) = if to_arr_f > from_arr_f { // Kingside castle (king went E->G, rook F->H)
                (File::H.to_index(), File::F.to_index())
            } else { // Queenside castle (king went E->C, rook D->A)
                (File::A.to_index(), File::D.to_index())
            };
            // from_arr_r is king's original rank
            if let Some(rook_piece) = self.squares[from_arr_r][rook_current_f_idx].take() {
                self.squares[from_arr_r][rook_original_f_idx] = Some(rook_piece);
            } else {
                panic!("Unmake castling error: Rook not found at expected castled position!");
            }
        }
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