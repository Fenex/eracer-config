#![allow(clippy::try_err)]

mod args;
mod error_code;
mod ratio;
mod resolution;

use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use clap::StructOpt;
use registry::{Data, Hive, Security};
use sha2::{Digest, Sha256};

use crate::args::Args;
use crate::error_code::ErrorCode;
use crate::ratio::*;
use crate::resolution::*;

const SHA2_ORIGINAL_BINARY: &[u8; 32] = &[
    0x7A, 0x3B, 0xF7, 0x73, 0xCB, 0x62, 0x0B, 0x8C, 0x54, 0x7E, 0xE0, 0x4B, 0x40, 0xCD, 0x43, 0xA3,
    0x8D, 0x9C, 0x96, 0x9B, 0x92, 0x5E, 0x9B, 0xEC, 0xA5, 0x36, 0x3B, 0x40, 0x58, 0x8F, 0x93, 0x80,
];

// offset from zero-byte of the binary file
const RATIO_OFFSET: usize = 0x000C7ED8;
const RATIO_LENGTH: usize = 3;
const RATIO_ORIGINAL: &[u8; RATIO_LENGTH] = &[0x3A, 0x46, 0x71];

const ENTRY_KEY: &str = r"Software\Rage Games Ltd\eRacer";
const EXECUTABLE: &str = r"eracer.exe";
const INSTALLDIR_KEY: &str = r"HOVAPPDATA";
const RESOLUTION_WIDTH_KEY: &str = r"PREFERRED WIDTH";
const RESOLUTION_HEIGHT_KEY: &str = r"PREFERRED HEIGHT";

#[derive(Clone)]
struct Settings {
    registry_path: PathBuf,
    override_path: Option<PathBuf>,
    resolution: Resolution,
}

impl std::fmt::Debug for Settings {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("Settings")
            .field("registry_path", &self.registry_path)
            .field("override_path", &self.override_path)
            .field("path", &self.path())
            .field("resolution", &self.resolution)
            .field("ratio", &self.ratio())
            .finish()
    }
}

impl Settings {
    pub fn load(override_path: Option<PathBuf>) -> Result<Self, ErrorCode> {
        let entry = Hive::CurrentUser
            .open(ENTRY_KEY, Security::Read)
            .map_err(|_| ErrorCode::RegistryEntryNotFound)?;

        let registry_path = match entry
            .value(INSTALLDIR_KEY)
            .map_err(|_| ErrorCode::RegistryInstallDirNotFound)?
        {
            Data::String(installed_dir) => {
                Path::new(&installed_dir.to_os_string()).join(EXECUTABLE)
            }
            _ => Err(ErrorCode::RegistryInstallDirIncorrectType)?,
        };

        let width = match entry
            .value(RESOLUTION_WIDTH_KEY)
            .map_err(|_| ErrorCode::RegistryResolutionWidthNotFound)?
        {
            Data::U32(width) => width,
            _ => Err(ErrorCode::RegistryResolutionWidthIncorrectType)?,
        };

        let height = match entry
            .value(RESOLUTION_HEIGHT_KEY)
            .map_err(|_| ErrorCode::RegistryResolutionHeightNotFound)?
        {
            Data::U32(height) => height,
            _ => Err(ErrorCode::RegistryResolutionHeightIncorrectType)?,
        };

        let this = Self {
            registry_path,
            override_path,
            resolution: (width, height).into(),
            // ratio: None,
        };

        Ok(this)
    }

    pub fn path(&self) -> &Path {
        self.override_path
            .as_deref()
            .unwrap_or_else(|| self.registry_path.as_path())
    }

    pub fn ratio(&self) -> Result<Ratio, ErrorCode> {
        if !self.path().is_file() {
            Err(ErrorCode::NotFoundBinary(
                self.path().to_string_lossy().to_string(),
            ))?;
        }

        let file = File::open(self.path()).map_err(ErrorCode::IO)?;
        let mut reader = BufReader::new(file);
        let path = self.path().to_string_lossy().to_string();

        match sha256_with_ignore_ratio(&mut reader) {
            Some((sha2, _)) if &sha2 != SHA2_ORIGINAL_BINARY => {
                Err(ErrorCode::IncorrectHashOfBinary(path))?
            }
            Some((_, Some(ratio_bytes))) => {
                match Ratio::variants().find(|r| r.hex() == &ratio_bytes) {
                    Some(r) => Ok(r),
                    None => Err(ErrorCode::UnknownRatio),
                }
            }
            _ => Err(ErrorCode::NotFoundBinary(path)),
        }
    }

    pub fn set_ratio(&mut self, ratio: Ratio) -> Result<(), ErrorCode> {
        let path = self.path().to_string_lossy().to_string();

        let sha = {
            let file = File::open(self.path()).map_err(ErrorCode::IO)?;
            let mut reader = BufReader::new(file);
            sha256_with_ignore_ratio(&mut reader)
        };

        let file = OpenOptions::new()
            .write(true)
            .open(self.path())
            .map_err(ErrorCode::IO)?;
        match sha {
            Some((sha2, _)) if &sha2 == SHA2_ORIGINAL_BINARY => {
                let mut writer = BufWriter::new(file);
                writer
                    .seek(SeekFrom::Current(RATIO_OFFSET as i64))
                    .map_err(ErrorCode::IO)?;
                writer.write(ratio.hex()).map_err(ErrorCode::IO)?;
            }
            _ => Err(ErrorCode::IncorrectHashOfBinary(path))?,
        }

        Ok(())
    }

    pub fn set_resolution(&mut self, resolution: impl Into<Resolution>) -> Result<(), ErrorCode> {
        let resolution: Resolution = resolution.into();

        let entry = Hive::CurrentUser
            .open(ENTRY_KEY, Security::Write)
            .map_err(|_| ErrorCode::RegistryEntryNotFound)?;

        entry
            .set_value(RESOLUTION_WIDTH_KEY, &Data::U32(resolution.width))
            .map_err(|_| ErrorCode::RegistryResolutionWidthChange)?;

        self.resolution.width = resolution.width;

        entry
            .set_value(RESOLUTION_HEIGHT_KEY, &Data::U32(resolution.height))
            .map_err(|_| ErrorCode::RegistryResolutionHeightChange)?;

        self.resolution.height = resolution.height;

        Ok(())
    }
}

fn main() {
    std::process::exit({
        match app(Args::parse()) {
            Ok(()) => 0,
            Err(e) => {
                eprintln!("{}", e);
                (&e).into()
            }
        }
    });
}

fn app(args: Args) -> Result<(), ErrorCode> {
    let mut settings = Settings::load(args.binary_path.clone())?;

    if let Some(resolution) = args.set_resolution {
        settings.set_resolution(resolution)?;
        println!("A resolution has been set to: {}", resolution);
    }

    if let Some(ratio) = args.set_aspect_ratio {
        settings.set_ratio(ratio)?;
        println!("A ratio has been set to: {}", ratio);
    }

    if args.reset_aspect_ratio {
        settings.set_ratio(Ratio::Original)?;
        println!("A ratio has been reset");
        return Ok(());
    }

    if args.set_aspect_ratio.is_none() && args.set_resolution.is_none() {
        println!("Current settings is:\r\n\r\n{:#?}", settings);
    }

    Ok(())
}

fn sha256_with_ignore_ratio<R: Read>(
    reader: &mut R,
) -> Option<([u8; 32], Option<[u8; RATIO_LENGTH]>)> {
    let mut hasher = Sha256::new();
    let mut buffer = [0; 32];
    let mut offset = 0;
    let mut ratio_from_bfile = None;

    loop {
        let count = reader.read(&mut buffer).ok()?;
        if count == 0 {
            break;
        }

        // We want to calc hash of the binary with original ratio bytes, thus we replace ratio bytes
        // from original binary (hardcoded as `RATIO_ORIGINAL`) on the fly. This works if and only if
        // ALL ratio bytes is included in the same buffer. In our case, all works ok with hardcoded
        // values `RATIO_OFFSET` and 32-byte buffer.
        //
        //   TODO: checks for split ratio bytes into different buffers to possible changing
        // buffer's length without any fear.
        if RATIO_OFFSET > offset && RATIO_OFFSET + RATIO_LENGTH < offset + count {
            let ratio = &mut buffer[RATIO_OFFSET - offset..RATIO_OFFSET - offset + RATIO_LENGTH];
            ratio_from_bfile = Some({
                let mut copied = [0, 0, 0];
                copied.copy_from_slice(ratio);
                copied
            });
            ratio.copy_from_slice(RATIO_ORIGINAL);
        }

        offset += count;

        hasher.update(&buffer[..count]);
    }

    let slice = &hasher.finalize();
    assert_eq!(slice.len(), 32);
    buffer.copy_from_slice(slice);
    Some((buffer, ratio_from_bfile))
}
