use crate::{Applicative, Functor, Monad};

impl<A: 'static> Functor<A> for Vec<A> {
	type Map<B: 'static> = Vec<B>;

	fn map<B: 'static, F: 'static + FnMut(A) -> B>(self, f: F) -> Self::Map<B> {
		Iterator::map(self.into_iter(), f).collect()
	}
}

impl<A: 'static> Applicative<A> for Vec<A> {
	type Apply<B: 'static> = Vec<B>;

	fn pure(a: A) -> Self {
		vec![a]
	}

	fn ap<B: 'static, F: 'static + FnMut(A) -> B>(
		self,
		_f: Self::Apply<F>,
	) -> Self::Apply<B> {
		todo!()
	}
}

impl<A: 'static> Monad<A> for Vec<A> {
	type Bind<B: 'static> = Vec<B>;

	fn bind<B: 'static, F: 'static + FnMut(A) -> Self::Bind<B>>(
		self,
		f: F,
	) -> Self::Bind<B> {
		self.into_iter().flat_map(f).collect()
	}
}
