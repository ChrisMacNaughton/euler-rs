extern crate euler;
extern crate time;

use time::PreciseTime;

use euler::problems;

fn time(name: &str, func: fn() -> u64) {
    let start = PreciseTime::now();
    let n = func();
    let end = PreciseTime::now();
    println!("{} ({}ms) answer: {}", name, start.to(end) * 1000, n);
}

fn main() {
    // let start_all = PreciseTime::now();

    time("p1", problems::p1);
    time("p2", problems::p2);
}
