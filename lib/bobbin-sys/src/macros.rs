/// Macro for sending `print!`-formatted messages over the Console
#[macro_export]
macro_rules! print {
    ($s:expr) => {
        $crate::console::write_str($s)
    };
    ($($arg:tt)*) => {
        $crate::console::write_fmt(format_args!($($arg)*))
    };
}

/// Macro for sending `print!`-formatted messages over the Console, with a
/// newline
#[macro_export]
macro_rules! println {
    ($fmt:expr) => {
        print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        print!(concat!($fmt, "\n"), $($arg)*)
    };
}

#[macro_export]
macro_rules! abort {
    ($s:expr) => {
        $crate::console::write_str($s);
        $crate::console::write_str("\n");
        abort!()
    };
    () => {
        $crate::console::write_str("# ABORT at ");
        $crate::console::write_str(file!());
        $crate::console::write_str(":");
        $crate::console::write_u32(line!(), 10);
        $crate::console::write_str("\n");
        loop {}
    };    
}

