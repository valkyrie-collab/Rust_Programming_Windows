fn get_bit_value(value: i32, srt_range: i32, end_range: i32) -> i32 {
    let bits: i32 = end_range - srt_range + 1;
    let shift_pos: i32 = value >> (end_range + 1 - bits);
    let mask: i32 = !(!0 << bits); 
    
    shift_pos & mask
}

fn main() {
    println!("{}", get_bit_value(26, 1, 3));
}
