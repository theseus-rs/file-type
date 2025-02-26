use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_3899: FileType = FileType {
    file_format: &FileFormat {
        id: 3_899,
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
                        Token::Literal(&[0x72, 0x69, 0x66, 0x66]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0xA5, 0xD6, 0x28, 0xDB]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_756,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 3_898,
            },
        ],
    },
};
