use std::{error::Error, path::Path};

use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    init_config, Config,
};

use crate::config::ServerConfig;

pub fn init_logger(config: &ServerConfig) -> Result<(), Box<dyn Error>> {
    let level = match config.cd.log_level.as_str() {
        "Debug" => log::LevelFilter::Debug,
        "Trace" => log::LevelFilter::Trace,
        "Info" => log::LevelFilter::Info,
        "Warn" => log::LevelFilter::Warn,
        "Error" => log::LevelFilter::Error,
        "Off" => log::LevelFilter::Off,
        _ => log::LevelFilter::Debug,
    };

    let path = Path::new("log.txt");

    let stdout = ConsoleAppender::builder()
        .target(log4rs::append::console::Target::Stdout)
        .encoder(Box::new(PatternEncoder::new(
            "{h([{l}])}(({t})) - {m}{n}\n",
        )))
        .build();

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d} {h([{l}])}(({t})) - {m}{n}\n",
        )))
        .build(path)?;

    let log_config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stdout")
                .build(level),
        )?;

    let _ = init_config(log_config)?;

    Ok(())
}
