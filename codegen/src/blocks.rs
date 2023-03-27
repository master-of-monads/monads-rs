use std::{
	collections::VecDeque,
	ops::{Deref, DerefMut},
};

use syn::{
	parse_quote_spanned,
	spanned::Spanned,
	token::{Brace, Semi},
	Block, Expr, Stmt,
};

use crate::{exprs::bind_expr, locals::bind_local_declaration};

pub(crate) fn bind_in_function_block(mut block: Block) -> Block {
	move_up_item_declarations(&mut block.stmts);
	block.stmts = bind_statements(block.stmts).bind_all_implicit_returns();
	block
}

pub(crate) fn bind_in_block(mut block: Block) -> Block {
	move_up_item_declarations(&mut block.stmts);
	block.stmts = bind_statements(block.stmts).bind_monadic_implicit_returns();
	block
}

fn move_up_item_declarations(stmts: &mut Vec<Stmt>) {
	for i in 0..stmts.len() {
		if matches!(stmts[i], Stmt::Item(_)) {
			stmts[0..=i].rotate_right(1);
		}
	}
}

pub(crate) fn bind_statements(stmts: impl Into<VecDeque<Stmt>>) -> Stmts {
	let mut stmts: VecDeque<Stmt> = stmts.into();

	let Some(stmt) = stmts.pop_front() else { return Stmts::Regular(stmts); };

	match stmt {
		Stmt::Expr(expr) => bind_expr_stmt(expr, stmts),
		item @ Stmt::Item(_) => leave_statement_as_is(item, stmts),
		Stmt::Local(local) => bind_local_declaration(local, stmts),
		Stmt::Semi(expr, semi) => bind_semi_stmt(expr, semi, stmts),
	}
}

fn leave_statement_as_is(stmt: Stmt, remaining_stmts: VecDeque<Stmt>) -> Stmts {
	let mut rest = bind_statements(remaining_stmts);
	rest.push_front(stmt);
	rest
}

fn bind_semi_stmt(
	expr: Expr,
	semi: Semi,
	remaining_stmts: VecDeque<Stmt>,
) -> Stmts {
	bind_stmt_and_restore(expr, |e| Stmt::Semi(e, semi), remaining_stmts)
}

fn bind_expr_stmt(expr: Expr, remaining_stmts: VecDeque<Stmt>) -> Stmts {
	bind_stmt_and_restore(expr, Stmt::Expr, remaining_stmts)
}

fn bind_stmt_and_restore(
	expr: Expr,
	restore: impl FnOnce(Expr) -> Stmt,
	remaining_stmts: VecDeque<Stmt>,
) -> Stmts {
	let mut rest = bind_statements(remaining_stmts);
	let (binder, bound_expr) = bind_expr(expr);
	rest.push_front(restore(bound_expr));

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

pub enum Stmts {
	Regular(VecDeque<Stmt>),
	Monadic(VecDeque<Stmt>),
}

impl Stmts {
	pub fn bind_all_implicit_returns<T: From<VecDeque<Stmt>>>(self) -> T {
		let stmts = match self {
			Stmts::Regular(stmts) => stmts,
			Stmts::Monadic(stmts) => stmts,
		};
		bind_implicit_returns(stmts).into()
	}

	pub fn bind_monadic_implicit_returns<T: From<VecDeque<Stmt>>>(self) -> T {
		match self {
			Stmts::Regular(stmts) => stmts.into(),
			Stmts::Monadic(stmts) => bind_implicit_returns(stmts).into(),
		}
	}

	pub fn into_vec<T: From<VecDeque<Stmt>>>(self) -> T {
		match self {
			Stmts::Regular(stmts) => stmts.into(),
			Stmts::Monadic(stmts) => stmts.into(),
		}
	}
}

fn bind_implicit_returns(mut stmts: VecDeque<Stmt>) -> VecDeque<Stmt> {
	for stmt in &mut stmts {
		if let Stmt::Expr(expr) = stmt {
			*expr = parse_quote_spanned! { expr.span() =>
				<::monads_rs::control_flow::ControlFlowAction<_, _> as ::monads_rs::control_flow::From2<_>>::from2(#expr)
			};
		}
	}
	stmts
}

impl Deref for Stmts {
	type Target = VecDeque<Stmt>;

	fn deref(&self) -> &Self::Target {
		match self {
			Stmts::Regular(ref stmts) => stmts,
			Stmts::Monadic(ref stmts) => stmts,
		}
	}
}

impl DerefMut for Stmts {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Stmts::Regular(ref mut stmts) => stmts,
			Stmts::Monadic(ref mut stmts) => stmts,
		}
	}
}
