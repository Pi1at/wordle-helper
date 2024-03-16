use derive_more::Display;
use std::fmt::{self, Formatter};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Display)]
enum Position {
    #[display("⬛")]
    Miss,
    #[display("🟨")]
    Misplaced,
    #[display("🟩")]
    Exact,
}

impl TryFrom<u8> for Position {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Miss),
            1 => Ok(Self::Misplaced),
            2 => Ok(Self::Exact),
            _ => Err("value must be less than 3"),
        }
    }
}

// impl From<Position> for u8 {
//     fn from(val: Position) -> Self {
//         match val {
//             Position::Miss => 0,
//             Position::Misplaced => 1,
//             Position::Exact => 2,
//         }
//     }
// }

// Rather than representing a color pattern as a lists of integers,
// store it as a single integer, whose ternary representations corresponds
// to that list of integers.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Pattern(u8);

impl From<&[Position; 5]> for Pattern {
    fn from(value: &[Position; 5]) -> Self {
        Self(
            value
                .iter()
                .rev()
                .map(|&p| p as u8)
                .fold(0u8, |acc, c| acc * 3u8 + c),
        )
    }
}

impl From<Pattern> for [u8; 5] {
    fn from(value: Pattern) -> Self {
        let mut result = [0u8; 5];
        let mut curr = value.0;
        (0..5).for_each(|i| {
            result[i] = curr % 3;
            curr /= 3;
        });
        result
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let d = ['⬛', '🟨', '🟩'];
        let y = Into::<[u8; 5]>::into(*self)
            .into_iter()
            .map(|s| d[s as usize])
            .collect::<String>();
        write!(f, "{y}")
    }
}

fn pattern_from_string(pattern: &str) -> usize {
    pattern
        .chars()
        .enumerate()
        .map(|(i, c)| c as usize * 3_usize.pow(i as u32))
        .sum()
}

fn pattern_to_int_list(pattern: usize) -> Vec<usize> {
    let mut result = Vec::with_capacity(5);
    let mut curr = pattern;
    for _ in 0..5 {
        result.push(curr % 3);
        curr /= 3;
    }
    result
}

fn pattern_to_string(pattern: usize) -> String {
    let d = ['⬛', '🟨', '🟩'];
    pattern_to_int_list(pattern)
        .into_iter()
        .map(|i| d[i])
        .collect()
}
#[derive(Debug)]
struct Word {
    bytes: [u8; 5],
    positions: [u8; 26], // bitmask of locations of that letter, e.g. for aabce, positions = 00011, 00100, 01000, 00000, 10000,....
    unique_bytes: u32,
    w: String,
}

impl Word {
    fn new(word: &'static str) -> Self {
        let mut unique_bytes = 0;
        let mut bytes = [0; 5];
        for i in 0..5 {
            bytes[i] = word.as_bytes()[i] - b'a';
            unique_bytes |= 1 << bytes[i];
        }
        let mut positions = [0u8; 26];
        for (i, &b) in bytes.iter().enumerate() {
            positions[b as usize] |= 1 << (i as u8);
        }
        Self {
            bytes,
            positions,
            unique_bytes,
            w: word.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        dbg!('4'.to_digit(3));
        let word = "hello";
        println!("{}", Position::Misplaced);
        println!(
            "{}",
            Pattern::from(&[
                Position::Misplaced,
                Position::Misplaced,
                Position::Miss,
                Position::Misplaced,
                Position::Misplaced
            ])
        );
        dbg!(Word::new("salet"));
    }

    #[test]
    fn test_word() {
        let w1 = Word::new("aaabc");
        let w2 = Word::new("aabyz");
        let mut common = dbg!(w1.unique_bytes & w2.unique_bytes);
        assert_eq!(common, 0b11);
        let mut v_green = vec![];
        while common != 0 {
            let j = dbg!(common.trailing_zeros() as usize);
            let green = w1.positions[j] & w2.positions[j];
            v_green.push(green);
            common -= 1 << j;
        }
        assert_eq!(v_green, vec![0b11, 0b0]);
    }
}
