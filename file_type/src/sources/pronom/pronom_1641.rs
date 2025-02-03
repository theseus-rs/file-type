use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1641: FileFormat = FileFormat {
    id: 1_641,
    source_type: SourceType::Pronom,
    name: "ADX Audio Format",
    extensions: &["adx"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x80, 0x00]),
                        Token::WildcardCount(16),
                        Token::Literal(&[0x03, 0x00]),
                        Token::WildcardCountRange(0, 65_535),
                        Token::Literal(&[0x28, 0x63, 0x29, 0x43, 0x52, 0x49]),
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
                        Token::Literal(&[0x80, 0x00]),
                        Token::WildcardCount(16),
                        Token::Literal(&[0x04, 0x00]),
                        Token::WildcardCountRange(0, 65_535),
                        Token::Literal(&[0x28, 0x63, 0x29, 0x43, 0x52, 0x49]),
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
                        Token::Literal(&[0x80, 0x00]),
                        Token::WildcardCount(16),
                        Token::Literal(&[0x05, 0x00]),
                        Token::WildcardCountRange(0, 65_535),
                        Token::Literal(&[0x28, 0x63, 0x29, 0x43, 0x52, 0x49]),
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
                        Token::Literal(&[0x80, 0x00]),
                        Token::WildcardCount(16),
                        Token::Literal(&[0x04, 0x08]),
                        Token::WildcardCountRange(0, 65_535),
                        Token::Literal(&[0x28, 0x63, 0x29, 0x43, 0x52, 0x49]),
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
                        Token::Literal(&[0x80, 0x00]),
                        Token::WildcardCount(16),
                        Token::Literal(&[0x04, 0x09]),
                        Token::WildcardCountRange(0, 65_535),
                        Token::Literal(&[0x28, 0x63, 0x29, 0x43, 0x52, 0x49]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[],
};
