use monads_rs::*;

/// The unhandled bind checker should not pass into local functions, or other
/// local item definitions such as structs or impls.
/// This test fails to compile if somethings is out of order.
/// Note that the "binds" in the local definitions must be unhandled or else the
/// test will not work as intended.
#[test]
fn local_monadic_bind() {
	let result = monadic_function();
	assert_eq!(result, Some(42));

	#[monadic]
	fn monadic_function() -> Option<i32> {
		let result = Struct::non_monadic_method().or(non_monadic_function())?;
		return Some(result);

		fn non_monadic_function() -> Option<i32> {
			let answer = Some(42)?;
			Some(answer)
		}

		struct Struct {}

		impl Struct {
			fn non_monadic_method() -> Option<i32> {
				let not_the_answer = None?;
				not_the_answer
			}
		}
	}
}
