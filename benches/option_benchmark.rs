use criterion::{
	black_box, criterion_group, criterion_main, BenchmarkId, Criterion,
};
use monads_rs::*;

fn is_even(v: u8) -> Option<u8> {
	if v % 2 == 0 {
		return Some(v);
	} else {
		return None;
	}
}

#[monadic]
fn monadic(v: u8) -> Option<u8> {
	let a = is_even(v)?;
	let b = is_even(v)?;
	let c = is_even(v)?;
	let result = is_even(a + b + c)?;
	let a = is_even(v)?;
	let b = is_even(v)?;
	let c = is_even(v)?;
	let result = is_even(a + b + c)?;
	let a = is_even(v)?;
	let b = is_even(v)?;
	let c = is_even(v)?;
	let result = is_even(a + b + c)?;
	Option::ret(result)
}

fn non_monadic(v: u8) -> Option<u8> {
	let a = is_even(v)?;
	let b = is_even(v)?;
	let c = is_even(v)?;
	let result = is_even(a + b + c)?;
	let a = is_even(v)?;
	let b = is_even(v)?;
	let c = is_even(v)?;
	let result = is_even(a + b + c)?;
	let a = is_even(v)?;
	let b = is_even(v)?;
	let c = is_even(v)?;
	let result = is_even(a + b + c)?;
	Option::ret(result)
}

pub fn criterion_benchmark(c: &mut Criterion) {
	let mut group = c.benchmark_group("Option");
	for i in [250].iter() {
		group.bench_with_input(
			BenchmarkId::new("Monadic Option", i),
			i,
			|b, i| b.iter(|| monadic(black_box(*i))),
		);
		group.bench_with_input(
			BenchmarkId::new("Refrence Option", i),
			i,
			|b, i| b.iter(|| non_monadic(black_box(*i))),
		);
	}
	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
