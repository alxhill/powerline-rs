use std::marker::PhantomData;
use crate::{Color, Powerline};
use crate::modules::Module;

pub struct Jenv<S> {
    scheme: PhantomData<S>
}

pub trait JenvScheme {
    const BG_COLOR: Color;
    const FG_COLOR: Color;
}

impl<S: JenvScheme> Module for Jenv<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        todo!();
    }
}

fn find_jenv() {
    todo!();
}