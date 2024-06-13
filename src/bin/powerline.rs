extern crate powerline;

use std::env::args;
use std::fs::File;
use std::io;

use thiserror::Error;

use powerline::config::Config;
use powerline::Powerline;
use powerline::powerline::PowerlineRightBuilder;
use powerline::themes::{RainbowTheme, SimpleTheme};

fn main() {
    let conf = load_config().expect("Failed to read config from file");

    for prompt in conf.rows {
        let powerline = match conf.theme.as_str() {
            "rainbow" => Powerline::from_conf::<RainbowTheme>(&prompt),
            "simple" => Powerline::from_conf::<SimpleTheme>(&prompt),
            _ => panic!("unknown theme, supported themes are simple and rainbow")
        };

        println!("{}", powerline.render(100));
    }
}

#[derive(Error, Debug)]
enum PowerlineError {
    #[error("config argument not found")]
    InvalidArgument,
    #[error("could not read config file")]
    IoError(#[from] io::Error),
    #[error("config file could not be parsed")]
    InvalidConfig(#[from] serde_json::Error),
}

fn load_config() -> Result<Config, PowerlineError> {
    let conf_file = args().nth(1)
        .ok_or(PowerlineError::InvalidArgument)?;

    let conf_file = File::open(conf_file)?;
    let conf: Config = serde_json::from_reader(conf_file)?;

    Ok(conf)
}
