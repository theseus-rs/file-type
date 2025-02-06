use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2222: FileFormat = FileFormat {
    id: 2_222,
    source_type: SourceType::Pronom,
    name: "Student Writing Center Newsletter",
    extensions: &["nl", "nlt"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x1A, 0x54, 0x4C, 0x43]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x46, 0x46, 0x00]),
                        Token::Any(&[
                            &[Token::Literal(&[0x03, 0x00])],
                            &[Token::Literal(&[0x00, 0x03])],
                        ]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1A, 0x46, 0x46, 0x1A])],
                },
            },
        ],
    }],
    related_formats: &[],
};
