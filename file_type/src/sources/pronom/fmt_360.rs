use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_360: FileFormat = FileFormat {
    id: 1_107,
    puid: "fmt/360",
    name: "pulse EKKO data file",
    extensions: &["dt1"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x00, 0x80, 0x3F]),
                    Token::WildcardCount(12),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x00]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x00, 0x00]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x00, 0x00]),
                    Token::WildcardCount(1),
                    Token::Range(&[0x41], &[0x42]),
                    Token::WildcardCount(8),
                    Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    ]),
                    Token::WildcardCount(28),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x00]),
                    Token::WildcardCount(3),
                    Token::Range(&[0x40], &[0x49]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
