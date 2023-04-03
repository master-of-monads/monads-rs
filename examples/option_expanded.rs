#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use monads_rs::*;
fn main() {
	if let Some(aggregated_reading) = aggregate_sensor_readings() {
		{
			::std::io::_print(format_args!(
				"Sensors read: {0}\n",
				aggregated_reading
			));
		};
	} else {
		{
			::std::io::_print(format_args!(
				"One or more sensors could not be read\n"
			));
		};
	}
}
fn aggregate_sensor_readings() -> Option<f32> {
	::monads_rs::Monad::bind(
		read_unreliable_sensor(),
		move |__monads_rs_temp_ident_0| {
			::monads_rs::Monad::bind(
				read_unreliable_sensor(),
				move |__monads_rs_temp_ident_1| {
					{
						Option::ret(
							(__monads_rs_temp_ident_0
								+ __monads_rs_temp_ident_1) / 2.0,
						)
					}
				},
			)
		},
	)
}
fn read_unreliable_sensor() -> Option<f32> {
	::monads_rs::Monad::bind(Some(false), move |__monads_rs_temp_ident_0| {
		if __monads_rs_temp_ident_0 {
			Option::ret(42.0)
		} else {
			::monads_rs::Monad::bind(
				Some(true),
				move |__monads_rs_temp_ident_0| {
					if __monads_rs_temp_ident_0 {
						Option::ret(24.0)
					} else {
						Option::ret(12.0)
					}
				},
			)
		}
	})
}
