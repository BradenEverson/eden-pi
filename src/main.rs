use std::{error::Error, thread, time::Duration};

use eden::core::{servo::Servo, linear_actuator::LinearActuator};
use rppal::gpio::Gpio;

const SERVO: u8 = 23;
const FWD: u8 = 21;
const BKWD: u8 = 20;

fn main() -> Result<(), Box<dyn Error>> {
    let mut lin: LinearActuator = LinearActuator::new(FWD, BKWD)?;

    lin.up(10000)?;
    lin.go_back();


    Ok(())
}
