use std::iter;

///  Represents each side of player. Blue player moves first.
///
/// # Examples
///
/// ```
/// use shuuro::Color;
///
/// let c = Color::Blue;
/// match c {
///    Color::Blue => assert!(true),
///    Color::Red => unreachable!(),
///    Color::NoColor => unreachable!()
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Color {
    Blue = 0,
    Red = 1,
    NoColor = 2,
}

impl Color {
    pub fn iter() -> ColorIter {
        ColorIter {
            current: Some(Color::Red),
        }
    }
    /// Returns an iterator of all variants.
    pub fn from_char(ch: char) -> Option<Color> {
        match ch {
            'b' => Some(Color::Blue),
            'r' => Some(Color::Red),
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
    /// assert_eq!(Color::Red, Color::Blue.flip());
    /// assert_eq!(Color::Blue, Color::Red.flip());
    pub fn flip(&self) -> Color {
        match self {
            Color::Red => Color::Blue,
            Color::Blue => Color::Red,
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
                Color::Blue => Some(Color::Red),
                Color::Red => Some(Color::NoColor),
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
        assert_eq!(Color::Red, Color::Blue.flip());
        assert_eq!(Color::Blue, Color::Red.flip());
    }
}
