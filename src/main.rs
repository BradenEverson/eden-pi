use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};

const SERVO: u8 = 23;

// Servo configuration. Change these values based on your servo's verified safe
// minimum and maximum values.
//
// Period: 20 ms (50 Hz). Pulse width: min. 1200 µs, neutral 1500 µs, max. 1800 µs.
const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 1200;
const PULSE_NEUTRAL_US: u64 = 1500;
const PULSE_MAX_US: u64 = 1800;

fn move_servo(servo: &mut OutputPin, to: u64) -> Result<(), Box<dyn Error>> {
    for pulse in (PULSE_MIN_US..=to).step_by(10) {
        servo.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(pulse),
            )?;
        thread::sleep(Duration::from_millis(20));
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(SERVO)?.into_output();

    move_servo(&mut pin, PULSE_NEUTRAL_US)?;
    // Rotate the servo to its neutral (center) position in small steps.

    Ok(())
}
