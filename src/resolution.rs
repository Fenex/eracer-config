use std::{fmt::Display, ops::Deref};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl From<(u32, u32)> for Resolution {
    fn from((width, height): (u32, u32)) -> Self {
        Self { width, height }
    }
}

#[allow(dead_code)]
impl Resolution {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

// This is a wrapper to avoid a trait blanket TryFrom problem
// https://github.com/rust-lang/rust/issues/50133
pub struct ResolutionStr<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> Deref for ResolutionStr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: AsRef<str>> TryFrom<ResolutionStr<T>> for Resolution {
    type Error = &'static str;

    fn try_from(value: ResolutionStr<T>) -> Result<Self, Self::Error> {
        let mut split = value.as_ref().split(|c| c == 'x' || c == 'X');
        if let (Some(w), Some(h), None) = (split.next(), split.next(), split.next()) {
            if let (Ok(w), Ok(h)) = (w.parse(), h.parse()) {
                return if w < 640 {
                    Err("minimal width is 640")
                } else if h < 480 {
                    Err("minimal height is 480")
                } else {
                    Ok((w, h).into())
                };
            }
        }

        Err("invalid format")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn resolution_from_str() {
        assert_eq!(
            ResolutionStr("800x600").try_into(),
            Ok(Resolution {
                width: 800,
                height: 600
            })
        );

        let resolution: Result<Resolution, _> = ResolutionStr("0x0").try_into();
        assert!(resolution.is_err());

        let resolution: Result<Resolution, _> = ResolutionStr("639x479").try_into();
        assert!(resolution.is_err());

        let resolution: Result<Resolution, _> = ResolutionStr("-10x10").try_into();
        assert!(resolution.is_err());

        let resolution: Result<Resolution, _> = ResolutionStr("x10x10").try_into();
        assert!(resolution.is_err());

        let resolution: Result<Resolution, _> = ResolutionStr("x1010").try_into();
        assert!(resolution.is_err());

        let resolution: Result<Resolution, _> = ResolutionStr("10x").try_into();
        assert!(resolution.is_err());
    }
}
