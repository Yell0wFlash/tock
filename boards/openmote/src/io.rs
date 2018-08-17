use cc2538;
use core::fmt::Write;
use core::panic::PanicInfo;
use cortexm3;
use kernel::debug;
use kernel::hil::led;
use kernel::hil::uart::{self, UART};


struct Writer {
    initialized: bool,
}

static mut WRITER: Writer = Writer { initialized: false };

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        let uart = unsafe { &mut cc2538::uart::UART0};
        if !self.initialized {
            self.initialized = true;
            uart.init(uart::UARTParams {
                baud_rate: 115200,
                stop_bits: uart::StopBits::One,
                parity: uart::Parity::None,
                hw_flow_control: false,
            });
        }
        for c in s.bytes() {
            uart.send_data(c);
            while uart.tx_ready() {}
        }
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
    debug::panic(&mut [led], writer, pi, &cortexm3::support::nop)
}
