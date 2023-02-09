mod blocks;
mod locals;
mod syntax_errors;

use blocks::bind_in_block;
use quote::ToTokens;
use syn::{parse_macro_input, ItemFn};
use syntax_errors::report_syntax_errors;

/// An attribute for writing monadic functions using the `?` operator as a
/// do-notation style binding operator.
#[proc_macro_attribute]
pub fn monadic(
	_attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let function = parse_macro_input!(item as ItemFn);
	let function = convert_monadic_binds(function);
	let function = report_syntax_errors(function);
	function.into_token_stream().into()
}

fn convert_monadic_binds(mut function: ItemFn) -> ItemFn {
	function.block = Box::new(bind_in_block(*function.block));
	function
}
