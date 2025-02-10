use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1380: FileType = FileType {
    file_format: &FileFormat {
        id: 1_380,
        source_type: SourceType::Pronom,
        name: "Redcode RAW (R3D) Media File",
        extensions: &["r3d"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x45, 0x44, 0x31]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x52, 0x31]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_843,
        }],
    },
};
