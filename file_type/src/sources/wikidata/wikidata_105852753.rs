use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852753: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_753,
        source_type: SourceType::Wikidata,
        name: "Kawai music score",
        extensions: &["sdf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4B, 0x41, 0x57, 0x41, 0x49, 0x20, 0x4D, 0x55, 0x53, 0x49, 0x43, 0x41,
                        0x4C, 0x20, 0x49, 0x4E, 0x53, 0x54, 0x2E, 0x20, 0x4D, 0x46, 0x47, 0x2E,
                        0x20, 0x43, 0x4F, 0x2E, 0x2C, 0x20, 0x4C, 0x54, 0x44, 0x2E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
