use super::Patch;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub(super) struct PatchRU_1008;

impl Patch for PatchRU_1008 {
    fn name(&self) -> &'static str {
        "RU 1008 KB"
    }

    fn sha2(&self) -> &'static [u8; 32] {
        &[
            0x7A, 0x3B, 0xF7, 0x73, 0xCB, 0x62, 0xB, 0x8C, 0x54, 0x7E, 0xE0, 0x4B, 0x40, 0xCD,
            0x43, 0xA3, 0x8D, 0x9C, 0x96, 0x9B, 0x92, 0x5E, 0x9B, 0xEC, 0xA5, 0x36, 0x3B, 0x40,
            0x58, 0x8F, 0x93, 0x80,
        ]
    }

    fn ratio_offset(&self) -> usize {
        0x000C7ED8
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub(super) struct PatchEN_992;

impl Patch for PatchEN_992 {
    fn name(&self) -> &'static str {
        "EN 992 KB"
    }

    fn sha2(&self) -> &'static [u8; 32] {
        &[
            0xFC, 0x4F, 0x5E, 0x80, 0xBD, 0x30, 0x20, 0xFE, 0x83, 0x15, 0x35, 0x02, 0x53, 0x36,
            0xCD, 0x61, 0x36, 0x4C, 0x3B, 0x2E, 0xDB, 0x0E, 0xAA, 0xA6, 0x2B, 0xAE, 0x97, 0x24,
            0x27, 0xE3, 0x49, 0xFC,
        ]
    }

    fn ratio_offset(&self) -> usize {
        0x000CAEFC
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub(super) struct Patch_Zoom;

impl Patch for Patch_Zoom {
    fn name(&self) -> &'static str {
        "Zoom Platform 992 KB"
    }

    fn sha2(&self) -> &'static [u8; 32] {
        &[
            0x5E, 0x04, 0xA6, 0x30, 0x4C, 0x4F, 0xBE, 0xBD, 0xE4, 0x66, 0xC3, 0x8C, 0xF4, 0xD5,
            0xBB, 0xE6, 0xF2, 0xD3, 0xFD, 0xC6, 0x21, 0xAF, 0x2F, 0xDA, 0xA3, 0xB9, 0xCD, 0x24,
            0xFE, 0xBE, 0x20, 0x35,
        ]
    }

    fn ratio_offset(&self) -> usize {
        0x000CAEFC
    }
}

// #[cfg(test)]
// mod test {
//     use std::{
//         fs::File,
//         io::{BufReader, Read},
//     };

//     use sha2::{Digest, Sha256};

//     use crate::error_code::ErrorCode;

//     use super::*;

    // #[test]
    // fn test1() {
    //     let file_path = "C:/repository/rust/eracer-config/eracer_nocd.orig.exe";
    //     let file = File::open(file_path).map_err(ErrorCode::IO).unwrap();

    //     let mut hasher = Sha256::new();

    //     let mut buf = vec![];
    //     let mut reader = BufReader::new(file);
    //     let size = reader.read_to_end(buf.as_mut()).unwrap();
    //     hasher.update(&buf[..size]);
    //     let slice = &hasher.finalize();

    //     for i in slice.iter() {
    //         print!("0x{:02X?}, ", i);
    //     }
    //     println!("end.");

    //     assert_eq!(slice.len(), 32);
    //     assert_eq!(&slice[..], PatchEN_992.sha2());
    //     println!("{:X?}", &slice[..]);
    // }
// }
