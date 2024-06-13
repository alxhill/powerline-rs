extern crate powerline;

use std::env::args;
use std::error::Error;
use std::fs::File;
use std::{env, io};
use std::path::PathBuf;
use std::time::Duration;
use clap::Parser;

use thiserror::Error;

use powerline::config::{Config, TerminalRuntimeMetadata};
use powerline::Powerline;
use powerline::themes::{RainbowTheme, SimpleTheme};


#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct PowerlineArgs {
    conf_file: String,
    #[arg(short, long)]
    columns: usize,
    #[arg(short = 'd', long = "duration")]
    last_cmd_duration: Option<u64>,
    #[arg(short, long)]
    status: String,
}

impl TerminalRuntimeMetadata for &PowerlineArgs {
    fn total_columns(&self) -> usize {
        self.columns
    }

    fn last_command_duration(&self) -> Option<Duration> {
        self.last_cmd_duration.map(Duration::from_millis)
    }

    fn last_command_status(&self) -> &str {
        self.status.as_str()
    }
}

fn main() {
    let args = PowerlineArgs::parse();

    match load_config(&args.conf_file) {
        Ok(conf) => {
            for prompt in conf.rows {
                let powerline = match conf.theme.as_str() {
                    "rainbow" => Powerline::from_conf::<RainbowTheme>(&prompt, &args),
                    "simple" => Powerline::from_conf::<SimpleTheme>(&prompt, &args),
                    _ => panic!("unknown theme, supported themes are simple and rainbow")
                };

                println!("{}", powerline.render(100));
            }
        }
        Err(e) => {
            eprintln!("error: {}", e);
            if let Some(source) = e.source() {
                eprintln!("source:\n\t{}", source);
            }
        }
    }
}

#[derive(Error, Debug)]
enum PowerlineError {
    #[error("could not read config file")]
    IoError(#[from] io::Error),
    #[error("config file could not be parsed")]
    InvalidConfig(#[from] serde_json::Error),
}

fn load_config(conf_file: &str) -> Result<Config, PowerlineError> {
    let conf_file = File::open(conf_file)?;
    let conf: Config = serde_json::from_reader(conf_file)?;
    Ok(conf)
}
