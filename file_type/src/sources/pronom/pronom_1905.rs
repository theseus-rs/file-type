use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1905: FileFormat = FileFormat {
    id: 1_905,
    source_type: SourceType::Pronom,
    name: "ZPAQ Archive Format",
    extensions: &["zpaq"],
    media_types: &[],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x37, 0x6B, 0x53, 0x74]),
                        Token::WildcardCount(9),
                        Token::Literal(&[0x7A, 0x50, 0x51]),
                        Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x02])]]),
                    ],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x7A, 0x50, 0x51]),
                        Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x02])]]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[],
};
