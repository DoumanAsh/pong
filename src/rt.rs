#[cfg(not(debug_assertions))]
fn logging() {
    lazy_panic::set_panic_message!(lazy_panic::formatter::Simple);

    let mut logger = amethyst::LoggerConfig::default();
    logger.level_filter = amethyst::LogLevelFilter::Warn;

    amethyst::start_logger(logger);
}

#[cfg(debug_assertions)]
fn logging() {
    lazy_panic::set_panic_message!(lazy_panic::formatter::Debug);
    amethyst::start_logger(Default::default());
}

pub fn init() {
    logging();
}
