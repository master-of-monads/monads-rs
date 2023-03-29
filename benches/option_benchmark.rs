use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use monads_rs::*;

fn is_even(v : u8) -> Option<u8> {
    if v % 2 == 0 {
        return Some(v);
    }
    else {
        return None;
    }
}

#[monadic]
fn monadic(v : u8) -> Option<u8> {
    let mut a = 0;
    a += is_even(v)?;
    a += is_even(v-2)?;
    a += is_even(v-4)?;
    a += is_even(v-8)?;
    a += is_even(v-16)?;
    a += is_even(v-32)?;
    a += is_even(v-64)?;
    a += is_even(v-128)?;
    a += is_even(v-10)?;
    a += is_even(v-12)?;
    Some(a)
}

fn non_monadic(v : u8) -> Option<u8> {
    let mut a = 0;
    a += is_even(v)?;
    a += is_even(v-2)?;
    a += is_even(v-4)?;
    a += is_even(v-8)?;
    a += is_even(v-16)?;
    a += is_even(v-32)?;
    a += is_even(v-64)?;
    a += is_even(v-128)?;
    a += is_even(v-10)?;
    a += is_even(v-12)?;
    Some(a)
}
pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Option");
    for i in [250].iter() {
        group.bench_with_input(BenchmarkId::new("Monadic Option",i), i, |b,i| b.iter(|| monadic(black_box(*i))));
        group.bench_with_input(BenchmarkId::new("Refrence Option",i), i, |b,i| b.iter(|| non_monadic(black_box(*i))));
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);