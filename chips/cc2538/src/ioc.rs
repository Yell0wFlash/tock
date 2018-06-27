//IOC for selecting peripheral and 

use kernel::common::regs::ReadWrite;
use kernel::hil;

#[repr(C)]
pub struct SelRegisters {
    selpins: [ReadWrite<u32>; 32],
}

#[repr(C)]
pub struct OverRegisters {
    overpins: [ReadWrite<u32>; 32],
}

#[repr(C)]
pub struct PeripheralRegisters {
    peripheralregs: [ReadWrite<u32>; 14],
}


const SEL_BASE: *mut SelRegisters =  0x400D4000 as *mut SelRegisters;

const OVER_BASE: *mut OverRegisters = 0x400D4080 as *mut OverRegisters;

const PERIPHERAL_BASE: *mut PeripheralRegisters = 0x400D4100 as *mut PeripheralRegisters;

//IOC_PXX_SEL
pub fn select_peripheral_output(pin: usize, peripheral: usize) {
    let sel_regs: &SelRegisters = unsafe { &*SEL_BASE };
    sel_regs.selpins[pin].set(peripheral as u32);
}

//IOC_PXX_OVER (Used ONLY to set OE and ANA)
pub fn override_config(pin: usize, mode: usize) {
    let over_regs: &OverRegisters = unsafe { &*OVER_BASE };
    over_regs.overpins[pin].set(mode as u32);
}

//IOC_PXX_OVER (set PDE and PUE)
pub fn set_input_mode(pin: usize, mode: hil::gpio::InputMode) {
    let over_regs: &OverRegisters = unsafe { &*OVER_BASE };
    
    let in_mode = match mode {
        hil::gpio::InputMode::PullDown => 0x2,
        hil::gpio::InputMode::PullUp => 0x4,
        
        //AND with 0x9 (1001) clears bits for PDE and PUE
        hil::gpio::InputMode::PullNone => over_regs.overpins[pin].get() & 0x9,
    };

    over_regs.overpins[pin].set(in_mode);
}


pub fn select_peripheral_input(peripheral: usize, pin: usize) {
    let peripheral_regs: &PeripheralRegisters = unsafe { &*PERIPHERAL_BASE };
    peripheral_regs.peripheralregs[peripheral].set(pin as u32);

 }
