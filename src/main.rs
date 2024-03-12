use std::{error::Error, thread, time::Duration};

use eden::core::servo::Servo;
use rppal::gpio::Gpio;

const SERVO: u8 = 23;

fn main() -> Result<(), Box<dyn Error>> {

    let mut servo: Servo = Servo::new(SERVO)?;
    
    loop {
        servo.move_to(90)?;
        thread::sleep(Duration::from_millis(500));
        servo.move_to(0)?;
        thread::sleep(Duration::from_millis(500));
    }


    Ok(())
}
