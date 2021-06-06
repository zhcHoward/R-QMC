use log::{LevelFilter, SetLoggerError};
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    Handle,
};

pub fn init_logger(level: LevelFilter) -> Result<Handle, SetLoggerError> {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{h({m})}{n}")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(level))
        .unwrap();

    log4rs::init_config(config)
}
