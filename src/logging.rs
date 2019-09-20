pub mod logging {
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Logger, Root};
    use log4rs::encode::pattern::PatternEncoder;
    use log::LevelFilter;

    const FILE_APPENDER_NAME: &str = "file";

    pub fn get_logging_config() -> Config {
        Config::builder()
            .appender(get_file_appender_definition())
            .logger(get_default_logger())
            .build(
            Root::builder()
                .appender(FILE_APPENDER_NAME)
                .build(LevelFilter::Info)
            )
            .unwrap()
    }

    fn get_file_appender_definition() -> Appender {
        Appender::builder()
            .build(FILE_APPENDER_NAME, Box::new(get_file_appender())
        )
    }

    fn get_file_appender() -> FileAppender {
        FileAppender::builder()
            .encoder(get_encoder())
            .build("site-discovery-flea.log")
            .unwrap()
    }

    fn get_encoder() -> Box<PatternEncoder> {
        Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} - {l} - {m}{n}"))
    }

    fn get_default_logger() -> Logger {
        Logger::builder()
                .build("default", LevelFilter::Info)
    }
}