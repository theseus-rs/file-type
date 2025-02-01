use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_540: FileFormat = FileFormat {
    id: 1_327,
    puid: "fmt/540",
    name: "Cinema 4D",
    extensions: &["c4d"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x43, 0x35, 0x30]),
                    Token::WildcardCount(4),
                    Token::Any(&[
                        &[Token::Literal(&[0x50, 0x52, 0x46, 0x35])],
                        &[Token::Literal(&[0x44, 0x4F, 0x4B, 0x35])],
                        &[Token::Literal(&[0x43, 0x41, 0x54, 0x35])],
                        &[Token::Literal(&[0x46, 0x43, 0x56, 0x35])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_990,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_194,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
