pub mod logging {
    use std::path::Path;

    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Logger, Root};
    use log4rs::encode::pattern::PatternEncoder;
    use log::LevelFilter;

    const FILE_APPENDER_NAME: &str = "file";

    pub fn get_logging_config(work_dir: &str) -> Config {
        Config::builder()
            .appender(get_file_appender_definition(work_dir))
            .logger(get_default_logger())
            .build(
            Root::builder()
                .appender(FILE_APPENDER_NAME)
                .build(LevelFilter::Info)
            )
            .unwrap()
    }

    fn get_file_appender_definition(work_dir: &str) -> Appender {
        Appender::builder()
            .build(FILE_APPENDER_NAME, Box::new(get_file_appender(work_dir))
        )
    }

    fn get_file_appender(work_dir: &str) -> FileAppender {
        let log_file_path = Path::new(work_dir).join("site-discovery-flea.log")
                                                          .display().to_string();

        FileAppender::builder()
            .encoder(get_encoder())
            .build(log_file_path)
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