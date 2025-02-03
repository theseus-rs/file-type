use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_403: FileFormat = FileFormat {
    id: 403,
    source_type: SourceType::Pronom,
    name: "dBASE Database",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x83]),
                    Token::SingleWildcard,
                    Token::Range(&[0x01], &[0x0C]),
                    Token::Range(&[0x01], &[0x1F]),
                    Token::WildcardCount(28),
                    Token::Any(&[
                        &[Token::Range(&[0x41], &[0x5A])],
                        &[Token::Range(&[0x61], &[0x7A])],
                    ]),
                    Token::WildcardCount(10),
                    Token::Any(&[
                        &[Token::Literal(&[0x43])],
                        &[Token::Literal(&[0x44])],
                        &[Token::Literal(&[0x4C])],
                        &[Token::Literal(&[0x4D])],
                        &[Token::Literal(&[0x4E])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
