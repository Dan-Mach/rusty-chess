use crate::pieces::{Piece, NUM_PIECES};
use crate::color::{Color, NUM_COLORS};
use crate::bitboard;
use crate::rank::Rank;
use crate::file::File;
use std::str::FromStr;


#[allow(unused_imports)]
use crate::genmove;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Board {
    squares:[[Option<Piece>;8]; 8],
    pieces: [u64; NUM_PIECES],
    color_combined: [u64; NUM_COLORS],
    combined: u64,
    side_to_move: Color,
    en_passant_target_square: Option<genmove::Square>,

}

impl Board {
    pub fn generate_moves(&self) -> genmove::MoveList {
        let mut moves = genmove::MoveList::new();
        let current_player = self.side_to_move;

        // Iterate through all pieces of the current player
        // For now, let's specifically target pawns
        self.generate_pawn_moves(&mut moves, current_player);
        // self.generate_knight_moves(&mut moves, current_player);
        // self.generate_bishop_moves(&mut moves, current_player);
        // self.generate_rook_moves(&mut moves, current_player);
        // self.generate_queen_moves(&mut moves, current_player);
        // self.generate_king_moves(&mut moves, current_player);

        moves
    }
    fn generate_pawn_moves(&self, moves: &mut genmove::MoveList, player: Color) {
        if player != Color::White {
            // We'll handle black pawns later or in a separate block
            return;
        }

        let white_pawns_bb = self.pieces[Piece::Pawn.to_index()] & self.color_combined[Color::White.to_index()];
        let opponent_pieces_bb = self.color_combined[Color::Black.to_index()];
        let all_occupied_bb = self.combined;

        // --- Pre-calculate file masks to prevent wrap-around captures ---
        const NOT_A_FILE: u64 = 0xfefefefefefefefe; // ~ (File A bitboard)
        const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f; // ~ (File H bitboard)

        let mut pawns_to_process = white_pawns_bb;
        while pawns_to_process != 0 {
            let from_square = bitboard::pop_bit(&mut pawns_to_process) as genmove::Square;
            let current_rank_index = from_square / 8; // 0-indexed rank (0=Rank1, 1=Rank2, ..., 7=Rank8)

            // 1. Single Pawn Push (White)
            let single_push_target = from_square + 8;
            if single_push_target < 64 { // Ensure it's on the board
                if (all_occupied_bb & (1u64 << single_push_target)) == 0 { // If target square is empty
                    if current_rank_index == 6 { // Pawn is on Rank 7, pushing to Rank 8 (promotion)
                        moves.push(genmove::Move::new_promotion(from_square, single_push_target, Piece::Queen));
                        moves.push(genmove::Move::new_promotion(from_square, single_push_target, Piece::Rook));
                        moves.push(genmove::Move::new_promotion(from_square, single_push_target, Piece::Bishop));
                        moves.push(genmove::Move::new_promotion(from_square, single_push_target, Piece::Knight));
                    } else {
                        moves.push(genmove::Move::new_quiet(from_square, single_push_target));
                    }

                    // 2. Double Pawn Push (White) - Can only happen if single push is also possible
                    if current_rank_index == 1 { // Pawn is on Rank 2 (starting rank for white)
                        let double_push_target = from_square + 16;
                        if (all_occupied_bb & (1u64 << double_push_target)) == 0 { // If double target is empty
                            // This move would set an en passant target square for the next turn.
                            // We'll add that flag/detail to the Move struct later.
                            moves.push(genmove::Move::new_quiet(from_square, double_push_target));
                        }
                    }
                }
            }

            // 3. Pawn Captures (White)
            // Capture Left (from white's perspective: e.g., d4 to c5, from_square + 7)
            if (from_square & 7) != 0 { // Check if not on A-File (file index 0)
                let capture_left_target = from_square + 7;
                if capture_left_target < 64 { // Ensure it's on the board
                    if (opponent_pieces_bb & (1u64 << capture_left_target)) != 0 { // If opponent piece is on target
                        if current_rank_index == 6 { // Pawn on Rank 7, capturing onto Rank 8 (promotion)
                            moves.push(genmove::Move::new_promotion(from_square, capture_left_target, Piece::Queen));
                            moves.push(genmove::Move::new_promotion(from_square, capture_left_target, Piece::Rook));
                            // ... Bishop, Knight promotions
                        } else {
                            moves.push(genmove::Move::new_quiet(from_square, capture_left_target));
                        }
                    }
                    // TODO: Add En Passant capture check for capture_left_target
                    // if capture_left_target == self.en_passant_target_square { ... }
                }
            }

            // Capture Right (from white's perspective: e.g., d4 to e5, from_square + 9)
            if (from_square & 7) != 7 { // Check if not on H-File (file index 7)
                let capture_right_target = from_square + 9;
                if capture_right_target < 64 { // Ensure it's on the board
                    if (opponent_pieces_bb & (1u64 << capture_right_target)) != 0 { // If opponent piece is on target
                        if current_rank_index == 6 { // Pawn on Rank 7, capturing onto Rank 8 (promotion)
                            moves.push(genmove::Move::new_promotion(from_square, capture_right_target, Piece::Queen));
                            moves.push(genmove::Move::new_promotion(from_square, capture_right_target, Piece::Rook));
                            // ... Bishop, Knight promotions
                        } else {
                            moves.push(genmove::Move::new_quiet(from_square, capture_right_target));
                        }
                    }
                    // TODO: Add En Passant capture check for capture_right_target
                    // if capture_right_target == self.en_passant_target_square { ... }
                }
            }
        }
        // --- End of White Pawn Move Generation ---
    }
    pub fn parse_fen(fen: &str) -> Self {
        let mut pieces = [0; NUM_PIECES];
        let mut color_combined = [0; NUM_COLORS];
        let mut combined:u64 = 0;
        let mut side_to_move = Color::White;
        let mut en_passant_target_square: Option<genmove::Square> = None;
        let mut squares = [[None; 8]; 8];
        let sections: Vec<&str> = fen.split_whitespace().collect();
        let ranks:Vec<&str> =sections[0].split('/').collect();
    
        for (rank_index, &rank) in ranks.iter().enumerate() {
            let mut file = 0;
            for c in rank.chars() {
                if let Some(d) = c.to_digit(10) {
                    file += d as usize;
                }
                else {
                    let piece = match c {
                        'P' => Some((Piece::Pawn, Color::White)),
                        'p' => Some((Piece::Pawn, Color::Black)),
                        'N' => Some((Piece::Knight, Color::White)),
                        'n' => Some((Piece::Knight, Color::Black)),
                        'B' => Some((Piece::Bishop, Color::White)),
                        'b' => Some((Piece::Bishop, Color::Black)),
                        'R' => Some((Piece::Rook, Color::White)),
                        'r' => Some((Piece::Rook, Color::Black)),
                        'Q' => Some((Piece::Queen, Color::White)),
                        'q' => Some((Piece::Queen, Color::Black)),
                        'K' => Some((Piece::King, Color::White)),
                        'k' => Some((Piece::King, Color::Black)),
                        _=> None,
                    };
    
                    if let Some((p, color)) = piece {
                        let square = (7 - rank_index) * 8 + file;
                        pieces[p.to_index()] |= 1 << square;
                        color_combined[color.to_index()] |= 1 << square;
                        combined |= 1 << square;
                        squares[7 - rank_index][file] = Some(p);
                    }
                    file += 1;
                }
            }
        }
        if sections[1] == "b" {
            side_to_move = Color::Black;
        }
        if sections.len() > 3 && sections[3] != "-" {
            let ep_str = sections[3];
            if ep_str.len() == 2 {
                if let (Ok(ep_file), Ok(ep_rank)) = 
                    (File::from_str(&ep_str[0..1]), Rank::from_str(&ep_str[1..2])) {
                        en_passant_target_square = Some((ep_rank.to_index() * 8 + ep_file.to_index()) as genmove::Square);
                }
            }
        }
        Board {
            squares,
            pieces,
            color_combined,
            combined,
            side_to_move,
            en_passant_target_square,
        }
    }

    pub fn new(_fen: &str) -> Self {
        let mut board = Board {
            squares: [[None;8]; 8 ],
            pieces: [0; NUM_PIECES],
            color_combined: [0; NUM_COLORS],
            combined: 0,
            side_to_move: Color::White,
            en_passant_target_square: None,
        };

        for file in 0..8 {
            board.squares[1][file] = Some(Piece::Pawn);
            board.squares[6][file] = Some(Piece::Pawn);
        }
        

        let back_rank = [
            Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen, 
            Piece::King, Piece::Bishop, Piece::Knight, Piece::Rook
        ];
       
        for( file, &piece) in back_rank.iter().enumerate() {
            board.squares[0][file] = Some(piece);
            board.squares[7][file] = Some(piece);
        }

        board
    }
    /// ```
    /// use engine::Board;
    /// let fen = "rnbqkbnr/pppppppp/8/8/4p3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1 ";
    /// 
    /// let board = Board::parse_fen(fen);
    /// board.print(fen);
    /// ```

    pub fn print(&self , _fen: &str) {
        for rank in (0..8).rev() {
            print!("{:3}", rank + 1);
            print!("  ");
            for file in 0..8 {
                match self.squares[rank][file] {
                    Some(pieces) => {
                        let color = if rank < 2 {Color::White}
                        else {Color::Black};
                        print!("{:4}", pieces.to_string(color));
                    },
                    None => print!(".   "),
                }
            }
            println!();
        }
        println!("     _____________________________");
        println!("     a   b   c   d   e   f   g   h");
        //for file in File::iter() {
         ///////print!("    {}", file.)
        //}
    }
}
