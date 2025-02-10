use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855642: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_642,
        source_type: SourceType::Wikidata,
        name: "Office Profile-Settings (v10.0)",
        extensions: &["ops"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x44, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x4F, 0x00, 0x50, 0x00, 0x53,
                        0x00, 0x20, 0x00, 0x48, 0x00, 0x65, 0x00, 0x61, 0x00, 0x64, 0x00, 0x65,
                        0x00, 0x72, 0x00, 0x20, 0x00, 0x56, 0x00, 0x65, 0x00, 0x72, 0x00, 0x73,
                        0x00, 0x69, 0x00, 0x6F, 0x00, 0x6E, 0x00, 0x20, 0x00, 0x31, 0x00, 0x30,
                        0x00, 0x2E, 0x00, 0x30, 0x00, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
