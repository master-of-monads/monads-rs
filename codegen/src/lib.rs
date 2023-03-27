mod blocks;
mod exprs;
mod locals;
mod syntax_errors;

use blocks::bind_in_function_block;
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use std::mem::take;
use syn::{
	parse_macro_input, parse_quote, parse_quote_spanned, spanned::Spanned,
	Expr, ExprCall, FnArg, Item, ItemFn, ReturnType, Stmt,
};
use syntax_errors::report_syntax_errors;

/// An attribute for writing monadic functions using the `?` operator as a
/// do-notation style binding operator.
#[proc_macro_attribute]
pub fn monadic(
	_attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let function = parse_macro_input!(item as ItemFn);
	let function = build_visible_function(function);
	let function = report_syntax_errors(function);
	function.into_token_stream().into()
}

fn build_visible_function(mut function: ItemFn) -> ItemFn {
	let mut visible_function = duplicate_function(&mut function);
	let shadow_function = build_shadow_function(function);
	let shadow_call = build_shadow_function_call(&shadow_function);
	visible_function.block.stmts = vec![
		Stmt::Item(Item::Fn(shadow_function)),
		Stmt::Expr(shadow_call),
	];
	visible_function
}

fn duplicate_function(function: &mut ItemFn) -> ItemFn {
	let stmts = take(&mut function.block.stmts);
	let other_function = function.clone();
	function.block.stmts = stmts;
	other_function
}

fn build_shadow_function(mut function: ItemFn) -> ItemFn {
	function.block = Box::new(bind_in_function_block(*function.block));
	function.sig.ident = prefix_shadow_function_ident(function.sig.ident);
	function.sig.output = wrap_return_type_in_control_flow(function.sig.output);
	function
}

fn prefix_shadow_function_ident(ident: Ident) -> Ident {
	let name = format!("__monadic{ident}");
	Ident::new(&name, ident.span().resolved_at(Span::mixed_site()))
}

fn wrap_return_type_in_control_flow(return_type: ReturnType) -> ReturnType {
	match return_type {
		ReturnType::Default => {
			parse_quote_spanned! { return_type.span() =>
				-> ::monads_rs::control_flow::ControlFlowAction<()>
			}
		}
		ReturnType::Type(r_arrow, ty) => {
			parse_quote! {
				#r_arrow ::monads_rs::control_flow::ControlFlowAction<#ty>
			}
		}
	}
}

fn build_shadow_function_call(shadow_function: &ItemFn) -> Expr {
	let function_name = &shadow_function.sig.ident;
	let mut call_expr: ExprCall = parse_quote! { #function_name() };
	call_expr
		.args
		.extend(shadow_function.sig.inputs.iter().map(fn_arg_to_expr));
	parse_quote! { #call_expr.unwrap() }
}

fn fn_arg_to_expr(arg: &FnArg) -> Expr {
	match arg {
		FnArg::Receiver(receiver) => {
			let self_token = receiver.self_token;
			parse_quote_spanned! { self_token.span() => #self_token }
		}
		FnArg::Typed(arg) => {
			let pat = &arg.pat;
			parse_quote_spanned! { pat.span() => #pat }
		}
	}
}
