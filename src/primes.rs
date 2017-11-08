
pub fn sieve(is_prime: &mut [bool]) {
    is_prime[0] = false;
    is_prime[1] = false;

    let l = is_prime.len();
    let sqrt = (l as f64).sqrt() as usize;

    let mut i = 2;
    let mut not_prime = i * i;
    while not_prime < l {
        is_prime[not_prime] = false;
        not_prime += i;
    }

    i = 3;
    while i < sqrt + 1 {
        if is_prime[i] {
            let mut not_prime = i * i;
            while not_prime < l {
                is_prime[not_prime] = false;
                not_prime += 2 * i;
            }
        }
        i += 2;
    }
}