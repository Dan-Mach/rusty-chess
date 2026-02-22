use engine::*;
pub fn get_piece_at_square(board: &Board, square: Square) -> Option<ColoredPiece> {
    let (arr_r, arr_f) = square_to_array_indices(square);
    board.squares[arr_r][arr_f]
}

pub fn undo_move(_board: &mut Board, game_move: &Move) {
    println!("Undoing move from {} to {}", game_move.from, game_move.to);

}

fn main() {

   
}