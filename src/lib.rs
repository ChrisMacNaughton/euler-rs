pub mod problems;
mod fibonacci;
mod maths;
mod primes;
mod reduce;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p01() {
        assert_eq!(problems::p1(), 233168);
    }

    #[test]
    fn p02() {
        assert_eq!(problems::p2(), 4613732);
    }

    #[test]
    fn p03() {
        assert_eq!(problems::p3(), 6857);
    }

    #[test]
    fn p04() {
        assert_eq!(problems::p4(), 906609);
    }

    #[test]
    fn p05() {
        assert_eq!(problems::p5(), 232792560);
    }

    #[test]
    fn p06() {
        assert_eq!(problems::p6(), 25164150);
    }

    #[test]
    fn p07() {
        assert_eq!(problems::p7(), 104743);
    }
}