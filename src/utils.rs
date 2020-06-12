
pub fn char_to_i32(c: char) -> i32 {
    c.to_digit(10).unwrap() as i32
}