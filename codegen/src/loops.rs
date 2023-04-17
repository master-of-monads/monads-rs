use std::fmt::Display;

use quote::ToTokens;
use syn::{
	fold::{fold_stmt, Fold},
	parse_quote, parse_quote_spanned,
	spanned::Spanned,
	token::Semi,
	Block, Error, Expr, ExprBreak, ExprClosure, ExprContinue, Stmt,
};

pub fn recursify_loop_blocks(folder: &mut impl Fold, block: Block) -> Block {
	folder.fold_block(
		LoopControlReturnReplacer()
			.fold_block(insert_final_loop_control_return(block)),
	)
}

fn insert_final_loop_control_return(mut block: Block) -> Block {
	let return_stmt: Stmt = parse_quote! {
		#[allow(unreachable_code)]
		{
			return ::monads_rs::Monad::ret(
				::monads_rs::loops::LoopControl::Continue()
			);
		}
	};
	block.stmts.push(return_stmt);
	block
}

struct LoopControlReturnReplacer();

impl Fold for LoopControlReturnReplacer {
	fn fold_expr_closure(&mut self, closure: ExprClosure) -> ExprClosure {
		// Ignore content of closure
		closure
	}

	fn fold_stmt(&mut self, stmt: Stmt) -> Stmt {
		match stmt {
			Stmt::Semi(Expr::Break(break_expr), semi) => {
				replace_break_with_return(break_expr, semi)
			}
			Stmt::Semi(Expr::Continue(cont), semi) => {
				replace_continue_with_return(cont, semi)
			}
			stmt => fold_stmt(self, stmt),
		}
	}
}

fn replace_break_with_return(break_expr: ExprBreak, semi: Semi) -> Stmt {
	if break_expr.label.is_some() {
		error_stmt(
			break_expr.break_token,
			"breaks with labels are not supported in monadic loops",
			semi,
		)
	} else if break_expr.expr.is_some() {
		error_stmt(
			break_expr.break_token,
			"breaks with values are not supported in monadic loops",
			semi,
		)
	} else {
		parse_quote_spanned! { break_expr.span() =>
			return ::monads_rs::Monad::ret(
				::monads_rs::loops::LoopControl::Break()
			);
		}
	}
}

fn replace_continue_with_return(cont: ExprContinue, semi: Semi) -> Stmt {
	if cont.label.is_some() {
		error_stmt(
			cont.continue_token,
			"continue with labels are not supported in monadic loops",
			semi,
		)
	} else {
		parse_quote_spanned! { cont.span() =>
			return ::monads_rs::Monad::ret(
				::monads_rs::loops::LoopControl::Continue()
			);
		}
	}
}

fn error_stmt(
	tokens: impl ToTokens,
	message: impl Display,
	semi: Semi,
) -> Stmt {
	Stmt::Semi(
		Expr::Verbatim(
			Error::new_spanned(tokens, message).into_compile_error(),
		),
		semi,
	)
}
