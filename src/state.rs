use super::Monad;

pub struct State<'a, S, V> {
	run_state: Box<dyn FnMut(S) -> (S, V) + 'a>,
}

impl<'a, S: Clone + 'a, V> State<'a, S, V> {
	pub fn run(mut self, s: S) -> (S, V) {
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

impl<'c, S: 'c, C: 'c + Clone> Monad<'c, C> for State<'c, S, C> {
	type Bind<B> = State<'c, S, B>;

	/// Sequentially compose two actions, passing any value produced by the
	/// first as an argument to the second.
	fn bind<B, F: FnMut(C) -> Self::Bind<B> + 'c>(
		mut self,
		mut f: F,
	) -> Self::Bind<B> {
		let new_f = move |s| {
			let (sp, vp) = self.run_state.as_mut()(s);
			let mut m = f(vp);
			return m.run_state.as_mut()(sp);
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
