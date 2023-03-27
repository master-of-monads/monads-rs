#![allow(incomplete_features)]
#![feature(specialization)]

pub mod control_flow;
pub mod logging;
mod option;
mod result;

pub use monads_rs_codegen::*;

pub use option::*;
pub use result::*;

/// A type is a `Functor` if it can be mapped over turning `A`s into `B`s while preserving the
/// structure of the original type.
pub trait Functor<A> {
	type Map<B>: Functor<B>;

	/// Maps a function `f` over each `A` element of the original type, returning a new type
	/// with the same structure but with `B` elements.
	fn map<B, F: Fn(A) -> B>(self, f: F) -> Self::Map<B>;
}

/// A `Functor` with application, providing operations to
/// * embed "pure" expressions, and
/// * sequence computations and combine their results.
pub trait Applicative<A>: Functor<A> {
	type Apply<B>: Applicative<B>;

	/// Embeds a value into the structure.
	fn pure(a: A) -> Self;

	/// Sequences computations and combines their results.
	fn ap<B, F: Fn(A) -> B>(self, f: Self::Apply<F>) -> Self::Apply<B>;
}

/// A `Applicative` with a monadic binding operation. Representing the idea of
/// a monad from mathematics category theory.
pub trait Monad: Applicative<Self::Item> {
	type Item;
	type Bind<B>: Monad<Item = B>;

	/// Sequentially compose two actions, passing any value produced by the
	/// first as an argument to the second.
	fn bind<B, F: FnOnce(Self::Item) -> Self::Bind<B>>(
		self,
		f: F,
	) -> Self::Bind<B>;
}

/// Embeds a "pure" expression `a` into a `Monad`.
pub fn ret<A, M: Monad<Item = A>>(a: A) -> M {
	M::pure(a)
}
