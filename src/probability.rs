use super::Monad;
use rand::{distributions::Standard, prelude::Distribution, Rng};

pub struct Probability<'f, T, R> {
	run_generator: Box<dyn FnOnce(R) -> T + 'f>,
}

impl<A, R: Rng> Probability<'_, A, R> {
	pub fn run(self, rng: R) -> A {
		(self.run_generator)(rng)
	}
}

impl<A, R: Rng> Probability<'static, A, R>
where
	Standard: Distribution<A>,
{
	pub fn rand() -> Self {
		Probability {
			run_generator: Box::new(|mut rng| rng.gen()),
		}
	}
}

impl<'a, A: 'a, R: Rng + Clone + 'static> Monad<'a, A>
	for Probability<'a, A, R>
{
	type Bind<B> = Probability<'a, B, R>;

	fn bind<B, F: FnMut(A) -> Self::Bind<B> + 'a>(
		self,
		mut f: F,
	) -> Self::Bind<B> {
		Probability {
			run_generator: Box::new(move |rng| {
				let first_value = self.run(rng.clone());
				f(first_value).run(rng)
			}),
		}
	}

	fn ret(a: A) -> Self {
		Probability {
			run_generator: Box::new(move |_| a),
		}
	}
}
