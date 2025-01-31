use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_158: FileFormat = FileFormat {
    id: 222,
    puid: "x-fmt/158",
    name: "Initial Graphics Exchange Specification (IGES)",
    extensions: &["iges", "igs"],
    media_types: &["model/iges"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(72),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x53]),
                    Token::Any(&[
                        &[Token::Literal(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20])],
                        &[Token::Literal(&[0x30, 0x30, 0x30, 0x30, 0x30, 0x30])],
                    ]),
                    Token::Literal(&[0x31]),
                    Token::Any(&[
                        &[Token::Literal(&[0x0D, 0x0A])],
                        &[Token::Literal(&[0x0D])],
                        &[Token::Literal(&[0x0A])],
                    ]),
                    Token::WildcardCount(72),
                    Token::Any(&[
                        &[Token::Literal(&[
                            0x53, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x32,
                        ])],
                        &[Token::Literal(&[
                            0x53, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x32,
                        ])],
                        &[Token::Literal(&[
                            0x47, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x31,
                        ])],
                        &[Token::Literal(&[
                            0x47, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x31,
                        ])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
