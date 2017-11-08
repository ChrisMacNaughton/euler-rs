extern crate euler;
extern crate time;

use time::PreciseTime;

use euler::problems;

fn time<T: Into<u64>>(name: &str, func: fn() -> T) {
    let start = PreciseTime::now();
    let n = func();
    let end = PreciseTime::now();
    println!("{} ({}) answer: {}", name, start.to(end), n.into());
}

fn main() {
    let start = PreciseTime::now();

    time("p1", problems::p1);
    time("p2", problems::p2);
    time("p3", problems::p3);
    time("p4", problems::p4);
    

    println!("Total {}", start.to( PreciseTime::now()));
}
