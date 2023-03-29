//pub mod logging;
//mod option;
mod result;
mod state;
pub use monads_rs_codegen::*;

pub use state::*;

//pub use option::*;
//pub use result::*;

/// A `Monad` with a monadic binding operation. Representing the idea of a
/// monad from mathematics category theory.
pub trait Monad<'c, C> {
	type Bind<B>;

	/// Sequentially compose two actions, passing any value produced by the
	/// first as an argument to the second.
	fn bind<B, F: Fn(C) -> Self::Bind<B> + 'c>(self, f: F) -> Self::Bind<B>;

	/// Lift a value into the `Monad`.
	fn ret(c: C) -> Self;
}
