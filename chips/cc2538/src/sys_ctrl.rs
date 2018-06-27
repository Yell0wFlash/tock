//System control driver for managing clock, power modes etc.

use kernel::common::regs::ReadWrite;


#[repr(C)]
struct SYSCTRLRegisters {
    clock_ctrl: ReadWrite<u32>,
    clock_sta: ReadWrite<u32>,
    rcgc_gpt: ReadWrite<u32>,
    scgc_gpt: ReadWrite<u32>,
    dcgc_gpt: ReadWrite<u32>,
    sr_gpt: ReadWrite<u32>,
    rcgc_ssi: ReadWrite<u32>,
    scgc_ssi: ReadWrite<u32>,
    dcgc_ssi: ReadWrite<u32>,
    sr_ssi: ReadWrite<u32>,
    rcgc_uart: ReadWrite<u32, Clocks::Register>,
    scgc_uart: ReadWrite<u32>,
    dcgc_uart: ReadWrite<u32>,
    sr_uart: ReadWrite<u32>,

    //more registers

}


register_bitfields![
    u32,
    Clocks [
        UART0_CLOCK_ENABLED OFFSET(0) NUMBITS(1) [],
        UART1_CLOCK_ENABLED OFFSET(1) NUMBITS(1) []
    ]
];

const SYS_CTRL_BASEADDR: *mut SYSCTRLRegisters =  0x400D2000 as *mut SYSCTRLRegisters;


pub fn enable_rcgc_uart(uart: usize){
	let regs: &SYSCTRLRegisters = unsafe { &*SYS_CTRL_BASEADDR };
	
	if uart == 0 {
		regs.rcgc_uart.write(Clocks::UART0_CLOCK_ENABLED::SET);
	}
	else if uart == 1 {
		regs.rcgc_uart.write(Clocks::UART1_CLOCK_ENABLED::SET);
	}
}