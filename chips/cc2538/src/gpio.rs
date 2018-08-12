use self::Pin::*;
use core::cell::Cell;
use core::ops::{Index, IndexMut};
use kernel::common::regs::ReadWrite;
use kernel::hil;
use ioc;


const PORT_PINS: usize = 8;
const BASE_ADDRESS: usize = 0x400D9000;
const PORT_OFFSET: usize = 0x1000;

#[repr(C)]
struct GPIORegisters {
    //Data control
    pub data: [ReadWrite<u8>; 0x400], //offset=0x400 means 0x100 32-bit words
    //_reserved0: [u8; 0x3FC],
    pub dir: ReadWrite<u32>,    
    //Pad interrupt control
    pub is: ReadWrite<u32>,
    pub ibe: ReadWrite<u32>,
    pub iev: ReadWrite<u32>,
    pub ie: ReadWrite<u32>,
    pub ris: ReadWrite<u32>,
    pub mis: ReadWrite<u32>,
    pub ic: ReadWrite<u32>,
    //Mode control
    pub afsel: ReadWrite<u32>,
    _reserved1: [u8; 0xFC],
    //Commit Control
    pub gpiolock: ReadWrite<u32>,
    pub gpiocr: ReadWrite<u32>,
    _reserved2: [u8; 0x1D8],
    
    pub pmux: ReadWrite<u32>,
    
    //Power up interrupt control
    pub p_edge_ctrl: ReadWrite<u32>,
    pub pi_ien: ReadWrite<u32>,
    pub irq_detect_ack: ReadWrite<u32>,
    pub usb_irq_ack: ReadWrite<u32>,
    pub irq_detect_unmask: ReadWrite<u32>,

}

pub struct GPIOPin {
    regs: *const GPIORegisters,
    pin: usize,
    pin_mask: u32,
    client_data: Cell<usize>,
    client: Cell<Option<&'static hil::gpio::Client>>,
}

#[derive(Copy,Clone)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub enum Pin {

    PA00, PA01, PA02, PA03, PA04, PA05, PA06, PA07,

    PB00, PB01, PB02, PB03, PB04, PB05, PB06, PB07,

    PC00, PC01, PC02, PC03, PC04, PC05, PC06, PC07,

    PD00, PD01, PD02, PD03, PD04, PD05, PD06, PD07,
} 


impl GPIOPin {
    const fn new(pin: Pin) -> GPIOPin {
        GPIOPin {
            regs: (BASE_ADDRESS + ((pin as usize) / 8) * PORT_OFFSET) as *mut GPIORegisters,
            pin: pin as usize,
            pin_mask: 1 << ((pin as u32) % 8),
            client_data: Cell::new(0),
            client: Cell::new(None),
        }
    }

    pub fn enable_gpio(&self) {
        let regs: &GPIORegisters = unsafe {&*self.regs};
        regs.afsel.set(regs.afsel.get() & !self.pin_mask);
    } 

    
    pub fn set_client<C: hil::gpio::Client>(&self, client: &'static C) {
        self.client.set(Some(client));
    }

    pub fn handle_interrupt(&self) {
        self.client.get().map(|client| {
            client.fired(self.client_data.get());
        });
    }

    //Peripheral (/hardware) output
    pub fn select_hardware_output(&self, peripheral: usize) {
        let regs: &GPIORegisters = unsafe {&*self.regs};
        regs.afsel.set(regs.afsel.get() | self.pin_mask);
        ioc::select_peripheral_output(self.pin, peripheral);   
    }

    //Peripheral (/hardware) input
    pub fn select_hardware_input(&self, peripheral: usize) {
        let regs: &GPIORegisters = unsafe {&*self.regs};
        regs.afsel.set(regs.afsel.get() | self.pin_mask);
        ioc::select_peripheral_input(peripheral, self.pin);    
    }

    pub fn alt_override(&self, mode: usize) {
        ioc::override_config(self.pin, mode);
    }

    pub fn get_pin(&self) -> u8 {
        self.pin as u8
    }

    
}

impl hil::gpio::PinCtl for GPIOPin {
    fn set_input_mode(&self, mode: hil::gpio::InputMode) {
        ioc::set_input_mode(self.pin, mode);
    }

}


impl hil::gpio::Pin for GPIOPin {

    fn make_output(&self) {
        self.enable_gpio();
        let regs: &GPIORegisters = unsafe {&*self.regs};
        regs.dir.set(regs.dir.get() | self.pin_mask);
    }

    fn make_input(&self) {
        self.enable_gpio();
        let regs: &GPIORegisters = unsafe {&*self.regs};
        regs.dir.set(regs.dir.get() & !self.pin_mask);
    }

    //TODO
    fn enable_interrupt(&self, client_data: usize, mode: hil::gpio::InterruptMode) {
        self.client_data.set(client_data);
        let regs: &GPIORegisters = unsafe {&*self.regs};

        //clear interrupt
        regs.ic.set(regs.ic.get() | self.pin_mask);
        //Edge-detected interrupt
        regs.is.set(regs.is.get() & !self.pin_mask);
        //enable interrupt
        regs.ie.set(regs.ie.get() | self.pin_mask);

        let edge_mode = match mode {
            hil::gpio::InterruptMode::EitherEdge => 0,
            hil::gpio::InterruptMode::FallingEdge => 1,
            hil::gpio::InterruptMode::RisingEdge => 1,
        };

        if edge_mode == 0 {
            regs.ibe.set(regs.ibe.get() | self.pin_mask);
        }
        else {
            let edge_iev = match mode {
            hil::gpio::InterruptMode::FallingEdge => (regs.iev.get() & !self.pin_mask),
            hil::gpio::InterruptMode::RisingEdge => (regs.iev.get() | self.pin_mask),
            _ => regs.iev.get(),
            };
            regs.iev.set(edge_iev);
        }

    }

    fn disable_interrupt(&self) {
        let regs: &GPIORegisters = unsafe {&*self.regs};
        regs.ie.set(regs.ie.get() & !self.pin_mask);
    }


    // ? IS THIS THE RIGHT WAY
    fn disable(&self) {
        hil::gpio::PinCtl::set_input_mode(self, hil::gpio::InputMode::PullNone);
    }

    fn set(&self) {
        let regs: &GPIORegisters = unsafe{&*self.regs};
        regs.data[(self.pin_mask as usize) << 2].set(0xFF);
    }

    fn clear(&self) {
        let regs: &GPIORegisters = unsafe{&*self.regs};
        regs.data[(self.pin_mask as usize) << 2].set(0x00);
    }

    //TODO - XOR
    fn toggle(&self) {
        let regs: &GPIORegisters = unsafe{&*self.regs};
        let toggled_data = regs.data[(self.pin_mask as usize) << 2].get() ^ 0xFF;
        regs.data[(self.pin_mask as usize) << 2].set(toggled_data);
    }
  
    //TODO - see if pin_mask and input != 0 ?
    fn read(&self) -> bool {
        let regs: &GPIORegisters = unsafe{&*self.regs};
        (regs.dir.get() & self.pin_mask) == 0
    } 

}



pub struct Port {
    regs: *mut GPIORegisters,
    pins: [GPIOPin; PORT_PINS],
}

impl Index<usize> for Port {
    type Output = GPIOPin;

    fn index(&self, index: usize) -> &GPIOPin {
        &self.pins[index]
    }
}

impl IndexMut<usize> for Port {
    fn index_mut(&mut self, index: usize) -> &mut GPIOPin {
        &mut self.pins[index]
    }
}

impl Port {
    pub fn handle_interrupt(&self) {
        let regs: &GPIORegisters = unsafe { &*self.regs };

        let ir_status = regs.mis.get();

        // ? About to handle all the interrupts, so just clear them now to get
        // over with it.
        //port.ifr.clear.set(!0); 

        loop {
            let pin = ir_status.trailing_zeros() as usize;
            if pin >= self.pins.len() {
                break;
            } 
            else {
                self.pins[pin].handle_interrupt();
            }
        }
    }
}


//Port A
pub static mut PA: Port = Port {
    regs: (BASE_ADDRESS + 0 * PORT_OFFSET) as *mut GPIORegisters,
    pins: [
        GPIOPin::new(PA00),
        GPIOPin::new(PA01),
        GPIOPin::new(PA02),
        GPIOPin::new(PA03),
        GPIOPin::new(PA04),
        GPIOPin::new(PA05),
        GPIOPin::new(PA06),
        GPIOPin::new(PA07),
    ],
};


//Port B
pub static mut PB: Port = Port {
    regs: (BASE_ADDRESS + 1 * PORT_OFFSET) as *mut GPIORegisters,
    pins: [
        GPIOPin::new(PB00),
        GPIOPin::new(PB01),
        GPIOPin::new(PB02),
        GPIOPin::new(PB03),
        GPIOPin::new(PB04),
        GPIOPin::new(PB05),
        GPIOPin::new(PB06),
        GPIOPin::new(PB07),
    ],
};

//Port C
pub static mut PC: Port = Port {
    regs: (BASE_ADDRESS + 2 * PORT_OFFSET) as *mut GPIORegisters,
    pins: [
        GPIOPin::new(PC00),
        GPIOPin::new(PC01),
        GPIOPin::new(PC02),
        GPIOPin::new(PC03),
        GPIOPin::new(PC04),
        GPIOPin::new(PC05),
        GPIOPin::new(PC06),
        GPIOPin::new(PC07),
    ],
};


//Port D
pub static mut PD: Port = Port {
    regs: (BASE_ADDRESS + 3 * PORT_OFFSET) as *mut GPIORegisters,
    pins: [
        GPIOPin::new(PD00),
        GPIOPin::new(PD01),
        GPIOPin::new(PD02),
        GPIOPin::new(PD03),
        GPIOPin::new(PD04),
        GPIOPin::new(PD05),
        GPIOPin::new(PD06),
        GPIOPin::new(PD07),
    ],
};
