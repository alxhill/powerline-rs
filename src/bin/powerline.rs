extern crate powerline_rs;

use std::env::VarError;
use std::error::Error;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use std::{env, io};

use clap::{Args, Parser, Subcommand, ValueEnum};
use thiserror::Error;

use powerline_rs::config::{Config, TerminalRuntimeMetadata};
use powerline_rs::terminal::{Shell, SHELL};
use powerline_rs::themes::{CustomTheme, RainbowTheme, SimpleTheme};
use powerline_rs::Powerline;

const FISH_CONF: &str = r#"
set -gx POWERLINE_RS 1

function __pl_cache_duration --on-event fish_postexec
  set -gx __pl_duration $CMD_DURATION
end

function fish_prompt
  powerline show -s $status -c $COLUMNS fish $__pl_duration
end

function fish_right_prompt
  powerline show-right -s $status -c $COLUMNS fish $__pl_duration
end
"#;

const FISH_INSTALL: &str = r#"
# automatically added by powerline-rs
powerline init fish | source
"#;

const ZSH_CONF: &str = r#"
export POWERLINE_RS=1

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
    RPS1="$(powerline show-right -s $? -c $COLUMNS zsh $_elapsed)"
    unset __pl_timer _elapsed _now
}

precmd_functions=(_update_ps1)
"#;

const ZSH_INSTALL: &str = r#"
# automatically added by powerline-rs
source <(powerline init zsh)
"#;

// note: does not support showing last cmd duration
const BASH_CONF: &str = r#"
export POWERLINE_RS=1

function _update_ps1() {
    PS1="$(powerline show -s $? -c $COLUMNS bash)"
}

if [ "$TERM" != "linux" ]; then
    PROMPT_COMMAND="_update_ps1; $PROMPT_COMMAND"
fi
"#;

const BASH_INSTALL: &str = r#"
# automatically added by powerline-rs
source <(powerline init bash)
"#;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
enum PowerlineArgs {
    #[command(subcommand)]
    Init(ShellSubcommand),
    Show(ShowArgs),
    ShowRight(ShowArgs),
    Install(InstallArgs),
    Config,
}

#[derive(Debug, Clone, Subcommand)]
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

#[derive(Debug, Args)]
struct InstallArgs {
    #[arg(value_enum)]
    shell: ShellArg,
    #[arg(long, action)]
    force: bool,
}

impl TerminalRuntimeMetadata for &ShowArgs {
    fn shell_name(&self) -> String {
        match self.shell {
            ShellArg::Bash => "bash".to_string(),
            ShellArg::Zsh => "zsh".to_string(),
            ShellArg::Fish => "fish".to_string(),
        }
    }

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
        PowerlineArgs::Show(args) => show(args, false),
        PowerlineArgs::ShowRight(args) => show(args, true),
        PowerlineArgs::Install(args) => install(args),
        PowerlineArgs::Config => open_config(),
    }
}

fn install(args: InstallArgs) {
    if env::var("POWERLINE_RS").is_ok() && !args.force {
        println!("powerline already installed in current shell");
        return;
    }

    let shell = args.shell;

    let home_dir = PathBuf::from(env::var("HOME").unwrap());

    assert!(home_dir.is_dir(), "home directory does not exist");

    println!("Installing powerline for {:?} shell", shell);

    match shell {
        ShellArg::Fish => append_conf(home_dir.join(".config/fish/config.fish"), FISH_INSTALL),
        ShellArg::Zsh => append_conf(home_dir.join(".zshrc"), ZSH_INSTALL),
        ShellArg::Bash => append_conf(home_dir.join("~/.bashrc"), BASH_INSTALL),
    }

    println!("Done, please restart your shell for changes to take effect");
}

fn append_conf(conf_path: PathBuf, conf_contents: &str) {
    let mut conf = OpenOptions::new()
        .append(true)
        .open(&conf_path)
        .unwrap_or_else(|_| {
            panic!(
                "could not open shell config file: {}",
                conf_path.to_str().unwrap_or("")
            )
        });

    conf.write_all(conf_contents.as_bytes())
        .expect("failed to append to config");
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

fn show(args: ShowArgs, right_only: bool) {
    match load_config(args.config.clone()) {
        Ok((conf, conf_root)) => {
            match args.shell {
                ShellArg::Bash => SHELL.set(Shell::Bash),
                ShellArg::Zsh => SHELL.set(Shell::Zsh),
                ShellArg::Fish => SHELL.set(Shell::Bare),
            }
            .expect("failed to set shell");

            if right_only {
                show_right(&args, conf, conf_root);
            } else {
                show_normal(&args, conf, conf_root);
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

fn show_right(args: &ShowArgs, conf: Config, conf_root: PathBuf) {
    if let Some(prompt) = conf.rows.last() {
        // todo: refactor to avoid repeating the theme loading code
        let powerline = match conf.theme.as_str() {
            "rainbow" => Powerline::from_conf::<RainbowTheme>(prompt, args),
            "simple" => Powerline::from_conf::<SimpleTheme>(prompt, args),
            theme_path => {
                let path = match theme_path.as_bytes() {
                    [b'/', ..] => PathBuf::from(theme_path),
                    _ => conf_root.join(theme_path),
                };

                CustomTheme::load(path);
                Powerline::from_conf::<CustomTheme>(prompt, args)
            }
        };

        powerline.print_right();
    }
}

fn show_normal(args: &ShowArgs, conf: Config, conf_root: PathBuf) {
    let mut powerlines = conf
        .rows
        .into_iter()
        .map(|prompt| match conf.theme.as_str() {
            "rainbow" => Powerline::from_conf::<RainbowTheme>(&prompt, args),
            "simple" => Powerline::from_conf::<SimpleTheme>(&prompt, args),
            theme_path => {
                let path = match theme_path.as_bytes() {
                    [b'/', ..] => PathBuf::from(theme_path),
                    _ => conf_root.join(theme_path),
                };

                CustomTheme::load(path);
                Powerline::from_conf::<CustomTheme>(&prompt, args)
            }
        })
        .collect::<Vec<Powerline>>();

    if let Some((last, all_bar_last)) = powerlines.split_last_mut() {
        for powerline in all_bar_last {
            powerline.print_left();
            powerline.print_padding(args.columns);
            powerline.print_right();
            println!();
        }
        // the shell handles printing the final right prompt
        last.print_left();
        println!();
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

fn load_config(conf_file: Option<PathBuf>) -> Result<(Config, PathBuf), PowerlineError> {
    let conf_path = conf_file.unwrap_or_else(|| get_or_create_conf_file().unwrap());
    let conf_file = File::open(&conf_path)?;
    let conf: Config = serde_json::from_reader(conf_file)?;
    Ok((conf, conf_path.parent().unwrap().into()))
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
