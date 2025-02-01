use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_324: FileFormat = FileFormat {
    id: 1_069,
    puid: "fmt/324",
    name: "EndNote Style File",
    extensions: &["ens"],
    media_types: &["application/x-endnote-style"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x08]),
                        Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0xFF])]]),
                        Token::Any(&[&[Token::Literal(&[0xFF])], &[Token::Literal(&[0x00])]]),
                        Token::Literal(&[0x00, 0x00]),
                        Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x10])]]),
                        Token::Any(&[&[Token::Literal(&[0x10])], &[Token::Literal(&[0x00])]]),
                        Token::Literal(&[0x52, 0x53, 0x46, 0x54, 0x53, 0x54, 0x59, 0x4C]),
                        Token::Any(&[
                            &[Token::Literal(&[0x00, 0x10, 0x01, 0x00])],
                            &[Token::Literal(&[0x10, 0x00, 0x00, 0x01])],
                        ]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x08]),
                        Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0xFF])]]),
                        Token::Any(&[&[Token::Literal(&[0xFF])], &[Token::Literal(&[0x00])]]),
                        Token::Literal(&[0x00, 0x00]),
                        Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x10])]]),
                        Token::Any(&[&[Token::Literal(&[0x10])], &[Token::Literal(&[0x00])]]),
                        Token::Literal(&[0x45, 0x4E, 0x44, 0x4E, 0x45, 0x4E, 0x46, 0x54]),
                        Token::Any(&[
                            &[Token::Literal(&[0x00, 0x10, 0x01, 0x00])],
                            &[Token::Literal(&[0x10, 0x00, 0x00, 0x01])],
                        ]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[],
};
