
use crate::pieces::Piece;
use crate::color::Color;
use crate::board::Board;

pub fn piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 320,
        Piece::Bishop => 330,
        Piece::Rook => 500,
        Piece::Queen => 900,
        Piece::King => 0, 
    }
}

pub fn evaluate(board: &Board) -> i32 {
    let mut score = 0;

    for rank in 0..8 {
        for file in 0..8{
            if let Some(piece) = board.squares[rank][file] {
               let value = piece_value(piece.kind);

               if piece.color == Color::White {
                   score += value;
               } else {
                   score -= value;
               }
            }
        }
    }
    score
}

