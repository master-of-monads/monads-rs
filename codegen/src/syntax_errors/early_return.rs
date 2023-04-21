use syn::{
	fold::{fold_expr, fold_item_fn, Fold},
	Error, Expr, ExprClosure, Item, ItemFn,
};

/// Outputs a diagnostics on all explicit returns, the purpose being to avoid any early returns.
pub(crate) fn report_error_for_early_return(function: ItemFn) -> ItemFn {
	EarlyReturnReporter::report_function(function)
}

#[derive(Default)]
struct EarlyReturnReporter {}

impl EarlyReturnReporter {
	fn report_function(function: ItemFn) -> ItemFn {
		fold_item_fn(&mut EarlyReturnReporter::default(), function)
	}
}

impl Fold for EarlyReturnReporter {
	fn fold_expr(&mut self, expr: Expr) -> Expr {
		if let Expr::Return(_) = expr {
			Expr::Verbatim(
				Error::new_spanned(
					&expr,
					"Explicit returns are not supported, please use implicit returns",
				)
				.into_compile_error(),
			)
		} else {
			fold_expr(self, expr)
		}
	}
}
