use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854191: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_191,
        source_type: SourceType::Wikidata,
        name: "ARB Fragment shader",
        extensions: &["arb"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x21, 0x21, 0x41, 0x52, 0x42, 0x66, 0x70, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
