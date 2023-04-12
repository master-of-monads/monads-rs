#![cfg(feature = "state")]

use monads_rs::state::*;
use monads_rs::*;

#[test]
fn if_else_statement() {
	assert_eq!(monadic_if_else_statement(true).run(0), (1, ()));
	assert_eq!(monadic_if_else_statement(false).run(0), (2, ()));

	#[monadic]
	fn monadic_if_else_statement(b: bool) -> State<'static, usize, ()> {
		if b {
			State::<_, ()>::set(1)?;
			State::ret(())
		} else {
			State::<_, ()>::set(2)?;
			State::ret(())
		}?;
		State::ret(())
	}
}

#[test]
fn if_no_else_statement() {
	assert_eq!(monadic_if_no_else_statement(true).run(0), (1, ()));
	assert_eq!(monadic_if_no_else_statement(false).run(0), (0, ()));

	#[monadic]
	fn monadic_if_no_else_statement(b: bool) -> State<'static, usize, ()> {
		if b {
			State::<_, ()>::set(1)?;
			State::ret(())
		}?;
		State::ret(())
	}
}

#[test]
fn if_else_chain_statement() {
	assert_eq!(
		monadic_if_else_chain_statement(true, false, false).run(0),
		(1, ())
	);
	assert_eq!(
		monadic_if_else_chain_statement(false, true, false).run(0),
		(2, ())
	);
	assert_eq!(
		monadic_if_else_chain_statement(false, false, true).run(0),
		(3, ())
	);
	assert_eq!(
		monadic_if_else_chain_statement(false, false, false).run(0),
		(4, ())
	);

	#[monadic]
	fn monadic_if_else_chain_statement(
		a: bool,
		b: bool,
		c: bool,
	) -> State<'static, usize, ()> {
		if a {
			State::<_, ()>::set(1)?;
			State::ret(())
		} else if b {
			State::<_, ()>::set(2)?;
			State::ret(())
		} else if c {
			State::<_, ()>::set(3)?;
			State::ret(())
		} else {
			State::<_, ()>::set(4)?;
			State::ret(())
		}?;
		State::ret(())
	}
}

#[test]
fn if_no_else_chain_statement() {
	assert_eq!(
		monadic_if_no_else_chain_statement(true, false, false).run(0),
		(1, ())
	);
	assert_eq!(
		monadic_if_no_else_chain_statement(false, true, false).run(0),
		(2, ())
	);
	assert_eq!(
		monadic_if_no_else_chain_statement(false, false, true).run(0),
		(3, ())
	);
	assert_eq!(
		monadic_if_no_else_chain_statement(false, false, false).run(0),
		(0, ())
	);

	#[monadic]
	fn monadic_if_no_else_chain_statement(
		a: bool,
		b: bool,
		c: bool,
	) -> State<'static, usize, ()> {
		if a {
			State::<_, ()>::set(1)?;
			State::ret(())
		} else if b {
			State::<_, ()>::set(2)?;
			State::ret(())
		} else if c {
			State::<_, ()>::set(3)?;
			State::ret(())
		}?;
		State::ret(())
	}
}
