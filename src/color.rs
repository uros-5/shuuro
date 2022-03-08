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
    pub fn iter() -> ColorIter {
        ColorIter {
            current: Some(Color::Black),
        }
    }
    /// Returns an iterator of all variants.
    pub fn from_char(ch: char) -> Option<Color> {
        match ch {
            'b' => Some(Color::Black),
            'w' => Some(Color::White),
            _ => Some(Color::NoColor),
        }
    }
    /// Returns the color of the opposite side.
    ///
    /// # Examples
    ///
    /// ```
    /// use shuuro::Color;
    ///
    /// assert_eq!(Color::White, Color::Black.flip());
    /// assert_eq!(Color::Black, Color::White.flip());
    pub fn flip(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
            _ => Color::NoColor,
        }
    }

    pub fn index(self) -> usize {
        self as usize
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
}
