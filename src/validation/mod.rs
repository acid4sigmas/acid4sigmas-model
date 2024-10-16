pub mod auth;

#[macro_export]
macro_rules! to_string_ {
    ($s:expr) => {
        $s.to_string()
    };
}
