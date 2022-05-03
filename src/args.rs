use std::path::PathBuf;

use clap::Parser;

use crate::{
    ratio::{Ratio, RatioStr},
    resolution::{Resolution, ResolutionStr},
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long, parse(try_from_str = parse_resolution), help="Set new resolution")]
    pub set_resolution: Option<Resolution>,
    #[clap(long, parse(try_from_str = parse_ratio), help="Set new aspect ratio")]
    pub set_aspect_ratio: Option<Ratio>,
    #[clap(long, help = "Reset aspect ratio to original")]
    pub reset_aspect_ratio: bool,
    #[clap(long, help = "Override path to eracer.exe (gets from windows' registry if not set)")]
    pub binary_path: Option<PathBuf>,
}

fn parse_resolution(s: &str) -> Result<Resolution, &'static str> {
    ResolutionStr(s).try_into()
}

fn parse_ratio(s: &str) -> Result<Ratio, String> {
    RatioStr(s).try_into().map_err(|_| {
        let ratios = Ratio::variants().skip(1).map(|r| r.to_string()).collect::<Vec<_>>();
        format!("\r\nIncorrect or unsupported aspect ratio. Given: `{}`, but expected one of: \r\n\t * {}", s, ratios.join("\r\n\t * "))
    })
}
