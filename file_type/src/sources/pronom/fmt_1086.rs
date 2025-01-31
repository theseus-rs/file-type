use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1086: FileFormat = FileFormat {
    id: 1_894,
    puid: "fmt/1086",
    name: "Monkey's Audio File",
    extensions: &["ape"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x41, 0x43, 0x20]),
                        Token::WildcardCount(48),
                        Token::Any(&[
                            &[Token::Literal(&[0xE8, 0x03])],
                            &[Token::Literal(&[0xD0, 0x07])],
                            &[Token::Literal(&[0xB8, 0x0B])],
                            &[Token::Literal(&[0xA0, 0x0F])],
                            &[Token::Literal(&[0x88, 0x13])],
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
                        Token::Literal(&[0x4D, 0x41, 0x43, 0x20]),
                        Token::WildcardCount(2),
                        Token::Any(&[
                            &[Token::Literal(&[0xE8, 0x03])],
                            &[Token::Literal(&[0xD0, 0x07])],
                            &[Token::Literal(&[0xB8, 0x0B])],
                            &[Token::Literal(&[0xA0, 0x0F])],
                            &[Token::Literal(&[0x88, 0x13])],
                        ]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[],
};
