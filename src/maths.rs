#[inline]
pub fn gcd(a: u32, b: u32) -> u32 {
    if b != 0 {
        gcd(b,a%b)
    } else {
        a
    }
}

#[inline]
pub fn lcm(a: u32, b: u32) -> u32 {
    a / gcd(a, b) * b
}
