pub enum Log {
    Instr,
}

#[macro_export]
macro_rules! log {
    (Log::Instr, $($arg:expr),*) => ({
        $(let _arg = &$arg;)*
        #[cfg(feature = "log-instr")]
        println!($($arg),*);
    });
}