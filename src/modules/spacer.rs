use crate::modules::Module;
use crate::{colors, Color, Powerline, Style};
use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct Spacer<S: SpacerScheme> {
    scheme: PhantomData<S>,
    large: bool,
}

pub trait SpacerScheme {
    const BG_COLOR: Color = colors::black();
}

impl<S: SpacerScheme> Spacer<S> {
    pub fn large() -> Spacer<S> {
        Spacer {
            scheme: PhantomData,
            large: true,
        }
    }

    pub fn small() -> Spacer<S> {
        Spacer {
            scheme: PhantomData,
            large: false,
        }
    }
}

impl<S: SpacerScheme> Module for Spacer<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        if self.large {
            powerline.add_segment("", Style::simple(colors::light_grey(), S::BG_COLOR));
        } else {
            powerline.add_short_segment("", Style::simple(colors::light_grey(), S::BG_COLOR));
        }
    }
}
