use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use fastsqrt::*;

fn bench_invs_sqrts(c: &mut Criterion) {
    let mut group = c.benchmark_group("invssqrt");
    /*group.bench_function("fast inv sqrt 25", |b| b.iter(|| fast_inv_sqrt(black_box(25.0))));
    group.bench_function("internal inv sqrt 25", |b| b.iter(|| normal_inv_sqrt(black_box(25.0))));
    group.bench_function("norm inv sqrt 25", |b| b.iter(|| internal_inv_sqrt(black_box(25.0))));*/
    
    for i in 20..26 {
        group.bench_with_input(BenchmarkId::new("fast inv sqrt 32 ", i), &(i as f32), |b, i| b.iter(|| fast_inv_sqrt_32(black_box(*i))));
        group.bench_with_input(BenchmarkId::new("fast inv sqrt 64 ", i), &(i as f64), |b, i| b.iter(|| fast_inv_sqrt_64(black_box(*i))));
        group.bench_with_input(BenchmarkId::new("internal inv sqrt ", i), &(i as f64), |b, i| b.iter(|| normal_inv_sqrt(black_box(*i))));
        group.bench_with_input(BenchmarkId::new("normal inv sqrt ", i), &(i as f64), |b, i| b.iter(|| internal_inv_sqrt(black_box(*i))));
    }

    group.finish();
}

criterion_group!(benches, bench_invs_sqrts);
criterion_main!(benches);