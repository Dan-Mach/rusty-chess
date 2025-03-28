use crate::pieces::{File, Pieces};
pub struct Board {
    pub squares:[[Option<Pieces>; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            squares: [[None;8]; 8],
        };

        for file in 0..8 {
            board.squares[1][file] = Some(Pieces::WhitePawn);
            board.squares[6][file] = Some(Pieces::BlackPawn);
        }

        let back_rank_white = [
            Pieces::WhiteRook,
            Pieces::WhiteKnight,
            Pieces::WhiteBishop,
            Pieces::WhiteQueen,
            Pieces::WhiteKing,
            Pieces::WhiteBishop,
            Pieces::WhiteKnight,
            Pieces::WhiteRook,
        ];

        let back_rank_black = [
            Pieces::BlackRook,
            Pieces::BlackKnight,
            Pieces::BlackBishop,
            Pieces::BlackQueen,
            Pieces::BlackKing,
            Pieces::BlackBishop,
            Pieces::BlackKnight,
            Pieces::BlackRook
        ];


        for file in 0..8 {
            board.squares[0][file] = Some(back_rank_white[file]);
            board.squares[7][file] = Some(back_rank_black[file]);
        }

        board
    }

    pub fn print(&self) {
        for rank in (0..8).rev() {
            print!("{:3}", rank + 1);
            print!("| ");
            for file in File::iter(){
                match self.squares[rank][file.index()] {
                    Some(pieces) => print!("{:3}", pieces.to_string()),
                    None => print!(" . "),
                }
            }
            println!();
        }
        println!("   ________________________________");
        //println!("      a   b   c   d   e   f   g   h");
        for file in File::iter() {
            print!("    {:1}", file.to_char())
        }
    }
}
