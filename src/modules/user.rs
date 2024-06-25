use std::marker::PhantomData;

use crate::{Powerline, Style, utils};
use crate::colors::Color;
use crate::themes::DefaultColors;

use super::Module;

pub struct User<S: UserScheme> {
    show_on_local: bool,
    scheme: PhantomData<S>,
}

pub trait UserScheme: DefaultColors {
    fn username_root_bg() -> Color {
        Self::default_bg()
    }
    fn username_bg() -> Color {
        Self::default_bg()
    }
    fn username_fg() -> Color {
        Self::default_fg()
    }
}

impl<S: UserScheme> Default for User<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: UserScheme> User<S> {
    pub fn new() -> User<S> {
        User {
            show_on_local: true,
            scheme: PhantomData,
        }
    }

    pub fn show_on_remote_shell() -> User<S> {
        User {
            show_on_local: false,
            scheme: PhantomData,
        }
    }
}

impl<S: UserScheme> Module for User<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        if self.show_on_local || utils::is_remote_shell() {
            let user = users::get_user_by_uid(users::get_current_uid()).unwrap();
            let bg = if user.uid() == 0 {
                S::username_root_bg()
            } else {
                S::username_bg()
            };

            powerline.add_segment(
                user.name().to_str().unwrap(),
                Style::simple(S::username_fg(), bg),
            );
        }
    }
}
