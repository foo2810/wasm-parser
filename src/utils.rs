#[macro_export]
macro_rules! not_implemented {
    ($msg:expr) => {
        panic!(msg);
    };
}
