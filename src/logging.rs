use super::Monad;

/// Simple logging monad.
pub struct Logging<A> {
	value: A,
	log: Vec<String>,
}

impl Logging<()> {
	/// Logs a singled message `msg`.
	pub fn log<M: ToString>(msg: M) -> Self {
		Self {
			value: (),
			log: vec![msg.to_string()],
		}
	}
}

impl<A> Logging<A> {
	/// Collects all logs and returns the computed value.
	pub fn run(&self) -> (&A, Vec<String>) {
		(&self.value, self.log.clone())
	}
}

impl<'a, A> Monad<'a, A> for Logging<A> {
	type Bind<B> = Logging<B>;

	/// Applies the function `f` to the value, making sure the logs are a
	/// continuation of the logs of `self`.
	fn bind<B, F: FnOnce(A) -> Self::Bind<B>>(mut self, f: F) -> Self::Bind<B> {
		let mut next = f(self.value);
		self.log.append(&mut next.log);
		next.log = self.log;
		next
	}

	fn ret(val: A) -> Self {
		Logging {
			value: val,
			log: vec![],
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Logs the path of the function, and returns a value.
	fn log_function(a: bool) -> Logging<i32> {
		Logging::log("Enter function").bind(|_| {
			if a {
				Logging::log("If statement A")
			} else {
				Logging::log("If statement B")
			}
			.bind(|_| {
				(1..4)
					.fold(Logging::ret(()), |b, i| {
						b.bind(|_| {
							Logging::log(format!("Logging iteration: {i}"))
						})
					})
					.bind(|_| Logging::ret(2))
			})
		})
	}

	#[test]
	fn false_test() {
		let f = log_function(false);
		let (res, log) = f.run();
		assert_eq!(*res, 2);
		assert_eq!(
			log,
			vec![
				"Enter function",
				"If statement B",
				"Logging iteration: 1",
				"Logging iteration: 2",
				"Logging iteration: 3",
			]
		);
	}

	#[test]
	fn true_test() {
		let f = log_function(true);
		let (res, log) = f.run();
		assert_eq!(*res, 2);
		assert_eq!(
			log,
			vec![
				"Enter function",
				"If statement A",
				"Logging iteration: 1",
				"Logging iteration: 2",
				"Logging iteration: 3",
			]
		);
	}
}
