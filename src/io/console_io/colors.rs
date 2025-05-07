use std::fmt;

#[derive(Debug)]
pub enum Color {
    Input=63,
    Command=172,
    Red=160,
    Green=40,
    Blue=21,
    Yellow=220,
    Orange=214,
    Cyan=87,
    Magenta=201,
    Purple=91,
    White=231,
    Black=0,
}

impl From<Color> for u8 {
    fn from(color: Color) -> Self {
        color as u8
    }
}

impl TryFrom<u8> for Color {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {

            63  => Ok(Color::Input),
            172 => Ok(Color::Command),
            0   => Ok(Color::Black),
            231 => Ok(Color::White),
            91  => Ok(Color::Purple),
            201 => Ok(Color::Magenta),
            87  => Ok(Color::Cyan),
            214 => Ok(Color::Orange),
            220 => Ok(Color::Yellow),
            160 => Ok(Color::Red),
            40  => Ok(Color::Green),
            21  => Ok(Color::Blue),
            _   => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Colorize {
    Fg(u8),
    Bg(u8),
    Reset,
}

impl fmt::Display for Colorize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Colorize::Fg(color) => write!(f, 
                "{}[38:5:{}m", crate::console_io::ESC, color),
            Colorize::Bg(color) => write!(f, 
                "{}[48:5:{}m", crate::console_io::ESC, color),
            Colorize::Reset => write!(f, 
                "{}[0m", crate::console_io::ESC),
        }
    }
}
