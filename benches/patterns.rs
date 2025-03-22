use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hashlife::life::Life;

pub fn clock_pattern(c: &mut Criterion) {
    let mut life = Life::from_rle(include_str!("../patterns/clock.rle"));

    c.bench_function("clock", |b| {
        b.iter(|| {
            life.step();
            black_box(life.root.alive())
        })
    });
}

criterion_group!(benches, clock_pattern);
criterion_main!(benches);
