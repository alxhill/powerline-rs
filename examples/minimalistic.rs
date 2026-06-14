use superline::modules::*;
use superline::powerline::{PowerlineRightBuilder, PowerlineShellBuilder};
use superline::terminal::Shell;
use superline::themes::SimpleTheme;

fn main() {
    superline::Powerline::builder()
        .set_shell(Shell::Bare) // override this to whatever shell you use
        .add_module(Cwd::<SimpleTheme>::new(45, 4, false))
        .add_module(Git::<SimpleTheme>::new())
        .add_module(ReadOnly::<SimpleTheme>::new())
        .add_module(Cmd::<SimpleTheme>::new("0"))
        .render(0);
}
