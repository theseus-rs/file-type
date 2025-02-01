use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_351: FileFormat = FileFormat {
    id: 1_096,
    puid: "fmt/351",
    name: "Paradox Database Table",
    extensions: &["db"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(2),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x08]),
                    Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x02])]]),
                    Token::Range(&[0x01], &[0x04]),
                    Token::WildcardCount(51),
                    Token::Range(&[0x05], &[0x09]),
                    Token::WildcardCount(34),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_097,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_095,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
