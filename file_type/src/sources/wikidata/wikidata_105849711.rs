use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849711: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_711,
        source_type: SourceType::Wikidata,
        name: "Circuit Diagram Component (compiled)",
        extensions: &["cdcom"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xB8, 0x43, 0x44, 0x43, 0x4F, 0x4D, 0x0D, 0x0A, 0x01, 0xD4, 0x1D, 0x8C,
                        0xD9, 0x8F, 0x00, 0xB2, 0x04, 0xE9, 0x80, 0x09, 0x98, 0xEC, 0xF8, 0x42,
                        0x7E, 0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
