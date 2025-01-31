use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1945: FileFormat = FileFormat {
    id: 2_807,
    puid: "fmt/1945",
    name: "Common Loudspeaker Format (CLF)",
    extensions: &["cf2"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x41, 0xBD, 0x0A, 0x00, 0x01]),
                    Token::WildcardCount(15),
                    Token::Literal(&[0x76]),
                    Token::Any(&[&[Token::Literal(&[0x31])], &[Token::Literal(&[0x32])]]),
                    Token::Literal(&[0x2E, 0x30]),
                    Token::WildcardCount(12),
                    Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_744,
            relationship_type: RelationshipType::EquivalentTo,
        },
        RelatedFormat {
            id: 2_805,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
