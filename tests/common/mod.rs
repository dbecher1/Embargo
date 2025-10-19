use std::sync::Once;

static INIT_LOGGER: Once = Once::new();

pub(crate) fn setup() {
    INIT_LOGGER.call_once(|| {
        let mut logger = env_logger::builder();
        logger.filter_level(log::LevelFilter::Trace);
        logger.init();
    });
}