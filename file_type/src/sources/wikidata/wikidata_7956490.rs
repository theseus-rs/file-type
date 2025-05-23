use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7956490: FileType = FileType {
    file_format: &FileFormat {
        id: 7_956_490,
        source_type: SourceType::Wikidata,
        name: "Windows Recorded TV Show",
        extensions: &["wtv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xB7, 0xD8, 0x00, 0x20, 0x37, 0x49, 0xDA, 0x11, 0xA6, 0x4E, 0x00, 0x07,
                        0xE9, 0x5E, 0xAD, 0x8D, 0x8C, 0xC3, 0xD2, 0xC2, 0x7E, 0x9A, 0xDA, 0x11,
                        0x8B, 0xF7, 0x00, 0x07, 0xE9, 0x5E, 0xAD, 0x8D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
