use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854434: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_434,
        source_type: SourceType::Wikidata,
        name: "BLINK compressed archive",
        extensions: &["bli"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x6C, 0x69, 0x6E, 0x6B, 0x20, 0x62, 0x79, 0x20, 0x44, 0x2E, 0x54,
                        0x2E, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
