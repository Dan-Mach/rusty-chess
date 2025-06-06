use crate::board::Board; // Assuming Board struct is in board.rs
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
                let (arr_r, arr_f) = square_to_array_indices(current_iter_sq);
                if self.squares[arr_r][arr_f].is_some() {
                    break; // Path is blocked by another piece (friendly or enemy)
                           // so target_sq (if further) is not attacked along this ray.
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
}