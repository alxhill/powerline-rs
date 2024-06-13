mod rainbow;
mod simple;

use crate::modules::{
    CmdScheme, CwdScheme, ExitCodeScheme, GitScheme, HostScheme, LastCmdDurationScheme,
    PythonEnvScheme, ReadOnlyScheme, SpacerScheme, UserScheme,
};
pub use rainbow::RainbowTheme;
pub use simple::SimpleTheme;

pub trait CompleteTheme:
    CmdScheme
    + CwdScheme
    + LastCmdDurationScheme
    + ExitCodeScheme
    + GitScheme
    + PythonEnvScheme
    + ReadOnlyScheme
    + SpacerScheme
    + HostScheme
    + UserScheme
{
}
