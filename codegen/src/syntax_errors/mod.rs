use syn::ItemFn;

mod unhandled_binds;
mod early_return;

/// Reports all monadic related syntax errors
pub(crate) fn report_syntax_after(function: ItemFn) -> ItemFn {
	unhandled_binds::report_error_for_binds(function)
}

pub(crate) fn report_syntax_prior(function: ItemFn) -> ItemFn {
	early_return::report_error_for_early_return(function)
}
