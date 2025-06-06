use engine::*;
pub fn get_piece_at_square(board: &Board, square: Square) -> Option<ColoredPiece> {
    let (arr_r, arr_f) = square_to_array_indices(square);
    board.squares[arr_r][arr_f]
}

pub fn example_usage() {
    let sq: Square = 0; 
    let (arr_r, arr_f) = square_to_array_indices(sq);
    println!("Square {} maps to array[{}][{}]", sq, arr_r, arr_f);
    let (rank_enum, file_enum) = square_to_rank_file_enums(sq);
    println!("Square {} is Rank: {:?}, File: {:?}", sq, rank_enum, file_enum);
}

fn main () {
    example_usage();
}