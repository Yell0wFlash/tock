extern crate nrf52;

use nrf52::uart::UARTE0;
use kernel::hil::uart::{self, UART};


const BUFFER_SIZE_2048: usize = 2048;

pub unsafe fn run() {

    &nrf52::uart::UARTE0.init(uart::UARTParams {
            baud_rate: 115200,
            stop_bits: uart::StopBits::One,
            parity: uart::Parity::None,
            hw_flow_control: false,
        });

    let buf = static_init!([u8; BUFFER_SIZE_2048], [0; BUFFER_SIZE_2048]);

    for (ascii_char, b) in (33..126).cycle().zip(buf.iter_mut()) {
        *b = ascii_char;
    }

	for _x in 0..3000 {

		for _y in 0..1000 {
	   
		}
	 }

    transmit_entire_buffer(buf);
}

#[allow(unused)]
unsafe fn transmit_entire_buffer(buf: &'static mut [u8]) {
    &UARTE0.transmit(buf, BUFFER_SIZE_2048);
}
