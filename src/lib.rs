//pub mod logging;
//mod option;
//mod result;
mod state;
pub use monads_rs_codegen::*;

pub use state::*;

//pub use option::*;
//pub use result::*;

/// A type is a `Functor` if it can be mapped over turning `A`s into `B`s while preserving the
/// structure of the original type.pub trait Functor<'a, A> {
pub trait Functor<'a, A> {
	type Target<T>;
	fn fmap<B, F>(self, f: F) -> Self::Target<B>
	where
		F: Fn(A) -> B + 'a;
}

/// A `Functor` with a monadic binding operation. Representing the idea of
/// a monad from mathematics category theory.
pub trait Monad<'c, C>: Functor<'c, C> {
	type Bind<B>;

	/// Sequentially compose two actions, passing any value produced by the
	/// first as an argument to the second.
	fn bind<B, F: Fn(C) -> Self::Bind<B> + 'c>(self, f: F) -> Self::Bind<B>;
	fn ret(c: C) -> Self;
}
