#![no_std]
#![no_main]
#![feature(lang_items, asm, panic_implementation)]

extern crate capsules;
extern crate cortexm3;
extern crate cc2538;

#[allow(unused_imports)]
#[macro_use(debug, debug_gpio, static_init)]
extern crate kernel;


#[macro_use]
pub mod io;

// How should the kernel respond when a process faults.
const FAULT_RESPONSE: kernel::procs::FaultResponse = kernel::procs::FaultResponse::Panic;

// Number of concurrent processes this platform supports.
const NUM_PROCS: usize = 2;
static mut PROCESSES: [Option<&'static mut kernel::procs::Process<'static>>; NUM_PROCS] =
    [None, None];

#[link_section = ".app_memory"]
// Give half of RAM to be dedicated APP memory
static mut APP_MEMORY: [u8; 0x4000] = [0; 0x4000];

/// Dummy buffer that causes the linker to reserve enough space for the stack.
#[no_mangle]
#[link_section = ".stack_buffer"]
pub static mut STACK_MEMORY: [u32; 256] = [0; 256];

pub struct Platform {

    //gpio: &'static capsules::gpio::GPIO<'static, cc2538::gpio::GPIOPin>,
    led: &'static capsules::led::LED<'static, cc2538::gpio::GPIOPin>,
    console: &'static capsules::console::Console<'static, cc2538::uart::UART>,
}

impl kernel::Platform for Platform {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
    where
        F: FnOnce(Option<&kernel::Driver>) -> R,
    {
        match driver_num {
            //capsules::gpio::DRIVER_NUM => f(Some(self.gpio)),
            capsules::led::DRIVER_NUM => f(Some(self.led)),
            //capsules::button::DRIVER_NUM => f(Some(self.button)),
            capsules::console::DRIVER_NUM => f(Some(self.console)),
            _ => f(None),
        }
    }
}


#[no_mangle]
pub unsafe fn reset_handler() {
    cc2538::init();

    // Setup AON event defaults
    //aon::AON_EVENT.setup();

    // Power on peripherals (eg. GPIO)
   // prcm::Power::enable_domain(prcm::PowerDomain::Peripherals);

    // Wait for it to turn on until we continue
    //while !prcm::Power::is_enabled(prcm::PowerDomain::Peripherals) {}

    let board_kernel = static_init!(kernel::Kernel, kernel::Kernel::new());

    // LEDs
    let led_pins = static_init!(
        [(&'static cc2538::gpio::GPIOPin, capsules::led::ActivationMode); 4],
        [
            (
                &cc2538::gpio::PC[4],
                capsules::led::ActivationMode::ActiveHigh
            ), // Red
            (
                &cc2538::gpio::PC[5],
                capsules::led::ActivationMode::ActiveLow
            ), // Orange
            (
                &cc2538::gpio::PC[6],
                capsules::led::ActivationMode::ActiveLow
            ), //Yellow
            (
                &cc2538::gpio::PC[7],
                capsules::led::ActivationMode::ActiveHigh
            ) //Green      
        ]
    );
    let led = static_init!(
        capsules::led::LED<'static, cc2538::gpio::GPIOPin>,
        capsules::led::LED::new(led_pins)
    );
    
    //UART
    cc2538::uart::UART0.set_uart_pins(cc2538::gpio::PA[1].get_pin(), cc2538::gpio::PA[0].get_pin());
    let console = static_init!(
        capsules::console::Console<cc2538::uart::UART>,
        capsules::console::Console::new(
            &cc2538::uart::UART0,
            115200,
            &mut capsules::console::WRITE_BUF,
            &mut capsules::console::READ_BUF,
            kernel::Grant::create()
        )
    );
    kernel::hil::uart::UART::set_client(&cc2538::uart::UART0, console);
    console.initialize();

    let mut chip = cc2538::chip::Cc2538::new();

    let openmote = Platform{
        led,
        console,
    };

    //debug!("Initialization complete. Entering main loop\r");

    extern "C" {
        /// Beginning of the ROM region containing app images.
        static _sapps: u8;
    }

    kernel::procs::load_processes(
        board_kernel,
        &_sapps as *const u8,
        &mut APP_MEMORY,
        &mut PROCESSES,
        FAULT_RESPONSE,
    );

    board_kernel.kernel_loop(
        &openmote,
        &mut chip,
        &mut PROCESSES,
        Some(&kernel::ipc::IPC::new()), 
    );
}

