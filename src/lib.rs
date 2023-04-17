#[cfg(feature = "logging")]
pub mod logging;
mod option;
#[cfg(feature = "probability")]
pub mod probability;
mod result;
#[cfg(feature = "state")]
pub mod state;

pub mod loops;

pub use monads_rs_codegen::*;

pub use option::*;
pub use result::*;

/// A `Monad` with a monadic binding operation. Representing the idea of a
/// monad from mathematics category theory.
pub trait Monad<'c, C> {
	type Bind<B>;

	/// Sequentially compose two actions, passing any value produced by the
	/// first as an argument to the second.
	fn bind<B, F: FnMut(C) -> Self::Bind<B> + 'c>(self, f: F) -> Self::Bind<B>;

	/// Lift a value into the `Monad`.
	fn ret(c: C) -> Self;
}
