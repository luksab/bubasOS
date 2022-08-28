#![no_std]
#![no_main]
#![feature(abi_efiapi)]
extern crate alloc;

use core::fmt::Write;

use alloc::string::String;
use log::info;
use uefi::{
    prelude::*,
    proto::console::text::{Input, Key, ScanCode},
    table::runtime::ResetType,
};
use uefi_services::{init, system_table};

static mut SYSTEM_TABLE: Option<SystemTable<Boot>> = None;
const PROMPT: &str = "> ";

macro_rules! print {
    ($($arg:tt)*) => {
        unsafe {
            let stdout = SYSTEM_TABLE
                .as_mut()
                .expect("The system table handle is not available")
                .stdout();
            write!(stdout, $($arg)*).unwrap();
        }
    }
}

macro_rules! println {
    ($($arg:tt)*) => {
        unsafe {
            let stdout = SYSTEM_TABLE
                .as_mut()
                .expect("The system table handle is not available")
                .stdout();
            write!(stdout, $($arg)*).unwrap();
            write!(stdout, "\n").unwrap();
        }
    }
}

fn read_line(keyboard: &mut Input) -> Result<String, uefi::Error> {
    let mut line = String::new();
    let stdout = unsafe {
        SYSTEM_TABLE
            .as_mut()
            .expect("The system table handle is not available")
            .stdout()
    };
    stdout.enable_cursor(true)?;

    loop {
        let key = match keyboard.read_key()? {
            Some(key) => key,
            None => continue,
        };
        match key {
            Key::Printable(c) => {
                let c = c.into();
                match c {
                    '\r' => break,
                    '\n' => {
                        line.push('\n');
                        break;
                    }
                    '\x08' => {
                        if line.len() > 0 {
                            line.pop();
                            print!("{}", c);
                        }
                    }
                    _ => {
                        print!("{}", c);
                        line.push(c)
                    }
                }
            }
            Key::Special(ScanCode::ESCAPE) => {
                println!("");
                return Ok(String::new());
            }
            _ => {}
        }
    }
    println!("");
    Ok(line)
}

fn sleep(microseconds: usize) {
    unsafe {
        SYSTEM_TABLE
            .as_mut()
            .expect("The system table handle is not available")
            .boot_services()
            .stall(microseconds);
    }
}

#[entry]
fn efi_main(_image: Handle, mut sys_table: SystemTable<Boot>) -> Status {
    init(&mut sys_table).unwrap();

    // Disable the watchdog timer
    sys_table
        .boot_services()
        .set_watchdog_timer(0, 0x10000, None)
        .unwrap();

    unsafe {
        SYSTEM_TABLE = Some(sys_table.unsafe_clone());
    }

    let protocol = unsafe {
        system_table()
            .as_ref()
            .boot_services()
            .locate_protocol::<Input>()
            .unwrap()
    };

    let keyboard = unsafe { &mut *protocol.get() };

    info!("Starting Bubas XP...");

    sys_table.stdout().clear()?;

    let start_time = unsafe {
        system_table()
            .as_ref()
            .runtime_services()
            .get_time()
            .unwrap()
    };

    loop {
        print!("{}", PROMPT);
        let line = read_line(keyboard)?;
        info!("{}", line);
        match line.as_str() {
            "exit" => break,
            "clear" => {
                sys_table.stdout().clear()?;
            }
            "shutdown" => sys_table
                .runtime_services()
                .reset(ResetType::Shutdown, Status(0), None),
            "uptime" => {
                let time = unsafe {
                    system_table()
                        .as_ref()
                        .runtime_services()
                        .get_time()
                        .unwrap()
                };
                let diff_years = time.year() - start_time.year();
                let diff_months = time.month() - start_time.month();
                let diff_days = time.day() - start_time.day();
                let diff_hours = time.hour() - start_time.hour();
                let diff_minutes = time.minute() - start_time.minute();
                let diff_seconds = time.second() - start_time.second();

                let diff = diff_years as u64 * 365 * 24 * 60 * 60
                    + diff_months as u64 * 30 * 24 * 60 * 60
                    + diff_days as u64 * 24 * 60 * 60
                    + diff_hours as u64 * 60 * 60
                    + diff_minutes as u64 * 60
                    + diff_seconds as u64;
                let diff_seconds = diff % 60;
                let diff_minutes = (diff / 60) % 60;
                let diff_hours = (diff / 60 / 60) % 24;
                let diff_days = (diff / 60 / 60 / 24) % 365;
                let diff_years = diff / 60 / 60 / 24 / 365;

                println!(
                    "Uptime: {} years, {} months, {} days, {} hours, {} minutes, {} seconds",
                    diff_years, diff_months, diff_days, diff_hours, diff_minutes, diff_seconds
                );
            }
            "ls" => {
                println!("Here be files");
            }
            _ => {}
        }
    }

    info!("Exiting Bubas OS...");

    sleep(1_000_000);

    Status(0)
}
