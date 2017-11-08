use fibonacci::Fibonacci;

/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get
/// 3, 5, 6 and 9. The sum of these multiples is 23.
/// ///
/// Find the sum of all the multiples of 3 or 5 below 1000.
#[allow(dead_code)]
pub fn p1() -> u64 {
    let max = |i| {
        let mut agg = 0;
        let mut next = 0;
        loop {
            next += i;
            if next >= 1000 {
                break;
            }
            agg += next;
        }
        agg
    };
    max(3) + max(5) - max(15)
}

/// Each new term in the Fibonacci sequence is generated by adding the previous two terms.
/// By starting with 1 and 2, the first 10 terms will be:
///
/// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
///
/// By considering the terms in the Fibonacci sequence whose values do not exceed four million,
/// find the sum of the even-valued terms.
#[allow(dead_code)]
pub fn p2() -> u64 {
    let mut sum = 0;
    let fibs = Fibonacci {
        current: 1,
        last: 0,
    };
    for number in fibs {
        if number >= 4_000_000 {
            break;
        }
        if number % 2 == 0 {
            sum += number;
        }
    }
    sum
}
