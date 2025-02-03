use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1327: FileFormat = FileFormat {
    id: 1_327,
    source_type: SourceType::Pronom,
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
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_990,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_194,
        },
    ],
};
