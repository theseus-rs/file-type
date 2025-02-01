use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_164: FileFormat = FileFormat {
    id: 236,
    puid: "x-fmt/164",
    name: "Portable Bitmap Image - ASCII",
    extensions: &["pbm"],
    media_types: &["image/x-portable-bitmap"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x50, 0x31]),
                        Token::Any(&[
                            &[Token::Literal(&[0x20])],
                            &[Token::Literal(&[0x09])],
                            &[Token::Literal(&[0x0D, 0x0A])],
                            &[Token::Literal(&[0x0A])],
                        ]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Any(&[
                            &[Token::Literal(&[0x30])],
                            &[Token::Literal(&[0x31])],
                            &[Token::Literal(&[0x20])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x30])],
                            &[Token::Literal(&[0x31])],
                            &[Token::Literal(&[0x20])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x30])],
                            &[Token::Literal(&[0x31])],
                            &[Token::Literal(&[0x20])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x30])],
                            &[Token::Literal(&[0x31])],
                            &[Token::Literal(&[0x20])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x30])],
                            &[Token::Literal(&[0x31])],
                            &[Token::Literal(&[0x20])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x30])],
                            &[Token::Literal(&[0x31])],
                            &[Token::Literal(&[0x20])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x30])],
                            &[Token::Literal(&[0x31])],
                            &[Token::Literal(&[0x20])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x30])],
                            &[Token::Literal(&[0x31])],
                            &[Token::Literal(&[0x20])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x30])],
                            &[Token::Literal(&[0x31])],
                            &[Token::Literal(&[0x20])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x30])],
                            &[Token::Literal(&[0x31])],
                            &[Token::Literal(&[0x20])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::Literal(&[0x0A]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[],
};
