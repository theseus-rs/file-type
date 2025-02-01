use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_450: FileFormat = FileFormat {
    id: 1_237,
    puid: "fmt/450",
    name: "VectorWorks",
    extensions: &["vwx"],
    media_types: &["application/vnd.vectorworks"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x00, 0x00, 0x08, 0x58, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94,
                            0x00, 0x20, 0x56, 0x57, 0x31, 0x32, 0x2E, 0x35,
                        ]),
                        Token::WildcardCountRange(0, 3),
                        Token::Literal(&[0x56, 0x57, 0x31, 0x32, 0x2E, 0x35]),
                        Token::WildcardCountRange(0, 29),
                        Token::Literal(&[
                            0x56, 0x65, 0x63, 0x74, 0x6F, 0x72, 0x57, 0x6F, 0x72, 0x6B, 0x73,
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
                        Token::Literal(&[
                            0x00, 0x00, 0x07, 0xF2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x54,
                            0x00, 0x20, 0x56, 0x57, 0x31, 0x32, 0x2E, 0x35,
                        ]),
                        Token::WildcardCountRange(0, 3),
                        Token::Literal(&[0x56, 0x57, 0x31, 0x32, 0x2E, 0x35]),
                        Token::WildcardCountRange(0, 29),
                        Token::Literal(&[
                            0x56, 0x65, 0x63, 0x74, 0x6F, 0x72, 0x57, 0x6F, 0x72, 0x6B, 0x73,
                        ]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[],
};
