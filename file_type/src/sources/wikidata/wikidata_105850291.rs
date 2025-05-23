use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850291: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_291,
        source_type: SourceType::Wikidata,
        name: "Portfolio BASIC compiled 16 bit MS-DOS binary",
        extensions: &["com"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x32, 0xE4, 0xCD, 0x61, 0xE8, 0xD1, 0x00, 0xBA, 0xFF, 0xFF, 0x75, 0x03,
                        0xE9, 0xE6, 0x00, 0xB8, 0x00, 0x0B, 0xCD, 0x61, 0x72, 0x29, 0xB8, 0x01,
                        0x24, 0xBA, 0x01, 0x01, 0xCD, 0x61, 0x72, 0x1F, 0x52, 0x1E, 0xB4, 0x1C,
                        0xB2, 0x01, 0xCD, 0x21, 0x1F, 0x5A, 0x81, 0xF9, 0x00, 0x01, 0x74, 0x11,
                        0x77, 0x1C, 0xB8, 0x20, 0xC0, 0xB3, 0x08, 0xB9, 0x60, 0xC0, 0xBD, 0x20,
                        0x00, 0xEB, 0x1A, 0xEB, 0x72, 0xB8, 0x30, 0xC0, 0xB3, 0x10, 0xB9, 0xB0,
                        0xC0, 0xBD, 0x40, 0x00, 0xEB, 0x0B, 0xB8, 0x40, 0xC0, 0xB3, 0x20, 0xB9,
                        0x40, 0xC1, 0xBD, 0x80, 0x00, 0x51, 0x1E, 0x0E, 0x1F, 0xE8, 0x0B, 0x00,
                        0x50, 0x42, 0x52, 0x55, 0x4E, 0x20, 0x20, 0x20, 0x52, 0x55, 0x4E, 0x5E,
                        0x8E, 0xC0, 0x33, 0xFF, 0xFC, 0x56, 0x57, 0xB9, 0x0B, 0x00, 0xF3, 0xA6,
                        0x5F, 0x5E, 0x74, 0x0A, 0x83, 0xC7, 0x20, 0x4D, 0x75, 0xEF, 0x1F, 0x59,
                        0xEB, 0x28, 0x1F, 0x59, 0x26, 0x8A, 0x45, 0x1A, 0x2C, 0x02, 0xF6, 0xE3,
                        0x03, 0xC1, 0x8E, 0xC0, 0x2D, 0x10, 0x00, 0x26, 0x8B, 0x1E, 0x03, 0x00,
                        0x33, 0xC9, 0x8E, 0xC1, 0x26, 0x89, 0x1E, 0x88, 0x01, 0x26, 0xA3, 0x8A,
                        0x01, 0xE8, 0x2C, 0x00, 0x74, 0x47, 0xB8, 0x01, 0x24, 0xCD, 0x61, 0xE8,
                        0x0F, 0x00, 0x4E, 0x6F, 0x20, 0x76, 0x61, 0x6C, 0x69, 0x64, 0x20, 0x50,
                        0x42, 0x52, 0x55, 0x4E, 0x24, 0x33, 0xC0, 0x8E, 0xD8, 0xA3, 0x8A, 0x01,
                        0x0E, 0x1F, 0x5A, 0xB4, 0x09, 0xCD, 0x21, 0xB8, 0x00, 0x4C, 0xCD, 0x21,
                        0x1E, 0x33, 0xC0, 0x8E, 0xD8, 0xC5, 0x1E, 0x88, 0x01, 0x81, 0x7F, 0xFA,
                        0x7A, 0x62, 0x75, 0x0B, 0x81, 0x7F, 0xFC, 0x00, 0x01, 0x75, 0x04, 0x83,
                        0x7F, 0xFE, 0x03, 0x1F, 0xC3, 0xCD, 0x62, 0x2F, 0x72, 0x7A, 0x03, 0x00,
                        0x00, 0x01, 0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x1A, 0x02,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
