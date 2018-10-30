pub fn init() {
    lazy_panic::set_panic_message!(lazy_panic::formatter::Simple);
    //lazy_panic::set_panic_message!(lazy_panic::formatter::Debug);
    cute_log::init().expect("To initialize log");
}
