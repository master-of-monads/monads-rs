use std::collections::VecDeque;

use proc_macro2::{Ident, Span};
use syn::{
	fold::{fold_expr, fold_expr_if, Fold},
	parse_quote, parse_quote_spanned,
	spanned::Spanned,
	token::Else,
	Block, Expr, ExprClosure, ExprIf, ExprPath, ExprTry, Pat, PatIdent,
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
		let bind_span = try_expr.span();
		let inner_expr =
			self.handle_expr_in_try_expr(*try_expr.expr, bind_span);
		let temp_bind = self.add_temp_bind(inner_expr, bind_span);
		Expr::Path(temp_bind.ident_as_expr_path())
	}

	fn handle_expr_in_try_expr(
		&mut self,
		mut expr: Expr,
		bind_span: Span,
	) -> Expr {
		match expr {
			Expr::If(ref mut if_expr) if if_expr.else_branch.is_none() => {
				let else_expr = parse_quote_spanned! { bind_span =>
					{
						::monads_rs::ret(())
					}
				};
				if_expr.else_branch =
					Some((Else(Span::call_site()), Box::new(else_expr)));
			}
			_ => {}
		}

		self.fold_expr(expr)
	}

	fn handle_else_branch(&mut self, else_expr: Expr) -> Expr {
		let Expr::If(else_if) = else_expr else {
			return else_expr;
		};

		parse_quote! {
			{
				#else_if
			}
		}
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

	fn fold_expr_if(&mut self, mut if_expr: ExprIf) -> ExprIf {
		if let Some(mut else_branch) = if_expr.else_branch {
			let else_expr = self.handle_else_branch(*else_branch.1);
			else_branch.1 = Box::new(else_expr);
			if_expr.else_branch = Some(else_branch);
		}
		fold_expr_if(self, if_expr)
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
