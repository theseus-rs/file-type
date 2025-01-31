use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1068: FileFormat = FileFormat {
    id: 1_875,
    puid: "fmt/1068",
    name: "Silo",
    extensions: &["silo"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x89, 0x48, 0x44, 0x46])],
                },
            },
            ByteSequence {
                position_type: PositionType::Variable,
                offset: None,
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5F, 0x73, 0x69, 0x6C, 0x6F, 0x6C, 0x69, 0x62, 0x69, 0x6E, 0x66, 0x6F,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_026,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_027,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_607,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
