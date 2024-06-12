#![allow(dead_code)]
use crate::Color;

macro_rules! term_color {
    ($name:ident, $code:expr) => {
        pub const fn $name() -> Color {
            Color($code)
        }
    };
}

term_color!(black, 0);
term_color!(red, 1);
term_color!(green, 2);
term_color!(light_green, 10);
term_color!(yellow, 3);
term_color!(light_yellow, 11);
term_color!(blue, 4);
term_color!(light_blue, 12);
term_color!(purple, 5);
term_color!(light_purple, 13);
term_color!(turquoise, 6);
term_color!(light_turquoise, 14);
term_color!(grey, 7);
term_color!(light_grey, 15);

term_color!(turquoise_blue, 31);
term_color!(dark_green, 22);
term_color!(mid_green, 28);
term_color!(mid_red, 124);
