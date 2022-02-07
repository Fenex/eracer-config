use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum ErrorCode {
    RegistryEntryNotFound,
    RegistryInstallDirNotFound,
    RegistryInstallDirIncorrectType,
    RegistryResolutionWidthNotFound,
    RegistryResolutionHeightNotFound,
    RegistryResolutionWidthIncorrectType,
    RegistryResolutionHeightIncorrectType,
    RegistryResolutionWidthChange,
    RegistryResolutionHeightChange,
    NotFoundBinary(String),
    IncorrectHashOfBinary(String),
    IO(std::io::Error),
    UnknownRatio,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use ErrorCode::*;

        match self {
            RegistryEntryNotFound => write!(f, r"Cannot find entry in windows registry. Try to run `eracer.exe` first or reinstall E-Racer."),
            RegistryInstallDirNotFound => write!(f, r"Cannot find entry `HOVAPPDATA` in windows registry. Try to run current `eracer.exe` first or reinstall E-Racer."),
            RegistryInstallDirIncorrectType => write!(f, r"Incorrect type of the entry `HOVAPPDATA` in windows registry. Try to reinstall E-Racer."),
            RegistryResolutionWidthNotFound => write!(f, r"Cannot find entry `PREFERRED WIDTH` in windows registry. Try to run current `eracer.exe` first or reinstall E-Racer.."),
            RegistryResolutionHeightNotFound => write!(f, r"Cannot find entry `PREFERRED HEIGHT` in windows registry. Try to run current `eracer.exe` first or reinstall E-Racer."),
            RegistryResolutionWidthIncorrectType => write!(f, r"Incorrect type of the entry `PREFERRED WIDTH` in windows registry. Try to reinstall E-Racer."),
            RegistryResolutionHeightIncorrectType => write!(f, r"Incorrect type of the entry `PREFERRED HEIGHT` in windows registry. Try to reinstall E-Racer."),
            RegistryResolutionWidthChange => write!(f, r"Cannot change the entry `PREFERRED WIDTH` in windows registry."),
            RegistryResolutionHeightChange => write!(f, r"Cannot change the entry `PREFERRED HEIGHT` in windows registry."),
            NotFoundBinary(s) => write!(f, "Not found a binary file: `{}`. You can try to specify a path to `eracer.exe` by passthrough key `--binary-path`.", s),
            IncorrectHashOfBinary(s) => write!(f, "Incorrect hash of a binary `{}`. Is path to `eracer.exe` correct?", s),
            IO(e) => write!(f, "IO error: {:?}", e),
            UnknownRatio => write!(f, "Unknown aspect ratio"),
        }
    }
}

impl From<&ErrorCode> for i32 {
    fn from(v: &ErrorCode) -> Self {
        use ErrorCode::*;

        match v {
            RegistryEntryNotFound => 2,
            RegistryInstallDirNotFound => 3,
            RegistryInstallDirIncorrectType => 4,
            RegistryResolutionWidthNotFound => 5,
            RegistryResolutionHeightNotFound => 6,
            RegistryResolutionWidthIncorrectType => 7,
            RegistryResolutionHeightIncorrectType => 8,
            RegistryResolutionWidthChange => 9,
            RegistryResolutionHeightChange => 10,
            NotFoundBinary(_) => 11,
            IncorrectHashOfBinary(_) => 12,
            IO(_) => 13,
            UnknownRatio => 14,
        }
    }
}
