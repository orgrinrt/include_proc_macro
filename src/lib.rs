#[macro_export]
macro_rules! include_proc_macro {
    ($file_name:expr) => {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $file_name, ".rs",));
    };
}
