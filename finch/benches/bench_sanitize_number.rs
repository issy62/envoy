use criterion::{black_box, criterion_group, criterion_main, Criterion};
use finch::twilio::e164::*;

fn bench_normalizer(c: &mut Criterion) {
    c.bench_function("sanitize_number", |b| b.iter(|| normalize_number(black_box("8139927504"))));
}

criterion_group!(benches,bench_normalizer);
criterion_main!(benches);

