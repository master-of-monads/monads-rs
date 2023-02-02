use super::{ret, Applicative, Functor, Monad};

impl<A> Functor<A> for Option<A> {
	type Map<B> = Option<B>;

	fn map<B, F: FnOnce(A) -> B>(self, f: F) -> Self::Map<B> {
		self.map(f)
	}
}

impl<A> Applicative<A> for Option<A> {
	type Apply<B> = Option<B>;

	fn pure(a: A) -> Self {
		Some(a)
	}

	fn ap<B, F: FnOnce(A) -> B>(self, f: Self::Apply<F>) -> Self::Apply<B> {
		self.and_then(|a| f.map(|f| f(a)))
	}
}

impl<A> Monad<A> for Option<A> {
	type Bind<B> = Option<B>;

	fn bind<B, F: FnOnce(A) -> Self::Bind<B>>(self, f: F) -> Self::Bind<B> {
		self.and_then(f)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn pure_test() {
		let val: Option<i32> = ret(42);
		assert_eq!(val, Some(42))
	}

	#[test]
	fn binding() {
		let val: Option<i32> = ret(42);
		let result = val.bind(|v| ret(v + 1));
		assert_eq!(result, Some(43))
	}

	#[test]
	fn short_curcuit() {
		let val: Option<i32> = Option::None;
		let result = val.bind(|v| ret(v + 1));
		assert_eq!(result, Option::None)
	}
}
