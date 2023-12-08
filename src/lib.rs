#[macro_export]
macro_rules! sample {
    () => {
        include_str!(concat!("../../samples/", env!("CARGO_BIN_NAME"), ".txt",),)
    };
}

#[macro_export]
macro_rules! sample1 {
    () => {
        include_str!(concat!("../../samples/", env!("CARGO_BIN_NAME"), "-1.txt",),)
    };
}

#[macro_export]
macro_rules! sample2 {
    () => {
        include_str!(concat!("../../samples/", env!("CARGO_BIN_NAME"), "-2.txt",),)
    };
}

#[macro_export]
macro_rules! sample3 {
    () => {
        include_str!(concat!("../../samples/", env!("CARGO_BIN_NAME"), "-3.txt",),)
    };
}

#[macro_export]
macro_rules! input {
    () => {
        include_str!(concat!("../../inputs/", env!("CARGO_BIN_NAME"), ".txt",),)
    };
}
