use criterion::{criterion_group, criterion_main, Criterion};
use gol_conc::Strategy;
use gol_lib::Field;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("gol-conc 1000 1000", |b| {
        let field = Field::random(1000, 1000);

        let mut strategy = Strategy::new(field);
        b.iter(|| strategy.next());
    });
    c.bench_function("gol-conc 500 500", |b| {
        let field = Field::random(500, 500);

        let mut strategy = Strategy::new(field);
        b.iter(|| strategy.next());
    });
    c.bench_function("gol-conc 1000 500", |b| {
        let field = Field::random(1000, 500);

        let mut strategy = Strategy::new(field);
        b.iter(|| strategy.next());
    });
    c.bench_function("gol-conc 500 1000", |b| {
        let field = Field::random(500, 1000);

        let mut strategy = Strategy::new(field);
        b.iter(|| strategy.next());
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
