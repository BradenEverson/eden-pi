use std::{error::Error, time::Duration, thread};

use rppal::gpio::OutputPin;


const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 1200;
const PULSE_NEUTRAL_US: u64 = 1500;
const PULSE_MAX_US: u64 = 2300;

fn deg_to_pulse(deg: u8) -> u64 {
    ((10.0 / 3.0) * (deg as f32) + 1200.0) as u64
}

fn move_servo(servo: &mut OutputPin, to: u64) -> Result<(), Box<dyn Error>> {
    for pulse in (PULSE_MIN_US..=to).step_by(10) {
        servo.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(pulse),
            )?;
        thread::sleep(Duration::from_millis(50));
    }
    Ok(())
}

pub fn move_to_deg(servo: &mut OutputPin, to_deg: u8) -> Result<(), Box<dyn Error>> {
    move_servo(servo, PULSE_MAX_US)
}
