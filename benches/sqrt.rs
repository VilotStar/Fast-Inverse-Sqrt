use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fastsqrt::*;

fn bench_invs_sqrts(c: &mut Criterion) {
    let mut group = c.benchmark_group("invssqrt");
    group.bench_function("fast inv sqrt 25", |b| b.iter(|| fast_inv_sqrt(black_box(25.0))));
    group.bench_function("norm inv sqrt 25", |b| b.iter(|| normal_inv_sqrt(black_box(25.0))));
    group.finish();
}

criterion_group!(benches, bench_invs_sqrts);
criterion_main!(benches);