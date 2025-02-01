use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1987: FileFormat = FileFormat {
    id: 2_860,
    puid: "fmt/1987",
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
                    Token::Literal(&[0x00, 0x3F, 0x00]),
                    Token::Range(&[0x00], &[0x0A]),
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
            id: 2_861,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_859,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 1_495,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
