use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860491: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_491,
        source_type: SourceType::Wikidata,
        name: "Real-DRAW Project",
        extensions: &["rdw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x08, 0x4F, 0x53, 0x43, 0x41, 0x52, 0x44, 0x41,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
