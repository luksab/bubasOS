use crate::{
    fs::crawl_tree,
    print, println,
    utils::{get_handle, get_systable},
    SYSTEM_TABLE,
};
use alloc::{string::String, vec::Vec};
use log::info;
use uefi::{
    proto::console::text::{Input, Key, ScanCode},
    table::runtime::ResetType,
};
use uefi::{
    proto::media::file::{File, FileAttribute, FileMode, FileType},
    CStr16,
};

const PROMPT: &str = "> ";

fn read_line(keyboard: &mut Input) -> Result<String, uefi::Error> {
    let mut line = String::new();
    let stdout = get_systable().stdout();
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

pub fn run(keyboard: &mut Input) -> Result<(), uefi::Error> {
    println!("Welcome to Bubas OS!");
    let start_time = get_systable().runtime_services().get_time().unwrap();
    loop {
        print!("{}", PROMPT);
        let line = read_line(keyboard)?;
        info!("{}", line);
        match line.as_str() {
            "exit" => break,
            "clear" => {
                get_systable().stdout().clear()?;
            }
            "shutdown" => {
                get_systable()
                    .runtime_services()
                    .reset(ResetType::Shutdown, uefi::Status(0), None)
            }
            "uptime" => {
                let time = get_systable().runtime_services().get_time().unwrap();
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
            "ls" => ls(),
            _ => {}
        }
    }
    Ok(())
}

fn ls() {
    let system_table = get_systable();


    let mut fs = system_table
        .boot_services()
        .get_image_file_system(get_handle().clone())
        .expect("Could not get image file system");

    let mut root = fs.open_volume().expect("Could not open volume");
    crawl_tree(&mut root);
}
