use std::error::Error;

use eden::core::servo::Servo;
use rppal::gpio::Gpio;

const SERVO: u8 = 23;

fn main() -> Result<(), Box<dyn Error>> {

    let mut servo: Servo = Servo::new(SERVO)?;
    
    servo.move_to(180)?;

    Ok(())
}
