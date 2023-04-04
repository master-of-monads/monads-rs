#[cfg(feature = "logging")]
pub mod logging;
mod option;
#[cfg(feature = "probability")]
pub mod probability;
mod result;
#[cfg(feature = "state")]
pub mod state;

pub use monads_rs_codegen::*;

pub use option::*;
pub use result::*;

/// A `Monad` with a monadic binding operation. Representing the idea of a
/// monad from mathematics category theory.
pub trait Monad<'a, A> {
	type Bind<'b, B: 'b>: Monad<'b, B>;

	/// Sequentially compose two actions, passing any value produced by the
	/// first as an argument to the second.
	fn bind<'b, B, F: Fn(A) -> Self::Bind<'b, B> + 'a>(
		self,
		f: F,
	) -> Self::Bind<'b, B>;

	/// Lift a value into the `Monad`.
	fn ret(c: A) -> Self;
}

pub fn fmap<'a, 'b, M: Monad<'a, A>, A, B, F: Fn(A) -> B + 'a>(
	ma: M,
	f: F,
) -> M::Bind<'b, B> {
	ma.bind(move |a| M::Bind::<'b, B>::ret(f(a)))
}

/// A `MonadTrans` is a monad transformer, which is a monad that operates on
/// another monad. It is a monad in its own right, which adds structure to the
/// monad it contains.
pub trait MonadTrans<'c, C, M: Monad<'c, C>> {
	/// Lift a value into the `MonadTrans`.
	fn lift<B>(mb: M) -> Self;
}
