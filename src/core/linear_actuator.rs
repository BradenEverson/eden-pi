use std::{error::Error, thread, time::Duration};

use rppal::gpio::{OutputPin, Gpio};

pub struct LinearActuator{
    fwd_pin: OutputPin,
    bkwd_pin: OutputPin,
    out_state: u64
}

impl LinearActuator {
    pub fn new(fwd: u8, gnd: u8) -> Result<Self, Box<dyn Error>> {
        let fwd_pin = Gpio::new()?.get(fwd)?.into_output();
        let bkwd_pin = Gpio::new()?.get(gnd)?.into_output();
        Ok(LinearActuator {fwd_pin, bkwd_pin, out_state: 0})
    }
    pub fn up(&mut self, dur_millis: u64) -> Result<(), Box<dyn Error>> {
        self.fwd_pin.set_high();
        self.bkwd_pin.set_low();
        thread::sleep(Duration::from_millis(dur_millis));
        self.fwd_pin.set_low();
        self.bkwd_pin.set_low();
        self.out_state += dur_millis;
        Ok(())
    }

    pub fn down(&mut self, dur_millis: u64) -> Result<(), Box<dyn Error>> {
        self.fwd_pin.set_low();
        self.bkwd_pin.set_high();
        thread::sleep(Duration::from_millis(dur_millis));
        self.fwd_pin.set_low();
        self.bkwd_pin.set_low();
        if dur_millis > self.out_state {
            self.out_state = 0;
        } else {
            self.out_state -= dur_millis;
        }
        Ok(())
    }
    pub fn go_back(&mut self) {
        self.bkwd_pin.set_high();
        self.fwd_pin.set_low();
        thread::sleep(Duration::from_millis(self.out_state));
        self.fwd_pin.set_low();
        self.bkwd_pin.set_low();
        self.out_state = 0;
    }
}
