#[cfg(not(debug_assertions))]
fn logging() {
    lazy_panic::set_panic_message!(lazy_panic::formatter::Simple);
    cute_log::init().expect("To initialize log");
}

#[cfg(debug_assertions)]
fn logging() {
    lazy_panic::set_panic_message!(lazy_panic::formatter::Debug);
    cute_log::init().expect("To initialize log");
}

pub fn init() {
    logging();
}
