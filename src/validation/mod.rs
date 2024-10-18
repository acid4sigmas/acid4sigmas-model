pub mod auth;
pub mod db;

#[macro_export]
macro_rules! to_string_ {
    ($s:expr) => {
        $s.to_string()
    };
}
