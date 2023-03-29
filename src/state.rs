use super::{Functor, Monad};

pub struct State<'a, S, V> {
	run_state: Box<dyn Fn(S) -> (S, V) + 'a>,
}

impl<'a, S: Clone + 'a, V> State<'a, S, V> {
	pub fn run(self, s: S) -> (S, V) {
		(self.run_state)(s)
	}
	pub fn get() -> State<'a, S, S> {
		let f = move |s: S| (s.clone(), s);
		State {
			run_state: Box::new(f),
		}
	}
	pub fn set(s: S) -> State<'a, S, ()> {
		let f = move |_| (s.clone(), ());
		State {
			run_state: Box::new(f),
		}
	}
}

impl<'a, S: 'a, A: 'a> Functor<'a, A> for State<'a, S, A> {
	type Target<B> = State<'a, S, B>;
	fn fmap<B, F>(self, f: F) -> Self::Target<B>
	where
		F: Fn(A) -> B + 'a,
	{
		let new_f = move |s| {
			let (new_state, new_value) = self.run_state.as_ref()(s);
			(new_state, f(new_value))
		};

		State {
			run_state: Box::new(new_f),
		}
	}
}

impl<'c, S: 'c, C: 'c + Clone> Monad<'c, C> for State<'c, S, C> {
	type Bind<B> = State<'c, S, B>;

	/// Sequentially compose two actions, passing any value produced by the
	/// first as an argument to the second.
	fn bind<B, F: Fn(C) -> Self::Bind<B> + 'c>(self, f: F) -> Self::Bind<B> {
		let new_f = move |s| {
			let (sp, vp) = self.run_state.as_ref()(s);
			let m = f(vp);
			return m.run_state.as_ref()(sp);
		};
		State {
			run_state: Box::new(new_f),
		}
	}
	fn ret(c: C) -> Self {
		let f = move |s| (s, c.clone());
		State {
			run_state: Box::new(f),
		}
	}
}
