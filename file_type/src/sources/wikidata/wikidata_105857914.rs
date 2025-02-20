use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857914: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_914,
        source_type: SourceType::Wikidata,
        name: "Commodore CP/M disk image (CBM)",
        extensions: &["d64"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x42, 0x4D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x78, 0x20, 0x84,
                        0xFF, 0xA9, 0x3E, 0x8D, 0x00, 0xFF, 0xA9, 0xC3, 0x8D, 0xEE, 0xFF, 0xA9,
                        0x08, 0x8D, 0xEF, 0xFF, 0xA9, 0x00, 0x8D, 0xF0, 0xFF, 0x4C, 0xD0, 0xFF,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
