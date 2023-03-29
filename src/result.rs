use super::Monad;

impl<'a, A, E> Monad<'a, A> for Result<A, E> {
	type Bind<B> = Result<B, E>;

	fn bind<B, F: FnOnce(A) -> Self::Bind<B>>(self, f: F) -> Self::Bind<B> {
		self.and_then(f)
	}

	fn ret(val: A) -> Self {
		Ok(val)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn pure_test() {
		let val: Result<i32, &'static str> = Result::ret(42);
		assert_eq!(val, Ok(42))
	}

	#[test]
	fn binding() {
		let val: Result<i32, &'static str> = Result::ret(42);
		let result = val.bind(|v| Result::ret(v + 1));
		assert_eq!(result, Ok(43))
	}

	#[test]
	fn short_curcuit() {
		fn meaning_of_life(v: i32) -> Result<i32, &'static str> {
			if v == 42 {
				Result::ret(42)
			} else {
				return Err("That answer is not the meaning of life");
			}
		}

		let val: Result<i32, &'static str> = Result::ret(43);
		let result = val.bind(meaning_of_life);
		assert!(result.is_err())
	}
}
