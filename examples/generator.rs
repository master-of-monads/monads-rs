use std::ops::Add;

use monads_rs::{monadic, ret};

fn main() {
	let s = sum(vec![10, 20, 30], vec![1, 2, 3]);
	println!("Sum: {s:?}"); // Sum: [11, 12, 13, 21, 22, 23, 31, 32, 33]
}

#[monadic]
fn sum<L, R, O>(l: Vec<L>, r: Vec<R>) -> Vec<O>
where
	L: 'static + Copy + Add<R, Output = O>,
	R: 'static + Clone,
	O: 'static,
{
	let l = l?;
	let r = r.clone()?;
	ret(l + r)
}
