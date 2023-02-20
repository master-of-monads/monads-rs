use std::collections::VecDeque;

use proc_macro2::{Ident, Span};
use syn::{
	fold::{fold_expr, Fold},
	parse_quote,
	spanned::Spanned,
	Block, Expr, ExprClosure, ExprPath, ExprTry, Pat, PatIdent,
};

use crate::{blocks::bind_in_block, locals::build_monadic_bind};

pub(crate) fn bind_expr(expr: Expr) -> (ExprBinder, Expr) {
	let mut binder = ExprBinder::default();
	let non_monadic_expr = binder.fold_expr(expr);
	(binder, non_monadic_expr)
}

#[derive(Default)]
pub(crate) struct ExprBinder {
	temp_binds: VecDeque<TempBind>,
}

impl ExprBinder {
	pub(crate) fn build_binds(mut self, then_block: Block) -> Expr {
		if let Some(temp_bind) = self.temp_binds.pop_front() {
			let then_expr = self.build_binds(then_block);
			let then_block: Block = parse_quote! { { #then_expr } };
			build_monadic_bind(
				&temp_bind.ident_as_pat(),
				&temp_bind.monadic_expr,
				&then_block,
			)
		} else {
			parse_quote! { #then_block }
		}
	}

	pub(crate) fn needs_binding(&self) -> bool {
		!self.temp_binds.is_empty()
	}

	fn add_temp_bind(&mut self, expr: Expr, span: Span) -> &TempBind {
		fn generate_locally_unique_ident(
			temp_binds: &VecDeque<TempBind>,
			span: Span,
		) -> Ident {
			let index = temp_binds.len();
			let name = format!("__monads_rs_temp_ident_{index}");
			Ident::new(&name, span.resolved_at(Span::mixed_site()))
		}

		let temp_ident = generate_locally_unique_ident(&self.temp_binds, span);
		let temp_bind = TempBind {
			ident: temp_ident,
			monadic_expr: expr,
		};
		self.temp_binds.push_back(temp_bind);
		&self.temp_binds[self.temp_binds.len() - 1]
	}

	fn handle_try_expr(&mut self, try_expr: ExprTry) -> Expr {
		let original_span = try_expr.span();
		let inner_expr = self.fold_expr(*try_expr.expr);
		let temp_bind = self.add_temp_bind(inner_expr, original_span);
		Expr::Path(temp_bind.ident_as_expr_path())
	}
}

impl Fold for ExprBinder {
	fn fold_block(&mut self, block: Block) -> Block {
		bind_in_block(block)
	}

	fn fold_expr_closure(&mut self, closure: ExprClosure) -> ExprClosure {
		// Ignore anything within closures
		closure
	}

	fn fold_expr(&mut self, expr: Expr) -> Expr {
		match expr {
			Expr::Try(try_expr) => self.handle_try_expr(try_expr),
			_ => fold_expr(self, expr),
		}
	}
}

struct TempBind {
	ident: Ident,
	monadic_expr: Expr,
}

impl TempBind {
	fn ident_as_expr_path(&self) -> ExprPath {
		ExprPath {
			attrs: Vec::new(),
			qself: None,
			path: self.ident.clone().into(),
		}
	}

	fn ident_as_pat(&self) -> Pat {
		Pat::Ident(PatIdent {
			attrs: Vec::new(),
			by_ref: None,
			mutability: None,
			ident: self.ident.clone(),
			subpat: None,
		})
	}
}
