use std::error::Error;

use eden::core::servo::move_to_deg;
use rppal::gpio::Gpio;

const SERVO: u8 = 23;

fn main() -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(SERVO)?.into_output();

    move_to_deg(&mut pin, 180)?;

    Ok(())
}
