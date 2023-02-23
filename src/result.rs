use super::Monad;

impl<A, E> Monad for Result<A, E> {
	type Item = A;
	type Output<B> = Result<B, E>;

	fn ret<AA>(a: AA) -> Result<AA, E> {
		Result::Ok(a)
	}

	fn bind<B, F: FnOnce(A) -> Self::Output<B>>(self, f: F) -> Self::Output<B> {
		self.and_then(f)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ret;

	#[test]
	fn pure_test() {
		let val: Result<i32, &'static str> = ret::<_, Result<_, _>>(42);
		assert_eq!(val, Ok(42))
	}

	#[test]
	fn binding() {
		let val: Result<i32, &'static str> = ret::<_, Result<_, _>>(42);
		let result = val.bind(|v| ret::<_, Result<_, _>>(v + 1));
		assert_eq!(result, Ok(43))
	}

	#[test]
	fn short_curcuit() {
		fn meaning_of_life(v: i32) -> Result<i32, &'static str> {
			if v == 42 {
				ret::<_, Result<_, _>>(42)
			} else {
				return Err("That answer is not the meaning of life");
			}
		}

		let val: Result<i32, &'static str> = ret::<_, Result<_, _>>(43);
		let result = val.bind(meaning_of_life);
		assert!(result.is_err())
	}
}
