use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1984: FileFormat = FileFormat {
    id: 2_856,
    puid: "fmt/1984",
    name: "Sibelius Score",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x0F, 0x53, 0x49, 0x42, 0x45, 0x4C, 0x49, 0x55, 0x53]),
                    Token::WildcardCount(1),
                    Token::Literal(&[0x00, 0x39]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_495,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_857,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_855,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 1_495,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
