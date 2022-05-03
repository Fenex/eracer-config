#![allow(clippy::try_err)]

mod args;
mod error_code;
mod patch;
mod ratio;
mod resolution;

use std::path::{Path, PathBuf};

use clap::StructOpt;
use registry::{Data, Hive, Security};

use crate::args::Args;
use crate::error_code::ErrorCode;
use crate::patch::*;
use crate::ratio::*;
use crate::resolution::*;

const ENTRY_KEY: &str = r"Software\Rage Games Ltd\eRacer";
const EXECUTABLE: &str = r"eracer.exe";
const INSTALLDIR_KEY: &str = r"HOVAPPDATA";
const RESOLUTION_WIDTH_KEY: &str = r"PREFERRED WIDTH";
const RESOLUTION_HEIGHT_KEY: &str = r"PREFERRED HEIGHT";

struct Settings {
    registry_path: PathBuf,
    override_path: Option<PathBuf>,
    resolution: Resolution,
    binary: Option<Binary>,
}

impl std::fmt::Debug for Settings {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("Settings")
            .field("registry_path", &self.registry_path)
            .field("override_path", &self.override_path)
            .field("path", &self.path())
            .field("resolution", &self.resolution)
            .field("binary", &self.binary)
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

        let binary = Binary::new(override_path.as_deref().unwrap_or_else(|| &registry_path)).ok();

        let this = Self {
            registry_path,
            override_path,
            resolution: (width, height).into(),
            binary,
        };

        Ok(this)
    }

    pub fn path(&self) -> &Path {
        self.override_path
            .as_deref()
            .unwrap_or_else(|| self.registry_path.as_path())
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

fn app(mut args: Args) -> Result<(), ErrorCode> {
    let mut settings = Settings::load(args.binary_path.clone())?;

    if let Some(resolution) = args.set_resolution {
        settings.set_resolution(resolution)?;
        println!("A resolution has been set to: {}", resolution);
    }

    if args.reset_aspect_ratio {
        args.set_aspect_ratio = Some(Ratio::Original);
    }

    if let Some(ratio) = args.set_aspect_ratio {
        if let Some(ref mut binary) = settings.binary {
            binary.set_ratio(ratio)?;
            println!("A ratio has been set to: {}", ratio);
        } else {
            println!("File not found or unknown version of the binary ({:?})", settings.path());
        }
    }

    if args.set_aspect_ratio.is_none() && args.set_resolution.is_none() && !args.reset_aspect_ratio
    {
        println!("Current settings is:\r\n\r\n{:#?}", settings);
    }

    Ok(())
}
