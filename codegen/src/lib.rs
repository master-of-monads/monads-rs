mod syntax_error;

use syntax_error::report_syntax_errors;

/// An attribute for writing monadic functions using the `?` operator as a
/// do-notation style binding operator.
#[proc_macro_attribute]
pub fn monadic(
	_attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	report_syntax_errors(item.into()).into()
}
