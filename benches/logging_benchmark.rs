use criterion::{
	black_box, criterion_group, criterion_main, BenchmarkId, Criterion,
};
use monads_rs::logging::Logging;
use monads_rs::*;

struct Logger {
	lines : Vec<String>,
}

impl Logger {
	pub fn log(&mut self, line : &str) {
		self.lines.push(line.to_string());
	}
}

fn non_monadic(v: usize, l : &mut Logger) -> usize {
	l.log("STARTING REACTION");
	let reaction_level = (v + 43) * 2;
	l.log(&format!("REACTION LEVEL: {}", reaction_level));
	l.log("STOPPING REACTION");
	reaction_level
}

#[monadic]
fn monadic(v : usize) -> Logging<usize> {
	Logging::log("STARTING REACTION")?;
	let reaction_level = (v + 43) * 2;
	Logging::log(&format!("REACTION LEVEL: {}", reaction_level))?;
	Logging::log("STOPPING REACTION")?;
	Logging::ret(reaction_level)
}

pub fn criterion_benchmark(c: &mut Criterion) {
	let mut group = c.benchmark_group("Logging");

	for i in [250].iter() {
		group.bench_with_input(
			BenchmarkId::new("Monadic Logging", i),
			i,
			|b, i| b.iter(|| monadic(*i)),
		);
		group.bench_with_input(
			BenchmarkId::new("Refrence Logging", i),
			i,
			|b, i| b.iter(|| non_monadic(*i,&mut Logger{lines:vec![]})),
		);
	}
	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
