macro_rules! term_color {
    ($name:ident, $code:expr) => {
        pub fn $name() -> Color {
            Color($code)
        }
    };
}

pub mod fg {
    use crate::Color;

    term_color!(black, 30);
    term_color!(red, 31);
    term_color!(green, 32);
    term_color!(yellow, 33);
    term_color!(blue, 34);
    term_color!(magenta, 35);
    term_color!(cyan, 36);
    term_color!(light_gray, 37);
    term_color!(gray, 90);
    term_color!(light_red, 91);
    term_color!(light_green, 92);
    term_color!(light_yellow, 93);
    term_color!(light_blue, 94);
    term_color!(light_magenta, 95);
    term_color!(light_cyan, 96);
    term_color!(white, 97);
}

pub mod bg {
    use crate::Color;

    term_color!(black, 40);
    term_color!(red,41);
    term_color!(green,42);
    term_color!(yellow,43);
    term_color!(blue,44);
    term_color!(magenta,45);
    term_color!(cyan,46);
    term_color!(light_gray,47);
    term_color!(gray,100);
    term_color!(light_red,101);
    term_color!(light_green,102);
    term_color!(light_yellow,103);
    term_color!(light_blue,104);
    term_color!(light_magenta,105);
    term_color!(light_cyan,106);
    term_color!(white,107);
}
