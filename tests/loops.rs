#![cfg(feature = "state")]

use monads_rs::state::*;
use monads_rs::*;

#[test]
fn binds_outside_for_loop() {
	assert_eq!(monadic_binds_outside_for_loop().run(5), (15, ()));

	#[monadic]
	fn monadic_binds_outside_for_loop() -> State<'static, usize, ()> {
		let x = State::<_, ()>::get()?;
		let mut sum = 0;
		for i in 1..=x {
			sum += i;
		}
		State::<_, ()>::set(sum)?;
		State::ret(())
	}
}

#[test]
fn binds_inside_for_loop() {
	assert_eq!(monadic_binds_inside_for_loop(5).run(0), (15, ()));

	#[monadic]
	fn monadic_binds_inside_for_loop(
		input: usize,
	) -> State<'static, usize, ()> {
		for i in 1..=input {
			let sum = State::<_, ()>::get()?;
			State::<_, ()>::set(sum + i)?;
			State::ret(())
		}?;
		State::ret(())
	}
}

#[test]
fn binds_and_for_loop() {
	assert_eq!(monadic_binds_and_for_loop().run(5), (15, 15));

	#[monadic]
	fn monadic_binds_and_for_loop() -> State<'static, usize, usize> {
		let x = State::<_, ()>::get()?;
		State::<_, ()>::set(0)?;
		for i in 1..=x {
			let sum = State::<_, ()>::get()?;
			State::<_, ()>::set(sum + i)?;
			State::ret(())
		}?;
		let sum = State::<_, ()>::get()?;
		State::ret(sum)
	}
}
