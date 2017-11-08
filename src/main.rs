extern crate euler;
extern crate time;

use time::PreciseTime;

use euler::problems;

fn time(name: &str, func: fn() -> u64) {
    let start = PreciseTime::now();
    let n = func();
    let end = PreciseTime::now();
    println!("{} ({}) answer: {}", name, start.to(end), n);
}

fn main() {
    let start = PreciseTime::now();

    time("p1", problems::p1);
    time("p2", problems::p2);
    time("p3", problems::p3);

    println!("Total {}", start.to( PreciseTime::now()));
}
