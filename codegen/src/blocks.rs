use std::collections::VecDeque;

use syn::{Block, Stmt};

use crate::locals::bind_local_declaration;

pub(crate) fn bind_in_block(mut block: Block) -> Block {
	move_up_item_declarations(&mut block.stmts);
	block.stmts = bind_statements(block.stmts).into();
	block
}

fn move_up_item_declarations(stmts: &mut Vec<Stmt>) {
	for i in 0..stmts.len() {
		if matches!(stmts[i], Stmt::Item(_)) {
			stmts[0..=i].rotate_right(1);
		}
	}
}

pub(crate) fn bind_statements(
	stmts: impl Into<VecDeque<Stmt>>,
) -> VecDeque<Stmt> {
	let mut stmts: VecDeque<Stmt> = stmts.into();

	let Some(stmt) = stmts.pop_front() else { return stmts; };

	match stmt {
		expr @ Stmt::Expr(_) => leave_statement_as_is(expr, stmts),
		item @ Stmt::Item(_) => leave_statement_as_is(item, stmts),
		Stmt::Local(local) => bind_local_declaration(local, stmts),
		semi @ Stmt::Semi(_, _) => leave_statement_as_is(semi, stmts),
	}
}

fn leave_statement_as_is(
	stmt: Stmt,
	remaining_stmts: VecDeque<Stmt>,
) -> VecDeque<Stmt> {
	let mut rest = bind_statements(remaining_stmts);
	rest.push_front(stmt);
	rest
}
