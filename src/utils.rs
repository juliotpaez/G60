#[inline]
pub fn div_rem(dividend: usize, divisor: usize) -> (usize, usize) {
    (dividend / divisor, dividend % divisor)
}
