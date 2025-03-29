use crate::error::Error;
use std::str::FromStr;
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

pub const NUM_FILES: usize = 8;
pub const ALL_FILES:[File; NUM_FILES] = [
    File::A ,
    File::B ,
    File::C ,
    File::D ,
    File::E ,
    File::F ,
    File::G ,
    File::H ,
    
];

  

impl File {
    #[inline]
    pub fn from_index(i: usize) -> File {
        match i & 7 {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _=> unreachable!(),
        }
    }
    pub fn iter() -> impl Iterator<Item = File>  {
        ALL_FILES.iter().copied()
    }

    #[inline]
    pub fn left(&self) -> File {
        File::from_index(self.to_index().wrapping_sub(1))
    }

    #[inline]
    pub fn right(&self) -> File {
        File::from_index(self.to_index() + 1)
    }   

    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}


impl FromStr for File {

    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 1 {
            return Err(Error::InvalidFile);
        }
        match s.chars().next().unwrap() {
            'a' => Ok(File::A),
            'b' => Ok(File::B),
            'c' => Ok(File::C),
            'd' => Ok(File::D),
            'e' => Ok(File::E),
            'f' => Ok(File::F),
            'g' => Ok(File::G),
            'h' => Ok(File::H),
               _=> Err(Error::InvalidFile),
        }
    }
    
}

