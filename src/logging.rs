pub mod logging {
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Logger, Root};
    use log4rs::encode::pattern::PatternEncoder;
    use log4rs::filter::threshold::ThresholdFilter;
    use log::LevelFilter;

    const FILE_APPENDER_NAME: &str = "file";

    const LOG_FILE_PATH: &str = "wszl.log";

    fn get_logging_level_from_string(level: &str) -> LevelFilter {
        return match level {
            "debug" => LevelFilter::Debug,
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "trace" => LevelFilter::Trace,
            "off" => LevelFilter::Off,
            _ => LevelFilter::Info
        };
    }

    pub fn get_logging_config(logging_level: &str) -> Config {
        let level = get_logging_level_from_string(logging_level);

        Config::builder()
            .appender(get_file_appender_definition(level))
            .logger(get_default_logger(level))
            .logger(Logger::builder().build("hyper", LevelFilter::Info))
            .build(
            Root::builder()
                .appender(FILE_APPENDER_NAME)
                .build(level)
            ).expect(&format!("unable to create log file '{}'", LOG_FILE_PATH))
    }

    fn get_file_appender_definition(level: LevelFilter) -> Appender {
        Appender::builder()
            .filter(Box::new(ThresholdFilter::new(level)))
            .build(FILE_APPENDER_NAME, Box::new(get_file_appender())
        )
    }

    fn get_file_appender() -> FileAppender {
        FileAppender::builder()
            .encoder(get_encoder())
            .build(LOG_FILE_PATH)
            .unwrap()
    }

    fn get_encoder() -> Box<PatternEncoder> {
        Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} - {l} - [{M}] - {m}{n}"))
    }

    fn get_default_logger(level: LevelFilter) -> Logger {
        Logger::builder()
                .build("default", level)
    }
}
