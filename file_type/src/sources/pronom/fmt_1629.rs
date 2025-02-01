use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1629: FileFormat = FileFormat {
    id: 2_456,
    puid: "fmt/1629",
    name: "Adobe InDesign Document",
    extensions: &["indd", "ind", "indt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(92),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x44, 0x4F, 0x43, 0x55, 0x4D, 0x45, 0x4E, 0x54]),
                    Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x02])]]),
                    Token::WildcardCount(4),
                    Token::Any(&[
                        &[Token::Literal(&[0x01, 0x00, 0x00, 0x00])],
                        &[Token::Literal(&[0x00, 0x00, 0x00, 0x01])],
                    ]),
                    Token::Literal(&[0x05]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_455,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_457,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_455,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
