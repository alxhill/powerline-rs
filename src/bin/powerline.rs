extern crate powerline;

use clap::{Args, Parser, Subcommand};
use std::env::{VarError};
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use std::{env, io};

use thiserror::Error;

use powerline::config::{Config, TerminalRuntimeMetadata};
use powerline::themes::{RainbowTheme, SimpleTheme};
use powerline::Powerline;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
enum PowerlineArgs {
    #[command(subcommand)]
    Shell(ShellArg),
    Show(ShowArgs),
}

#[derive(Debug, Subcommand)]
enum ShellArg {
    Bash,
    Zsh,
    Fish,
}

#[derive(Debug, Args)]
struct ShowArgs {
    // not an arg to allow it to be empty easily
    duration: Option<u64>,
    #[arg(short, long)]
    columns: usize,
    #[arg(short, long)]
    status: String,
    #[arg(long)]
    config: Option<PathBuf>,
}

impl TerminalRuntimeMetadata for &ShowArgs {
    fn total_columns(&self) -> usize {
        self.columns
    }

    fn last_command_duration(&self) -> Option<Duration> {
        self.duration.map(Duration::from_millis)
    }

    fn last_command_status(&self) -> &str {
        self.status.as_str()
    }
}

fn main() {
    let args = PowerlineArgs::parse();

    match args {
        PowerlineArgs::Shell(shell) => print_shell_conf(shell),
        PowerlineArgs::Show(args) => show(args),
    }
}

fn print_shell_conf(shell: ShellArg) {
    match shell {
        ShellArg::Bash => println!("bash"),
        ShellArg::Zsh => println!("zsh"),
        ShellArg::Fish => println!("fish"),
    }
}

fn show(args: ShowArgs) {
    match load_config(args.config.clone()) {
        Ok(conf) => {
            for prompt in conf.rows {
                let powerline = match conf.theme.as_str() {
                    "rainbow" => Powerline::from_conf::<RainbowTheme>(&prompt, &args),
                    "simple" => Powerline::from_conf::<SimpleTheme>(&prompt, &args),
                    _ => panic!("unknown theme, supported themes are simple and rainbow"),
                };

                println!("{}", powerline.render(args.columns));
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
    #[error("could not read value of $HOME env var")]
    HomeEnvNotFound(#[from] VarError),
    #[error("could not read config file")]
    IoError(#[from] io::Error),
    #[error("config file could not be parsed")]
    InvalidConfig(#[from] serde_json::Error),
}

fn load_config(conf_file: Option<PathBuf>) -> Result<Config, PowerlineError> {
    let conf_file = conf_file.unwrap_or_else(|| get_or_create_conf_file().unwrap());
    let conf_file = File::open(conf_file)?;
    let conf: Config = serde_json::from_reader(conf_file)?;
    Ok(conf)
}

fn get_or_create_conf_file() -> Result<PathBuf, PowerlineError> {
    let home_dir = PathBuf::from(env::var("HOME")?);
    let config_dir = home_dir.join(".config/powerline-rs");
    if !config_dir.exists() {
        create_dir_all(&config_dir)?;
    }

    let conf_file = config_dir.join("config.json");
    if !conf_file.exists() {
        println!(
            "config file not found, creating default conf at {:?}",
            &conf_file
        );
        File::create(&conf_file)?
            .write_all(serde_json::to_string_pretty(&Config::default())?.as_bytes())?;
    }

    Ok(conf_file)
}
