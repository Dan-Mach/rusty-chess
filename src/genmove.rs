use crate::pieces::Piece;

pub type Square  = u8;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<Piece>, 
}

impl Move {
    pub fn new(from: Square, to: Square) -> Self {
        Move { from, to, promotion: None }
    }

    pub fn new_quiet(from: Square, to: Square) -> Self {
        Move { from, to, promotion: None }
    }
    pub fn new_promotion(from: Square, to: Square, promotion_piece:Piece) -> Self {
        Move { from, to, promotion: Some(promotion_piece) }
    }
}

pub type MoveList = Vec<Move>;