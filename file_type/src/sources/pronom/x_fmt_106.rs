use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_106: FileFormat = FileFormat {
    id: 154,
    puid: "x-fmt/106",
    name: "Microsoft Symbolic Link (SYLK) File",
    extensions: &["slk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x44, 0x3B, 0x50])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Any(&[
                            &[Token::Literal(&[0x0D])],
                            &[Token::Literal(&[0x0D, 0x0A])],
                            &[Token::Literal(&[0x0A])],
                        ]),
                        Token::Literal(&[0x45]),
                        Token::Any(&[
                            &[Token::Literal(&[0x0D])],
                            &[Token::Literal(&[0x0D, 0x0A])],
                            &[Token::Literal(&[0x0A])],
                        ]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[],
};
