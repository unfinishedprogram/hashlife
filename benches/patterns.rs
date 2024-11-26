use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hashlife::life::Life;

pub fn clock_pattern(c: &mut Criterion) {
    let mut life = Life::from_rle(include_str!("../patterns/clock.rle"));

    for _ in 0..1000 {
        life.step();
    }

    c.bench_function("clock 1000-1001", |b| {
        b.iter(|| {
            let mut life = life.clone();
            life.step();
            black_box(life.root.alive())
        })
    });
}

criterion_group!(benches, clock_pattern);
criterion_main!(benches);
