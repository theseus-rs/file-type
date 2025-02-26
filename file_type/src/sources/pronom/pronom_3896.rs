use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_3896: FileType = FileType {
    file_format: &FileFormat {
        id: 3_896,
        source_type: SourceType::Pronom,
        name: "askSam Document for Windows",
        extensions: &["ask"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x61, 0x73, 0x6B, 0x77, 0x34, 0x30, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 3_895,
        }],
    },
};
