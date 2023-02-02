use super::{Applicative, Functor, Monad};

impl<A, E> Functor<A> for Result<A, E> {
	type Map<B> = Result<B, E>;

	fn map<B, F: FnOnce(A) -> B>(self, f: F) -> Self::Map<B> {
		self.map(f)
	}
}

impl<A, E> Applicative<A> for Result<A, E> {
	type Apply<B> = Result<B, E>;

	fn pure(a: A) -> Self {
		Ok(a)
	}

	fn ap<B, F: FnOnce(A) -> B>(self, f: Self::Apply<F>) -> Self::Apply<B> {
		self.and_then(|a| f.map(|f| f(a)))
	}
}

impl<A, E> Monad<A> for Result<A, E> {
	type Bind<B> = Result<B, E>;

	fn bind<B, F: FnOnce(A) -> Self::Bind<B>>(self, f: F) -> Self::Bind<B> {
		self.and_then(f)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn pure_test() {
		let val: Result<i32, &'static str> = Result::pure(42);
		assert_eq!(val, Ok(42))
	}

	#[test]
	fn binding() {
		let val: Result<i32, &'static str> = Result::pure(42);
		let result = val.bind(|v| Result::pure(v + 1));
		assert_eq!(result, Ok(43))
	}

	#[test]
	fn short_curcuit() {
		fn meaning_of_life(v: i32) -> Result<i32, &'static str> {
			if v == 42 {
				return Result::pure(42);
			} else {
				return Err("That answer is not the meaning of life");
			}
		}

		let val: Result<i32, &'static str> = Result::pure(43);
		let result = val.bind(meaning_of_life);
		assert!(result.is_err())
	}
}
