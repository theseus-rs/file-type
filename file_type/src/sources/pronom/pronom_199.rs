use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_199: FileFormat = FileFormat {
    id: 199,
    source_type: SourceType::Pronom,
    name: "NeXT/Sun sound",
    extensions: &["au"],
    media_types: &["audio/basic"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x2E, 0x73, 0x6E, 0x64, 0x00, 0x00, 0x00]),
                    Token::WildcardCount(5),
                    Token::Literal(&[0x00, 0x00, 0x00]),
                    Token::Any(&[
                        &[Token::Literal(&[0x01])],
                        &[Token::Literal(&[0x02])],
                        &[Token::Literal(&[0x03])],
                        &[Token::Literal(&[0x04])],
                        &[Token::Literal(&[0x05])],
                        &[Token::Literal(&[0x06])],
                        &[Token::Literal(&[0x07])],
                        &[Token::Literal(&[0x17])],
                        &[Token::Literal(&[0x18])],
                        &[Token::Literal(&[0x19])],
                        &[Token::Literal(&[0x1A])],
                        &[Token::Literal(&[0x1B])],
                    ]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x00, 0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
