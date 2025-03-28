#[derive(Debug, Copy, Clone)]

pub enum Pieces{ WhitePawn, WhiteKnight, WhiteBishop, WhiteRook, WhiteQueen, WhiteKing,
             BlackPawn, BlackKnight, BlackBishop, BlackRook, BlackQueen, BlackKing,}
impl Pieces {
    pub fn to_string(self) -> &'static str{
        match self {
            Pieces::WhitePawn => "P",
            Pieces::WhiteKnight => "N",
            Pieces::WhiteBishop => "B",
            Pieces::WhiteRook => "R",
            Pieces::WhiteQueen => "Q",
            Pieces::WhiteKing => "K",
            Pieces::BlackPawn => "p",
            Pieces::BlackKnight => "n",
            Pieces::BlackBishop => "b",
            Pieces::BlackRook => "r",
            Pieces::BlackQueen => "q",
            Pieces::BlackKing => "k",
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub enum File { A, B, C, D, E, F, G, H, }
impl File {
    pub fn index(&self) -> usize {
        *self as usize
    }
    pub fn iter () -> impl
        Iterator<Item = File> {
        [
            File::A ,
            File::B ,
            File::C ,
            File::D ,
            File::E ,
            File::F ,
            File::G ,
            File::H ,
            
        ].iter().copied()
    }
}
impl File {
    pub fn to_char(self) -> char {
    match self {
        File::A => 'a',
        File::B => 'b',
        File::C => 'c',
        File::D => 'd',
        File::E => 'e',
        File::F => 'f',
        File::G => 'g',
        File::H => 'h',
    }
}
}

#[derive(Debug, Copy, Clone)]
pub enum Rank { One, Two, Three, Four, Five, Six, Seven, Eight, }
impl Rank {
    pub fn iter() -> impl 
        DoubleEndedIterator<Item = Rank> {
        vec![
            Rank::One,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
        ].into_iter()
    }
}


   