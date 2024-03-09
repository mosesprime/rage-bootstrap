#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        panic!("\x1b[0;31mERROR\x1b[0m {}", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("\x1b[0;33mWARN\x1b[0m {}", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("\x1b[0;32mINFO\x1b[0m {}", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!("\x1b[0;34mDEBUG\x1b[0m {}", format!($($arg)*))
    };
}
