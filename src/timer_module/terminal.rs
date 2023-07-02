#[derive(Clone, Copy)]
pub enum ANSICode {
    Reset,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl ANSICode {
    fn value(&self) -> &str {
        match self {
            ANSICode::Reset => "\x1b[0m",
            ANSICode::Red => "\x1b[31m",
            ANSICode::Green => "\x1b[32m",
            ANSICode::Yellow => "\x1b[33m",
            ANSICode::Blue => "\x1b[34m",
            ANSICode::Magenta => "\x1b[35m",
            ANSICode::Cyan => "\x1b[36m",
            ANSICode::White => "\x1b[37m",
        }
    }
}

pub struct Terminal {
    ansi_color: ANSICode,
    ansi_reset: ANSICode,
}

impl Terminal {
    pub fn new() -> Terminal {
        let ansi_color: ANSICode = ANSICode::White;
        let ansi_reset: ANSICode = ANSICode::Reset;

        Terminal {
            ansi_color,
            ansi_reset,
        }
    }

    pub fn write(&self, text: &str) {
        let ansi_color_val: &str = self.ansi_color.value();
        let ansi_reset_val: &str = self.ansi_reset.value();
        println!("{}{}{}", ansi_color_val, text, ansi_reset_val);
    }

    pub fn set_ansi_color(&mut self, ansi_color: ANSICode) {
        self.ansi_color = ansi_color;
    }
}
