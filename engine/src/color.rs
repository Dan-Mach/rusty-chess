use std::ops::Not;
use crate::rank::Rank;

#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Color {
    White,
    Black,
}

pub const NUM_COLORS: usize = 2;
pub const ALL_COLOR: [Color;NUM_COLORS] = [Color::White, Color::Black];


    

impl Color {
    //white side
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }


    #[inline]
    pub fn from_index(i: usize) -> Color {
        match i % 2 {
            0 => Color::White,
            1 => Color::Black,
            _=> unreachable!(),
        }
    }

    pub fn iter() -> impl Iterator<Item = Color> {
        ALL_COLOR.iter().copied()
    }
    
    #[inline]
    pub fn to_my_backrank(&self) -> Rank {
        match *self {
            Color::White => Rank::First,
            Color::Black => Rank::Eighth,
        }
        
    }
    //black side
    #[inline]
    pub fn to_their_backrank(&self) -> Rank {
        match *self {
            Color::White => Rank::Eighth,
            Color::Black => Rank::First,
        }
    }

    /// Convert a `Color` to my second rank, which represents the starting rank for my pawns.
    #[inline]
    pub fn to_second_rank(&self) -> Rank {
        match *self {
            Color::White => Rank::Second,
            Color::Black => Rank::Seventh,
        }
    }
       /// Convert a `Color` to my fourth rank, which represents the rank of my pawns when
    /// moving two squares forward.
    #[inline]
    pub fn to_fourth_rank(&self) -> Rank {
        match *self {
            Color::White => Rank::Fourth,
            Color::Black => Rank::Fifth,
        }
    }

    /// Convert a `Color` to my seventh rank, which represents the rank before pawn promotion.
    #[inline]
    pub fn to_seventh_rank(&self) -> Rank {
        match *self {
            Color::White => Rank::Seventh,
            Color::Black => Rank::Second,
        }
    }

}

impl Not for Color {
    type Output = Color;

    /// Get the other color.
    #[inline]
    fn not(self) -> Color {
        if self == Color::White {
            Color::Black
        } else {
            Color::White
        }
    }
}

