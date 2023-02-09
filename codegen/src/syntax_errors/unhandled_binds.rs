use syn::{
	fold::{fold_expr, fold_item_fn, Fold},
	Error, Expr, Item, ItemFn,
};

/// Outputs a diagnostics error for all binds (`?`)
pub(crate) fn report_error_for_binds(function: ItemFn) -> ItemFn {
	BindReporter::report_function(function)
}

#[derive(Default)]
struct BindReporter {}

impl BindReporter {
	fn report_function(function: ItemFn) -> ItemFn {
		fold_item_fn(&mut BindReporter::default(), function)
	}
}

impl Fold for BindReporter {
	fn fold_expr(&mut self, expr: Expr) -> Expr {
		if let Expr::Try(try_expr) = expr {
			Expr::Verbatim(
				Error::new_spanned(
					try_expr.question_token,
					"monadic bind cannot be used at this point",
				)
				.into_compile_error(),
			)
		} else {
			fold_expr(self, expr)
		}
	}

	fn fold_item(&mut self, item: Item) -> Item {
		// Ignore anything within local item definitions
		item
	}
}
