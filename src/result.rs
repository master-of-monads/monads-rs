use super::{Applicative, Functor, Monad};

impl<A: 'static, E> Functor<A> for Result<A, E> {
	type Map<B: 'static> = Result<B, E>;

	fn map<B: 'static, F: 'static + FnOnce(A) -> B>(self, f: F) -> Self::Map<B> {
		self.map(f)
	}
}

impl<A: 'static, E> Applicative<A> for Result<A, E> {
	type Apply<B: 'static> = Result<B, E>;

	fn pure(a: A) -> Self {
		Ok(a)
	}

	fn ap<B: 'static, F: 'static + FnOnce(A) -> B>(self, f: Self::Apply<F>) -> Self::Apply<B> {
		self.and_then(|a| f.map(|f| f(a)))
	}
}

impl<A: 'static, E> Monad<A> for Result<A, E> {
	type Bind<B: 'static> = Result<B, E>;

	fn bind<B: 'static, F: 'static + FnOnce(A) -> Self::Bind<B>>(self, f: F) -> Self::Bind<B> {
		self.and_then(f)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ret;

	#[test]
	fn pure_test() {
		let val: Result<i32, &'static str> = ret(42);
		assert_eq!(val, Ok(42))
	}

	#[test]
	fn binding() {
		let val: Result<i32, &'static str> = ret(42);
		let result = val.bind(|v| ret(v + 1));
		assert_eq!(result, Ok(43))
	}

	#[test]
	fn short_curcuit() {
		fn meaning_of_life(v: i32) -> Result<i32, &'static str> {
			if v == 42 {
				ret(42)
			} else {
				return Err("That answer is not the meaning of life");
			}
		}

		let val: Result<i32, &'static str> = ret(43);
		let result = val.bind(meaning_of_life);
		assert!(result.is_err())
	}
}
