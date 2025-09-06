pub fn bit_table () -> &'static [u32;64]{
    static  BIT_TABLE:[u32;64] = [
    0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63
    ];
    &BIT_TABLE
}

pub fn pop_bit(bb: &mut u64) -> i32 { 
    if *bb == 0 {
        return -1;
    }
    let lsb_index = bb.trailing_zeros() as i32;
    *bb &= *bb - 1; // Clear the LSB
    lsb_index
}

pub fn count_bits(mut b:u64) -> i32 {
    let mut r:i32 = 0;
    while b != 0 {
        r += 1;
        b &= b - 1;
    }
    return r;
}

