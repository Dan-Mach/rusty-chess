use crate::{File, PROMOTION_PIECES, PreviousBoardState, Rank};
use crate::board::Board; 
use crate::color::Color;
use crate::pieces::{Piece as PieceKindEnum, ColoredPiece};
use crate::genmove::{Move, MoveList, Square};
use crate::coordinates::{array_indices_to_square, square_to_array_indices, square_to_rank_file_enums, rank_file_enums_to_square};

impl Board {
    pub fn is_square_attacked(&self, target_sq: Square, attacker_color: Color) -> bool {
        for r_idx in 0..8 {
            for f_idx in 0..8 {
                if let Some(piece_on_square) = self.squares[r_idx][f_idx] {
                    if piece_on_square.color == attacker_color {
                        let attacker_current_sq = array_indices_to_square(r_idx, f_idx);
                        match piece_on_square.kind {
                            PieceKindEnum::Pawn => {
                                if self.is_pawn_attacking(attacker_current_sq, attacker_color, target_sq) {
                                    return true;
                                }
                            }
                            PieceKindEnum::Knight => {
                                if self.is_knight_attacking(attacker_current_sq, target_sq) {
                                    return true;
                                }
                            }
                            PieceKindEnum::Bishop => {
                                if self.is_bishop_attacking(attacker_current_sq, target_sq) {
                                    return true;  
                                }
                            }
                            PieceKindEnum::Rook => {
                                if self.is_rook_attacking(attacker_current_sq, target_sq) {
                                    return true;
                                }
                            }
                            PieceKindEnum::Queen => {
                                // Queen attacks like a rook OR a bishop
                                if self.is_queen_attacking(attacker_current_sq, target_sq) {
                                    return true;
                                }
                            }
                            PieceKindEnum::King => {
                                if self.is_king_attacking(attacker_current_sq, target_sq) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }
    pub(crate) fn is_pawn_attacking(&self, pawn_sq: Square, pawn_color: Color, target_sq: Square) -> bool {
        let (pawn_rank_enum, pawn_file_enum) = square_to_rank_file_enums(pawn_sq);
        let pawn_rank_val = pawn_rank_enum.to_index() as i8;
        let pawn_file_val = pawn_file_enum.to_index() as i8;
        let forward_delta_rank = if pawn_color == Color::White { 1 } else { -1 };

        for df_capture in [-1i8, 1i8].iter() {
            let attack_rank_val = pawn_rank_val + forward_delta_rank;
            let attack_file_val = pawn_file_val + *df_capture;

            if attack_rank_val >= 0 && attack_rank_val <= 7 &&
               attack_file_val >= 0 && attack_file_val <= 7 {
                
                let attack_sq = (attack_rank_val as u8 * 8 + attack_file_val as u8) as Square;

                if attack_sq == target_sq {
                    return true;
                }
            }
        }
        false
    }

    pub(crate) fn is_knight_attacking(&self, knight_sq: Square, target_sq: Square) -> bool {
        const KNIGHT_OFFSETS: [(i8, i8); 8] = [
            (1, 2), (1, -2), (-1, 2), (-1, -2),
            (2, 1), (2, -1), (-2, 1), (-2, -1),
        ];
        let (knight_rank_enum, knight_file_enum) = square_to_rank_file_enums(knight_sq);
        let knight_rank_val = knight_rank_enum.to_index() as i8;
        let knight_file_val = knight_file_enum.to_index() as i8;

        for (dr, df) in KNIGHT_OFFSETS.iter() {
            let potential_target_rank = knight_rank_val + dr;
            let potential_target_file = knight_file_val + df;

            if potential_target_rank >= 0 && potential_target_rank <= 7 &&
               potential_target_file >= 0 && potential_target_file <= 7 {
                
                let attack_sq = (potential_target_rank as u8 * 8 + potential_target_file as u8) as Square;

                if attack_sq == target_sq {
                    return true; 
                }
            }
        }
        false
    }

    pub(crate) fn is_sliding_piece_attacking(&self,piece_sq: Square,target_sq: Square,directions: &[(i8, i8)]) -> bool {
        let (piece_rank_enum, piece_file_enum) = square_to_rank_file_enums(piece_sq);
        let initial_rank_val = piece_rank_enum.to_index() as i8;
        let initial_file_val = piece_file_enum.to_index() as i8;

        for (dr, df) in directions.iter() {
            let mut current_rank_val = initial_rank_val;
            let mut current_file_val = initial_file_val;

            loop { 
                current_rank_val += *dr;
                current_file_val += *df;

                if !(current_rank_val >= 0 && current_rank_val <= 7 &&
                     current_file_val >= 0 && current_file_val <= 7) {
                    break; 
                }

                let current_iter_sq: Square = (current_rank_val as u8 * 8 + current_file_val as u8) as Square;

                if current_iter_sq == target_sq {
                    return true; // The piece attacks the target square along this ray
                }
                let (arr_r, arr_f) = square_to_array_indices(current_iter_sq);
                if self.squares[arr_r][arr_f].is_some() {
                    break; 
                }
            }
        }
        false 
    }
    pub(crate) fn is_rook_attacking(&self, rook_sq: Square, target_sq: Square) -> bool {
        const ROOK_DIRECTIONS: [(i8, i8); 4] = [
            (0, 1), (0, -1), (1, 0), (-1, 0)
        ];
        self.is_sliding_piece_attacking(rook_sq, target_sq, &ROOK_DIRECTIONS)
    }

    pub(crate) fn is_bishop_attacking(&self, bishop_sq: Square, target_sq: Square) -> bool {
        const BISHOP_DIRECTIONS: [(i8, i8); 4] = [
            (1, 1), (1, -1), (-1, 1), (-1, -1)
        ];
        self.is_sliding_piece_attacking(bishop_sq, target_sq, &BISHOP_DIRECTIONS)
    }
    
    pub(crate) fn is_king_attacking(&self, king_sq: Square, target_sq: Square) -> bool {
        const KING_DIRECTIONS: [(i8, i8); 8] = [
            (0, 1), (0, -1), (1, 0), (-1, 0), 
            (1, 1), (1, -1), (-1, 1), (-1, -1),
        ];
        let (king_rank_enum, king_file_enum) = square_to_rank_file_enums(king_sq);
        let king_rank_val = king_rank_enum.to_index() as i8;
        let king_file_val = king_file_enum.to_index() as i8;

        for (dr, df) in KING_DIRECTIONS.iter() {
            let potential_target_rank = king_rank_val + dr;
            let potential_target_file = king_file_val + df;

            if potential_target_rank >= 0 && potential_target_rank <= 7 &&
               potential_target_file >= 0 && potential_target_file <= 7 {
                
                let attack_sq = (potential_target_rank as u8 * 8 + potential_target_file as u8) as Square;

                if attack_sq == target_sq {
                    return true; 
                }   
            }
        }
        false
    }
    pub(crate) fn is_queen_attacking(&self, queen_sq: Square, target_sq: Square) -> bool {
        const QUEEN_DIRECTIONS: [(i8, i8); 8] = [
            (0, 1), (0, -1), (1, 0), (-1, 0), 
            (1, 1), (1, -1), (-1, 1), (-1, -1),
        ];
        self.is_sliding_piece_attacking(queen_sq, target_sq, &QUEEN_DIRECTIONS) 
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
        } 
        else {
            forward_delta_rank = -1;
            start_rank_val = Rank::Seventh.to_index() as i8;
            promotion_rank_val = Rank::First.to_index() as i8;
        }
        let add_move_with_promotion_check = |current_from_sq: Square, target_rank_val: i8, target_file_val: i8, moves_list: &mut MoveList| {
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
        

        let one_step_fwd_rank_val = from_rank_val + forward_delta_rank;
        if one_step_fwd_rank_val >= 0 && one_step_fwd_rank_val <= 7 { 
            let target_sq_one_step: Square = (one_step_fwd_rank_val as u8 * 8 + from_file_val as u8) as Square;
            let (target_arr_r, target_arr_f) = square_to_array_indices(target_sq_one_step);
            if self.squares[target_arr_r][target_arr_f].is_none() {
                add_move_with_promotion_check(from_sq, one_step_fwd_rank_val, from_file_val, moves);
      
                if from_rank_val == start_rank_val {
                    let two_steps_fwd_rank_val = from_rank_val + (2 * forward_delta_rank);

                    let target_sq_two_steps: Square = (two_steps_fwd_rank_val as u8 * 8 + from_file_val as u8) as Square;
                    let (target_arr_r_two, target_arr_f_two) = square_to_array_indices(target_sq_two_steps);
                    if self.squares[target_arr_r_two][target_arr_f_two].is_none() {
                        moves.push(Move::new_quiet(from_sq, target_sq_two_steps));
                    }
                }
            }
        }

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

        let king_sq = from_sq;

        if piece_color == Color::White {
            // --- Kingside ---
            if self.castling_kingside_white {
                let f1 = rank_file_enums_to_square(Rank::First, File::F);
                let g1 = rank_file_enums_to_square(Rank::First, File::G);

                let (f1_r, f1_f) = square_to_array_indices(f1);
                let (g1_r, g1_f) = square_to_array_indices(g1);

                if self.squares[f1_r][f1_f].is_none() &&
                self.squares[g1_r][g1_f].is_none() &&
                !self.is_square_attacked(king_sq, Color::Black) &&
                !self.is_square_attacked(f1, Color::Black) &&
                !self.is_square_attacked(g1, Color::Black) {

                    moves.push(Move::new_quiet(king_sq, g1));
                }
            }

            // --- Queenside ---
            if self.castling_queenside_white {
                let d1 = rank_file_enums_to_square(Rank::First, File::D);
                let c1 = rank_file_enums_to_square(Rank::First, File::C);
                let b1 = rank_file_enums_to_square(Rank::First, File::B);

                let (d1_r, d1_f) = square_to_array_indices(d1);
                let (c1_r, c1_f) = square_to_array_indices(c1);
                let (b1_r, b1_f) = square_to_array_indices(b1);

                if self.squares[d1_r][d1_f].is_none() &&
                self.squares[c1_r][c1_f].is_none() &&
                self.squares[b1_r][b1_f].is_none() &&
                !self.is_square_attacked(king_sq, Color::Black) &&
                !self.is_square_attacked(d1, Color::Black) &&
                !self.is_square_attacked(c1, Color::Black) {

                    moves.push(Move::new_quiet(king_sq, c1));
                }
            }
        }
        if  piece_color == Color::Black {
            // --- Kingside ---
            if self.castling_kingside_black {
                let f8 = rank_file_enums_to_square(Rank::Eighth, File::F);
                let g8 = rank_file_enums_to_square(Rank::Eighth, File::G);

                let (f8_r, f8_f) = square_to_array_indices(f8);
                let (g8_r, g8_f) = square_to_array_indices(g8);

                if self.squares[f8_r][f8_f].is_none() &&
                self.squares[g8_r][g8_f].is_none() &&
                !self.is_square_attacked(king_sq, Color::White) &&
                !self.is_square_attacked(f8, Color::White) &&
                !self.is_square_attacked(g8, Color::White) {

                    moves.push(Move::new_quiet(king_sq, g8));
                }
            }

            // --- Queenside ---
            if self.castling_queenside_black {
                let d8 = rank_file_enums_to_square(Rank::Eighth, File::D);
                let c8 = rank_file_enums_to_square(Rank::Eighth, File::C);
                let b8 = rank_file_enums_to_square(Rank::Eighth, File::B);

                let (d8_r, d8_f) = square_to_array_indices(d8);
                let (c8_r, c8_f) = square_to_array_indices(c8);
                let (b8_r, b8_f) = square_to_array_indices(b8);

                if self.squares[d8_r][d8_f].is_none() &&
                self.squares[c8_r][c8_f].is_none() &&
                self.squares[b8_r][b8_f].is_none() &&
                !self.is_square_attacked(king_sq, Color::White) &&
                !self.is_square_attacked(d8, Color::White) &&
                !self.is_square_attacked(c8, Color::White) {

                    moves.push(Move::new_quiet(king_sq, c8));
                }
            }
        }
    }
    pub fn unmake_move(&mut self, mv: &Move, prev_state: &PreviousBoardState) {
        let (from_r, from_f) = square_to_array_indices(mv.from);
        let (to_r, to_f) = square_to_array_indices(mv.to);

        // Restore side to move FIRST
        self.active_color = !self.active_color;

        // Restore state
        self.castling_kingside_white = prev_state.castling_kingside_white;
        self.castling_queenside_white = prev_state.castling_queenside_white;
        self.castling_kingside_black = prev_state.castling_kingside_black;
        self.castling_queenside_black = prev_state.castling_queenside_black;
        self.en_passant_target = prev_state.previous_en_passant_target;
        self.halfmove_clock = prev_state.previous_halfmove_clock;
        self.fullmove_number = prev_state.previous_fullmove_number;
        self.game_result = prev_state.previous_game_result;

        let piece = match self.squares[to_r][to_f] {
            Some(p) => p,
            None => {
                tracing::error!(
                    "CRITICAL BUG: to square empty during undo\nmv={:?}\nfen={}",
                    mv,
                    self.to_fen_string()
                );
                return; // prevent crash so we can see bug
            }
        };

        // Handle castling FIRST
        if piece.kind == PieceKindEnum::King && (to_f as i8 - from_f as i8).abs() == 2 {
            let (rook_from, rook_to) = if to_f > from_f {
                (File::H.to_index(), File::F.to_index())
            } else {
                (File::A.to_index(), File::D.to_index())
            };

            let rook = self.squares[from_r][rook_to]
                .take()
                .expect("rook missing in undo castling");

            self.squares[from_r][rook_from] = Some(rook);
        }

        // Move piece back (handle promotion)
        let restored_piece = if mv.promotion.is_some() {
            ColoredPiece {
                kind: PieceKindEnum::Pawn,
                color: piece.color,
            }
        } 
        else {
                piece
            };

        self.squares[from_r][from_f] = Some(restored_piece);

        // Restore captured piece normally
        self.squares[to_r][to_f] = prev_state.captured_piece;

        // Handle EN PASSANT LAST (IMPORTANT)
        if restored_piece.kind == PieceKindEnum::Pawn
            && from_f != to_f
            && prev_state.captured_piece.is_none()
            && prev_state.previous_en_passant_target
                .map_or(false, |ep| mv.to == array_indices_to_square(ep.0, ep.1))
        {
            let cap_r = if restored_piece.color == Color::White {
                to_r + 1
            } else {
                to_r - 1
            };

            // DO NOT TOUCH self.squares[to_r][to_f] here
            self.squares[cap_r][to_f] = Some(ColoredPiece {
                kind: PieceKindEnum::Pawn,
                color: !restored_piece.color,
            });
        }
    }
}