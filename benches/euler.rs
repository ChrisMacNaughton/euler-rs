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

fn p03(bench: &mut Bencher) {
    bench.iter(|| problems::p3())
}

fn p04(bench: &mut Bencher) {
    bench.iter(|| problems::p4())
}

fn p05(bench: &mut Bencher) {
    bench.iter(|| problems::p5())
}

fn p06(bench: &mut Bencher) {
    bench.iter(|| problems::p6())
}

benchmark_group!(
    benches,
    // p01,
    // p02,
    // p03,
    // p04,
    // p05,
    p06
);

benchmark_main!(benches);
