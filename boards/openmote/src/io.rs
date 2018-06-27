use cc2538;
use core::fmt::Write;
use core::panic::PanicInfo;
use cortexm3;
use kernel::debug;
use kernel::hil::led;


struct Writer {
    initialized: bool,
}

static mut WRITER: Writer = Writer { initialized: false };

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        Ok(())
    }
}


#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub unsafe extern "C" fn panic_fmt(pi: &PanicInfo) -> ! {
    // PC4 = Red led

    let led = &mut led::LedLow::new(&mut cc2538::gpio::PC[4]);
    let writer = &mut WRITER;
    debug::panic(led, writer, pi, &cortexm3::support::nop)
}
