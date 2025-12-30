use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_3898: FileType = FileType {
    file_format: &FileFormat {
        id: 3_898,
        source_type: SourceType::Pronom,
        name: "CD Architect Project File",
        extensions: &["cdp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x49, 0x46, 0x46]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x53, 0x46, 0x50, 0x4A]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 2_741,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 3_899,
            },
        ],
    },
};
