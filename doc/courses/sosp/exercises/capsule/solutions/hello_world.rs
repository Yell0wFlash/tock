//! Sample capsule for Tock course at SOSP. Prints 'Hello World'

#![feature(const_fn,const_cell_new)]
#![no_std]

#[allow(unused_imports)]
#[macro_use(debug)]
extern crate kernel;

use kernel::hil::time::{self, Alarm, Frequency};
use kernel::hil::sensors::{AmbientLight, AmbientLightClient};

pub struct Sosp<'a, A: Alarm >  {
    alarm: &'a A,
    light: &'a AmbientLight,
}

impl<'a, A: Alarm> Sosp<'a, A> {
    pub fn new(alarm: &'a A, light: &'a AmbientLight) -> Sosp<'a, A> {
        Sosp {
           alarm: alarm,
           light: light,
        }
    }

    pub fn start(&self) {
        debug!("Hello World");
    }
}

impl<'a, A: Alarm> time::Client for Sosp<'a, A> {
    fn fired(&self) {
    }
}

impl<'a, A: Alarm> AmbientLightClient for Sosp<'a, A> {
    fn callback(&self, lux: usize) {
    }
}
