#![feature(prelude_import)]
#![cfg(feature = "logging")]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use monads_rs::logging::Logging;
use monads_rs::*;
fn main() {
	let program = startup_nuclear_reactor();
	let (success, logs) = program.run();
	{
		::std::io::_print(format_args!("Startup success: {0}\n", success));
	};
	{
		::std::io::_print(format_args!("Logs:\n"));
	};
	for log in logs {
		{
			::std::io::_print(format_args!("\t{0}\n", log));
		};
	}
}
fn startup_nuclear_reactor() -> Logging<bool> {
	let __monads_rs_temp_ident_0: usize = 12;
	::monads_rs::Monad::bind(
		run_safety_checks(),
		move |__monads_rs_temp_ident_0| {
			__monads_rs_temp_ident_0;
			::monads_rs::Monad::bind(
				activate_warning_lights(),
				move |__monads_rs_temp_ident_0| {
					__monads_rs_temp_ident_0;
					::monads_rs::Monad::bind(
						get_core_temp(),
						move |__monads_rs_temp_ident_0| {
							let core_temp = __monads_rs_temp_ident_0;
							if core_temp < 200.0 {
								::monads_rs::Monad::bind(
									Logging::log({
										let res = ::alloc::fmt::format(
                                                        format_args!(
                                                            "Reactor core temperature is nominal {0}",
                                                            __monads_rs_temp_ident_0
                                                        ),
                                                    );
										res
									}),
									move |__monads_rs_temp_ident_0| {
										__monads_rs_temp_ident_0;
										Logging::ret(true)
									},
								)
							} else {
								::monads_rs::Monad::bind(
									Logging::log(
										"Reactor core is too hot, abort",
									),
									move |__monads_rs_temp_ident_0| {
										__monads_rs_temp_ident_0;
										Logging::ret(false)
									},
								)
							}
						},
					)
				},
			)
		},
	)
}
fn run_safety_checks() -> Logging<()> {
	::monads_rs::Monad::bind(
		Logging::log("Running safety checks..."),
		move |__monads_rs_temp_ident_0| {
			__monads_rs_temp_ident_0;
			::monads_rs::Monad::bind(
				Logging::log("All is OK"),
				move |__monads_rs_temp_ident_0| {
					__monads_rs_temp_ident_0;
					Logging::ret(())
				},
			)
		},
	)
}
fn activate_warning_lights() -> Logging<()> {
	::monads_rs::Monad::bind(
		Logging::log("Warning lights activated"),
		move |__monads_rs_temp_ident_0| {
			__monads_rs_temp_ident_0;
			Logging::ret(())
		},
	)
}
fn get_core_temp() -> Logging<f32> {
	let mut core_temp_arg = 0.0;
	Logging::ret(core_temp_arg / 4.0)
}
