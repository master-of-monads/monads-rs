use std::collections::VecDeque;

use syn::{
	parse_quote,
	token::{Brace, Eq},
	Block, Expr, ExprTry, Local, Pat, Stmt,
};

use crate::blocks::bind_statements;

pub(crate) fn bind_local_declaration(
	local: Local,
	remaining_stmts: VecDeque<Stmt>,
) -> VecDeque<Stmt> {
	let mut rest = bind_statements(remaining_stmts);
	let Some(try_expr) = retrieve_try_expr(&local.init) else {
		rest.push_front(Stmt::Local(local));
		return rest;
	};

	let then_block = Block {
		brace_token: Brace::default(),
		stmts: rest.into(),
	};
	let stmt_expr =
		build_monadic_bind(&local.pat, try_expr.expr.as_ref(), &then_block);

	let stmts = vec![Stmt::Expr(stmt_expr)];
	stmts.into()
}

fn retrieve_try_expr(init: &Option<(Eq, Box<Expr>)>) -> Option<&ExprTry> {
	let (_, expr) = init.as_ref()?;
	if let Expr::Try(try_expr) = expr.as_ref() {
		Some(try_expr)
	} else {
		None
	}
}

fn build_monadic_bind(
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
