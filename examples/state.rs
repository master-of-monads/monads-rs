use monads_rs::{monadic, ret, Applicative, Functor, Monad};

fn main() {
	let comp = stateful_computation();
	println!("Look at the order of execution");
	let (output_state, result) = comp.run(0);
	println!("Hello World: {output_state}, {result}");
}

#[monadic]
fn stateful_computation() -> State<usize, usize> {
	let _ = State::set(10)?;
	let _ = stateful_function()?;
	ret(0)
}

#[monadic]
fn stateful_function() -> State<usize, ()> {
	println!("This function does nothing...");
	stateful_function2()
}

#[monadic]
fn stateful_function2() -> State<usize, ()> {
	println!("This function does something!");
	let base = State::get()?;
	let _ = State::set(base + 2)?;
	ret(())
}

struct State<S, A>(Box<dyn FnOnce(S) -> (S, A)>);

impl<S, A> State<S, A> {
	fn new(f: impl FnOnce(S) -> (S, A) + 'static) -> Self {
		Self(Box::new(f))
	}

	fn run(self, initial_state: S) -> (S, A) {
		self.0(initial_state)
	}
}

impl<S: Clone> State<S, S> {
	fn get() -> Self {
		Self::new(|s| (s.clone(), s))
	}
}

impl<S: 'static> State<S, ()> {
	fn set(state: S) -> Self {
		Self::new(|_| (state, ()))
	}
}

impl<S: 'static, A: 'static> Functor<A> for State<S, A> {
	type Map<B: 'static> = State<S, B>;

	fn map<B: 'static, F: 'static + FnOnce(A) -> B>(
		self,
		f: F,
	) -> Self::Map<B> {
		State::new(|s| {
			let (s, v) = self.0(s);
			(s, f(v))
		})
	}
}

impl<S: 'static, A: 'static> Applicative<A> for State<S, A> {
	type Apply<B: 'static> = State<S, B>;

	fn pure(a: A) -> Self {
		State::new(|s| (s, a))
	}

	fn ap<B: 'static, F: 'static + FnOnce(A) -> B>(
		self,
		f: Self::Apply<F>,
	) -> Self::Apply<B> {
		State::new(|s| {
			let (s, a) = self.0(s);
			let (s, f) = f.0(s);
			(s, f(a))
		})
	}
}

impl<S: 'static, A: 'static> Monad<A> for State<S, A> {
	type Bind<B: 'static> = State<S, B>;

	fn bind<B: 'static, F: 'static + FnOnce(A) -> Self::Bind<B>>(
		self,
		f: F,
	) -> Self::Bind<B> {
		State::new(|s| {
			let (s, a) = self.0(s);
			let m = f(a);
			m.0(s)
		})
	}
}
