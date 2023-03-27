#![feature(prelude_import)]
#![allow(unreachable_code, unused_braces, unused_variables)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use monads_rs::*;
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "simple"]
pub const simple: test::TestDescAndFn = test::TestDescAndFn {
	desc: test::TestDesc {
		name: test::StaticTestName("simple"),
		ignore: false,
		ignore_message: ::core::option::Option::None,
		compile_fail: false,
		no_run: false,
		should_panic: test::ShouldPanic::No,
		test_type: test::TestType::IntegrationTest,
	},
	testfn: test::StaticTestFn(|| test::assert_test_result(simple())),
};
fn simple() {
	match (&simple_some(), &Some(12)) {
		(left_val, right_val) => {
			if !(*left_val == *right_val) {
				let kind = ::core::panicking::AssertKind::Eq;
				::core::panicking::assert_failed(
					kind,
					&*left_val,
					&*right_val,
					::core::option::Option::None,
				);
			}
		}
	};
	match (&simple_none(), &None) {
		(left_val, right_val) => {
			if !(*left_val == *right_val) {
				let kind = ::core::panicking::AssertKind::Eq;
				::core::panicking::assert_failed(
					kind,
					&*left_val,
					&*right_val,
					::core::option::Option::None,
				);
			}
		}
	};
	fn simple_some() -> Option<usize> {
		fn __monadicsimple_some(
		) -> ::monads_rs::control_flow::ControlFlowAction<Option<usize>> {
			return ::monads_rs::control_flow::ControlFlowAction::Exit(Some(
				12,
			));
			<::monads_rs::control_flow::ControlFlowAction<
                _,
                _,
            > as ::monads_rs::control_flow::FlatFrom<_>>::flat_from(Some(24))
		}
		__monadicsimple_some().unwrap()
	}
	fn simple_none() -> Option<usize> {
		fn __monadicsimple_none(
		) -> ::monads_rs::control_flow::ControlFlowAction<Option<usize>> {
			return ::monads_rs::control_flow::ControlFlowAction::Exit(None);
			<::monads_rs::control_flow::ControlFlowAction<
                _,
                _,
            > as ::monads_rs::control_flow::FlatFrom<_>>::flat_from(Some(24))
		}
		__monadicsimple_none().unwrap()
	}
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "bind_after"]
pub const bind_after: test::TestDescAndFn = test::TestDescAndFn {
	desc: test::TestDesc {
		name: test::StaticTestName("bind_after"),
		ignore: false,
		ignore_message: ::core::option::Option::None,
		compile_fail: false,
		no_run: false,
		should_panic: test::ShouldPanic::No,
		test_type: test::TestType::IntegrationTest,
	},
	testfn: test::StaticTestFn(|| test::assert_test_result(bind_after())),
};
fn bind_after() {
	match (&bind_after_return(), &Some(12)) {
		(left_val, right_val) => {
			if !(*left_val == *right_val) {
				let kind = ::core::panicking::AssertKind::Eq;
				::core::panicking::assert_failed(
					kind,
					&*left_val,
					&*right_val,
					::core::option::Option::None,
				);
			}
		}
	};
	fn bind_after_return() -> Option<usize> {
		fn __monadicbind_after_return(
		) -> ::monads_rs::control_flow::ControlFlowAction<Option<usize>> {
			return ::monads_rs::control_flow::ControlFlowAction::Exit(Some(
				12,
			));
			<::monads_rs::control_flow::ControlFlowAction<
                _,
                _,
            > as ::monads_rs::control_flow::FlatFrom<
                _,
            >>::flat_from(
                <::monads_rs::control_flow::ControlFlowAction<
                    _,
                    _,
                > as ::monads_rs::control_flow::FlatFrom<_>>::flat_from(Some(24))
                    .bind(|__monads_rs_temp_ident_0| {
                        let a = __monads_rs_temp_ident_0;
                        <::monads_rs::control_flow::ControlFlowAction<
                            _,
                            _,
                        > as ::monads_rs::control_flow::FlatFrom<_>>::flat_from(Some(a))
                    }),
            )
		}
		__monadicbind_after_return().unwrap()
	}
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "bind_before"]
pub const bind_before: test::TestDescAndFn = test::TestDescAndFn {
	desc: test::TestDesc {
		name: test::StaticTestName("bind_before"),
		ignore: false,
		ignore_message: ::core::option::Option::None,
		compile_fail: false,
		no_run: false,
		should_panic: test::ShouldPanic::No,
		test_type: test::TestType::IntegrationTest,
	},
	testfn: test::StaticTestFn(|| test::assert_test_result(bind_before())),
};
fn bind_before() {
	match (&bind_before_return(), &Some(12)) {
		(left_val, right_val) => {
			if !(*left_val == *right_val) {
				let kind = ::core::panicking::AssertKind::Eq;
				::core::panicking::assert_failed(
					kind,
					&*left_val,
					&*right_val,
					::core::option::Option::None,
				);
			}
		}
	};
	fn bind_before_return() -> Option<usize> {
		fn __monadicbind_before_return(
		) -> ::monads_rs::control_flow::ControlFlowAction<Option<usize>> {
			<::monads_rs::control_flow::ControlFlowAction<
                _,
                _,
            > as ::monads_rs::control_flow::FlatFrom<
                _,
            >>::flat_from(
                <::monads_rs::control_flow::ControlFlowAction<
                    _,
                    _,
                > as ::monads_rs::control_flow::FlatFrom<_>>::flat_from(Some(24))
                    .bind(|__monads_rs_temp_ident_0| {
                        let a = __monads_rs_temp_ident_0;
                        return ::monads_rs::control_flow::ControlFlowAction::Exit(
                            Some(12),
                        );
                        <::monads_rs::control_flow::ControlFlowAction<
                            _,
                            _,
                        > as ::monads_rs::control_flow::FlatFrom<_>>::flat_from(Some(a))
                    }),
            )
		}
		__monadicbind_before_return().unwrap()
	}
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "bind_before_no_implicit_return"]
pub const bind_before_no_implicit_return: test::TestDescAndFn =
	test::TestDescAndFn {
		desc: test::TestDesc {
			name: test::StaticTestName("bind_before_no_implicit_return"),
			ignore: false,
			ignore_message: ::core::option::Option::None,
			compile_fail: false,
			no_run: false,
			should_panic: test::ShouldPanic::No,
			test_type: test::TestType::IntegrationTest,
		},
		testfn: test::StaticTestFn(|| {
			test::assert_test_result(bind_before_no_implicit_return())
		}),
	};
fn bind_before_no_implicit_return() {
	match (&bind_bind_before_no_implicit_return(), &Some(12)) {
		(left_val, right_val) => {
			if !(*left_val == *right_val) {
				let kind = ::core::panicking::AssertKind::Eq;
				::core::panicking::assert_failed(
					kind,
					&*left_val,
					&*right_val,
					::core::option::Option::None,
				);
			}
		}
	};
	fn bind_bind_before_no_implicit_return() -> Option<usize> {
		fn __monadicbind_bind_before_no_implicit_return(
		) -> ::monads_rs::control_flow::ControlFlowAction<Option<usize>> {
			<::monads_rs::control_flow::ControlFlowAction<
                _,
                _,
            > as ::monads_rs::control_flow::FlatFrom<
                _,
            >>::flat_from(
                <::monads_rs::control_flow::ControlFlowAction<
                    _,
                    _,
                > as ::monads_rs::control_flow::FlatFrom<_>>::flat_from(Some(12))
                    .bind(|__monads_rs_temp_ident_0| {
                        let a = __monads_rs_temp_ident_0;
                        return ::monads_rs::control_flow::ControlFlowAction::Exit(
                            Some(a),
                        );
                    }),
            )
		}
		__monadicbind_bind_before_no_implicit_return().unwrap()
	}
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "deep_bind"]
pub const deep_bind: test::TestDescAndFn = test::TestDescAndFn {
	desc: test::TestDesc {
		name: test::StaticTestName("deep_bind"),
		ignore: false,
		ignore_message: ::core::option::Option::None,
		compile_fail: false,
		no_run: false,
		should_panic: test::ShouldPanic::No,
		test_type: test::TestType::IntegrationTest,
	},
	testfn: test::StaticTestFn(|| test::assert_test_result(deep_bind())),
};
fn deep_bind() {
	match (&deep_bind_return(), &Some(12)) {
		(left_val, right_val) => {
			if !(*left_val == *right_val) {
				let kind = ::core::panicking::AssertKind::Eq;
				::core::panicking::assert_failed(
					kind,
					&*left_val,
					&*right_val,
					::core::option::Option::None,
				);
			}
		}
	};
	fn deep_bind_return() -> Option<usize> {
		fn __monadicdeep_bind_return(
		) -> ::monads_rs::control_flow::ControlFlowAction<Option<usize>> {
			<::monads_rs::control_flow::ControlFlowAction<
                _,
                _,
            > as ::monads_rs::control_flow::FlatFrom<
                _,
            >>::flat_from(
                <::monads_rs::control_flow::ControlFlowAction<
                    _,
                    _,
                > as ::monads_rs::control_flow::FlatFrom<_>>::flat_from(Some(24))
                    .bind(|__monads_rs_temp_ident_0| {
                        let a = __monads_rs_temp_ident_0;
                        <::monads_rs::control_flow::ControlFlowAction<
                            _,
                            _,
                        > as ::monads_rs::control_flow::FlatFrom<
                            _,
                        >>::flat_from(
                            <::monads_rs::control_flow::ControlFlowAction<
                                _,
                                _,
                            > as ::monads_rs::control_flow::FlatFrom<
                                _,
                            >>::flat_from(Some(a))
                                .bind(|__monads_rs_temp_ident_0| {
                                    let a = __monads_rs_temp_ident_0;
                                    <::monads_rs::control_flow::ControlFlowAction<
                                        _,
                                        _,
                                    > as ::monads_rs::control_flow::FlatFrom<
                                        _,
                                    >>::flat_from(
                                        <::monads_rs::control_flow::ControlFlowAction<
                                            _,
                                            _,
                                        > as ::monads_rs::control_flow::FlatFrom<
                                            _,
                                        >>::flat_from(Some(a))
                                            .bind(|__monads_rs_temp_ident_0| {
                                                let a = __monads_rs_temp_ident_0;
                                                <::monads_rs::control_flow::ControlFlowAction<
                                                    _,
                                                    _,
                                                > as ::monads_rs::control_flow::FlatFrom<
                                                    _,
                                                >>::flat_from(
                                                    <::monads_rs::control_flow::ControlFlowAction<
                                                        _,
                                                        _,
                                                    > as ::monads_rs::control_flow::FlatFrom<
                                                        _,
                                                    >>::flat_from(Some(a))
                                                        .bind(|__monads_rs_temp_ident_0| {
                                                            let a = __monads_rs_temp_ident_0;
                                                            return ::monads_rs::control_flow::ControlFlowAction::Exit(
                                                                Some(12),
                                                            );
                                                            <::monads_rs::control_flow::ControlFlowAction<
                                                                _,
                                                                _,
                                                            > as ::monads_rs::control_flow::FlatFrom<
                                                                _,
                                                            >>::flat_from(
                                                                <::monads_rs::control_flow::ControlFlowAction<
                                                                    _,
                                                                    _,
                                                                > as ::monads_rs::control_flow::FlatFrom<
                                                                    _,
                                                                >>::flat_from(Some(a))
                                                                    .bind(|__monads_rs_temp_ident_0| {
                                                                        let a = __monads_rs_temp_ident_0;
                                                                        <::monads_rs::control_flow::ControlFlowAction<
                                                                            _,
                                                                            _,
                                                                        > as ::monads_rs::control_flow::FlatFrom<
                                                                            _,
                                                                        >>::flat_from(
                                                                            <::monads_rs::control_flow::ControlFlowAction<
                                                                                _,
                                                                                _,
                                                                            > as ::monads_rs::control_flow::FlatFrom<
                                                                                _,
                                                                            >>::flat_from(Some(a))
                                                                                .bind(|__monads_rs_temp_ident_0| {
                                                                                    let a = __monads_rs_temp_ident_0;
                                                                                    <::monads_rs::control_flow::ControlFlowAction<
                                                                                        _,
                                                                                        _,
                                                                                    > as ::monads_rs::control_flow::FlatFrom<
                                                                                        _,
                                                                                    >>::flat_from(
                                                                                        <::monads_rs::control_flow::ControlFlowAction<
                                                                                            _,
                                                                                            _,
                                                                                        > as ::monads_rs::control_flow::FlatFrom<
                                                                                            _,
                                                                                        >>::flat_from(Some(a))
                                                                                            .bind(|__monads_rs_temp_ident_0| {
                                                                                                let a = __monads_rs_temp_ident_0;
                                                                                                <::monads_rs::control_flow::ControlFlowAction<
                                                                                                    _,
                                                                                                    _,
                                                                                                > as ::monads_rs::control_flow::FlatFrom<
                                                                                                    _,
                                                                                                >>::flat_from(Some(a))
                                                                                            }),
                                                                                    )
                                                                                }),
                                                                        )
                                                                    }),
                                                            )
                                                        }),
                                                )
                                            }),
                                    )
                                }),
                        )
                    }),
            )
		}
		__monadicdeep_bind_return().unwrap()
	}
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "nested_implicit_return"]
pub const nested_implicit_return: test::TestDescAndFn = test::TestDescAndFn {
	desc: test::TestDesc {
		name: test::StaticTestName("nested_implicit_return"),
		ignore: false,
		ignore_message: ::core::option::Option::None,
		compile_fail: false,
		no_run: false,
		should_panic: test::ShouldPanic::No,
		test_type: test::TestType::IntegrationTest,
	},
	testfn: test::StaticTestFn(|| {
		test::assert_test_result(nested_implicit_return())
	}),
};
fn nested_implicit_return() {
	match (&bind_nested_implicit_return(), &Some(12)) {
		(left_val, right_val) => {
			if !(*left_val == *right_val) {
				let kind = ::core::panicking::AssertKind::Eq;
				::core::panicking::assert_failed(
					kind,
					&*left_val,
					&*right_val,
					::core::option::Option::None,
				);
			}
		}
	};
	fn bind_nested_implicit_return() -> Option<usize> {
		fn __monadicbind_nested_implicit_return(
		) -> ::monads_rs::control_flow::ControlFlowAction<Option<usize>> {
			<::monads_rs::control_flow::ControlFlowAction<
                _,
                _,
            > as ::monads_rs::control_flow::FlatFrom<_>>::flat_from({ Some(12) })
		}
		__monadicbind_nested_implicit_return().unwrap()
	}
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "monadic_block"]
pub const monadic_block: test::TestDescAndFn = test::TestDescAndFn {
	desc: test::TestDesc {
		name: test::StaticTestName("monadic_block"),
		ignore: false,
		ignore_message: ::core::option::Option::None,
		compile_fail: false,
		no_run: false,
		should_panic: test::ShouldPanic::No,
		test_type: test::TestType::IntegrationTest,
	},
	testfn: test::StaticTestFn(|| test::assert_test_result(monadic_block())),
};
fn monadic_block() {
	match (&bind_monadic_block(), &Some(12)) {
		(left_val, right_val) => {
			if !(*left_val == *right_val) {
				let kind = ::core::panicking::AssertKind::Eq;
				::core::panicking::assert_failed(
					kind,
					&*left_val,
					&*right_val,
					::core::option::Option::None,
				);
			}
		}
	};
	fn bind_monadic_block() -> Option<usize> {
		fn __monadicbind_monadic_block(
		) -> ::monads_rs::control_flow::ControlFlowAction<Option<usize>> {
			<::monads_rs::control_flow::ControlFlowAction<
                _,
                _,
            > as ::monads_rs::control_flow::FlatFrom<
                _,
            >>::flat_from(
                <::monads_rs::control_flow::ControlFlowAction<
                    _,
                    _,
                > as ::monads_rs::control_flow::FlatFrom<
                    _,
                >>::flat_from({ Some(12_usize) })
                    .bind(|__monads_rs_temp_ident_0| {
                        let a = __monads_rs_temp_ident_0;
                        <::monads_rs::control_flow::ControlFlowAction<
                            _,
                            _,
                        > as ::monads_rs::control_flow::FlatFrom<_>>::flat_from(Some(a))
                    }),
            )
		}
		__monadicbind_monadic_block().unwrap()
	}
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "non_monadic_block"]
pub const non_monadic_block: test::TestDescAndFn = test::TestDescAndFn {
	desc: test::TestDesc {
		name: test::StaticTestName("non_monadic_block"),
		ignore: false,
		ignore_message: ::core::option::Option::None,
		compile_fail: false,
		no_run: false,
		should_panic: test::ShouldPanic::No,
		test_type: test::TestType::IntegrationTest,
	},
	testfn: test::StaticTestFn(
		|| test::assert_test_result(non_monadic_block()),
	),
};
fn non_monadic_block() {
	match (&bind_non_monadic_block(), &Some(12)) {
		(left_val, right_val) => {
			if !(*left_val == *right_val) {
				let kind = ::core::panicking::AssertKind::Eq;
				::core::panicking::assert_failed(
					kind,
					&*left_val,
					&*right_val,
					::core::option::Option::None,
				);
			}
		}
	};
	fn bind_non_monadic_block() -> Option<usize> {
		fn __monadicbind_non_monadic_block(
		) -> ::monads_rs::control_flow::ControlFlowAction<Option<usize>> {
			let a = { 12 };
			<::monads_rs::control_flow::ControlFlowAction<
                _,
                _,
            > as ::monads_rs::control_flow::FlatFrom<_>>::flat_from(Some(a))
		}
		__monadicbind_non_monadic_block().unwrap()
	}
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "bind_for_other_types"]
pub const bind_for_other_types: test::TestDescAndFn = test::TestDescAndFn {
	desc: test::TestDesc {
		name: test::StaticTestName("bind_for_other_types"),
		ignore: false,
		ignore_message: ::core::option::Option::None,
		compile_fail: false,
		no_run: false,
		should_panic: test::ShouldPanic::No,
		test_type: test::TestType::IntegrationTest,
	},
	testfn: test::StaticTestFn(|| {
		test::assert_test_result(bind_for_other_types())
	}),
};
fn bind_for_other_types() {
	match (&bind_for_other_types_return(), &Some(12)) {
		(left_val, right_val) => {
			if !(*left_val == *right_val) {
				let kind = ::core::panicking::AssertKind::Eq;
				::core::panicking::assert_failed(
					kind,
					&*left_val,
					&*right_val,
					::core::option::Option::None,
				);
			}
		}
	};
	fn bind_for_other_types_return() -> Option<usize> {
		fn __monadicbind_for_other_types_return(
		) -> ::monads_rs::control_flow::ControlFlowAction<Option<usize>> {
			<::monads_rs::control_flow::ControlFlowAction<
                _,
                _,
            > as ::monads_rs::control_flow::FlatFrom<
                _,
            >>::flat_from(
                <::monads_rs::control_flow::ControlFlowAction<
                    _,
                    _,
                > as ::monads_rs::control_flow::FlatFrom<
                    _,
                >>::flat_from({
                        <::monads_rs::control_flow::ControlFlowAction<
                            _,
                            _,
                        > as ::monads_rs::control_flow::FlatFrom<
                            _,
                        >>::flat_from(Some(true))
                            .bind(|__monads_rs_temp_ident_0| {
                                let b = __monads_rs_temp_ident_0;
                                return ::monads_rs::control_flow::ControlFlowAction::Exit(
                                    Some(12),
                                );
                                <::monads_rs::control_flow::ControlFlowAction<
                                    _,
                                    _,
                                > as ::monads_rs::control_flow::FlatFrom<
                                    _,
                                >>::flat_from(Some(b))
                            })
                    })
                    .bind(|__monads_rs_temp_ident_0| {
                        let b = __monads_rs_temp_ident_0;
                        <::monads_rs::control_flow::ControlFlowAction<
                            _,
                            _,
                        > as ::monads_rs::control_flow::FlatFrom<
                            _,
                        >>::flat_from(if b { Some(24) } else { Some(25) })
                    }),
            )
		}
		__monadicbind_for_other_types_return().unwrap()
	}
}
#[rustc_main]
pub fn main() -> () {
	extern crate test;
	test::test_main_static(&[
		&bind_after,
		&bind_before,
		&bind_before_no_implicit_return,
		&bind_for_other_types,
		&deep_bind,
		&monadic_block,
		&nested_implicit_return,
		&non_monadic_block,
		&simple,
	])
}
