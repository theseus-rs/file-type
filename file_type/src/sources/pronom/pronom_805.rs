use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_805: FileFormat = FileFormat {
    id: 805,
    source_type: SourceType::Pronom,
    name: "Icon file format",
    extensions: &["ico"],
    media_types: &["image/vnd.microsoft.icon", "image/x-icon"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x00, 0x01, 0x00]),
                    Token::Range(&[0x01], &[0x09]),
                    Token::Literal(&[0x00]),
                    Token::WildcardCount(3),
                    Token::Literal(&[0x00]),
                    Token::Range(&[0x00], &[0x01]),
                    Token::Literal(&[0x00]),
                    Token::WildcardCount(1),
                    Token::Literal(&[0x00]),
                    Token::WildcardCount(3),
                    Token::Literal(&[0x00]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x00, 0x00, 0x28, 0x00, 0x00, 0x00]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x00, 0x00]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x00, 0x00, 0x01, 0x00]),
                    Token::Range(&[0x01], &[0x20]),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
