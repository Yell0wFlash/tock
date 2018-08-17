// Implementation of UART

use core::cell::Cell;
use kernel;
use gpio;
use kernel::common::regs::{ReadOnly, ReadWrite, WriteOnly};
use kernel::hil::uart;
use sys_ctrl;


const UART_BASE_ADDRS: [*mut UARTRegisters; 2] = [
    0x4000C000 as *mut UARTRegisters,
    0x4000D000 as *mut UARTRegisters,
];

//? 20 MHz
const UART_CLOCK: u32 = 20000000;


//? registers not totally equal between USART0 and 1, 
#[repr(C)]
struct UARTRegisters {
    dr: ReadWrite<u32>,
    rsr_ecr: ReadWrite<u32>,
    _reserved0: [u32; 0x4],
    fr: ReadOnly<u32, Flags::Register>,
    _reserved1: [u32; 0x2],
    ilpr: ReadWrite<u32>,
    ibrd: ReadWrite<u32, IntDivisor::Register>,
    fbrd: ReadWrite<u32, FracDivisor::Register>,
    lcrh: ReadWrite<u32, LineControl::Register>,
    ctl: ReadWrite<u32, Control::Register>,
    ifls: ReadWrite<u32>,
    im: ReadWrite<u32, Interrupts::Register>,
    ris: ReadOnly<u32, Interrupts::Register>,
    mis: ReadOnly<u32, Interrupts::Register>,
    icr: WriteOnly<u32, Interrupts::Register>,
    dmactl: ReadWrite<u32>,

    //There are more registers, unneccessary atm.
}


register_bitfields![
    u32,
    Flags [
        TX_FIFO_FULL OFFSET(5) NUMBITS(1) [],
        FX_FIFO_FULL OFFSET(6) NUMBITS(1) []
    ],
    IntDivisor [
        DIVINT OFFSET(0) NUMBITS(16) []
    ],
    FracDivisor [
        DIVFRAC OFFSET(0) NUMBITS(6) []
    ],
    LineControl [
        FIFO_ENABLE OFFSET(4) NUMBITS(1) [],
        WORD_LENGTH OFFSET(5) NUMBITS(2) [
            Len5 = 0x0,
            Len6 = 0x1,
            Len7 = 0x2,
            Len8 = 0x3
        ]
    ],
    Control [
        UART_ENABLE OFFSET(0) NUMBITS(1) [],
        HIGHSPEED_ENABLE OFFSET(5) NUMBITS(1) [],
        TX_ENABLE OFFSET(8) NUMBITS(1) [],
        RX_ENABLE OFFSET(9) NUMBITS(1) []
    ],
    Interrupts [
        ALL_INTERRUPTS OFFSET(4) NUMBITS(12) [] //? 
    ]
];


pub struct UART {
    regs: *mut UARTRegisters,
    client: Cell<Option<&'static uart::Client>>,
    tx_pin: Cell<Option<u8>>,
    rx_pin: Cell<Option<u8>>,
    uart_number: usize,
    //tx_gpio: &'static gpio::GPIOPin,
    //rx_gpio: &'static gpio::GPIOPin,
}


impl UART {
    const fn new(
        base_addr: *mut UARTRegisters,
        uart_nr: usize,
        //tx: &'static gpio::GPIOPin,
        //rx: &'static gpio::GPIOPin,
    ) -> UART {
        UART {
            regs: base_addr,
            client: Cell::new(None),
            tx_pin: Cell::new(None),
            rx_pin: Cell::new(None), 
            //tx_gpio: tx,
            //rx_gpio: rx,
            uart_number: uart_nr,
        }
    }

    //select pins to be connected to UART. Must be set before configuring. 
    pub fn set_uart_pins(&self, tx: u8, rx: u8) {
        self.tx_pin.set(Some(tx));
        self.rx_pin.set(Some(rx));
    }

    fn configure_uart(&self, params: kernel::hil::uart::UARTParams){
        
        //Check if set_uart_pins has been used
        let tx_pin = match self.tx_pin.get() {
            Some(pin) => pin,
            None => panic!("Tx pin not configured for UART. Set UART TX pin."),
        };

        let rx_pin = match self.rx_pin.get() {
            Some(pin) => pin,
            None => panic!("Rx pin not configured for UART, Set UART RX pin."),
        };
        
        //get the right port of the pin
        let txport = tx_pin/8; 
        let rxport = rx_pin/8;

        let uart_nbr = self.uart_number;
        
        //ENABLE UARTRCGC
        sys_ctrl::enable_rcgc_uart(uart_nbr);


        unsafe {

            //Finding correct port to enable GPIO Pin as UART
            match txport {
                1 => {
                    gpio::PA[tx_pin as usize].select_hardware_output(uart_nbr * 2);
                    gpio::PA[tx_pin as usize].alt_override(0x8);
                    gpio::PA[rx_pin as usize].select_hardware_input(uart_nbr * 2)
                },
                2 => {
                    gpio::PB[tx_pin as usize].select_hardware_output(uart_nbr * 2);
                    gpio::PB[tx_pin as usize].alt_override(0x8);
                    gpio::PB[rx_pin as usize].select_hardware_input(uart_nbr * 2)
                },
                3 => {
                    gpio::PC[tx_pin as usize].select_hardware_output(uart_nbr * 2);
                    gpio::PC[tx_pin as usize].alt_override(0x8);
                    gpio::PC[rx_pin as usize].select_hardware_input(uart_nbr * 2)
                },
                4 => {
                    gpio::PD[tx_pin as usize].select_hardware_output(uart_nbr * 2);
                    gpio::PD[tx_pin as usize].alt_override(0x8);
                    gpio::PD[rx_pin as usize].select_hardware_input(uart_nbr * 2)
                },
                _ => (),
            }
           
           /* 

           self.tx_gpio.select_hardware_output(uart_nbr * 2);

           self.tx_gpio.alt_override(0x8);

           self.rx_gpio.select_hardware_input(uart_nbr * 2);

           */


        }

        //sys_ctrl.. clocks ?

        self.disable_uart();

        self.set_baudrate(params.baud_rate);

        let regs = unsafe { &*self.regs };
        regs.lcrh.write(LineControl::WORD_LENGTH::Len8);

        self.enable_fifo();

        regs.ctl.write(
        	Control::UART_ENABLE::SET + Control::TX_ENABLE::SET + Control::RX_ENABLE::SET
        );

    }

    fn disable_uart(&self) {
    	let regs = unsafe { &*self.regs };
        regs.ctl.modify(Control::UART_ENABLE::CLEAR);
    }
    
    fn set_baudrate(&self, baudrate: u32) {
        let regs = unsafe { &*self.regs };
        let hse = regs.ctl.is_set(Control::HIGHSPEED_ENABLE);
        let clkdiv = match hse {
        	true => 16,
        	false => 8,
        };

        let brd: f64 = (UART_CLOCK/(clkdiv * baudrate)) as f64;
        let ibrd = brd as u32;
        let fbrd = ((brd-(ibrd as f64)) * 64.0 + 0.5) as u32;
        regs.ibrd.write(IntDivisor::DIVINT.val(ibrd));
        regs.fbrd.write(FracDivisor::DIVFRAC.val(fbrd));

    }

    fn enable_fifo(&self) {
        let regs = unsafe { &*self.regs };
        regs.lcrh.write(LineControl::FIFO_ENABLE::SET);
    }

    fn disable_fifo(&self) {
        let regs = unsafe { &*self.regs };
        regs.lcrh.modify(LineControl::FIFO_ENABLE::CLEAR);
    }

    pub fn tx_ready(&self) -> bool {
        let regs = unsafe { &*self.regs };
        !regs.fr.is_set(Flags::TX_FIFO_FULL)
    }

    pub fn send_data(&self, c: u8) {
        let regs = unsafe { &*self.regs };

        //Wait if FIFO is full
        while regs.fr.is_set(Flags::TX_FIFO_FULL) {}

        //write to data register
        regs.dr.set(c as u32);
    }
}


impl kernel::hil::uart::UART for UART {

    fn set_client(&self, client: &'static kernel::hil::uart::Client) {
        self.client.set(Some(client));
    }
   
    fn init(&self, params: kernel::hil::uart::UARTParams) {
        //Interrupts?
        self.configure_uart(params);
    }

    fn transmit(&self, tx_data: &'static mut [u8], tx_len: usize) {
        if tx_len == 0 {
            return;
        }

        for q in 0..tx_len {
            self.send_data(tx_data[q]);
        } 
        
        //?
        self.client.get().map(move |client| {
            client.transmit_complete(tx_data, kernel::hil::uart::Error::CommandComplete);
        });
    }

    #[allow(unused)]
    fn receive(&self, rx_buffer: &'static mut [u8], rx_len: usize) {

    }

    #[allow(unused)]
    fn abort_receive(&self){

    }
}


pub static mut UART0: UART = UART::new(
    UART_BASE_ADDRS[0],
    0,
    //unsafe {&gpio::PA[1]},
    //unsafe {&gpio::PA[0]},
);
pub static mut UART1: UART = UART::new(
    UART_BASE_ADDRS[1],
    1,
    //unsafe {&gpio::PA[2]},
    //unsafe {&gpio::PA[3]},
);