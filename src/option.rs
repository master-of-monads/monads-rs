use super::Monad;

impl<A> Monad for Option<A> {
	type Item = A;
	type Output<B> = Option<B>;

	fn ret<AA>(a: AA) -> Option<AA> {
		Some(a)
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
		let val: Option<i32> = ret::<_, Option<_>>(42);
		assert_eq!(val, Some(42))
	}

	#[test]
	fn binding() {
		let val: Option<i32> = ret::<_, Option<_>>(42);
		let result = val.bind(|v| ret::<_, Option<_>>(v + 1));
		assert_eq!(result, Some(43))
	}

	#[test]
	fn short_curcuit() {
		let val: Option<i32> = Option::None;
		let result = val.bind(|v| ret::<_, Option<_>>(v + 1));
		assert_eq!(result, Option::None)
	}
}
