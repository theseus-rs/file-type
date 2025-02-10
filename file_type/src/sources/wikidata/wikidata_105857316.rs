use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857316: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_316,
        source_type: SourceType::Wikidata,
        name: "MADS HAG game data archive",
        extensions: &["hag"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x41, 0x44, 0x53, 0x43, 0x4F, 0x4E, 0x43, 0x41, 0x54, 0x20, 0x31,
                        0x2E, 0x30, 0x1A, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
