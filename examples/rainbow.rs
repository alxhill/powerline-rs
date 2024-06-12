use powerline::modules::*;
use powerline::theme::SimpleTheme;

#[derive(Copy, Clone)]
pub struct RainbowTheme;

impl CmdScheme for SimpleTheme {

}
fn main() {
    let mut top_prompt = powerline::Powerline::new();

    top_prompt.add_module(Cwd::<SimpleTheme>::new(45, 4, false));
    top_prompt.add_module(Git::<SimpleTheme>::new());
    top_prompt.add_module(ReadOnly::<SimpleTheme>::new());
    top_prompt.add_module(VirtualEnv::<SimpleTheme>::new());

    let mut mini_prompt = powerline::Powerline::new();
    mini_prompt.add_module(Cmd::<SimpleTheme>::new());
    mini_prompt.add_module(ExitCode::<SimpleTheme>::new());

    println!("{}", top_prompt);
    println!("{}", mini_prompt);
}