mod board;
mod pieces;
mod bitboard;

use board::Board;
use bitboard::{bit_table, count_bits, pop_bit, print_bitboard};
fn main () {
    let mut bitboard:u64 = 45;
    let board = Board::new();
    board.print();
    print_bitboard(100);
    count_bits(1);
    
    println!();
    let bit_index = pop_bit(&mut bitboard, &bit_table());

    println!("Bit index: {}", bit_index);

}