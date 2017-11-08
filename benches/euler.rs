#[macro_use]
extern crate bencher;
extern crate euler;

use euler::problems;
use bencher::Bencher;

fn p01(bench: &mut Bencher) {
    bench.iter(|| problems::p1())
}

fn p02(bench: &mut Bencher) {
    bench.iter(|| problems::p2())
}

benchmark_group!(
    benches,
    p01,
    p02
);
benchmark_main!(benches);
