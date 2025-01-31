use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1738: FileFormat = FileFormat {
    id: 2_584,
    puid: "fmt/1738",
    name: "UDF Disc Image",
    extensions: &["toast", "iso", "cdr", "dmg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(32_768),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x42, 0x45, 0x41, 0x30, 0x31, 0x01]),
                    Token::WildcardCount(2_041),
                    Token::Literal(&[0x00, 0x4E, 0x53, 0x52, 0x30]),
                    Token::Any(&[&[Token::Literal(&[0x32])], &[Token::Literal(&[0x33])]]),
                    Token::Literal(&[0x01]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_585,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_605,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_585,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
        RelatedFormat {
            id: 1_255,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
