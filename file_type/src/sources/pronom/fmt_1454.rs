use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1454: FileFormat = FileFormat {
    id: 2_277,
    puid: "fmt/1454",
    name: "Web Video Text Tracks (WebVTT) Format",
    extensions: &["vtt"],
    media_types: &["text/vtt"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x57, 0x45, 0x42, 0x56, 0x54, 0x54]),
                    Token::WildcardCountRange(5, 400),
                    Token::Range(&[0x30], &[0x35]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Literal(&[0x3A]),
                    Token::Range(&[0x30], &[0x35]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Literal(&[0x2E]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Literal(&[0x20, 0x2D, 0x2D, 0x3E, 0x20]),
                    Token::WildcardCountRange(0, 5),
                    Token::Range(&[0x30], &[0x35]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Literal(&[0x3A]),
                    Token::Range(&[0x30], &[0x35]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Literal(&[0x2E]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x39]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
