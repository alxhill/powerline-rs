mod rainbow;
mod simple;

use crate::modules::{
    CmdScheme, CwdScheme, ExitCodeScheme, GitScheme, LastCmdDurationScheme, PythonEnvScheme,
    ReadOnlyScheme, SpacerScheme, TimeScheme,
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
{
}
