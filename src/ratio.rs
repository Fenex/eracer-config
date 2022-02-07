use std::{fmt::Display, ops::Deref};

use crate::RATIO_ORIGINAL;

#[derive(Debug, Default)]
pub struct RatioIterator(usize);

impl Iterator for RatioIterator {
    type Item = Ratio;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        Some(match self.0 {
            1 => Ratio::Original,
            2 => Ratio::W5H4,
            3 => Ratio::W25H16,
            4 => Ratio::W16H10,
            5 => Ratio::W15H9,
            6 => Ratio::W16H9,
            7 => Ratio::W21H9,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Ratio {
    Original,
    W5H4,
    W25H16,
    W16H10,
    W15H9,
    W16H9,
    W21H9,
}

impl Ratio {
    pub fn hex(&self) -> &[u8; 3] {
        match self {
            Ratio::Original => RATIO_ORIGINAL,
            Ratio::W5H4 => &[0x66, 0x66, 0x66],
            Ratio::W25H16 => &[0x3B, 0xDF, 0x87],
            Ratio::W16H10 => &[0x61, 0xE0, 0x89],
            Ratio::W15H9 => &[0xBA, 0x2C, 0x8E],
            Ratio::W16H9 => &[0xE3, 0xA5, 0x93],
            Ratio::W21H9 => &[0x29, 0x5C, 0xAF],
        }
    }

    pub fn variants() -> RatioIterator {
        RatioIterator::default()
    }

    pub fn value(&self) -> (u32, u32) {
        match self {
            Ratio::Original | Ratio::W5H4 => (5, 4),
            Ratio::W25H16 => (25, 16),
            Ratio::W16H10 => (16, 10),
            Ratio::W15H9 => (15, 9),
            Ratio::W16H9 => (16, 9),
            Ratio::W21H9 => (21, 9),
        }
    }

    pub fn w(&self) -> u32 {
        self.value().0
    }

    pub fn h(&self) -> u32 {
        self.value().1
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.w(), self.h())
    }
}

// This is a wrapper to avoid a trait blanket TryFrom problem
// https://github.com/rust-lang/rust/issues/50133
pub struct RatioStr<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> Deref for RatioStr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: AsRef<str>> TryFrom<RatioStr<T>> for Ratio {
    type Error = &'static str;

    fn try_from(value: RatioStr<T>) -> Result<Self, Self::Error> {
        let mut width = 0;
        let mut height = 0;

        for (count, char) in value.as_ref().chars().enumerate() {
            if char == ':' {
                width = (&value.as_ref()[0..count])
                    .parse()
                    .map_err(|_| "parse `W` error")?;
                height = (&value.as_ref()[count + 1..])
                    .parse()
                    .map_err(|_| "parse `H` error")?;
                break;
            }
        }

        if width == 8 && height == 5 {
            width = 16;
            height = 10;
        }

        for ratio in Ratio::variants() {
            if ratio.w() == width && ratio.h() == height {
                return Ok(ratio);
            }
        }

        Err("Incorrect value of aspect ratio arg.")
    }
}
