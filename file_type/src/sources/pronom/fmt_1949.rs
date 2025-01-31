use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1949: FileFormat = FileFormat {
    id: 2_812,
    puid: "fmt/1949",
    name: "Common Data Format dotCDF",
    extensions: &["cdf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xCD, 0xF2, 0x60, 0x02]),
                    Token::Any(&[
                        &[Token::Literal(&[0x00, 0x00, 0xFF, 0xFF])],
                        &[Token::Literal(&[0xCC, 0xCC, 0x00, 0x01])],
                    ]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x01]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_813,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_810,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
