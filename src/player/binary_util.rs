pub fn get_segment(i: u32, mask: u32) -> u32 {
    (i & mask) >> mask.trailing_zeros()
}