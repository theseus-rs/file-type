use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1218: FileFormat = FileFormat {
    id: 2_028,
    puid: "fmt/1218",
    name: "SubRip Subtitle File",
    extensions: &["srt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Literal(&[0x3A]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Literal(&[0x3A]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Literal(&[0x2C]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Literal(&[0x20, 0x2D, 0x2D, 0x3E, 0x20]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Literal(&[0x3A]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Literal(&[0x3A]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Literal(&[0x2C]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x39]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
