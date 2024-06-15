pub use rainbow::RainbowTheme;
pub use simple::SimpleTheme;

use crate::modules::{
    CargoScheme, CmdScheme, CwdScheme, ExitCodeScheme, GitScheme, HostScheme,
    LastCmdDurationScheme, PythonEnvScheme, ReadOnlyScheme, SpacerScheme, TimeScheme, UserScheme,
};

mod rainbow;
mod simple;

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
    + CargoScheme
    + TimeScheme
{
}
