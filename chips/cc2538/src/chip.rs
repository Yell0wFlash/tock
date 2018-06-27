use cortexm3::{self, nvic};
use kernel;
use gpio;
use peripheral_interrupts::*;


pub struct Cc2538 {
    mpu: (),    
    systick: cortexm3::systick::SysTick,
}

impl Cc2538 {
    pub unsafe fn new() -> Cc2538 {
        Cc2538 {
            mpu: (),
            // The systick clocks with 32MHz by default
            systick: cortexm3::systick::SysTick::new_with_calibration(32 * 1000000),
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
                
                match interrupt {
                    GPIOA => gpio::PA.handle_interrupt(),
                    GPIOB => gpio::PB.handle_interrupt(),
                    GPIOC => gpio::PC.handle_interrupt(),
                    GPIOD => gpio::PD.handle_interrupt(),
                    // AON Programmable interrupt
                    // We need to ignore JTAG events since some debuggers emit these
                    //AON_PROG => (),
                    _ => panic!("unhandled interrupt {}", interrupt),
                }
                
                let n = nvic::Nvic::new(interrupt);
                n.clear_pending();
                n.enable();
            }
        }
    }

    fn has_pending_interrupts(&self) -> bool {
        unsafe { nvic::has_pending() }
    }

    fn sleep(&self) {
        unsafe {
            cortexm3::support::wfi();
        }
    }

    unsafe fn atomic<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        cortexm3::support::atomic(f)
    }
}
