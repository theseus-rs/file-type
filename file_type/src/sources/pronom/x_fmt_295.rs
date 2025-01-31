use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_295: FileFormat = FileFormat {
    id: 448,
    puid: "x-fmt/295",
    name: "Micrografx Draw",
    extensions: &["drw", "drt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x01, 0xFF, 0x02, 0x04, 0x03, 0x02, 0x00, 0x02]),
                    Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x02])]]),
                    Token::Literal(&[0x02, 0x21, 0x05]),
                    Token::WildcardCount(3),
                    Token::Literal(&[
                        0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73, 0x20, 0x44, 0x72, 0x61, 0x77,
                        0x20, 0x44, 0x72, 0x61, 0x77, 0x69, 0x6E, 0x67,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 78,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 447,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
