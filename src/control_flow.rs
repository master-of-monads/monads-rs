use crate::Monad;

pub enum ControlFlowAction<R, O = R> {
	Exit(O),
	Result(R),
}

pub struct CFA<M>(M);

impl<A, M: Monad<Item = ControlFlowAction<A, A>>> CFA<M> {
	pub fn unwrap(self) -> M::Output<A> {
		self.0.bind(|cfa| match cfa {
			ControlFlowAction::Exit(value) => M::ret(value),
			ControlFlowAction::Result(value) => M::ret(value),
		})
	}
}

impl<A, M: Monad<Item = ControlFlowAction<A, O>>, O> Monad for CFA<M> {
	type Item = A;
	type Output<B> = CFA<M::Output<ControlFlowAction<B, O>>>;

	fn ret<AA>(a: AA) -> Self::Output<AA> {
		CFA(M::ret(ControlFlowAction::Result(a)))
	}

	fn bind<B, F: FnOnce(A) -> Self::Output<B>>(self, f: F) -> Self::Output<B> {
		CFA(self.0.bind(|cfa| match cfa {
			ControlFlowAction::Exit(value) => {
				M::ret(ControlFlowAction::Exit(value))
			}
			ControlFlowAction::Result(value) => f(value).0,
		}))
	}
}

#[test]
fn bind_for_other_types() {
	assert_eq!(bind_for_other_types_return(), Some(12));

	fn bind_for_other_types_return() -> Option<usize> {
		{
			CFA(Some(ControlFlowAction::Result(true))).bind(|bound_expr| {
				let _b = bound_expr;
				return CFA(Some(ControlFlowAction::Exit(12)));
				#[allow(unreachable_code)]
				CFA(Some(ControlFlowAction::Result(_b)))
			})
		}
		.bind(|bound_block| {
			let b = bound_block;
			if b {
				CFA(Some(ControlFlowAction::Result(24)))
			} else {
				CFA(Some(ControlFlowAction::Result(25)))
			}
		})
		.unwrap()
	}
}
