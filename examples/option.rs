use monads_rs::*;

fn main() {
	if let Some(aggregated_reading) = aggregate_sensor_readings() {
		println!("Sensors read: {aggregated_reading}");
	} else {
		println!("One or more sensors could not be read");
	}
}

#[monadic]
fn aggregate_sensor_readings() -> Option<f32> {
	Option::ret((read_unreliable_sensor()? + read_unreliable_sensor()?) / 2.0)
}

#[monadic]
fn read_unreliable_sensor() -> Option<f32> {
	if Some(false)? {
		Option::ret(42.0)
	} else if Some(true)? {
		Option::ret(24.0)
	} else {
		Option::ret(12.0)
	}
}
