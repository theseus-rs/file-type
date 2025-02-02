use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_801: FileFormat = FileFormat {
    id: 801,
    source_type: SourceType::Pronom,
    name: "Windows Cabinet File",
    extensions: &["cab"],
    media_types: &["application/vnd.ms-cab-compressed"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x53, 0x43, 0x46]),
                    Token::WildcardCount(20),
                    Token::Literal(&[0x03, 0x01]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 306,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_792,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_305,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_691,
        },
    ],
};
