
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        unsafe {
            use core::fmt::Write;
            let stdout = SYSTEM_TABLE
                .as_mut()
                .expect("The system table handle is not available")
                .stdout();
            write!(stdout, $($arg)*).unwrap();
        }
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        unsafe {
            use core::fmt::Write;
            let stdout = SYSTEM_TABLE
                .as_mut()
                .expect("The system table handle is not available")
                .stdout();
            write!(stdout, $($arg)*).unwrap();
            write!(stdout, "\n").unwrap();
        }
    }
}
