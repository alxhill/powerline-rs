use crate::powerline::Powerline;

mod cmd;
mod cwd;
mod exit_code;
mod git;
mod host;
mod readonly;
mod user;

mod cargo;
mod duration;
mod python_env;
mod spacer;
mod time;

pub use cargo::{Cargo, CargoScheme};
pub use cmd::{Cmd, CmdScheme};
pub use cwd::{Cwd, CwdScheme};
pub use duration::{LastCmdDuration, LastCmdDurationScheme};
pub use exit_code::{ExitCode, ExitCodeScheme};
pub use git::{Git, GitScheme};
pub use host::{Host, HostScheme};
pub use python_env::{PythonEnv, PythonEnvScheme};
pub use readonly::{ReadOnly, ReadOnlyScheme};
pub use spacer::{Spacer, SpacerScheme};
pub use time::{Time, TimeScheme};
pub use user::{User, UserScheme};

pub trait Module {
    fn append_segments(&mut self, powerline: &mut Powerline);
}
