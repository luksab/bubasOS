#![no_std]
#![no_main]
#![feature(abi_efiapi)]
extern crate alloc;

mod print;
mod utils;
mod fs;
mod terminal;

use log::info;
use uefi::{prelude::*, proto::console::text::Input};
use uefi_services::{init, system_table};

use crate::utils::sleep;

static mut SYSTEM_TABLE: Option<SystemTable<Boot>> = None;
static mut SYSTEM_HANDLE: Option<Handle> = None;

#[entry]
fn efi_main(handle: Handle, mut sys_table: SystemTable<Boot>) -> Status {
    init(&mut sys_table).unwrap();

    // Disable the watchdog timer
    sys_table
        .boot_services()
        .set_watchdog_timer(0, 0x10000, None)
        .unwrap();

    unsafe {
        SYSTEM_TABLE = Some(sys_table.unsafe_clone());
    }

    unsafe {
        SYSTEM_HANDLE = Some(handle);
    }

    let protocol = unsafe {
        system_table()
            .as_ref()
            .boot_services()
            .locate_protocol::<Input>()
            .unwrap()
    };

    let keyboard = unsafe { &mut *protocol.get() };

    info!("Starting Bubas OS...");

    sys_table.stdout().clear().unwrap();

    terminal::run(keyboard).unwrap();

    info!("Exiting Bubas OS...");

    sleep(1_000_000);

    Status(0)
}
