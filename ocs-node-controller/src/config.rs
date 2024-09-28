use std::cell::LazyCell;

pub const ROOT_SAVE_PATH: LazyCell<String> = LazyCell::new(|| {
    std::env::var("ROOT_SAVE_PATH").unwrap_or(String::from("/tmp"))
});
