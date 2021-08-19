use criterion::{criterion_group, criterion_main, Criterion};
use gol_lib::Field;
use gol_naive::Strategy;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("gol-naive 10000 10000", |b| {
        let field = Field::random(10000, 10000);

        let mut strategy = Strategy::new(field);
        b.iter(|| strategy.next());
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
