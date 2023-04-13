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

#[test]
fn binds_outside_while_loop() {
	assert_eq!(monadic_binds_outside_while_loop().run(5), (15, ()));

	#[monadic]
	fn monadic_binds_outside_while_loop() -> State<'static, usize, ()> {
		let x = State::<_, ()>::get()?;
		let mut sum = 0;
		let mut i = 1;
		while i <= x {
			sum += i;
			i += 1;
		}
		State::<_, ()>::set(sum)?;
		State::ret(())
	}
}

#[test]
fn binds_inside_while_loop() {
	assert_eq!(
		monadic_binds_inside_while_loop(5).run(0.into()),
		(Loop::new(15, 6), ())
	);

	#[monadic]
	fn monadic_binds_inside_while_loop(input: usize) -> LoopState<usize, ()> {
		while State::ret(get_i()? <= input) {
			let sum = get_state()?;
			set_state(sum + get_i()?)?;
			inc_i()?;
			State::ret(())
		}?;
		State::ret(())
	}
}

#[test]
fn binds_and_while_loop() {
	assert_eq!(
		monadic_binds_and_while_loop().run(5.into()),
		(Loop::new(15, 6), 15)
	);

	#[monadic]
	fn monadic_binds_and_while_loop() -> LoopState<usize, usize> {
		let x = get_state()?;
		set_state(0)?;
		while State::ret(get_i()? <= x) {
			let sum = get_state()?;
			set_state(sum + get_i()?)?;
			inc_i()?;
			State::ret(())
		}?;
		let sum = get_state()?;
		State::ret(sum)
	}
}

type LoopState<S, T> = State<'static, Loop<S>, T>;

#[monadic]
fn get_i<S: Clone + 'static>() -> LoopState<S, usize> {
	let state = LoopState::<S, usize>::get()?;
	LoopState::<S, usize>::ret(state.i)
}

#[monadic]
fn inc_i<S: Clone + 'static>() -> LoopState<S, ()> {
	let mut state = LoopState::<S, ()>::get()?;
	state.i += 1;
	LoopState::<S, ()>::set(state)
}

#[monadic]
fn get_state<S: Clone + 'static>() -> LoopState<S, S> {
	let state = LoopState::<S, S>::get()?;
	LoopState::<S, S>::ret(state.state)
}

#[monadic]
fn set_state<S: Clone + 'static>(state: S) -> LoopState<S, ()> {
	let mut loop_state = LoopState::<S, ()>::get()?;
	loop_state.state = state.clone();
	LoopState::<S, ()>::set(loop_state)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Loop<S> {
	state: S,
	i: usize,
}

impl<S> Loop<S> {
	fn new(state: S, i: usize) -> Self {
		Self { state, i }
	}
}

impl<S> From<S> for Loop<S> {
	fn from(state: S) -> Self {
		Self::new(state, 1)
	}
}
