
use crate::rank::Rank;

pub fn bit_table () -> &'static [u32;64]{
    static  BIT_TABLE:[u32;64] = [
    0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63

    ];
    
    &BIT_TABLE
}


pub fn pop_bit(bb: &mut u64, bit_table:&[u32;64]) -> i32{
    
     if *bb == 0 {
        return -1;
        }
    let fold: usize = (*bb & (!*bb + 1)).trailing_zeros() as usize;
    
    *bb &= *bb - 1;

    if fold < 64 {
        bit_table[fold] as i32
    }
    else {
        -1
    }
    
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop_bit() {
        let mut bb: u64 = 0b10010; // Binary representation
        let bit_table = bit_table();
        assert_eq!(pop_bit(&mut bb, bit_table), 1); // Pop the first bit (index 1)
        assert_eq!(bb, 0b10000); // Remaining bits
        assert_eq!(pop_bit(&mut bb, bit_table), 4); // Pop the next bit (index 4)
        assert_eq!(bb, 0); // No bits left
        assert_eq!(pop_bit(&mut bb, bit_table), -1); // No bits to pop
    }

    #[test]
    fn test_count_bits() {
        assert_eq!(count_bits(0b10101), 3); // 3 bits set
        assert_eq!(count_bits(0b11111111), 8); // 8 bits set
        assert_eq!(count_bits(0), 0); // No bits set
    }

    #[test]
    fn test_bit_shifting() {
        let mut bb: u64 = 0b1; // Start with the least significant bit set
        for i in 0..64 {
            assert_eq!(count_bits(bb), 1); // Only one bit should be set
            assert_eq!(bb, 1 << i); // Ensure the bit is in the correct position
            bb <<= 1; // Shift the bit to the left
        }
    }

    #[test]
    fn test_print_bitboard() {
        let bb: u64 = 0b100000001; // Bits set at positions 0 and 8
        print_bitboard(bb, "Test Bitboard");
        // Visually inspect the output to ensure correctness
    }
}
pub fn count_bits(mut b:u64) -> i32 {
    let mut r:i32 = 0;
    while b != 0 {
        r += 1;
        b &= b - 1;
    }
    return r;
}

pub fn print_bitboard(bb:u64, label:&str) {
    println!("{}", label);
    let  shiftme:u64 = 1;
    //let mut sq = 0;
    println!("\n");


    for rank in Rank::iter().rev() {
        for file in 0..8 {
            let sq = rank as i32* 8 + file;
            if (shiftme << sq) & bb != 0 {
                print!( "   P");
            }
            else {
                print!( "   .");
            }
        }
        println!();
    }
    println!();
}