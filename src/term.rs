// TODO: read all of this from terminfo

#[derive(Default, Debug)]
pub struct ClearScreen;

impl ClearScreen {
    pub fn new() -> Self {
        Self::default()
    }
}

impl std::fmt::Display for ClearScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\x1b[H\x1b[J")
    }
}

#[derive(Default, Debug)]
pub struct CRLF;

impl CRLF {
    pub fn new() -> Self {
        Self::default()
    }
}

impl std::fmt::Display for CRLF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\r\n")
    }
}

#[derive(Default, Debug)]
pub struct MoveTo {
    row: u16,
    col: u16,
}

impl MoveTo {
    pub fn new(pos: crate::grid::Pos) -> Self {
        Self {
            row: pos.row,
            col: pos.col,
        }
    }
}

impl std::fmt::Display for MoveTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.row == 0 && self.col == 0 {
            f.write_str("\x1b[H")
        } else {
            write!(f, "\x1b[{};{}H", self.row + 1, self.col + 1)
        }
    }
}

#[derive(Default, Debug)]
pub struct Attrs {
    fgcolor: Option<crate::attrs::Color>,
    bgcolor: Option<crate::attrs::Color>,
    bold: Option<bool>,
    italic: Option<bool>,
    underline: Option<bool>,
    inverse: Option<bool>,
}

impl Attrs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fgcolor(mut self, fgcolor: crate::attrs::Color) -> Self {
        self.fgcolor = Some(fgcolor);
        self
    }

    pub fn bgcolor(mut self, bgcolor: crate::attrs::Color) -> Self {
        self.bgcolor = Some(bgcolor);
        self
    }

    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
        self
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = Some(italic);
        self
    }

    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = Some(underline);
        self
    }

    pub fn inverse(mut self, inverse: bool) -> Self {
        self.inverse = Some(inverse);
        self
    }
}

impl std::fmt::Display for Attrs {
    #[allow(unused_assignments, clippy::cognitive_complexity)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\x1b[")?;
        let mut first = true;

        macro_rules! write_param {
            ($i:expr) => {
                if first {
                    first = false;
                } else {
                    f.write_str(";")?;
                }
                write!(f, "{}", $i)?;
            };
        }

        if let Some(fgcolor) = self.fgcolor {
            match fgcolor {
                crate::attrs::Color::Default => {
                    write_param!(39);
                }
                crate::attrs::Color::Idx(i) => {
                    if i < 8 {
                        write_param!(i + 30);
                    } else if i < 16 {
                        write_param!(i + 82);
                    } else {
                        write_param!(38);
                        write_param!(5);
                        write_param!(i);
                    }
                }
                crate::attrs::Color::Rgb(r, g, b) => {
                    write_param!(38);
                    write_param!(2);
                    write_param!(r);
                    write_param!(g);
                    write_param!(b);
                }
            }
        }

        if let Some(bgcolor) = self.bgcolor {
            match bgcolor {
                crate::attrs::Color::Default => {
                    write_param!(49);
                }
                crate::attrs::Color::Idx(i) => {
                    if i < 8 {
                        write_param!(i + 40);
                    } else if i < 16 {
                        write_param!(i + 92);
                    } else {
                        write_param!(48);
                        write_param!(5);
                        write_param!(i);
                    }
                }
                crate::attrs::Color::Rgb(r, g, b) => {
                    write_param!(48);
                    write_param!(2);
                    write_param!(r);
                    write_param!(g);
                    write_param!(b);
                }
            }
        }

        if let Some(bold) = self.bold {
            if bold {
                write_param!(1);
            } else {
                write_param!(22);
            }
        }

        if let Some(italic) = self.italic {
            if italic {
                write_param!(3);
            } else {
                write_param!(23);
            }
        }

        if let Some(underline) = self.underline {
            if underline {
                write_param!(4);
            } else {
                write_param!(24);
            }
        }

        if let Some(inverse) = self.inverse {
            if inverse {
                write_param!(7);
            } else {
                write_param!(27);
            }
        }

        f.write_str("m")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct MoveRight {
    count: u16,
}

impl MoveRight {
    pub fn new(count: u16) -> Self {
        Self { count }
    }
}

impl Default for MoveRight {
    fn default() -> Self {
        Self { count: 1 }
    }
}

impl std::fmt::Display for MoveRight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.count {
            0 => Ok(()),
            1 => f.write_str("\x1b[C"),
            n => write!(f, "\x1b[{}C", n),
        }
    }
}

#[derive(Debug)]
pub struct EraseChar {
    count: u16,
}

impl Default for EraseChar {
    fn default() -> Self {
        Self { count: 1 }
    }
}

impl std::fmt::Display for EraseChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.count {
            0 => Ok(()),
            1 => f.write_str("\x1b[X"),
            n => write!(f, "\x1b[{}X", n),
        }
    }
}

#[derive(Default, Debug)]
pub struct HideCursor {
    hide: bool,
}

impl HideCursor {
    pub fn new(hide: bool) -> Self {
        Self { hide }
    }
}

impl std::fmt::Display for HideCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.hide {
            f.write_str("\x1b[?25l")
        } else {
            f.write_str("\x1b[?25h")
        }
    }
}