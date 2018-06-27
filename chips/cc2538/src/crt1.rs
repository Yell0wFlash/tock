use cortexm3::{generic_isr, nvic, systick_handler, SVC_Handler};

/*
 * Adapted from crt1.c which was relicensed by the original author from
 * GPLv3 to Apache 2.0.
 * The original version of the file, under GPL can be found at
 * https://github.com/SoftwareDefinedBuildings/
 *     stormport/blob/rebase0/tos/platforms/storm/stormcrt1.c
 *
 * Copyright 2016, Michael Andersen <m.andersen@eecs.berkeley.edu>
 */

extern "C" {
    // Symbols defined in the linker file
    static mut _erelocate: u32;
    static mut _etext: u32;
    static mut _ezero: u32;
    static mut _srelocate: u32;
    static mut _szero: u32;
    fn reset_handler();

    // _estack is not really a function, but it makes the types work
    // You should never actually invoke it!!
    fn _estack();
}

unsafe extern "C" fn unhandled_interrupt() {
    'loop0: loop {}
}

unsafe extern "C" fn hard_fault_handler() {
    'loop0: loop {}
}

#[link_section=".vectors"]
//#[cfg_attr(rustfmt, rustfmt_skip)]
// no_mangle Ensures that the symbol is kept until the final binary
//Must somehow direct vector table to correct address ?
#[no_mangle]
pub static BASE_VECTORS: [unsafe extern fn(); 164] = [
    _estack, // -
    reset_handler, //reset
    unhandled_interrupt, // -
    hard_fault_handler, // 3 Hard Fault
    unhandled_interrupt, // memory management
    unhandled_interrupt, // Bus fault
    unhandled_interrupt, // Usage fault
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // 10 Reserved
    SVC_Handler, // SVC
    unhandled_interrupt, // Debug monitor,
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // PendSV
    systick_handler, // Systick
    generic_isr, // GPIO Port A
    generic_isr, // GPIO Port B
    generic_isr, // GPIO Port C
    generic_isr, // GPIO Port D
    unhandled_interrupt, // 20 Reserved
    generic_isr, // UART0
    generic_isr, // UART1
    unhandled_interrupt, // SSI0
    unhandled_interrupt, // I2C
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // 30 ADC
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved    
    unhandled_interrupt, // Watchdog Timer
    unhandled_interrupt, // GPTimer 0A
    unhandled_interrupt, // GPTimer 0B
    unhandled_interrupt, // GPTimer 1A
    unhandled_interrupt, // GPTimer 1B
    unhandled_interrupt, // GPTimer 2A    
    unhandled_interrupt, //40  GPTimer 2B
    unhandled_interrupt, // Analog Comparator
    unhandled_interrupt, // RF TX/RX (Alternate)
    unhandled_interrupt, // RF Error (Alternate)
    unhandled_interrupt, // System Control
    unhandled_interrupt, // Flash memory control    
    unhandled_interrupt, // AES (Alternate)
    unhandled_interrupt, // PKA (Alternate)
    unhandled_interrupt, // SM Timer (Alternate)
    unhandled_interrupt, // MAC Timer (Alternate)
    unhandled_interrupt, //50 SSI1
    unhandled_interrupt, // GPTimer 3A
    unhandled_interrupt, // GPTimer 3B 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, //60 Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // µDMA software
    unhandled_interrupt, // µDMA error
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, //70 Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, //80 Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, //90 Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, //100 Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, //110 Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, //120 Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, //130 Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, //140 Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, //150 Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // Reserved 
    unhandled_interrupt, // Reserved
    unhandled_interrupt, // USB
    unhandled_interrupt, // RF Core Rx/Tx
    unhandled_interrupt, // RF Core Error 
    unhandled_interrupt, // AES
    unhandled_interrupt, //160 PKA
    unhandled_interrupt, // SM Timer
    unhandled_interrupt, // MAC Timer
    unhandled_interrupt, // Reserved
];

pub struct flash_cca_lock_page_t {
    bootldr_cfg: u32,
    image_valid: u32,
    vector_table: u32, 
    lock: [u8; 32],
}

#[no_mangle]
#[link_section = ".ccfg"]
pub static CCFG_CONF: flash_cca_lock_page_t = flash_cca_lock_page_t {
    bootldr_cfg: 0xF6,
    image_valid: 0x0,
    vector_table: 0x00200000,//&BASE_VECTORS as *const u32,
    lock:  [ 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
     0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
     0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
     0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF ],

};

/* #[link_section = ".vectors"]
#[no_mangle] // Ensures that the symbol is kept until the final binary
pub static IRQS: [unsafe extern "C" fn(); 80] = [generic_isr; 80]; */

#[no_mangle]
pub unsafe extern "C" fn init() {
    let mut current_block;
    let mut p_src: *mut u32;
    let mut p_dest: *mut u32;

    // Move the relocate segment. This assumes it is located after the text
    // segment, which is where the storm linker file puts it
    p_src = &mut _etext as (*mut u32);
    p_dest = &mut _srelocate as (*mut u32);
    if p_src != p_dest {
        current_block = 1;
    } else {
        current_block = 2;
    }
    'loop1: loop {
        if current_block == 1 {
            if !(p_dest < &mut _erelocate as (*mut u32)) {
                current_block = 2;
                continue;
            }
            *{
                let _old = p_dest;
                p_dest = p_dest.offset(1isize);
                _old
            } = *{
                let _old = p_src;
                p_src = p_src.offset(1isize);
                _old
            };
            current_block = 1;
        } else {
            p_dest = &mut _szero as (*mut u32);
            break;
        }
    }
    'loop3: loop {
        if !(p_dest < &mut _ezero as (*mut u32)) {
            break;
        }
        *{
            let _old = p_dest;
            p_dest = p_dest.offset(1isize);
            _old
        } = 0u32;
    }
    nvic::enable_all();
}
