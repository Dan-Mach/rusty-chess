use crate::structures::Board;
use crate::data::PIECE_COUNTS;


pub fn print_board(board: &Board) {
    println!(" a b c d e f g h");
    println!("--------------------");

    for (i, row) in board.squares.iter().enumerate() {
        print!("{} |", 8-i);
        for &square in row.iter() {
            print!("{}", square);
        }
        println!("|");
    }
    println!("-------------------");
    println!("Piece count ");
    let piece_names = ["Pawns", "knights", "Bishops", "Rooks", "Queens", "Kings"];
    for (i, &count) in PIECE_COUNTS.iter().enumerate() {
        println!("{}: {}", piece_names[i], count);
    }
}

