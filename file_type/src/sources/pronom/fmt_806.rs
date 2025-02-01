use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_806: FileFormat = FileFormat {
    id: 1_606,
    puid: "fmt/806",
    name: "MATLAB Mat File",
    extensions: &["mat", "fig"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x41, 0x54, 0x4C, 0x41, 0x42, 0x20, 0x35, 0x2E, 0x30]),
                    Token::WildcardCount(114),
                    Token::Any(&[
                        &[Token::Literal(&[0x00, 0x01, 0x49, 0x4D])],
                        &[Token::Literal(&[0x01, 0x00, 0x4D, 0x49])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_375,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_629,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_375,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
