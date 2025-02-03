use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1698: FileFormat = FileFormat {
    id: 1_698,
    source_type: SourceType::Pronom,
    name: "Gaussian Input Data File",
    extensions: &["gjf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(1),
                regex: Regex {
                    tokens: &[
                        Token::Any(&[
                            &[Token::Literal(&[0x41, 0x4D, 0x31])],
                            &[Token::Literal(&[0x50, 0x4D, 0x33])],
                            &[Token::Literal(&[0x53, 0x54, 0x4F, 0x2D, 0x33, 0x47])],
                            &[Token::Literal(&[0x33, 0x2D, 0x32, 0x31, 0x47])],
                            &[Token::Literal(&[0x33, 0x2D, 0x32, 0x31, 0x67])],
                            &[Token::Literal(&[0x36, 0x2D, 0x33, 0x31, 0x47])],
                            &[Token::Literal(&[0x36, 0x2D, 0x33, 0x31, 0x67])],
                            &[Token::Literal(&[
                                0x61, 0x75, 0x67, 0x2D, 0x63, 0x63, 0x2D, 0x70, 0x76, 0x74, 0x7A,
                            ])],
                        ]),
                        Token::WildcardCountRange(0, 128),
                        Token::Literal(&[0x0A]),
                        Token::Any(&[
                            &[Token::Literal(&[0x2D, 0x31])],
                            &[Token::Literal(&[0x30])],
                            &[Token::Literal(&[0x31])],
                        ]),
                        Token::Any(&[&[Token::Literal(&[0x20])], &[Token::Literal(&[0x2C])]]),
                        Token::Range(&[0x31], &[0x39]),
                        Token::WildcardCountRange(1, 128),
                        Token::Range(&[0x30], &[0x39]),
                        Token::Literal(&[0x2E]),
                        Token::Range(&[0x30], &[0x39]),
                        Token::WildcardCountRange(0, 24),
                        Token::Range(&[0x30], &[0x39]),
                        Token::Literal(&[0x2E]),
                        Token::Range(&[0x30], &[0x39]),
                        Token::WildcardCountRange(0, 24),
                        Token::Range(&[0x30], &[0x39]),
                        Token::Literal(&[0x2E]),
                        Token::Range(&[0x30], &[0x39]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23])],
                },
            },
        ],
    }],
    related_formats: &[],
};
