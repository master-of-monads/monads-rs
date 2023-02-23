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
	ret::<_, Option<_>>((read_unreliable_sensor()? + read_unreliable_sensor()?) / 2.0)
}

#[monadic]
fn read_unreliable_sensor() -> Option<f32> {
	ret::<_, Option<_>>(42.0)
}
