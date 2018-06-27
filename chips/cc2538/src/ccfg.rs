//! CCFG - Customer Configuration
//!
//! For details see p. 710 in the cc2650 technical reference manual.
//!
//! Currently setup to use the default settings.

#[no_mangle]
#[link_section = ".ccfg"]
pub static CCFG_CONF: [u32; 22] = [
    0x01800000, 0xFF820010, 0x0058FFFD, 0xF3FFFF3A, //0xF3BFFF3A,
    0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
    0x00FFFFFF, 0xFFFFFFFF, 0xFFFFFF00, 0xFFC5C5C5, 0xFFC5C5C5,
    0x00000000, // Set image as valid
    0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
];
/* We need to define the bootloader config */
// typedef struct {
//   uint32_t bootldr_cfg; /**< Bootloader backdoor configuration (page bytes 2004 - 2007) */
//   uint32_t image_valid; /**< Image valid (page bytes 2008 - 2011) */
//   const void *app_entry_point; /**< Flash vector table address (page bytes 2012 - 2015) */
//   uint8_t lock[32]; /**< Page and debug lock bits (page bytes 2016 - 2047) */
// } flash_cca_lock_page_t;
// __attribute__((__section__(".flashcca")))
// const flash_cca_lock_page_t CCFG_CONF = {
//   FLASH_CCA_BOOTLDR_CFG,        /* Boot loader backdoor configuration */
//   FLASH_CCA_IMAGE_VALID,        /* Image valid */
//   &vectors,                     /* Vector table */
//   /* Unlock all pages and debug */
//   { 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
//     0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
//     0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
//     0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF }
// };