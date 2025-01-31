use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_395: FileFormat = FileFormat {
    id: 1_143,
    puid: "fmt/395",
    name: "vCard",
    extensions: &["vcf", "vcard"],
    media_types: &["text/vcard"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x42, 0x45, 0x47, 0x49, 0x4E, 0x3A, 0x56, 0x43, 0x41, 0x52, 0x44,
                        ]),
                        Token::WildcardCountRange(1, 3),
                        Token::Literal(&[0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x3A]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x4E, 0x44, 0x3A, 0x56, 0x43, 0x41, 0x52, 0x44,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_733,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_734,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_735,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
    ],
};
