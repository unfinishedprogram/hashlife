use std::io::Write;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hashlife::life::Life;

criterion_group!(bench_main, clock_pattern, clock_render);
criterion_main!(bench_main);

pub fn clock_pattern(c: &mut Criterion) {
    let mut life = Life::from_rle(include_str!("../patterns/clock.rle"));

    c.bench_function("clock step", |b| {
        b.iter(|| {
            life.step();
            black_box(life.root.alive())
        })
    });
}

pub fn clock_render(c: &mut Criterion) {
    let life = Life::from_rle(include_str!("../patterns/clock.rle"));

    let mut viewer = hashlife::rendering::life_viewer::LifeViewer::new((80, 24), life);

    c.bench_function("clock rendering ", |b| {
        b.iter(|| {
            viewer.render(black_box(&mut StubWriter));
        })
    });
}

pub struct StubWriter;
impl Write for StubWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
