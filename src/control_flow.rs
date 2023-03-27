use crate::{ret, Monad};

pub enum ControlFlowAction<R, E = R> {
	Exit(E),
	Result(R),
}

impl<R, NextR, E> ControlFlowAction<R, E>
where
	R: Monad<Bind<()> = NextR>,
	NextR: Monad<Item = ()>,
{
	pub fn bind<B>(
		self,
		f: impl FnOnce(R::Item) -> ControlFlowAction<NextR::Bind<B>, E>,
	) -> ControlFlowAction<NextR::Bind<B>, E> {
		match self {
			ControlFlowAction::Exit(value) => ControlFlowAction::Exit(value),
			ControlFlowAction::Result(value) => {
				let mut output = None;
				let b = value.bind::<(), _>(|v| {
					output = Some(f(v));
					ret(())
				});
				match output.unwrap() {
					ControlFlowAction::Exit(exit) => {
						ControlFlowAction::Exit(exit)
					}
					ControlFlowAction::Result(value) => {
						ControlFlowAction::Result(b.bind(|_| value))
					}
				}
			}
		}
	}
}

impl<R> ControlFlowAction<R> {
	pub fn unwrap(self) -> R {
		match self {
			Self::Exit(value) => value,
			Self::Result(value) => value,
		}
	}
}

pub trait FlatFrom<T> {
	type Output;

	fn flat_from(value: T) -> Self::Output;
}

impl<R: Monad, E> FlatFrom<R> for ControlFlowAction<R, E> {
	type Output = ControlFlowAction<R, E>;

	fn flat_from(value: R) -> Self::Output {
		ControlFlowAction::Result(value)
	}
}

impl<R, E> FlatFrom<ControlFlowAction<R, E>> for ControlFlowAction<R, E> {
	type Output = ControlFlowAction<R, E>;

	fn flat_from(value: ControlFlowAction<R, E>) -> Self::Output {
		value
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn nested_implicit_return() {
		assert_eq!(bind_nested_implicit_return(), Some(12));

		fn bind_nested_implicit_return() -> Option<usize> {
			fn bind_nested_implicit_return() -> ControlFlowAction<Option<usize>>
			{
				ControlFlowAction::flat_from({
					ControlFlowAction::Result(Some(12))
				})
			}
			bind_nested_implicit_return().unwrap()
		}
	}

	#[test]
	fn monadic_block() {
		assert_eq!(bind_monadic_block(), Some(12));

		fn bind_monadic_block() -> Option<usize> {
			fn bind_monadic_block() -> ControlFlowAction<Option<usize>> {
				ControlFlowAction::flat_from(ControlFlowAction::flat_from(
					Some(12),
				))
				.bind(|a| ControlFlowAction::flat_from(Some(a)))
			}
			bind_monadic_block().unwrap()
		}
	}

	#[test]
	fn bind_for_other_types() {
		assert_eq!(bind_for_other_types_return(), Some(12));

		fn bind_for_other_types_return() -> Option<usize> {
			ControlFlowAction::flat_from({
				ControlFlowAction::flat_from(Some(true)).bind(|bound_expr| {
					let _b = bound_expr;
					return ControlFlowAction::Exit(Some(12));
					#[allow(unreachable_code)]
					ControlFlowAction::flat_from(Some(_b))
				})
			})
			.bind(|bound_block| {
				let b = bound_block;
				if b {
					ControlFlowAction::flat_from(Some(24))
				} else {
					ControlFlowAction::flat_from(Some(25))
				}
			})
			.unwrap()
		}
	}
}
