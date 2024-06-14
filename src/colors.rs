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
term_color!(light_red, 9);
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
term_color!(white, 15);
term_color!(dark_grey, 234);
term_color!(light_grey, 250);
term_color!(mid_grey, 240);

term_color!(turquoise_blue, 31);
term_color!(dark_green, 22);
term_color!(mid_green, 28);
term_color!(mid_red, 124);
term_color!(forest_green, 22);
term_color!(warning_red, 160);
term_color!(burgundy, 52);

term_color!(dark_red, 52);
term_color!(orange, 130);
term_color!(bright_orange, 202);
term_color!(dark_yellow, 136);
term_color!(dark_blue, 19);
term_color!(nice_puple, 55);
term_color!(burnt_orange, 214);
