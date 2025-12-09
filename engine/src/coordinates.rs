// src/coordinates.rs
use crate::rank::Rank;
use crate::file::File;
use crate::genmove::Square; // Assuming Square is u8 from genmove.rs

pub fn square_to_rank_file_enums(square: Square) -> (Rank, File) {
    let rank_val = (square / 8) as usize;
    let file_val = (square % 8) as usize;

    (Rank::from_index(rank_val), File::from_index(file_val))
}
/// Converts (Rank, File) enums to a Square (u8, 0-63).
pub fn rank_file_enums_to_square(rank: Rank, file: File) -> Square {
    (rank.to_index() * 8 + file.to_index()) as Square
}
//converts a Square to an array indices
pub fn square_to_array_indices(square: Square) -> (usize, usize) {
    let rank_val = (square / 8) as usize ;// 0 for 1st rank (Rank::First), 7 for 8th rank (Rank::Eighth)
    let file_val = (square % 8) as usize; // 0 for A-file, 7 for H-file

    let array_rank_idx = 7usize.saturating_sub(rank_val); 
    let array_file_idx = file_val as usize;

    (array_rank_idx, array_file_idx)
}
/// Converts array indices (where array_rank_idx = 0 is 8th rank) to a Square (u8, 0-63, A1=0).
pub fn array_indices_to_square(array_rank_idx: usize, array_file_idx: usize) -> Square {
    let rank_val = (7 - array_rank_idx) as u8; // Convert array rank to 0-indexed rank (0=Rank1)
    let file_val = array_file_idx as u8;

    rank_val * 8 + file_val
}