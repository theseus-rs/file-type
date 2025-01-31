use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_371: FileFormat = FileFormat {
    id: 1_118,
    puid: "fmt/371",
    name: "Enhanced Compression Wavelet",
    extensions: &["ecw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x65, 0x02, 0x01, 0x02]),
                    Token::WildcardCount(1),
                    Token::Literal(&[0x04, 0x00]),
                    Token::Any(&[
                        &[Token::Literal(&[0x00])],
                        &[Token::Literal(&[0x01])],
                        &[Token::Literal(&[0x02])],
                        &[Token::Literal(&[0x06])],
                    ]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x00]),
                    Token::Any(&[
                        &[Token::Literal(&[0x00])],
                        &[Token::Literal(&[0x02])],
                        &[Token::Literal(&[0x03])],
                        &[Token::Literal(&[0x04])],
                        &[Token::Literal(&[0x06])],
                    ]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x00, 0x03, 0x00, 0x01]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
