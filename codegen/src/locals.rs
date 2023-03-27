use std::collections::VecDeque;

use syn::{
	parse_quote_spanned, spanned::Spanned, token::Brace, Block, Expr, Local,
	Pat, Stmt,
};

use crate::{
	blocks::{bind_statements, Stmts},
	exprs::bind_expr,
};

pub(crate) fn bind_local_declaration(
	mut local: Local,
	remaining_stmts: VecDeque<Stmt>,
) -> Stmts {
	let mut rest = bind_statements(remaining_stmts);

	let Some((eq, init_expr)) = local.init else {
		rest.push_front(Stmt::Local(local));
		return rest;
	};

	let (binder, bound_expr) = bind_expr(*init_expr);
	local.init = Some((eq, Box::new(bound_expr)));
	rest.push_front(Stmt::Local(local));

	if !binder.needs_binding() {
		return rest;
	}

	let then_block = Block {
		brace_token: Brace::default(),
		stmts: rest.into_vec(),
	};
	let then_block = binder.build_binds(then_block);
	Stmts::Monadic(then_block.stmts.into())
}

pub(crate) fn build_monadic_bind(
	bind_pattern: &Pat,
	monadic_expr: &Expr,
	then_block: &mut Block,
) -> Expr {
	if let Some(Stmt::Expr(expr)) = then_block.stmts.last_mut() {
		*expr = parse_quote_spanned! { expr.span() =>
			<::monads_rs::control_flow::ControlFlowAction<_, _> as ::monads_rs::control_flow::From2<_>>::from2(#expr)
		};
	}

	parse_quote_spanned! { monadic_expr.span() =>
		<::monads_rs::control_flow::ControlFlowAction<_, _> as ::monads_rs::control_flow::From2<_>>::from2(#monadic_expr)
			.bind(|#bind_pattern| #then_block)
	}
}
