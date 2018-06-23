//! Sample capsule for Tock course at SenSys. It handles an alarm to
//! sample the ambient light sensor.

#![feature(const_fn, const_cell_new)]
#![feature(infer_outlives_requirements)]
#![no_std]

#[allow(unused_imports)]
#[macro_use(debug)]
extern crate kernel;

use kernel::hil::sensors::{AmbientLight, AmbientLightClient};
#[allow(unused_imports)]
use kernel::hil::time::{self, Alarm, Frequency};

#[allow(unused)]
pub struct Sensys<'a, A: Alarm> {
    alarm: &'a A,
    light: &'a AmbientLight,
}

impl<'a, A: Alarm> Sensys<'a, A> {
    pub fn new(alarm: &'a A, light: &'a AmbientLight) -> Sensys<'a, A> {
        Sensys {
            alarm: alarm,
            light: light,
        }
    }

    pub fn start(&self) {
        debug!("Hello World");
    }
}

impl<'a, A: Alarm> time::Client for Sensys<'a, A> {
    fn fired(&self) {}
}

impl<'a, A: Alarm> AmbientLightClient for Sensys<'a, A> {
    fn callback(&self, _lux: usize) {}
}
