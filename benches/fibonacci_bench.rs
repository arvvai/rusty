extern crate criterion;
extern crate rusty;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rusty::calc;

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("Fibonacci 20", |b| {
        b.iter(|| calc::fibonacci_loop(black_box(20)))
    });
    c.bench_function("Fibonacci Recursive 20", |b| {
        b.iter(|| calc::fibonacci_rec(black_box(20)))
    });
}

criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);
