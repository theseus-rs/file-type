use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2723: FileFormat = FileFormat {
    id: 2_723,
    source_type: SourceType::Pronom,
    name: "SPSS PC File Format",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x64, 0x61, 0x74, 0x61, 0x20, 0x6C, 0x69, 0x73, 0x74, 0x20, 0x66, 0x72,
                            0x65, 0x65, 0x20, 0x2F,
                        ]),
                        Token::WildcardCountRange(1, 45),
                        Token::Literal(&[
                            0x76, 0x61, 0x72, 0x69, 0x61, 0x62, 0x6C, 0x65, 0x20, 0x6C, 0x61, 0x62,
                            0x65, 0x6C, 0x73, 0x20, 0x2F,
                        ]),
                        Token::WildcardCountRange(1, 200),
                        Token::Literal(&[
                            0x62, 0x65, 0x67, 0x69, 0x6E, 0x20, 0x64, 0x61, 0x74, 0x61,
                        ]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(5),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x65, 0x6E, 0x64, 0x20, 0x64, 0x61, 0x74, 0x61,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 1_438,
    }],
};
