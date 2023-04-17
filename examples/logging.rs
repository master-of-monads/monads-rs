#![cfg(feature = "logging")]

use std::cell::RefCell;
use std::rc::Rc;

use monads_rs::logging::Logging;
use monads_rs::*;

fn main() {
	let startup = startup_nuclear_reactor();
	let (success, logs) = startup.run();
	println!("Startup success: {success}");
	for log in logs {
		println!("{log}");
	}
}

#[monadic]
fn startup_nuclear_reactor() -> Logging<bool> {
	run_safety_checks()?;
	activate_warning_lights()?;

	let core_temp = get_core_temp()?;

	if core_temp < 200.0 {
		Logging::log(format!(
			"Reactor core temperature is nominal ({core_temp})"
		))?;
		Logging::ret(true)
	} else {
		Logging::log(format!("Reactor core is too hot, abort ({core_temp})"))?;
		Logging::ret(false)
	}
}

#[monadic]
fn run_safety_checks() -> Logging<()> {
	Logging::log("Running safety checks...")?;
	Logging::log("All is OK")?;
	Logging::ret(())
}

#[monadic]
fn activate_warning_lights() -> Logging<()> {
	Logging::log("Warning lights activated")?;
	Logging::ret(())
}

#[monadic]
fn get_core_temp() -> Logging<f32> {
	let core_temp_arg = Rc::new(RefCell::new(0.0));
	let core_temp_arg_clone = core_temp_arg.clone();
	for i in 0..4 {
		*core_temp_arg_clone.borrow_mut() += 40.0 + (i as f32);
		Logging::log(format!(
			"Reading core temperature sensor #{0}: {1}c",
			i,
			40.0 + (i as f32)
		))?;
	}?;
	Logging::ret(*core_temp_arg.borrow() / 4.0)
}
