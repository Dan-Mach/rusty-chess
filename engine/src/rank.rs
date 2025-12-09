use crate::error::Error;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Hash)]
#[repr(u8)]
pub enum Rank {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
    Fifth = 4,
    Sixth = 5,
    Seventh = 6,
    Eighth = 7,
}

pub const NUM_RANKS: usize = 8;
pub const ALL_RANKS: [Rank; NUM_RANKS] = [
    Rank::First,
    Rank::Second,
    Rank::Third,
    Rank::Fourth,
    Rank::Fifth,
    Rank::Sixth,
    Rank::Seventh,
    Rank::Eighth,
];
impl  Rank {
    pub fn iter() -> impl DoubleEndedIterator<Item = Rank> {
        ALL_RANKS.iter().copied()
    }
}

impl Rank {
    #[inline]
    pub fn from_index(i: usize) -> Rank {
        // match is optimized to no-op with opt-level=1 with rustc 1.53.0
        match i & 7 {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eighth,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn down(&self) -> Rank {
        Rank::from_index(self.to_index().wrapping_sub(1))
    }

    #[inline]
    pub fn up(&self) -> Rank {
        Rank::from_index(self.to_index() + 1)
    }

    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

impl FromStr for Rank {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 1 {
            return Err(Error::InvalidRank);
        }
        match s.chars().next().unwrap() {
            '1' => Ok(Rank::First),
            '2' => Ok(Rank::Second),
            '3' => Ok(Rank::Third),
            '4' => Ok(Rank::Fourth),
            '5' => Ok(Rank::Fifth),
            '6' => Ok(Rank::Sixth),
            '7' => Ok(Rank::Seventh),
            '8' => Ok(Rank::Eighth),
            _ => Err(Error::InvalidRank),
        }
    }
}

