#![cfg(feature = "probability")]

use monads_rs::{probability::Probability, *};
use rand::{distributions::Standard, prelude::Distribution, rngs::ThreadRng};

fn main() {
	let count = 1000000;
	let value = throw_n_darts(count, rand::thread_rng());
	let pi = value * 4.0;
	println!("pi: {pi}");
}

fn throw_n_darts(n: usize, rng: ThreadRng) -> f64 {
	let mut hits = 0;
	for _ in 0..n {
		if throw_dart().run(rng.clone()) {
			hits += 1;
		}
	}
	hits as f64 / n as f64
}

#[monadic]
fn throw_dart() -> Probability<'static, bool, ThreadRng> {
	let coord: BoxCoordinate = Probability::rand()?;
	Probability::ret(coord.is_in_unit_circle())
}

#[derive(Debug)]
struct BoxCoordinate(f64, f64);

impl BoxCoordinate {
	fn is_in_unit_circle(&self) -> bool {
		self.0 * self.0 + self.1 * self.1 <= 1.0
	}
}

impl Distribution<BoxCoordinate> for Standard {
	fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> BoxCoordinate {
		BoxCoordinate(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
	}
}
