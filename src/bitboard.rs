use crate::rank::Rank; 
/// ```
/// use engine::bit_table;
/// let table = bit_table();
/// assert_eq!(table[0], 0);
/// assert_eq!(table[63], 63);
/// assert_eq!(table.len(), 64);
/// ```
pub fn bit_table () -> &'static [u32;64]{
    static  BIT_TABLE:[u32;64] = [
    0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63
    ];
    &BIT_TABLE
}

/// ```
/// use engine::pop_bit;
/// let mut bb = 0b10101; // bits at index 0, 2, 4. Decimal 21.
/// assert_eq!(pop_bit(&mut bb), 0); 
/// assert_eq!(bb, 0b10100);        
/// assert_eq!(pop_bit(&mut bb), 2); 
/// assert_eq!(bb, 0b10000);      
/// assert_eq!(pop_bit(&mut bb), 4); 
/// assert_eq!(bb, 0b0);          
/// assert_eq!(pop_bit(&mut bb), -1);
///
/// let mut bb_single = 1 << 63; 
/// assert_eq!(pop_bit(&mut bb_single), 63);
/// assert_eq!(bb_single, 0);  
///
/// let mut bb_empty = 0u64;
/// assert_eq!(pop_bit(&mut bb_empty), -1); 
/// ```
pub fn pop_bit(bb: &mut u64) -> i32 { 
    if *bb == 0 {
        return -1;
    }
    let lsb_index = bb.trailing_zeros() as i32;
    *bb &= *bb - 1; // Clear the LSB
    lsb_index
}

/// ```
/// use engine::count_bits;
/// assert_eq!(count_bits(0), 0);
/// assert_eq!(count_bits(0b1), 1);
/// assert_eq!(count_bits(0b10111), 4); // Indices 0,1, 2, 4 are set
/// assert_eq!(count_bits(u64::MAX), 64); // All bits set
/// let bb: u64 = (1 << 0) | (1 << 15) | (1 << 63); // A1, H2, H8
/// assert_eq!(count_bits(bb), 3);
/// let bb_octal: u64 = 0o14444444444444;
/// assert_eq!(count_bits(bb_octal), 14);
/// ```
pub fn count_bits(mut b:u64) -> i32 {
    let mut r:i32 = 0;
    while b != 0 {
        r += 1;
        b &= b - 1;
    }
    return r;
}


/// ```
/// use engine::print_bitboard;
/// use engine::Rank;
/// let bb_layout = (1 << 0)  | // A1 (square 0)
///                 (1 << 7)  | // H1 (square 7)
///                 (1 << 27) | // D4 (square 27) (Rank 3, File 3)
///                 (1 << 56) | // A8 (square 56) (Rank 7, File 0)
///                 (1 << 63);  // H8 (square 63) (Rank 7, File 7)
///
/// print_bitboard(bb_layout, "Layout Test:");
/// ```
pub fn print_bitboard(bb:u64, label:&str) {
    println!("{}", label);
    let  shiftme:u64 = 1;
    println!("\n"); // Produces two newlines (one from \n, one from println!)

    // This loop structure relies on Rank::iter() and casting Rank to i32
    // as it was in your original code.
    for rank in Rank::iter().rev() { // rank is of type crate::rank::Rank
        for file in 0..8 { // file is 0..7 (representing A-H)
            // Original calculation for square index:
            let sq = rank as i32 * 8 + file;
            // Ensure Rank enum can be cast to i32 (0 for First, 1 for Second, ..., 7 for Eighth)
            // This is true if Rank is defined like `#[repr(u8)] enum Rank { First = 0, ... }`

            if (shiftme << sq) & bb != 0 {
                print!( "   P");
            }
            else {
                print!( "   .");
            }
        }
        println!(); // Newline after each rank
    }
    println!(); // One more newline at the end
}