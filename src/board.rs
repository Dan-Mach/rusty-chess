use crate::pieces::{Piece, NUM_PIECES};
use crate::color::{self, Color, NUM_COLORS};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Board {
    squares:[[Option<Piece>;8]; 8],
    pieces: [u64; NUM_PIECES],
    color_combined: [u64; NUM_COLORS],
    combined: u64,
    side_to_move: Color,


}




impl Board {
    pub fn parse_fen(fen: &str) -> Self {
        let mut pieces = [0; NUM_PIECES];
        let mut color_combined = [0; NUM_COLORS];
        let mut combined = 0;
        let mut side_to_move = Color::White;
        
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
        Board {
            squares,
            pieces,
            color_combined,
            combined,
            side_to_move,
        }
        
    }
    
    /// ```
    /// let fen = "rnbqkbnr/pppppppp/8/8/4p3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1 ";
    /// let board = Board::parse_fen(fen);
    /// assert!(board.pieces[Piece::Pawn.to_index()] & (1 << 4) != 0); // Check if a pawn is on e4
    /// ```

    pub fn print(&self, _fen: &str) {
        for rank in (0..8).rev() {
            print!("{:3}  ", rank + 1);
            for file in 0..8 {
                match self.squares[rank][file] {
                    Some(piece) => {
                        let color = if self.color_combined[Color::White.to_index()] & (1 << (rank * 8 + file)) != 0 {
                            Color::White
                        } else {
                            Color::Black
                        };
                        print!(
                            "{:3} ",
                            piece.to_string(color)
                        );
                    }
                    None => {
                        print!(".   ");
                    }
                }
            }
            println!();
        }
        println!("     _____________________________");
        println!("     a   b   c   d   e   f   g   h");
    }

    
    
}
