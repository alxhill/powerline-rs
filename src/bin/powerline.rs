extern crate powerline_rs;

use std::{env, io};
use std::env::VarError;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

use clap::{Args, Parser, Subcommand, ValueEnum};
use thiserror::Error;

use powerline_rs::config::{Config, TerminalRuntimeMetadata};
use powerline_rs::Powerline;
use powerline_rs::terminal::{Shell, SHELL};
use powerline_rs::themes::{RainbowTheme, SimpleTheme};

const FISH_CONF: &str = r#"
function __pl_cache_duration --on-event fish_postexec
  set -gx __pl_duration $CMD_DURATION
end

function fish_prompt
  powerline show -s $status -c $COLUMNS fish $__pl_duration
  set -gx __pl_duration 0
end
"#;

const ZSH_CONF: &str = r#"
function preexec() {
    if command -v gdate >/dev/null 2>&1; then
        __pl_timer=$(($(gdate +%s%0N)/1000000))
    fi
}

function _update_ps1() {
    if [ $__pl_timer ]; then
        _now=$(($(gdate +%s%0N)/1000000))
        if [ $_now -ge $__pl_timer ]; then
            _elapsed=$(($_now-$__pl_timer))
        fi
    fi
    PS1="$(powerline show -s $? -c $COLUMNS zsh $_elapsed)"
    unset __pl_timer _elapsed _now
}

precmd_functions=(_update_ps1)
"#;

// note: does not support showing last cmd duration
const BASH_CONF: &str = r#"
function _update_ps1() {
    PS1="$(powerline show -s $? -c $COLUMNS bash)"
}

if [ "$TERM" != "linux" ]; then
    PROMPT_COMMAND="_update_ps1; $PROMPT_COMMAND"
fi
"#;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
enum PowerlineArgs {
    #[command(subcommand)]
    Init(ShellSubcommand),
    Show(ShowArgs),
    Config,
}

#[derive(Debug, Subcommand)]
enum ShellSubcommand {
    Bash,
    Zsh,
    Fish,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ShellArg {
    Bash,
    Zsh,
    Fish,
}

#[derive(Debug, Args)]
struct ShowArgs {
    #[arg(value_enum)]
    shell: ShellArg,
    // not an arg to allow passing a variable that may be empty
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
        PowerlineArgs::Init(shell) => print_shell_conf(shell),
        PowerlineArgs::Show(args) => show(args),
        PowerlineArgs::Config => open_config(),
    }
}

fn open_config() {
    let conf = get_or_create_conf_file().unwrap();

    let editor = env::var("EDITOR").unwrap_or("vim".to_string());

    Command::new(editor)
        .arg(conf)
        .status()
        .expect("Failed to get editor exit status");
}

fn print_shell_conf(shell: ShellSubcommand) {
    match shell {
        ShellSubcommand::Bash => println!("{}", BASH_CONF),
        ShellSubcommand::Zsh => println!("{}", ZSH_CONF),
        ShellSubcommand::Fish => println!("{}", FISH_CONF),
    }
}

fn show(args: ShowArgs) {
    match load_config(args.config.clone()) {
        Ok(conf) => {
            match args.shell {
                ShellArg::Bash => SHELL.set(Shell::Bash),
                ShellArg::Zsh => SHELL.set(Shell::Zsh),
                ShellArg::Fish => SHELL.set(Shell::Bare),
            }
                .expect("failed to set shell");

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
            eprintln!("powerline error: {}", e);
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
