pub use custom::CustomTheme;
pub use rainbow::RainbowTheme;
pub use simple::SimpleTheme;

use crate::Color;
use crate::modules::{
    CargoScheme, CmdScheme, CwdScheme, ExitCodeScheme, GitScheme, HostScheme,
    LastCmdDurationScheme, PythonEnvScheme, ReadOnlyScheme, SpacerScheme, TimeScheme, UserScheme,
};

mod custom;
mod light;
mod rainbow;
mod simple;

pub trait DefaultColors {
    fn default_bg() -> Color;
    fn default_fg() -> Color;
}

pub trait CompleteTheme:
    DefaultColors
    + CmdScheme
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
