use std::{error::Error, time::Duration, thread};

use rppal::gpio::{OutputPin, Gpio};

const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 1200;
const PULSE_NEUTRAL_US: u64 = 1500;
const PULSE_MAX_US: u64 = 2500;

pub struct Servo {
    pub state: u64,
    out_pin: OutputPin
}

impl Servo {
    pub fn new(pin: u8) -> Result<Self, Box<dyn Error>> {
        let mut out_pin = Gpio::new()?.get(pin)?.into_output();
        Servo::move_servo(&mut out_pin, PULSE_NEUTRAL_US, PULSE_MIN_US)?;
        thread::sleep(Duration::from_millis(500));

        Ok(Servo { state: PULSE_MIN_US, out_pin })
    }
    
    pub fn deg_to_pulse(deg: u8) -> u64 {
        (11.1111 * (deg as f32)) as u64 + 500
    }

    pub fn move_to(&mut self, to_deg: u8) -> Result<(), Box<dyn Error>> {
        let pulse_to = Servo::deg_to_pulse(to_deg);

        Servo::move_servo(&mut self.out_pin, self.state, Servo::deg_to_pulse(to_deg))?;
        self.state = pulse_to;
        println!("{}", self.state);

        Ok(())
    }

    fn move_servo(servo: &mut OutputPin, from: u64, to: u64) -> Result<(), Box<dyn Error>> {
        for pulse in (from..=to).step_by(10) {
            servo.set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(pulse),
                )?;
            thread::sleep(Duration::from_millis(20));
        }
        Ok(())
    }

}
