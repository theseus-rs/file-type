use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2369: FileFormat = FileFormat {
    id: 2_369,
    source_type: SourceType::Pronom,
    name: "ELAN Preference File",
    extensions: &["pfsx"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(27),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x3C, 0x70, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6E, 0x63, 0x65, 0x73,
                        ]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x78, 0x73, 0x69, 0x3A, 0x6E, 0x6F, 0x4E, 0x61, 0x6D, 0x65, 0x73, 0x70,
                            0x61, 0x63, 0x65, 0x53, 0x63, 0x68, 0x65, 0x6D, 0x61, 0x4C, 0x6F, 0x63,
                            0x61, 0x74, 0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x68, 0x74, 0x74, 0x70, 0x3A,
                            0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E, 0x6D, 0x70, 0x69, 0x2E, 0x6E, 0x6C,
                            0x2F, 0x74, 0x6F, 0x6F, 0x6C, 0x73, 0x2F, 0x65, 0x6C, 0x61, 0x6E, 0x2F,
                            0x50, 0x72, 0x65, 0x66, 0x73, 0x5F, 0x76,
                        ]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x2F, 0x70, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6E, 0x63, 0x65,
                        0x73, 0x3E, 0x0A,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 638,
    }],
};
