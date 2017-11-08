pub struct Fibonacci {
    pub current: u64,
    pub last: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.current = self.current + self.last;
        self.last = self.current - self.last;
        Some(self.current)
    }
}
