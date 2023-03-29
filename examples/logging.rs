#![cfg(feature = "logging")]

use monads_rs::logging::Logging;
use monads_rs::*;

fn main() {
	let success = startup_nuclear_reactor().run();
}

#[monadic]
fn startup_nuclear_reactor() -> Logging<bool> {
	run_safety_checks()?;
	activate_warning_lights()?;

	let core_temp = get_core_temp()?;

	if core_temp < 200.0 {
		Logging::log("Reactor core temperature is nominal")?;
		Logging::ret(true)
	} else {
		Logging::log("Reactor core is too hot, abort")?;
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
	let mut core_temp_arg = 0.0;
	for i in 1..=3 {
		core_temp_arg += 40.0 + (i as f32);
		Logging::log(format!(
			"Reading core temperature sensor #{0}: {1}c",
			i,
			40.0 + (i as f32)
		))?;
	}
	Logging::ret(core_temp_arg / 4.0)
}
