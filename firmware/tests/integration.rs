#![no_std]
#![no_main]

use ptubasehub as _; // memory layout + panic handler

// See https://crates.io/crates/defmt-test/0.3.0 for more documentation (e.g. about the 'state'
// feature)
#[defmt_test::tests]
mod tests {
    use defmt::assert;

    #[test]
    fn flash() {
        use embassy_stm32::flash;
        const FLASH_BASE: u32 = 0x8000000;
        const NEW_IMAGE_OFFSET: u32 = 0x8011000 - FLASH_BASE;
        // 30 2KB pages for each image
        const MAX_IMAGE_SIZE: usize = 29 * 2048;

        let p = unsafe { embassy_stm32::Peripherals::steal() };
        let mut flash = flash::Flash::new_blocking(p.FLASH).into_blocking_regions().bank1_region;
        let addr: u32 = 0x8000;
        flash.blocking_erase(addr, addr + 2 * 1024).unwrap();

        assert!(true)
    }
}
