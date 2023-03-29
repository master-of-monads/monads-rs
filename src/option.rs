use super::Monad;

impl<'a, A> Monad<'a, A> for Option<A> {
	type Bind<B> = Option<B>;

	fn bind<B, F: FnOnce(A) -> Self::Bind<B>>(self, f: F) -> Self::Bind<B> {
		self.and_then(f)
	}

	fn ret(val: A) -> Self {
		Some(val)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn pure_test() {
		let val: Option<i32> = Option::ret(42);
		assert_eq!(val, Some(42))
	}

	#[test]
	fn binding() {
		let val: Option<i32> = Option::ret(42);
		let result = val.bind(|v| Option::ret(v + 1));
		assert_eq!(result, Some(43))
	}

	#[test]
	fn short_curcuit() {
		let val: Option<i32> = Option::None;
		let result = val.bind(|v| Option::ret(v + 1));
		assert_eq!(result, Option::None)
	}
}
