#[inline]
pub(crate) fn ascii_to_digit(c: u8) -> u32 {
    (c - b'0') as u32
}
