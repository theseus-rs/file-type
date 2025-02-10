use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862952: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_952,
        source_type: SourceType::Wikidata,
        name: "TROFF markup",
        extensions: &["me"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2E, 0x5C, 0x22, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
