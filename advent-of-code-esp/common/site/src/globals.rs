use lazy_static::lazy_static;
use parking_lot::Mutex;

lazy_static! {
    pub static ref CURRENT_YEAR: Mutex<u32> = Mutex::new(2023);
}
