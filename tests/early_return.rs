use monads_rs::*;

#[test]
fn simple() {
	assert_eq!(simple_some(), Some(12));
	assert_eq!(simple_none(), None);

	#[monadic]
	fn simple_some() -> Option<usize> {
		return Some(12);
		Some(24)
	}

	#[monadic]
	fn simple_none() -> Option<usize> {
		return None;
		Some(24)
	}
}

#[test]
fn bind_after() {
	assert_eq!(bind_after_return(), Some(12));

	#[monadic]
	fn bind_after_return() -> Option<usize> {
		return Some(12);
		let a = Some(24)?;
		Some(a)
	}
}

#[test]
fn bind_before() {
	assert_eq!(bind_before_return(), Some(12));

	#[monadic]
	fn bind_before_return() -> Option<usize> {
		let a = Some(24)?;
		return Some(12);
		Some(a)
	}
}

#[test]
fn deep_bind() {
	assert_eq!(deep_bind_return(), Some(12));

	#[monadic]
	fn deep_bind_return() -> Option<usize> {
		let a = Some(24)?;
		let a = Some(a)?;
		let a = Some(a)?;
		let a = Some(a)?;
		return Some(12);
		let a = Some(a)?;
		let a = Some(a)?;
		let a = Some(a)?;
		Some(a)
	}
}

#[test]
fn bind_for_other_types() {
	assert_eq!(bind_for_other_types_return(), Some(12));

	#[monadic]
	fn bind_for_other_types_return() -> Option<usize> {
		let b = {
			let b = Some(true)?;
			return Some(12);
			Some(b)
		}?;
		if b {
			Some(24)
		} else {
			Some(25)
		}
	}
}
