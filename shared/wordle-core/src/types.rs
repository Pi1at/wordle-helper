use std::{
    collections::{HashMap, HashSet},
    ops::Index,
};

use unicode_segmentation::UnicodeSegmentation;

/// Codepage - converting between allowed chars and u8
/// mappping grapheme into Letter (u8)
/// get grapheme by Letter (u8)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Codepage {
    graphemes: Box<[Box<str>]>,
}

impl Codepage {
    fn idx(&self, s: &str) -> usize {
        for i in 0..self.graphemes.len() {
            if *s == *self.graphemes[i] {
                return i;
            }
        }
        0
    }
}

/// Assumes that every &str is unique grapheme for our codepage
impl<'a> FromIterator<&'a str> for Codepage {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let graphemes =
            iter.into_iter().map(Box::from).collect::<Box<[Box<str>]>>();
        Self { graphemes }
    }
}

impl Index<Letter> for Codepage {
    type Output = str;

    fn index(&self, index: Letter) -> &Self::Output {
        self.graphemes[index.into_byte() as usize].as_ref()
    }
}

impl From<&str> for Codepage {
    /// building alphabet from string with allowed chars
    fn from(allowed_chars: &str) -> Self {
        let gi = UnicodeSegmentation::graphemes(allowed_chars, true);
        // store unique graphemes in HashMap
        let g_map = UnicodeSegmentation::graphemes(allowed_chars, true)
            //.map(Box::<str>::from)
            .enumerate()
            .fold(HashMap::<_, usize>::new(), |mut acc, (v, k)| {
                acc.entry(k).or_insert(v);
                acc
            });
        let g_set = gi.collect::<HashSet<_>>();
        let graphemes = g_set
            .into_iter()
            .map(Box::<str>::from)
            .collect::<Box<[Box<str>]>>();
        Self { graphemes }
    }
}

/// A possible letter
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Letter(u8);

impl Letter {
    /// Create a new letter
    ///
    /// Returns None if the letter is not in [a-zA-Z]
    #[must_use]
    pub const fn try_new(b: u8) -> Option<Self> {
        match b {
            b'a'..=b'z' => Some(Self(b - b'a')),
            b'A'..=b'Z' => Some(Self(b - b'A')),
            _ => None,
        }
    }

    /// Create a new letter
    ///
    /// # Panics
    /// Panics if the letter is not in [a-zA-Z]
    #[must_use]
    pub const fn new(b: u8) -> Self {
        match b {
            b'a'..=b'z' => Self(b - b'a'),
            b'A'..=b'Z' => Self(b - b'A'),
            _ => panic!("Invalid letter, only accept [a-zA-Z]"),
        }
    }

    /// Return the byte representation of this letter
    #[must_use]
    pub const fn into_byte(self) -> u8 {
        self.0 + b'a'
    }
}

impl From<Letter> for u8 {
    fn from(letter: Letter) -> Self {
        letter.into_byte()
    }
}

impl From<Letter> for usize {
    fn from(value: Letter) -> Self {
        value.into_byte() as Self
    }
}

// for every wordlist we need build tree
//
//
struct Node {
    word: String, // or just index in wordlist
    next: HashMap<u8, Node>,
}

impl Node {
    fn new() -> Self {
        Self { word: String::new(), next: HashMap::new() }
    }
}

#[cfg(test)]
mod tests {
    type Error = Box<dyn std::error::Error>;
    type Result<T> = core::result::Result<T, Error>; // For tests.

    use super::*;
    use crate::wordlist;

    #[test]
    fn test_name() {
        let x = dbg!(Codepage::from(wordlist::CHARACTERS));
        let x = dbg!(Codepage::from("⬛🟨🟩⬛"));
    }
}
