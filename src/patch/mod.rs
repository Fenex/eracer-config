mod items;

use std::{
    fmt::Debug,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};

use crate::{
    error_code::ErrorCode,
    ratio::{Ratio, RATIO_ORIGINAL},
};

pub struct Binary {
    path: PathBuf,
    patch: Box<dyn Patch>,
    aspect: [u8; 3],
}

impl Debug for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Binary")
            .field("path", &self.path)
            .field("patch", &self.patch.name())
            .field("aspect", &Ratio::try_from(&self.aspect))
            .finish()
    }
}

impl Binary {
    pub fn new(path: &Path) -> Result<Self, ErrorCode> {
        let file = File::open(path).map_err(ErrorCode::IO)?;
        let mut reader = BufReader::new(file);
        let (patch, aspect) = get_patch_by_binary(&mut reader).ok_or(
            ErrorCode::IncorrectHashOfBinary(path.to_string_lossy().to_string()),
        )?;

        Ok(Self {
            path: path.to_owned(),
            patch,
            aspect,
        })
    }

    pub fn set_ratio(&mut self, ratio: Ratio) -> Result<(), ErrorCode> {
        let file = OpenOptions::new()
            .write(true)
            .open(&self.path)
            .map_err(ErrorCode::IO)?;

        let mut writer = BufWriter::new(file);
        writer
            .seek(SeekFrom::Current(self.patch.ratio_offset() as i64))
            .map_err(ErrorCode::IO)?;
        writer.write(ratio.hex()).map_err(ErrorCode::IO)?;
        self.aspect.copy_from_slice(ratio.hex());

        Ok(())
    }
}

fn get_patch_by_binary<R: Read>(reader: &mut R) -> Option<(Box<dyn Patch>, [u8; 3])> {
    let mut patch_detectors = get_all_patches().map(|i| BinaryDetector::from(i));
    let mut buffer = [0; 32];
    // let mut offset = 0;

    loop {
        let count = reader.read(&mut buffer).ok()?;
        if count == 0 {
            break;
        }

        for p in &mut patch_detectors {
            p.update(&buffer[..count]);
        }
    }

    patch_detectors
        .into_iter()
        .map(|i| i.finish())
        .find(Option::is_some)?
}

const PATCHES_COUNT: usize = 3;

fn get_all_patches() -> [Box<dyn Patch>; PATCHES_COUNT] {
    [Box::new(items::PatchRU_1008), Box::new(items::PatchEN_992), Box::new(items::Patch_Zoom)]
}

pub trait Patch {
    fn name(&self) -> &'static str;

    /// sha2 of the original binary file
    fn sha2(&self) -> &'static [u8; 32];

    /// offset from zero-byte of the binary file
    fn ratio_offset(&self) -> usize;

    fn ratio_len(&self) -> usize {
        crate::ratio::RATIO_LENGTH
    }
}

struct BinaryDetector {
    patch: Box<dyn Patch>,
    hasher: Sha256,
    ratio: Option<[u8; 3]>,
    byte_count: usize,
}

impl BinaryDetector {
    pub fn update(&mut self, data: &[u8]) {
        let count = data.len();
        let ratio_offset = self.patch.ratio_offset();
        let ratio_len = self.patch.ratio_len();

        // We want to calc hash of the binary with original ratio bytes, thus we replace ratio bytes
        // from original binary (hardcoded as `RATIO_ORIGINAL`) on the fly. This works if and only if
        // ALL ratio bytes is included in the same buffer. In our case, all works ok with hardcoded
        // values in all `Patch::ratio_offset` methods and a 32-byte buffer.
        //
        //   TODO: checks for split ratio bytes into different buffers to possible changing
        // buffer's length without any fear.
        if ratio_offset > self.byte_count && ratio_offset + ratio_len < self.byte_count + count {
            // println!("{}\r\n{:X?}", &self.patch.name(), &data[..]);
            // println!(
            //     "{}\r\n{:X?}",
            //     &self.patch.name(),
            //     &data[..ratio_offset - self.byte_count]
            // );
            // println!(
            //     "{}\r\n{:X?}",
            //     &self.patch.name(),
            //     &data[ratio_offset - self.byte_count + ratio_len..]
            // );

            self.hasher.update(&data[..ratio_offset - self.byte_count]);
            self.hasher.update(RATIO_ORIGINAL);
            self.hasher
                .update(&data[ratio_offset - self.byte_count + ratio_len..]);
            self.ratio = Some({
                let mut copied = [0, 0, 0];
                copied.copy_from_slice(
                    &data[ratio_offset - self.byte_count
                        ..ratio_offset - self.byte_count + ratio_len],
                );
                copied
            });
        } else {
            self.hasher.update(data);
        }

        self.byte_count += count;
    }

    pub fn finish(self) -> Option<(Box<dyn Patch>, [u8; 3])> {
        if &self.hasher.finalize()[..] == self.patch.sha2() {
            Some((self.patch, self.ratio?))
        } else {
            None
        }
    }
}

impl From<Box<dyn Patch>> for BinaryDetector {
    fn from(from: Box<dyn Patch>) -> Self {
        BinaryDetector {
            patch: from,
            hasher: Sha256::new(),
            ratio: None,
            byte_count: 0,
        }
    }
}
