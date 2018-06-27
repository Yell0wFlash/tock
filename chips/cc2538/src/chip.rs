use cortexm3::{self, nvic};
use kernel;


pub struct Cc2538 {
    mpu: (),
    systick: cortexm3::systick::SysTick,
}

impl Cc2538 {
    pub unsafe fn new() -> Cc2538 {
        Cc2538 {
            mpu: (),
            // The systick clocks with 48MHz by default
            systick: cortexm3::systick::SysTick::new_with_calibration(48 * 1000000),
        }
    }
}

impl kernel::Chip for Cc2538 {
    type MPU = ();
    type SysTick = cortexm3::systick::SysTick;

    fn mpu(&self) -> &Self::MPU {
    &self.mpu
    }

    fn systick(&self) -> &Self::SysTick {
        &self.systick
    }

    fn service_pending_interrupts(&mut self) {
        unsafe {
            while let Some(interrupt) = nvic::next_pending() {
                /*
                match interrupt {
                    GPIO => gpio::PORT.handle_interrupt(),
                    // AON Programmable interrupt
                    // We need to ignore JTAG events since some debuggers emit these
                    AON_PROG => (),
                    _ => panic!("unhandled interrupt {}", interrupt),
                }
                */
                let n = nvic::Nvic::new(interrupt);
                n.clear_pending();
                n.enable();
            }
        }
    }

    fn has_pending_interrupts(&self) -> bool {
        unsafe { nvic::has_pending() }
    }
}
