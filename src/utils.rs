use uefi::{table::{Boot, SystemTable}, Handle};

use crate::{SYSTEM_TABLE, SYSTEM_HANDLE};

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

pub fn get_handle() -> &'static mut Handle {
    unsafe {
        SYSTEM_HANDLE
            .as_mut()
            .expect("The system handle is not available")
    }
}
