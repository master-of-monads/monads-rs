use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse2, ItemFn};

mod unhandled_bind;

/// Reports all monadic related syntax errors
pub(crate) fn report_syntax_errors(token_stream: TokenStream) -> TokenStream {
	let function: ItemFn = match parse2(token_stream) {
		Ok(fun) => fun,
		Err(err) => return err.into_compile_error(),
	};

	let function = unhandled_bind::report_error_for_binds(function);

	function.into_token_stream()
}
