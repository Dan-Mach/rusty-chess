use crate::color:: Color;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Ord)]
//chess piecees
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,

}

pub const NUM_PIECES: usize = 6;
pub const ALL_PIECES: [Piece; NUM_PIECES] = [
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
];

pub const NUM_PROMOTION_PIECES: usize = 4;

pub const PROMOTION_PIECES: [Piece; 4] = [Piece::Queen, Piece::Knight, Piece::Rook, Piece::Bishop];

impl Piece {
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }
    #[inline]
    pub fn to_string(&self, color: Color) -> String {
        let piece = format!("{}", self);
        if color == Color::White { 
            piece.to_uppercase()
        } else {
            piece
        }
    }
    pub const MAJ_PCE:[Piece;3]  = [ 
        Piece::Queen, Piece::King, Piece::Rook
    ];
    pub const MIN_PCE:[Piece;3] = [
        Piece::Bishop, Piece::Knight, Piece::Pawn
    ];
    pub fn iter() -> impl Iterator<Item=Piece> {
        ALL_PIECES.iter().copied()
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Piece::Pawn => "p",
                Piece::Knight => "n",
                Piece::Bishop => "b",
                Piece::Rook => "r",
                Piece::Queen => "q",
                Piece::King => "k",
            }
        )
    }
} 

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub struct ColoredPiece {
    pub kind: Piece,
    pub color: Color,
}
impl ColoredPiece {
    pub fn new(kind: Piece, color: Color) -> Self {
        ColoredPiece { kind, color }
    }

    pub fn to_char(&self) -> char {
        let mut c = match self.kind {
            Piece::Pawn => 'P',
            Piece::Knight => 'N',
            Piece::Bishop => 'B',
            Piece::Rook => 'R',
            Piece::Queen => 'Q',
            Piece::King => 'K',
        };
        if self.color == Color::Black {
            c = c.to_ascii_lowercase();
        }
        c
    }
}