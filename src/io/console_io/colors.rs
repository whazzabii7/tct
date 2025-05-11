use std::fmt;

#[derive(Debug)]
#[repr(u8)]
pub enum Color {
    Input,
    Command,
    BannerColor,
    TctGreen,
    InfoBlue,
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Cyan,
    Magenta,
    Purple,
    White,
    Black,
    C256(u8),
}

impl Color {
    fn convert(&self) -> u8 {
        match self {
            Color::C256(val) => *val,
            Color::Input => 63,
            Color::Command => 172,
            Color::BannerColor => 115,
            Color::TctGreen => 192,
            Color::InfoBlue => 75,
            Color::Red => 160,
            Color::Green => 40,
            Color::Blue => 21,
            Color::Yellow => 220,
            Color::Orange => 214,
            Color::Cyan => 87,
            Color::Magenta => 201,
            Color::Purple => 91,
            Color::White => 231,
            Color::Black => 0,
        }
    }
}

#[derive(Debug)]
pub enum Colorize {
    Fg(Color),
    Bg(Color),
    Reset,
}

// uses fmt::Display trait as substitution for a
// colorize function to reduce overhead
impl fmt::Display for Colorize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Colorize::Fg(color) => write!(f, 
                "{}[38;5;{}m", crate::console_io::ESC, color.convert()),
            Colorize::Bg(color) => write!(f, 
                "{}[48;5;{}m", crate::console_io::ESC, color.convert()),
            Colorize::Reset => write!(f, 
                "{}[0m", crate::console_io::ESC),
        }
    }
}
