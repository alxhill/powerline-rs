use std::marker::PhantomData;
use crate::Color;
use crate::themes::DefaultColors;

pub struct SdkmanJava<S> {
    scheme: PhantomData<S>,
}

pub trait SdkmanScheme: DefaultColors {
    fn sdkman_fg() -> Color {
        Self::default_fg()
    }

    fn sdkman_bg() -> Color {
        Self::default_fg()
    }

    fn icon() -> &'static str {
        "\u{e738}"
    }
}

impl <S: SdkmanScheme> Default for SdkmanJava<S> {
    fn default() -> Self {
        Self::new()
    }
}
impl <S: SdkmanScheme> SdkmanJava<S> {
    pub fn new() -> SdkmanJava<S> {
        SdkmanJava {
            scheme: PhantomData,
        }
    }
}
