use alloc::vec::Vec;
use uefi::{
    proto::media::file::{Directory, File, FileAttribute, FileHandle, FileMode, FileType},
    table::{Boot, SystemTable},
    CStr16,
};

use crate::{println, utils::get_systable};

pub fn crawl_tree(dir: &mut Directory) {
    let system_table = get_systable();
    crawl_tree_internal(
        system_table,
        dir,
        0,
        &mut |system_table: &mut SystemTable<Boot>,
              file: FileHandle,
              filename: &CStr16,
              depth: u8| {
            match file.into_type().unwrap() {
                FileType::Regular(_) => println!("{}{}", " ".repeat(depth as usize), filename),
                FileType::Dir(_) => println!("{}{}/", " ".repeat(depth as usize), filename),
            }
        },
    );
}

pub fn crawl_tree_internal(
    system_table: &mut SystemTable<Boot>,
    dir: &mut Directory,
    depth: u8,
    visitor: &mut dyn FnMut(&mut SystemTable<Boot>, FileHandle, &CStr16, u8),
) {
    let mut buf: Vec<u8> = Vec::new();
    loop {
        match dir.read_entry(&mut buf[..]).map_err(|err| err.split()) {
            Ok(ret) => {
                if let Some(f) = ret {
                    let filename = f.file_name();
                    let mut file = match dir.handle().open(
                        f.file_name(),
                        FileMode::Read,
                        FileAttribute::READ_ONLY,
                    ) {
                        Ok(f) => f,
                        Err(_) => {
                            continue;
                        }
                    };

                    // Compare with "." and ".."
                    if filename == unsafe { CStr16::from_u16_with_nul_unchecked(&[0x002E, 0x0]) }
                        || filename
                            == unsafe {
                                CStr16::from_u16_with_nul_unchecked(&[0x002E, 0x002E, 0x0])
                            }
                    {
                        if depth == 0 {
                            visitor(system_table, file, filename, depth);
                        }
                        continue;
                    }
                    visitor(system_table, file, &filename, depth);

                    match dir
                        .handle()
                        .open(f.file_name(), FileMode::Read, FileAttribute::READ_ONLY)
                    {
                        Ok(f) => {
                            if let Ok(c) = f.into_type() {
                                match c {
                                    FileType::Dir(mut d) => {
                                        crawl_tree_internal(
                                            system_table,
                                            &mut d,
                                            depth + 1,
                                            visitor,
                                        );
                                    }
                                    FileType::Regular(_) => {}
                                }
                            }
                        }
                        Err(_) => {}
                    }
                } else {
                    break;
                }
            }
            Err((_, Some(new_size))) => {
                buf.extend((0..new_size - buf.len()).map(|_| 0));
            }
            Err((status, None)) => panic!("Can't read root dir status: {:?}", status),
        };
    }
}
