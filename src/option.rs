use super::{fmap, Monad, MonadTrans};

impl<'a, A> Monad<'a, A> for Option<A> {
	type Bind<'b, B: 'b> = Option<B>;

	fn bind<'b, B: 'b, F: FnOnce(A) -> Self::Bind<'b, B>>(
		self,
		f: F,
	) -> Self::Bind<'b, B> {
		self.and_then(f)
	}

	fn ret(val: A) -> Self {
		Some(val)
	}
}

pub struct OptionT<M> {
	inner: M,
}

impl<M> OptionT<M> {
	pub fn run(self) -> M {
		self.inner
	}
}

impl<'a, A: 'a, M: Monad<'a, Option<A>>> Monad<'a, A> for OptionT<M> {
	type Bind<'b, B: 'b> = OptionT<M::Bind<'b, Option<B>>>;

	fn bind<'b, B: 'b, F: Fn(A) -> Self::Bind<'b, B> + 'a>(
		self,
		f: F,
	) -> Self::Bind<'b, B> {
		OptionT {
			inner: self.inner.bind(move |opt| match opt {
				Some(a) => f(a).inner,
				None => M::Bind::<'_, _>::ret(None),
			}),
		}
	}

	fn ret(c: A) -> Self {
		OptionT {
			inner: M::ret(Some(c)),
		}
	}
}

impl<'a, A: 'a, M: Monad<'a, A>> MonadTrans<'a, A, M>
	for OptionT<M::Bind<'a, Option<A>>>
{
	fn lift<B>(m: M) -> Self {
		Self {
			inner: fmap(m, Some),
		}
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

	#[test]
	fn option_t() {
		let val: OptionT<Result<Option<usize>, String>> =
			MonadTrans::lift::<Result<usize, String>>(Result::ret(42));
		let result = val.bind(|v| {
			MonadTrans::lift::<Result<usize, String>>(Result::ret(v + 1))
		});
		assert_eq!(result.run(), Ok(Some(43)))
	}
}
