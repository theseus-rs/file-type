use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856318: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_318,
        source_type: SourceType::Wikidata,
        name: "ATK Data Flow Diagram",
        extensions: &["dfd"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2A, 0x20, 0x44, 0x66, 0x64, 0x20, 0x54, 0x69, 0x74, 0x6C, 0x65, 0x20,
                        0x2A, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
