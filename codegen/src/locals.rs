use std::collections::VecDeque;

use syn::{parse_quote, token::Brace, Block, Expr, Local, Pat, Stmt};

use crate::{blocks::bind_statements, exprs::bind_expr};

pub(crate) fn bind_local_declaration(
	mut local: Local,
	remaining_stmts: VecDeque<Stmt>,
) -> VecDeque<Stmt> {
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
		stmts: rest.into(),
	};
	let stmt_expr = binder.build_binds(then_block);
	let stmts = vec![Stmt::Expr(stmt_expr)];
	stmts.into()
}

pub(crate) fn build_monadic_bind(
	bind_pattern: &Pat,
	monadic_expr: &Expr,
	then_block: &Block,
) -> Expr {
	parse_quote! {
		::monads_rs::Monad::bind(
			#monadic_expr,
			move |#bind_pattern| #then_block
		)
	}
}
