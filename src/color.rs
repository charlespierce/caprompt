#[derive(Clone, Copy)]
pub enum Color {
    Red,
    Yellow,
    Cyan,
    Reset,
}

impl Color {
    pub fn as_str(self) -> &'static str {
        match self {
            Color::Red => "\u{001b}[31m",
            Color::Yellow => "\u{001b}[33m",
            Color::Cyan => "\u{001b}[36m",
            Color::Reset => "\u{001b}[0m",
        }
    }
}
