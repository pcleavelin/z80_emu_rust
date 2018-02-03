pub enum Log {
    Instr,
    Debug,
}

#[macro_export]
macro_rules! log {
    (Log::Instr, $($arg:expr),*) => ({
        $(let _arg = &$arg;)*
        #[cfg(feature = "log-instr")]
        println!($($arg),*);
    });
    (Log::Debug, $($arg:expr),*) => ({
        $(let _arg = &$arg;)*
        #[cfg(feature = "log-debug")]
        println!($($arg),*);
    });
}