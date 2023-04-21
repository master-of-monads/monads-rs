use core::fmt::Debug;
use monads_rs::*;

#[derive(Debug)]
struct Identity<V:Debug>(V);

impl<'a, A : Debug> Monad<'a, A> for Identity<A> {
	type Bind<B> = Identity<A>;

	fn bind<B, F: FnOnce(A) -> Self::Bind<B>>(self, f: F) -> Self::Bind<B> {
        f(self.0)
	}

	fn ret(val: A) -> Self {
		Identity(val)
	}
}

fn main() {
	let a = early_block_return();
	let b = early_if_return(true);
    println!("{0:?} {1:?}", a,b);
}

#[monadic]
fn early_block_return() -> Identity<bool> {
	let foo = { return Identity(false); Identity(true)}?;
	Identity::ret(foo)
}

#[monadic]
fn early_if_return(foo : bool) -> Identity<bool> {
	let boo = if foo {
		return Identity(false);
		Identity(true)
	} else {
		Identity(false)
	}?;

	Identity(boo)
}