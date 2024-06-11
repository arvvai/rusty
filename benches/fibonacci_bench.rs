extern crate criterion;
extern crate temp_converter;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use temp_converter::calc;

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("Fibonacci 20", |b| {
        b.iter(|| calc::fibonacci(black_box(20)))
    });
    c.bench_function("Fibonacci Recursive 20", |b| {
        b.iter(|| calc::fibonacci_recursive(black_box(20)))
    });
}

criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);
