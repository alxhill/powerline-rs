extern crate superline;

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

use superline::config::{Config, TerminalRuntimeMetadata};
use superline::modules::refresh_pr;
use superline::terminal::{Shell, SHELL};
use superline::themes::{CustomTheme, RainbowTheme, SimpleTheme};
use superline::Powerline;

const FISH_CONF: &str = r#"
set -gx SUPERLINE_FISH 1

function __pl_cache_duration --on-event fish_postexec
  set -gx __pl_duration $CMD_DURATION
end

function fish_prompt
  superline show -s $status -c $COLUMNS fish $__pl_duration
end

function fish_right_prompt
  superline show-right -s $status -c $COLUMNS fish $__pl_duration
end
"#;

const FISH_INSTALL: &str = r#"
# automatically added by superline
superline init fish | source
"#;

const ZSH_CONF: &str = r#"
export SUPERLINE_ZSH=1

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
    PS1="$(superline show -s $? -c $COLUMNS zsh $_elapsed)"
    RPS1="$(superline show-right -s $? -c $COLUMNS zsh $_elapsed)"
    unset __pl_timer _elapsed _now
}

precmd_functions=(_update_ps1)
"#;

const ZSH_INSTALL: &str = r#"
# automatically added by superline
source <(superline init zsh)
"#;

// note: does not support showing last cmd duration
const BASH_CONF: &str = r#"
export SUPERLINE_BASH=1

function _update_ps1() {
    PS1="$(superline show -s $? -c $COLUMNS bash)"
}

if [ "$TERM" != "linux" ]; then
    PROMPT_COMMAND="_update_ps1; $PROMPT_COMMAND"
fi
"#;

const BASH_INSTALL: &str = r#"
# automatically added by superline
source <(superline init bash)
"#;

const PWSH_CONF: &str = r#"
$env:SUPERLINE_PWSH = 1

function global:prompt {
    # Capture command state first: every statement below (even an assignment)
    # resets $?, so read it before anything else - including before reading
    # $LASTEXITCODE, which a plain assignment would otherwise flip back to true.
    $__pl_ok = $?
    $__pl_exit = $LASTEXITCODE

    # Mirror the bash/zsh convention: 0 on success, otherwise the native exit
    # code (falling back to 1 for cmdlet failures that leave $LASTEXITCODE unset).
    if ($__pl_ok) {
        $__pl_status = 0
    } elseif ($__pl_exit) {
        $__pl_status = $__pl_exit
    } else {
        $__pl_status = 1
    }

    $__pl_cols = 0
    try { $__pl_cols = $Host.UI.RawUI.WindowSize.Width } catch {}
    if (-not $__pl_cols -or $__pl_cols -le 0) { $__pl_cols = 80 }

    $__pl_args = @('show', '-s', $__pl_status, '-c', $__pl_cols, 'pwsh')

    # Duration of the last command, in milliseconds, from session history.
    $__pl_last = Get-History -Count 1
    if ($__pl_last) {
        $__pl_ms = [long][math]::Round(($__pl_last.EndExecutionTime - $__pl_last.StartExecutionTime).TotalMilliseconds)
        if ($__pl_ms -ge 0) { $__pl_args += $__pl_ms }
    }

    # Join lines with `n (not Out-String, which can wrap/pad to the host width).
    $__pl_out = (& superline @__pl_args) -join "`n"

    # Restore $LASTEXITCODE so our own commands don't clobber the user's value.
    $global:LASTEXITCODE = $__pl_exit

    $__pl_out
}
"#;

const PWSH_INSTALL: &str = r#"
# automatically added by superline
(& superline init pwsh) -join "`n" | Invoke-Expression
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
    /// Internal: refresh the cached PR lookup for a branch. Spawned in the
    /// background by the `pr` module - not intended to be called by hand.
    #[command(hide = true)]
    RefreshPr(RefreshPrArgs),
}

#[derive(Debug, Clone, Subcommand)]
enum ShellSubcommand {
    Bash,
    Zsh,
    Fish,
    Pwsh,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ShellArg {
    Bash,
    Zsh,
    Fish,
    Pwsh,
}

impl ShellArg {
    fn name(&self) -> &'static str {
        match self {
            ShellArg::Bash => "bash",
            ShellArg::Zsh => "zsh",
            ShellArg::Fish => "fish",
            ShellArg::Pwsh => "pwsh",
        }
    }

    /// Per-shell marker env var exported by that shell's init snippet (e.g.
    /// `SUPERLINE_BASH`).
    fn marker_env_var(&self) -> String {
        format!("SUPERLINE_{}", self.name().to_uppercase())
    }
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
struct RefreshPrArgs {
    #[arg(long)]
    branch: String,
    #[arg(long)]
    repo_dir: PathBuf,
    #[arg(long)]
    cache: PathBuf,
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
        self.shell.name().to_string()
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
        PowerlineArgs::RefreshPr(args) => refresh_pr(&args.branch, &args.repo_dir, &args.cache),
    }
}

fn install(args: InstallArgs) {
    let shell = args.shell;

    // A nested shell only inherits its parent's marker, so keying detection off
    // the target shell's own marker lets install still run in nested shells.
    if env::var(shell.marker_env_var()).is_ok() && !args.force {
        println!(
            "superline already installed in current {} shell",
            shell.name()
        );
        return;
    }

    println!("Installing superline for {} shell", shell.name());

    match shell {
        ShellArg::Fish => append_conf(home_config(".config/fish/config.fish"), FISH_INSTALL),
        ShellArg::Zsh => append_conf(home_config(".zshrc"), ZSH_INSTALL),
        ShellArg::Bash => append_conf(home_config(".bashrc"), BASH_INSTALL),
        ShellArg::Pwsh => append_conf(powershell_profile_path(), PWSH_INSTALL),
    }

    println!("Done, please restart your shell for changes to take effect");
}

/// Resolve a path inside the Unix `$HOME` directory used by the bash/zsh/fish
/// config files.
fn home_config(rel: &str) -> PathBuf {
    let home_dir = PathBuf::from(env::var("HOME").expect("could not read $HOME env var"));
    assert!(home_dir.is_dir(), "home directory does not exist");
    home_dir.join(rel)
}

/// Ask PowerShell itself for the current-user profile path: it varies per
/// platform and avoids relying on `$HOME`, which Windows does not set.
fn powershell_profile_path() -> PathBuf {
    let run = |cmd: &str| {
        Command::new(cmd)
            .args(["-NoProfile", "-Command", "$PROFILE.CurrentUserCurrentHost"])
            .output()
    };

    let output = run("pwsh").or_else(|_| run("powershell")).expect(
        "could not run pwsh/powershell to locate the profile - is PowerShell installed and on PATH?",
    );

    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    assert!(
        !path.is_empty(),
        "PowerShell returned an empty profile path"
    );
    PathBuf::from(path)
}

fn append_conf(conf_path: PathBuf, conf_contents: &str) {
    if let Some(parent) = conf_path.parent() {
        create_dir_all(parent).unwrap_or_else(|e| {
            panic!(
                "could not create config directory {}: {e}",
                parent.display()
            )
        });
    }

    let mut conf = OpenOptions::new()
        .create(true)
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
        ShellSubcommand::Pwsh => println!("{}", PWSH_CONF),
    }
}

fn show(args: ShowArgs, right_only: bool) {
    match load_config(args.config.clone()) {
        Ok((conf, conf_root)) => {
            match args.shell {
                ShellArg::Bash => SHELL.set(Shell::Bash),
                ShellArg::Zsh => SHELL.set(Shell::Zsh),
                ShellArg::Fish => SHELL.set(Shell::Bare),
                // PowerShell's PSReadLine handles raw ANSI escapes itself, so it
                // uses the same bare escapes as fish (no non-printing markers).
                ShellArg::Pwsh => SHELL.set(Shell::Bare),
            }
            .expect("failed to set shell");

            if right_only {
                show_right(&args, conf, conf_root);
            } else {
                show_normal(&args, conf, conf_root);
            }
        }
        Err(e) => {
            eprintln!("superline error: {}", e);
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

                if CustomTheme::load(path.clone()) {
                    Powerline::from_conf::<CustomTheme>(prompt, args)
                } else {
                    eprintln!(
                        "Powerline could not load custom theme {}, falling back to default",
                        path.display()
                    );
                    Powerline::from_conf::<RainbowTheme>(prompt, args)
                }
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

                if CustomTheme::load(path.clone()) {
                    Powerline::from_conf::<CustomTheme>(&prompt, args)
                } else {
                    eprintln!(
                        "Powerline could not load custom theme {}, falling back to default",
                        path.display()
                    );
                    Powerline::from_conf::<RainbowTheme>(&prompt, args)
                }
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
    let config_dir = home_dir.join(".config/superline");
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
