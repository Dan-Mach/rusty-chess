use crate::board::Board; // Assuming Board struct is in board.rs
use crate::color::Color;
use crate::pieces::{Piece as PieceKindEnum, ColoredPiece};
use crate::genmove::{Move, MoveList, Square};
use crate::coordinates::{array_indices_to_square, square_to_array_indices, square_to_rank_file_enums, rank_file_enums_to_square};

impl Board {
    pub fn is_square_attacked(&self, target_sq: Square, attacker_color: Color) -> bool {
        // Iterate through all squares on the board to find pieces of `attacker_color`.
        for r_idx in 0..8 {
            for f_idx in 0..8 {
                if let Some(piece_on_square) = self.squares[r_idx][f_idx] {
                    if piece_on_square.color == attacker_color {
                        let attacker_current_sq = array_indices_to_square(r_idx, f_idx);
                        
                        // Check if this specific piece attacks `target_sq`
                        // This requires checking based on the piece_on_square.kind
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
                                if self.is_rook_attacking(attacker_current_sq, target_sq) || 
                                   self.is_bishop_attacking(attacker_current_sq, target_sq) {
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

        // Check two diagonal forward squares
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

            loop { // Loop along one direction (ray)
                current_rank_val += *dr;
                current_file_val += *df;

                // Boundary check
                if !(current_rank_val >= 0 && current_rank_val <= 7 &&
                     current_file_val >= 0 && current_file_val <= 7) {
                    break; // Off board, stop this direction
                }

                let current_iter_sq: Square = (current_rank_val as u8 * 8 + current_file_val as u8) as Square;

                if current_iter_sq == target_sq {
                    return true; // The piece attacks the target square along this ray
                }

                // If the current iterated square is not the target,
                // check if it's occupied. If so, the path is blocked.
                let (arr_r, arr_f) = square_to_array_indices(current_iter_sq);
                if self.squares[arr_r][arr_f].is_some() {
                    break; // Path is blocked by another piece (friendly or enemy)
                           // so target_sq (if further) is not attacked along this ray.
                }
                // If square is empty and not the target, continue sliding
            }
        }
        false // Target square was not attacked along any of the provided directions
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
    pub fn generate_legal_moves(&self) -> MoveList {
        let mut moves = MoveList::new();
        let player_color = self.active_color;

        for r_idx in 0..8 {
            for f_idx in 0..8 {
                if let Some(piece_on_square) = self.squares[r_idx][f_idx] {
                    if piece_on_square.color == player_color {
                        let from_square: Square = array_indices_to_square(r_idx, f_idx);
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

    // --- Piece-Specific Move Generation Helper Methods (Private to this module or crate) ---
    // (These were defined previously, they would all be moved here)

    pub(crate) fn generate_pawn_moves(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList) {
        // ... (implementation as previously discussed) ...
        // Ensure PROMOTION_PIECES is accessible, e.g., use crate::pieces::PROMOTION_PIECES
        let (from_rank_enum, from_file_enum) = square_to_rank_file_enums(from_sq);
        let from_rank_val = from_rank_enum.to_index() as i8;
        let from_file_val = from_file_enum.to_index() as i8;

        let forward_delta_rank: i8;
        let start_rank_val: i8;
        let promotion_rank_val: i8;

        if piece_color == Color::White {
            forward_delta_rank = 1;
            start_rank_val = crate::rank::Rank::Second.to_index() as i8;
            promotion_rank_val = crate::rank::Rank::Eighth.to_index() as i8;
        } else {
            forward_delta_rank = -1;
            start_rank_val = crate::rank::Rank::Seventh.to_index() as i8;
            promotion_rank_val = crate::rank::Rank::First.to_index() as i8;
        }

        let add_pawn_move = |current_from_sq: Square, target_rank_val: i8, target_file_val: i8, moves_list: &mut MoveList| {
            if target_rank_val >=0 && target_rank_val <= 7 && target_file_val >=0 && target_file_val <=7 {
                let target_sq = (target_rank_val as u8 * 8 + target_file_val as u8) as Square;
                if target_rank_val == promotion_rank_val {
                    for &promo_piece_kind in crate::pieces::PROMOTION_PIECES.iter() {
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
                add_pawn_move(from_sq, one_step_fwd_rank_val, from_file_val, moves);

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
                let (target_arr_r_cap, target_arr_f_cap) = square_to_array_indices(target_capture_sq);

                if let Some(piece_on_target) = self.squares[target_arr_r_cap][target_arr_f_cap] {
                    if piece_on_target.color != piece_color {
                        add_pawn_move(from_sq, target_capture_rank_val, target_capture_file_val, moves);
                    }
                }
            }
        }
        
        if let Some(ep_target_sq_indices) = self.en_passant_target {
            let ep_target_sq = array_indices_to_square(ep_target_sq_indices.0, ep_target_sq_indices.1);
            let (ep_rank_enum, ep_file_enum) = square_to_rank_file_enums(ep_target_sq);
            let ep_rank_val = ep_rank_enum.to_index() as i8;

            let can_ep_from_this_rank = if piece_color == Color::White {
                from_rank_val == crate::rank::Rank::Fifth.to_index() as i8 
            } else {
                from_rank_val == crate::rank::Rank::Fourth.to_index() as i8
            };

            if can_ep_from_this_rank {
                for df_capture in [-1i8, 1i8].iter() {
                    let potential_ep_capture_rank_val = from_rank_val + forward_delta_rank;
                    let potential_ep_capture_file_val = from_file_val + *df_capture;

                    if potential_ep_capture_rank_val == ep_rank_val && 
                       potential_ep_capture_file_val == ep_file_enum.to_index() as i8 {
                        moves.push(Move::new_quiet(from_sq, ep_target_sq));
                        break;
                    }
                }
            }
        }
    }

    pub(crate) fn generate_knight_moves(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList) {
        // ... (implementation as previously discussed) ...
        const KNIGHT_OFFSETS: [(i8, i8); 8] = [
            (1, 2), (1, -2), (-1, 2), (-1, -2),
            (2, 1), (2, -1), (-2, 1), (-2, -1),
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

    pub(crate) fn generate_bishop_moves(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList) {
        // ... (implementation as previously discussed) ...
        const BISHOP_DIRECTIONS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
        let (from_rank_enum, from_file_enum) = square_to_rank_file_enums(from_sq);
        let initial_rank_val = from_rank_enum.to_index() as i8;
        let initial_file_val = from_file_enum.to_index() as i8;
        for (dr, df) in BISHOP_DIRECTIONS.iter() {
            let mut current_rank_val = initial_rank_val;
            let mut current_file_val = initial_file_val;
            loop {
                current_rank_val += dr;
                current_file_val += df;
                if current_rank_val >= 0 && current_rank_val <= 7 &&
                   current_file_val >= 0 && current_file_val <= 7 {
                    let target_sq = (current_rank_val as u8 * 8 + current_file_val as u8) as Square;
                    let (target_arr_r, target_arr_f) = square_to_array_indices(target_sq);
                    match self.squares[target_arr_r][target_arr_f] {
                        None => { moves.push(Move::new_quiet(from_sq, target_sq)); }
                        Some(p) => { if p.color != piece_color { moves.push(Move::new_quiet(from_sq, target_sq)); } break; }
                    }
                } else { break; }
            }
        }
    }

    pub(crate) fn generate_rook_moves(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList) {
        // ... (implementation as previously discussed) ...
        const ROOK_DIRECTIONS: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let (from_rank_enum, from_file_enum) = square_to_rank_file_enums(from_sq);
        let initial_rank_val = from_rank_enum.to_index() as i8;
        let initial_file_val = from_file_enum.to_index() as i8;
        for (dr, df) in ROOK_DIRECTIONS.iter() {
            let mut current_rank_val = initial_rank_val;
            let mut current_file_val = initial_file_val;
            loop {
                current_rank_val += dr;
                current_file_val += df;
                if current_rank_val >= 0 && current_rank_val <= 7 &&
                   current_file_val >= 0 && current_file_val <= 7 {
                    let target_sq = (current_rank_val as u8 * 8 + current_file_val as u8) as Square;
                    let (target_arr_r, target_arr_f) = square_to_array_indices(target_sq);
                    match self.squares[target_arr_r][target_arr_f] {
                        None => { moves.push(Move::new_quiet(from_sq, target_sq)); }
                        Some(p) => { if p.color != piece_color { moves.push(Move::new_quiet(from_sq, target_sq)); } break; }
                    }
                } else { break; }
            }
        }
    }

    pub(crate) fn generate_queen_moves(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList) {
        // ... (implementation as previously discussed, combining rook and bishop directions) ...
        const QUEEN_DIRECTIONS: [(i8, i8); 8] = [
            (0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1),
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
                if current_rank_val >= 0 && current_rank_val <= 7 &&
                   current_file_val >= 0 && current_file_val <= 7 {
                    let target_sq = (current_rank_val as u8 * 8 + current_file_val as u8) as Square;
                    let (target_arr_r, target_arr_f) = square_to_array_indices(target_sq);
                    match self.squares[target_arr_r][target_arr_f] {
                        None => { moves.push(Move::new_quiet(from_sq, target_sq)); }
                        Some(p) => { if p.color != piece_color { moves.push(Move::new_quiet(from_sq, target_sq)); } break; }
                    }
                } else { break; }
            }
        }
    }

    pub(crate) fn generate_king_moves(&self, from_sq: Square, piece_color: Color, moves: &mut MoveList) {
        // ... (implementation as previously discussed, including basic castling logic) ...
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
                let target_sq = (target_rank_val as u8 * 8 + target_file_val as u8) as Square;
                let (target_arr_r, target_arr_f) = square_to_array_indices(target_sq);
                match self.squares[target_arr_r][target_arr_f] {
                    None => { moves.push(Move::new_quiet(from_sq, target_sq)); }
                    Some(p) => { if p.color != piece_color { moves.push(Move::new_quiet(from_sq, target_sq)); } }
                }
            }
        }
        // Castling
        let home_array_rank = if piece_color == Color::White { 7 } else { 0 };
        let can_castle_ks = if piece_color == Color::White { self.castling_kingside_white } else { self.castling_kingside_black };
        if can_castle_ks && self.squares[home_array_rank][5].is_none() && self.squares[home_array_rank][6].is_none() {
            // TODO: Check for attacks on squares king passes through
            moves.push(Move::new_quiet(from_sq, array_indices_to_square(home_array_rank, 6)));
        }
        let can_castle_qs = if piece_color == Color::White { self.castling_queenside_white } else { self.castling_queenside_black };
        if can_castle_qs && self.squares[home_array_rank][1].is_none() && self.squares[home_array_rank][2].is_none() && self.squares[home_array_rank][3].is_none() {
            // TODO: Check for attacks on squares king passes through
            moves.push(Move::new_quiet(from_sq, array_indices_to_square(home_array_rank, 2)));
        }
    }
}