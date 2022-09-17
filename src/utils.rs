use uefi::table::{Boot, SystemTable};

use crate::SYSTEM_TABLE;

pub fn sleep(microseconds: usize) {
    get_systable().boot_services().stall(microseconds);
}

pub fn get_systable() -> &'static mut SystemTable<Boot> {
    unsafe {
        SYSTEM_TABLE
            .as_mut()
            .expect("The system table handle is not available")
    }
}