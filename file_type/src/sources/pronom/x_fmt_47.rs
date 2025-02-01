use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_47: FileFormat = FileFormat {
    id: 78,
    puid: "x-fmt/47",
    name: "Micrografx Draw",
    extensions: &["drw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x01, 0xFF, 0x02, 0x04, 0x03]),
                    Token::Any(&[
                        &[Token::Literal(&[0x00])],
                        &[Token::Literal(&[0x01])],
                        &[Token::Literal(&[0x02])],
                    ]),
                    Token::Literal(&[0x00, 0x02]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 447,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 448,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
    ],
};
