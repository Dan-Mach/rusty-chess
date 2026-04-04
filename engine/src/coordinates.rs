// src/coordinates.rs
use crate::rank::Rank;
use crate::file::File;
use crate::genmove::Square; 

pub fn square_to_rank_file_enums(square: Square) -> (Rank, File) {
    let rank_val = (square / 8) as usize;
    let file_val = (square % 8) as usize;

    (Rank::from_index(rank_val), File::from_index(file_val))
}

pub fn rank_file_enums_to_square(rank: Rank, file: File) -> Square {
    (rank.to_index() * 8 + file.to_index()) as Square
}

pub fn square_to_array_indices(square: Square) -> (usize, usize) {
    let rank_val = (square / 8) as usize ;
    let file_val = (square % 8) as usize; 

    let array_rank_idx = 7usize.saturating_sub(rank_val); 
    let array_file_idx = file_val as usize;

    (array_rank_idx, array_file_idx)
}

pub fn array_indices_to_square(array_rank_idx: usize, array_file_idx: usize) -> Square {
    let rank_val = (7 - array_rank_idx) as u8; 
    let file_val = array_file_idx as u8;

    rank_val * 8 + file_val
}