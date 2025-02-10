use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854724: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_724,
        source_type: SourceType::Wikidata,
        name: "AMOS source (v1.34)",
        extensions: &["amos"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4D, 0x4F, 0x53, 0x20, 0x42, 0x61, 0x73, 0x69, 0x63, 0x20, 0x76,
                        0x31, 0x2E, 0x33, 0x34,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
