use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1405: FileFormat = FileFormat {
    id: 2_223,
    puid: "fmt/1405",
    name: "Student Writing Center Letter",
    extensions: &["lt", "ltt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
                            &[Token::Literal(&[0x04, 0x00])],
                            &[Token::Literal(&[0x00, 0x04])],
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
