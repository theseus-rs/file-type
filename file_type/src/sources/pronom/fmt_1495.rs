use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1495: FileFormat = FileFormat {
    id: 2_318,
    puid: "fmt/1495",
    name: "QuarkXPress Project",
    extensions: &["qxp", "qwd", "qpt"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(2),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x4D, 0x58, 0x50, 0x52]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x51]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(2),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x49, 0x49, 0x58, 0x50, 0x52]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x51]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[
        RelatedFormat {
            id: 255,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_881,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
    ],
};
