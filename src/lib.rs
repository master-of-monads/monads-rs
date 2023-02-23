pub mod control_flow;
pub mod logging;
mod option;
mod result;

pub use monads_rs_codegen::*;

pub use option::*;
pub use result::*;

/// A `Applicative` with a monadic binding operation. Representing the idea of
/// a monad from mathematics category theory.
pub trait Monad {
	type Item;
	type Output<B>;

	/// Embeds a "pure" expression `a` into a `Monad`.
	fn ret<A>(a: A) -> Self::Output<A>;

	/// Sequentially compose two actions, passing any value produced by the
	/// first as an argument to the second.
	fn bind<B, F: FnOnce(Self::Item) -> Self::Output<B>>(
		self,
		f: F,
	) -> Self::Output<B>;
}

/// Embeds a "pure" expression `a` into a `Monad`.
pub fn ret<A, M: Monad<Item = A>>(a: A) -> M::Output<A> {
	M::ret(a)
}
