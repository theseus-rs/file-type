use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2738: FileFormat = FileFormat {
    id: 2_738,
    source_type: SourceType::Pronom,
    name: "OPML File",
    extensions: &["opml"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x3C, 0x6F, 0x70, 0x6D, 0x6C]),
                        Token::WildcardCountRange(0, 90),
                        Token::Literal(&[
                            0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x32, 0x2E,
                            0x30, 0x22,
                        ]),
                        Token::WildcardCountRange(0, 60),
                        Token::Literal(&[0x3E]),
                        Token::WildcardCountRange(0, 5),
                        Token::Literal(&[0x3C, 0x68, 0x65, 0x61, 0x64]),
                        Token::WildcardCountRange(0, 69),
                        Token::Literal(&[0x3E]),
                        Token::WildcardCountRange(0, 9),
                        Token::Literal(&[0x3C, 0x74, 0x69, 0x74, 0x6C, 0x65, 0x3E]),
                        Token::WildcardCountRange(0, 300),
                        Token::Literal(&[0x3C, 0x2F, 0x74, 0x69, 0x74, 0x6C, 0x65, 0x3E]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x3C, 0x2F, 0x62, 0x6F, 0x64, 0x79, 0x3E]),
                        Token::WildcardCountRange(0, 4),
                        Token::Literal(&[0x3C, 0x2F, 0x6F, 0x70, 0x6D, 0x6C, 0x3E]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 638,
    }],
};
