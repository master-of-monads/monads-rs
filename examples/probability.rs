#![cfg(feature = "probability")]

use monads_rs::{probability::Probability, *};
use rand::rngs::ThreadRng;

fn main() {
	let value = program().run(rand::thread_rng());
	println!("value: {value}");
}

#[monadic]
fn program<'a>() -> Probability<'a, usize, ThreadRng> {
	let x: usize = Probability::rand()?;
	let y = Probability::rand()?;
	let z = Probability::rand()?;
	Probability::ret(x.overflowing_add(y).0.overflowing_add(z).0)
}
