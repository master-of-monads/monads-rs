use syn::ItemFn;

mod unhandled_binds;

/// Reports all monadic related syntax errors
pub(crate) fn report_syntax_errors(function: ItemFn) -> ItemFn {
	unhandled_binds::report_error_for_binds(function)
}
