use std::iter;

///  Represents each side of player. Black player moves first.
///
/// # Examples
///
/// ```
/// use shuuro::Color;
///
/// let c = Color::Black;
/// match c {
///    Color::Black => assert!(true),
///    Color::White => unreachable!(),
///    Color::NoColor => unreachable!()
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Color {
    Black = 0,
    White = 1,
    NoColor = 2,
}

impl Color {
    /// Returns the color of the opposite side.
    ///
    /// # Examples
    ///
    /// ```
    /// use shuuro::Color;
    ///
    /// assert_eq!(Color::White, Color::Black.flip());
    /// assert_eq!(Color::Black, Color::White.flip());
    /// ```
    pub fn flip(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
            _ => Color::NoColor,
        }
    }

    /// Convert char to `Color`.
    /// # Examples
    ///
    /// ```
    /// use shuuro::Color;
    /// assert_eq!(Some(Color::White), Color::from_char('w'));
    /// assert_eq!(Some(Color::Black), Color::from_char('b'));
    pub fn from_char(ch: char) -> Option<Color> {
        match ch {
            'b' => Some(Color::Black),
            'w' => Some(Color::White),
            _ => Some(Color::NoColor),
        }
    }

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn iter() -> ColorIter {
        ColorIter {
            current: Some(Color::Black),
        }
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::White => String::from("w"),
            Color::Black => String::from("b"),
            Color::NoColor => String::from(""),
        }
    }
}

impl From<usize> for Color {
    fn from(u: usize) -> Self {
        match u {
            0 => Self::White,
            1 => Self::Black,
            _ => Self::NoColor,
        }
    }
}

pub struct ColorIter {
    current: Option<Color>,
}

impl iter::Iterator for ColorIter {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        if let Some(current) = self.current {
            self.current = match current {
                Color::Black => Some(Color::White),
                Color::White => Some(Color::NoColor),
                Color::NoColor => None,
            }
        }

        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip() {
        assert_eq!(Color::White, Color::Black.flip());
        assert_eq!(Color::Black, Color::White.flip());
    }

    #[test]
    fn from_char() {
        assert_eq!(Some(Color::White), Color::from_char('w'));
        assert_eq!(Some(Color::Black), Color::from_char('b'));
        assert_eq!(Some(Color::NoColor), Color::from_char('l'));
        assert_eq!(Some(Color::NoColor), Color::from_char('d'));
    }

    #[test]
    fn from_index() {
        assert_eq!(Color::Black.index(), 0);
        assert_eq!(Color::White.index(), 1);
        assert_eq!(Color::NoColor.index(), 2);
    }
}
