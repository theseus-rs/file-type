use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_376: FileFormat = FileFormat {
    id: 1_123,
    puid: "fmt/376",
    name: "FoxPro Report",
    extensions: &["frx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xF5]),
                    Token::SingleWildcard,
                    Token::Range(&[0x01], &[0x0C]),
                    Token::Range(&[0x01], &[0x1F]),
                    Token::WildcardCount(1),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x61, 0x09, 0x1D]),
                    Token::WildcardCount(21),
                    Token::Literal(&[
                        0x50, 0x4C, 0x41, 0x54, 0x46, 0x4F, 0x52, 0x4D, 0x00, 0x00, 0x00, 0x43,
                    ]),
                    Token::WildcardCount(20),
                    Token::Literal(&[
                        0x55, 0x4E, 0x49, 0x51, 0x55, 0x45, 0x49, 0x44, 0x00, 0x00, 0x00, 0x43,
                    ]),
                    Token::WildcardCount(52),
                    Token::Literal(&[
                        0x4F, 0x42, 0x4A, 0x54, 0x59, 0x50, 0x45, 0x00, 0x00, 0x00, 0x00, 0x4E,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_120,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
