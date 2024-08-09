use crate::powerline::Powerline;

mod cmd;
mod cwd;
mod exit_code;
mod git;
mod host;
mod readonly;
mod user;

mod cargo;
mod cmd_duration;
mod nvm;
mod python_env;
mod sdkman_java;
mod shell_name;
mod spacer;
mod time;

pub use cargo::{Cargo, CargoScheme};
pub use cmd::{Cmd, CmdScheme};
pub use cmd_duration::{LastCmdDuration, LastCmdDurationScheme};
pub use cwd::{Cwd, CwdScheme};
pub use exit_code::{ExitCode, ExitCodeScheme};
pub use git::{Git, GitScheme};
pub use host::{Host, HostScheme};
pub use nvm::{Nvm, NvmScheme};
pub use python_env::{PythonEnv, PythonEnvScheme};
pub use readonly::{ReadOnly, ReadOnlyScheme};
pub use sdkman_java::{SdkmanJava, SdkmanScheme};
pub use shell_name::{ShellName, ShellScheme};
pub use spacer::{Spacer, SpacerScheme};
pub use time::{Time, TimeScheme};
pub use user::{User, UserScheme};

pub trait Module {
    fn append_segments(&mut self, powerline: &mut Powerline);
}
