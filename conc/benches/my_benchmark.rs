use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gol_conc::Strategy;
use gol_lib::Field;
use std::collections::HashSet;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("gol 1000 1000 1000", |b| {
        let field = Field::random(width, height);

        let mut strategy = Strategy::new(field.clone());
        b.iter(|| strategy.next());
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
